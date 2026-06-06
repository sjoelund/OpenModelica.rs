// Manually written file.
//
// Rust port of `OMCompiler/Compiler/FrontEnd/ParserExt.mo`'s
// `external "C"` declarations.  The MetaModelica module is a thin shim
// over the C entry points defined in `OMCompiler/Parser/Parser_omc.c`
// (which in turn drive the ANTLR3 grammar at `grammars/Modelica.g`).
//
// Here we forward to the winnow-based parser already living in the
// same crate at `crate::parser`, so callers like `Parser.mo` /
// `openmodelica_frontend::Parser` keep working without going through
// any C runtime.
//
// Grammar selection (`acceptedGram`) follows the integer encoding used
// by `Flags.GRAMMAR` (see `OMCompiler/Compiler/Util/Flags.mo:154-158`):
//
//   1 = Modelica       → `Grammar::Modelica2` if `languageStandardInt < 30`
//                        otherwise `Grammar::Modelica3`
//   2 = MetaModelica   → `Grammar::MetaModelica`
//   3 = ParModelica    → `Grammar::MetaModelica`     (parmodelica keywords are
//                        lexed by the MetaModelica lexer in mmwinnow)
//   4 = Optimica       → `Grammar::Optimica`
//   5 = PDEModelica    → `Grammar::Modelica3`        (no dedicated grammar yet)
//
// The interactive entry points (`parseexp`, `parsestringexp`, `stringPath`,
// `stringCref`, `stringMod`, `stringEq`) forward to the corresponding
// per-construct parser entry points (`parser::parse_statements` etc.),
// mirroring how `parse.c` selects an ANTLR entry rule from the `PARSE_*`
// flags.

#![allow(non_snake_case)]

use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use arcstr::ArcStr;

use crate::Absyn;
use crate::GlobalScript;
use crate::parser::{self, Grammar};

/// Map `(acceptedGram, languageStandardInt)` to the parser's [`Grammar`]
/// enum. Mirrors the `set_grammar_flag` switch in
/// `OMCompiler/Parser/Parser_omc.c`.
fn select_grammar(acceptedGram: i32, languageStandardInt: i32) -> Grammar {
    match acceptedGram {
        2 | 3 => Grammar::MetaModelica,
        4 => Grammar::Optimica,
        // 1 = Modelica, 5 = PDEModelica, and anything unknown falls back
        // to the Modelica grammar.  The language-standard integer follows
        // `Flags.LANGUAGE_STANDARD`: values 10/20 are Modelica 1.x / 2.x,
        // 30+ are Modelica 3.x.
        _ => {
            if languageStandardInt < 30 {
                Grammar::Modelica2
            } else {
                Grammar::Modelica3
            }
        }
    }
}

/// Forward the syntax diagnostics recorded by the most recent parser
/// invocation to the Error subsystem, the way the C parser's
/// `displayRecognitionError` calls `c_add_source_message` (Parser/parse.c).
/// Must run after every entry-point call, success or failure: a successful
/// parse can still record warnings (e.g. the `der(cr) :=` compatibility
/// warning).
fn report_syntax_messages(info_filename: &str) {
    use openmodelica_util::ErrorTypes::{MessageType, Severity};
    for m in parser::take_syntax_messages() {
        openmodelica_util::ErrorExt::addSourceMessage(
            // Error id used by the C parser for every syntax diagnostic
            // (the literal `2` in its c_add_source_message calls).
            2,
            MessageType::SYNTAX,
            match m.severity {
                parser::SyntaxSeverity::Error => Severity::ERROR,
                parser::SyntaxSeverity::Warning => Severity::WARNING,
            },
            m.line1 as i32,
            m.col1 as i32,
            m.line2 as i32,
            m.col2 as i32,
            false,
            ArcStr::from(info_filename),
            ArcStr::from(m.message),
            metamodelica::nil(),
        );
    }
}

/// Wrap [`parser::parse`]'s `Box<dyn Error>` into an `anyhow::Error` so
/// the MetaModelica-facing signatures (which return `anyhow::Result`)
/// can use `?` directly. `filename` is the real path stored into SOURCEINFO;
/// `info_filename` (the possibly testsuite-friendly name) is only used to
/// display syntax errors — same split as the C parser's `filename_C` vs
/// `filename_C_testsuiteFriendly` (Parser/parse.c).
fn run_parse(src: &str, filename: &str, info_filename: &str, grammar: Grammar, readonly: bool) -> Result<Absyn::Program> {
    let result = parser::parse(src, filename, info_filename, grammar, readonly).map_err(|e| anyhow!(e.to_string()));
    report_syntax_messages(info_filename);
    result
}

pub fn parse(
    filename: ArcStr,
    infoFilename: ArcStr,
    acceptedGram: i32,
    encoding: ArcStr,
    languageStandardInt: i32,
    _strict: bool,
    _runningTestsuite: bool,
    _libraryPath: ArcStr,
    _lveInstance: Option<i32>,
) -> Result<Absyn::Program> {
    // The Rust parser operates on UTF-8 `&str` directly.  Anything else
    // would need transcoding via the `encoding_rs` crate; bail explicitly
    // rather than silently misinterpreting the bytes.
    if !encoding.is_empty() && !encoding.eq_ignore_ascii_case("UTF-8") && !encoding.eq_ignore_ascii_case("UTF8") {
        return Err(anyhow!(
            "ParserExt::parse: only UTF-8 input is supported, got encoding {:?}",
            encoding.as_str()
        ));
    }
    let src = std::fs::read_to_string(filename.as_str())
        .with_context(|| format!("ParserExt::parse: cannot read {filename}"))?;
    let grammar = select_grammar(acceptedGram, languageStandardInt);
    // Like parseFile in Parser/parse.c: classes parsed from a file the user
    // cannot write to are flagged read-only in their SOURCEINFO, so the
    // interactive API refuses to modify them.
    let readonly = !openmodelica_util::System::regularFileWritable(filename.clone());
    run_parse(&src, filename.as_str(), infoFilename.as_str(), grammar, readonly)
}

pub fn parsestring(
    r#str: ArcStr,
    infoFilename: ArcStr,
    acceptedGram: i32,
    languageStandardInt: i32,
    _strict: bool,
    _runningTestsuite: bool,
) -> Result<Absyn::Program> {
    let grammar = select_grammar(acceptedGram, languageStandardInt);
    // String input has no on-disk path; the interactive name serves as both
    // the SOURCEINFO and the error-display name (like the C `parseString`).
    run_parse(r#str.as_str(), infoFilename.as_str(), infoFilename.as_str(), grammar, /*readonly=*/false)
}

// ---------------------------------------------------------------------
// Interactive-mode entry points: parse a .mos script / statement
// sequence, or a single path / cref / modification / equation.  Each
// maps to one ANTLR entry rule selected by `parse.c`'s `PARSE_*` flags;
// the Rust parser exposes them as dedicated `parse_*` functions.
// ---------------------------------------------------------------------

pub fn parseexp(
    filename: ArcStr,
    infoFilename: ArcStr,
    acceptedGram: i32,
    languageStandardInt: i32,
    _runningTestsuite: bool,
) -> Result<GlobalScript::Statements> {
    let src = std::fs::read_to_string(filename.as_str())
        .with_context(|| format!("ParserExt::parseexp: cannot read {filename}"))?;
    let grammar = select_grammar(acceptedGram, languageStandardInt);
    let readonly = !openmodelica_util::System::regularFileWritable(filename.clone());
    let result = parser::parse_statements(&src, filename.as_str(), infoFilename.as_str(), grammar, readonly).map_err(|e| anyhow!(e.to_string()));
    report_syntax_messages(infoFilename.as_str());
    result
}

pub fn parsestringexp(
    r#str: ArcStr,
    infoFilename: ArcStr,
    acceptedGram: i32,
    languageStandardInt: i32,
    _runningTestsuite: bool,
) -> Result<GlobalScript::Statements> {
    let grammar = select_grammar(acceptedGram, languageStandardInt);
    let result = parser::parse_statements(r#str.as_str(), infoFilename.as_str(), infoFilename.as_str(), grammar, /*readonly=*/false).map_err(|e| anyhow!(e.to_string()));
    report_syntax_messages(infoFilename.as_str());
    result
}

pub fn stringPath(
    r#str: ArcStr,
    infoFilename: ArcStr,
    acceptedGram: i32,
    languageStandardInt: i32,
    _runningTestsuite: bool,
) -> Result<Arc<Absyn::Path>> {
    let grammar = select_grammar(acceptedGram, languageStandardInt);
    let result = parser::parse_path(r#str.as_str(), infoFilename.as_str(), grammar)
        .map(Arc::new)
        .map_err(|e| anyhow!(e.to_string()));
    report_syntax_messages(infoFilename.as_str());
    result
}

pub fn stringCref(
    r#str: ArcStr,
    infoFilename: ArcStr,
    acceptedGram: i32,
    languageStandardInt: i32,
    _runningTestsuite: bool,
) -> Result<Arc<Absyn::ComponentRef>> {
    let grammar = select_grammar(acceptedGram, languageStandardInt);
    let result = parser::parse_cref(r#str.as_str(), infoFilename.as_str(), grammar)
        .map(Arc::new)
        .map_err(|e| anyhow!(e.to_string()));
    report_syntax_messages(infoFilename.as_str());
    result
}

pub fn stringMod(
    r#str: ArcStr,
    infoFilename: ArcStr,
    acceptedGram: i32,
    languageStandardInt: i32,
    _runningTestsuite: bool,
) -> Result<Arc<Absyn::ElementArg>> {
    let grammar = select_grammar(acceptedGram, languageStandardInt);
    let result = parser::parse_modification(r#str.as_str(), infoFilename.as_str(), grammar)
        .map(Arc::new)
        .map_err(|e| anyhow!(e.to_string()));
    report_syntax_messages(infoFilename.as_str());
    result
}

pub fn stringEq(
    r#str: ArcStr,
    infoFilename: ArcStr,
    acceptedGram: i32,
    languageStandardInt: i32,
    _runningTestsuite: bool,
) -> Result<Arc<Absyn::EquationItem>> {
    let grammar = select_grammar(acceptedGram, languageStandardInt);
    let result = parser::parse_equation(r#str.as_str(), infoFilename.as_str(), grammar)
        .map(Arc::new)
        .map_err(|e| anyhow!(e.to_string()));
    report_syntax_messages(infoFilename.as_str());
    result
}

// ---------------------------------------------------------------------
// Library Vendor Executable (LVE) hooks.  These wrap a proprietary
// shared library used by some commercial libraries to validate license
// tokens; OpenModelica's open-source builds disable the feature by
// returning "not started".  Mirror that behaviour here so unrelated
// flows still type-check without dragging in dlopen plumbing.
// ---------------------------------------------------------------------

pub fn startLibraryVendorExecutable(_lvePath: ArcStr) -> (bool, Option<i32>) {
    (false, None)
}

pub fn checkLVEToolLicense(_lveInstance: Option<i32>, _packageName: ArcStr) -> bool {
    false
}

pub fn checkLVEToolFeature(_lveInstance: Option<i32>, _feature: ArcStr) -> bool {
    false
}

pub fn stopLibraryVendorExecutable(_lveInstance: Option<i32>) -> () {
    ()
}
