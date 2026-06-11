// Auto-generated from MetaModelica source
/*
 * This file is part of OpenModelica.
 *
 * Copyright (c) 1998-2026, Open Source Modelica Consortium (OSMC),
 * c/o Linköpings universitet, Department of Computer and Information Science,
 * SE-58183 Linköping, Sweden.
 *
 * All rights reserved.
 *
 * THIS PROGRAM IS PROVIDED UNDER THE TERMS OF AGPL VERSION 3 LICENSE OR
 * THIS OSMC PUBLIC LICENSE (OSMC-PL) VERSION 1.8.
 * ANY USE, REPRODUCTION OR DISTRIBUTION OF THIS PROGRAM CONSTITUTES
 * RECIPIENT'S ACCEPTANCE OF THE OSMC PUBLIC LICENSE OR THE GNU AGPL
 * VERSION 3, ACCORDING TO RECIPIENTS CHOICE.
 *
 * The OpenModelica software and the OSMC (Open Source Modelica Consortium)
 * Public License (OSMC-PL) are obtained from OSMC, either from the above
 * address, from the URLs:
 * http://www.openmodelica.org or
 * https://github.com/OpenModelica/ or
 * http://www.ida.liu.se/projects/OpenModelica,
 * and in the OpenModelica distribution.
 *
 * GNU AGPL version 3 is obtained from:
 * https://www.gnu.org/licenses/licenses.html#GPL
 *
 * This program is distributed WITHOUT ANY WARRANTY; without
 * even the implied warranty of MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE, EXCEPT AS EXPRESSLY SET FORTH
 * IN THE BY RECIPIENT SELECTED SUBSIDIARY LICENSE CONDITIONS OF OSMC-PL.
 *
 * See the full OSMC Public License conditions for more details.
 *
 */
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use openmodelica_ast::Absyn;
use openmodelica_ast::GlobalScript;
use openmodelica_ast::ParserExt;
use openmodelica_ast_collections::HashTableStringToProgram;
use openmodelica_error::ErrorExt;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Testsuite;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub fn parse(mut filename: ArcStr, mut encoding: ArcStr, mut libraryPath: ArcStr, mut lveInstance: Option<i32>, mut acceptedGram: i32, mut languageStandardInt: i32, mut strict: bool) -> Result<Absyn::Program> {
    let mut outProgram: Absyn::Program;
    let mut classes: Arc<metamodelica::List<Arc<Absyn::Class>>>;
    let mut classes1: Arc<metamodelica::List<Arc<Absyn::Class>>>;
    let mut w: Absyn::Within;
    let mut cs: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut realpath: ArcStr;
    realpath = (Util::replaceWindowsBackSlashWithPathDelimiter((System::realpath((filename).clone())?).clone())?).clone();
    outProgram = ParserExt::parse((realpath.clone()).clone(), (Testsuite::friendly((realpath).clone())?).clone(), acceptedGram, (encoding).clone(), languageStandardInt, strict, Testsuite::isRunning()?, (libraryPath).clone(), lveInstance.clone())?;
    if isSome(lveInstance.clone()) {
        let Absyn::PROGRAM { classes: __pa0, within_: __pa1 } = (outProgram) else { bail!("pattern mismatch") };
        classes = __pa0.clone();
        w = __pa1.clone();
        classes1 = metamodelica::nil();
        for mut cs in &*classes {
            let mut cs = cs.clone();
            if checkLicenseAndFeatures(cs.clone(), lveInstance.clone())? {
                classes1 = metamodelica::cons(cs.clone(), classes1.clone());
            }
        }
        outProgram = Absyn::Program { classes: classes1, within_: w };
    }
    Ok(outProgram)
}

pub fn parseexp(mut filename: ArcStr) -> Result<GlobalScript::Statements> {
    let mut outStatements: GlobalScript::Statements;
    outStatements = ParserExt::parseexp((System::realpath((filename.clone()).clone())?).clone(), (Testsuite::friendly((System::realpath((filename).clone())?).clone())?).clone(), Config::acceptedGrammar()?, Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, Testsuite::isRunning()?)?;
    Ok(outStatements)
}

pub fn parsestring(mut r#str: ArcStr, mut infoFilename: ArcStr, mut grammar: i32, mut languageStd: i32, mut strict: bool) -> Result<Absyn::Program> {
    let mut outProgram: Absyn::Program;
    outProgram = ParserExt::parsestring((r#str).clone(), (infoFilename).clone(), grammar, languageStd, strict, Testsuite::isRunning()?)?;
    Ok(outProgram)
}

pub fn parsestringexp(mut r#str: ArcStr, mut infoFilename: ArcStr) -> Result<GlobalScript::Statements> {
    let mut outStatements: GlobalScript::Statements;
    outStatements = ParserExt::parsestringexp((r#str).clone(), (infoFilename).clone(), Config::acceptedGrammar()?, Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, Testsuite::isRunning()?)?;
    Ok(outStatements)
}

pub fn stringPath(mut r#str: ArcStr) -> Result<Arc<Absyn::Path>> {
    let mut path: Arc<Absyn::Path>;
    path = ParserExt::stringPath((r#str).clone(), (literal!("<internal>")).clone(), Config::acceptedGrammar()?, Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, Testsuite::isRunning()?)?;
    Ok(path)
}

pub fn stringCref(mut r#str: ArcStr) -> Result<Arc<Absyn::ComponentRef>> {
    let mut cref: Arc<Absyn::ComponentRef>;
    cref = ParserExt::stringCref((r#str).clone(), (literal!("<internal>")).clone(), Config::acceptedGrammar()?, Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, Testsuite::isRunning()?)?;
    Ok(cref)
}

pub fn stringMod(mut r#str: ArcStr, mut filename: ArcStr) -> Result<Arc<Absyn::ElementArg>> {
    let mut r#mod: Arc<Absyn::ElementArg>;
    r#mod = ParserExt::stringMod((r#str).clone(), (filename).clone(), Config::acceptedGrammar()?, Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, Testsuite::isRunning()?)?;
    Ok(r#mod)
}

pub fn stringEq(mut r#str: ArcStr, mut filename: ArcStr) -> Result<Arc<Absyn::EquationItem>> {
    let mut eq: Arc<Absyn::EquationItem>;
    eq = ParserExt::stringEq((r#str).clone(), (filename).clone(), Config::acceptedGrammar()?, Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, Testsuite::isRunning()?)?;
    Ok(eq)
}

pub(crate) fn parallelParseFiles(mut filenames: Arc<metamodelica::List<ArcStr>>, mut encoding: ArcStr, mut numThreads: i32, mut libraryPath: ArcStr, mut lveInstance: Option<i32>) -> Result<(metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Absyn::Program)>>), i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Absyn::Program) -> Result<ArcStr> + 'static>))> {
    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Absyn::Program)>>), i32, (HashTableStringToProgram::FuncHashCref, HashTableStringToProgram::FuncCrefEqual, HashTableStringToProgram::FuncCrefStr, HashTableStringToProgram::FuncExpStr));
    let mut partialResults: Arc<metamodelica::List<ParserResult>>;
    partialResults = parallelParseFilesWork(filenames, (encoding).clone(), numThreads, (libraryPath).clone(), lveInstance)?;
    ht = HashTableStringToProgram::emptyHashTableSized(Util::nextPrime((partialResults.clone().len() as i32)));
    for mut res in &*partialResults {
        let mut res = res.clone();
        ht = (match res.clone() {
        ParserResult { program: Some(mut p), .. } => {
            BaseHashTable::add((res.filename.clone(), p.clone()), ht.clone())?
        },
        _ => bail!("match: no arm matched"),
    });
    }
    Ok(ht)
}

pub fn parallelParseFilesToProgramList(mut filenames: Arc<metamodelica::List<ArcStr>>, mut encoding: ArcStr, mut numThreads: i32) -> Result<Arc<metamodelica::List<Absyn::Program>>> {
    let mut result: Arc<metamodelica::List<Absyn::Program>> = metamodelica::nil();
    for mut r in &*parallelParseFilesWork(filenames, (encoding).clone(), numThreads, (literal!("")).clone(), None)? {
        let mut r = r.clone();
        result = metamodelica::cons((match r.clone() {
        ParserResult { program: Some(mut p), .. } => {
            p.clone()
        },
        _ => bail!("match: no arm matched"),
    }), result.clone());
    }
    result = metamodelica::Dangerous::listReverseInPlace(result);
    Ok(result)
}

pub fn startLibraryVendorExecutable(mut lvePath: ArcStr) -> (bool, Option<i32>) {
    let mut success: bool;
    let mut lveInstance: Option<i32>;
    (success, lveInstance) = ParserExt::startLibraryVendorExecutable((lvePath).clone());
    (success, lveInstance)
}

pub(crate) fn checkLVEToolLicense(mut lveInstance: Option<i32>, mut packageName: ArcStr) -> bool {
    let mut status: bool;
    status = ParserExt::checkLVEToolLicense(lveInstance, (packageName).clone());
    status
}

pub(crate) fn checkLVEToolFeature(mut lveInstance: Option<i32>, mut feature: ArcStr) -> bool {
    let mut status: bool;
    status = ParserExt::checkLVEToolFeature(lveInstance, (feature).clone());
    status
}

pub fn stopLibraryVendorExecutable(mut lveInstance: Option<i32>) -> () {
    ParserExt::stopLibraryVendorExecutable(lveInstance);
    ()
}

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct ParserResult {
    pub filename: ArcStr,
    pub program: Option<Absyn::Program>,
}

impl metamodelica::gc::MMTrace for ParserResult {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.filename, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.program, __mmv)?;
        Ok(())
    }
}
impl Default for ParserResult {
    fn default() -> Self {
        Self {
            filename: Default::default(),
            program: Default::default(),
        }
    }
}

pub type PARSERRESULT = ParserResult;


fn parallelParseFilesWork(mut filenames: Arc<metamodelica::List<ArcStr>>, mut encoding: ArcStr, mut numThreads: i32, mut libraryPath: ArcStr, mut lveInstance: Option<i32>) -> Result<Arc<metamodelica::List<ParserResult>>> {
    let mut partialResults: Arc<metamodelica::List<ParserResult>>;
    let mut workList: Arc<metamodelica::List<(ArcStr, ArcStr, ArcStr, Option<i32>)>> = ({
        let mut __acc: Arc<metamodelica::List<(ArcStr, ArcStr, ArcStr, Option<i32>)>> = metamodelica::nil();
        for mut file in (filenames.clone()).into_iter().cloned() {
            let __x = (file.clone(), encoding.clone(), libraryPath.clone(), lveInstance.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    if Testsuite::isRunning()? || Config::noProc()? == 1 || numThreads == 1 || (filenames.len() as i32) < 2 || isSome(lveInstance) {
        partialResults = ({
        let mut __acc: Arc<metamodelica::List<ParserResult>> = metamodelica::nil();
        for mut t in (workList).into_iter().cloned() {
            let __x = loadFileThread(t.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    } else {
        partialResults = System::launchParallelTasks(std::cmp::min(8, numThreads), workList, (std::sync::Arc::new(fnptr!(loadFileThread, (ArcStr, ArcStr, ArcStr, Option<i32>))) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, ArcStr, ArcStr, Option<i32>)) -> Result<ParserResult> + 'static>))?;
    }
    Ok(partialResults)
}

fn loadFileThread(mut inFileEncoding: (ArcStr, ArcStr, ArcStr, Option<i32>)) -> ParserResult {
    let mut result: ParserResult;
    result = 'mc: {
        let __mc_input = inFileEncoding;
        if let Ok(__v) = (|| -> Result<_> {
            let (mut filename, mut encoding, mut libraryPath, mut lveInstance) = __mc_input.clone() else { bail!("nomatch") };
            Ok(ParserResult { filename: (filename.clone()).clone(), program: Some(parse((filename.clone()).clone(), (encoding.clone()).clone(), (libraryPath.clone()).clone(), lveInstance.clone(), Config::acceptedGrammar()?, Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, Flags::getConfigBool(Flags::STRICT.clone())?)?) })
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut filename, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            Ok(ParserResult { filename: (filename.clone()).clone(), program: None })
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    if ErrorExt::getNumMessages() > 0 {
        ErrorExt::moveMessagesToParentThread();
    }
    result
}

pub(crate) fn checkLicenseAndFeatures(mut c1: Arc<Absyn::Class>, mut lveInstance: Option<i32>) -> Result<bool> {
    let mut result: bool;
    let mut orFeatures: Arc<metamodelica::List<ArcStr>>;
    let mut andFeatures: Arc<metamodelica::List<ArcStr>>;
    result = true;
    orFeatures = getFeaturesAnnotation(c1);
    for mut orFeature in &*orFeatures {
        let mut orFeature = orFeature.clone();
        andFeatures = Util::stringSplitAtChar((orFeature.clone()).clone(), (literal!(" ")).clone())?;
        result = true;
        for mut andFeature in &*andFeatures.clone() {
            let mut andFeature = andFeature.clone();
            if !(checkLVEToolFeature(lveInstance.clone(), (andFeature.clone()).clone())) {
                result = false;
                break;
            }
        }
        if result {
            break;
        }
    }
    Ok(result)
}

fn getLicenseAnnotation(mut className: Arc<Absyn::Class>) -> (ArcStr, ArcStr) {
    let mut license: (ArcStr, ArcStr);
    let mut opt_license: Option<(ArcStr, ArcStr)>;
    opt_license = AbsynUtil::getNamedAnnotationInClass(className, Arc::new(Absyn::Path::IDENT { name: (literal!("Protection")).clone() }), (std::sync::Arc::new(getLicenseAnnotationWork1) as std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<Absyn::Modification>>) -> Result<(ArcStr, ArcStr)> + 'static>));
    license = Util::getOptionOrDefault(opt_license, (literal!(""), literal!("")));
    license
}

fn getLicenseAnnotationWork1(mut r#mod: Option<Arc<Absyn::Modification>>) -> Result<(ArcStr, ArcStr)> {
    let mut license: (ArcStr, ArcStr);
    license = (::match_deref::match_deref! { match &(r#mod) {
        Some(Deref @ Absyn::Modification { elementArgLst: arglst, .. }) => {
            let mut libraryKey: ArcStr;
            let mut licenseFile: ArcStr;
            (libraryKey, licenseFile) = getLicenseAnnotationWork2(arglst.clone())?;
            (libraryKey, licenseFile)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(license)
}

fn getLicenseAnnotationWork2(mut eltArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Result<(ArcStr, ArcStr)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(eltArgs) {
        Deref @ metamodelica::List::Nil => {
            return Ok((literal!(""), literal!("")))
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "License" }, modification: r#mod, .. }, tail: _ } => {
            let mut libraryKey: ArcStr;
            let mut licenseFile: ArcStr;
            return Ok(getLicenseAnnotationTuple(r#mod.clone())?)
        },
        Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
            let mut libraryKey: ArcStr;
            let mut licenseFile: ArcStr;
            { eltArgs = xs.clone(); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn getLicenseAnnotationTuple(mut r#mod: Option<Arc<Absyn::Modification>>) -> Result<(ArcStr, ArcStr)> {
    let mut license: (ArcStr, ArcStr);
    license = (::match_deref::match_deref! { match &(r#mod) {
        Some(Deref @ Absyn::Modification { elementArgLst: arglst, .. }) => {
            let mut libraryKey: ArcStr;
            let mut licenseFile: ArcStr;
            libraryKey = (getLicenseAnnotationLibraryKey(arglst.clone())).clone();
            licenseFile = (getLicenseAnnotationLicenseFile(arglst.clone())).clone();
            (libraryKey, licenseFile)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(license)
}

fn getLicenseAnnotationLibraryKey(mut eltArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> ArcStr {
    '__tco: loop {
        ::match_deref::match_deref! { match &(eltArgs) {
        Deref @ metamodelica::List::Nil => {
            return literal!("")
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "libraryKey" }, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::STRING { value: s }, .. }, .. }), .. }, tail: _ } => {
            return s.clone()
        },
        Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
            let mut s: ArcStr;
            { eltArgs = xs.clone(); continue '__tco; }
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

fn getLicenseAnnotationLicenseFile(mut eltArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> ArcStr {
    '__tco: loop {
        ::match_deref::match_deref! { match &(eltArgs) {
        Deref @ metamodelica::List::Nil => {
            return literal!("")
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "licenseFile" }, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::STRING { value: s }, .. }, .. }), .. }, tail: _ } => {
            return s.clone()
        },
        Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
            let mut s: ArcStr;
            { eltArgs = xs.clone(); continue '__tco; }
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

fn getFeaturesAnnotation(mut className: Arc<Absyn::Class>) -> Arc<metamodelica::List<ArcStr>> {
    let mut features: Arc<metamodelica::List<ArcStr>>;
    let mut opt_featuresList: Option<Arc<metamodelica::List<ArcStr>>>;
    opt_featuresList = AbsynUtil::getNamedAnnotationInClass(className, Arc::new(Absyn::Path::IDENT { name: (literal!("Protection")).clone() }), (std::sync::Arc::new(getFeaturesAnnotationList) as std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<Absyn::Modification>>) -> Result<Arc<metamodelica::List<ArcStr>>> + 'static>));
    features = Util::getOptionOrDefault(opt_featuresList, metamodelica::nil());
    features
}

fn getFeaturesAnnotationList(mut r#mod: Option<Arc<Absyn::Modification>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut features: Arc<metamodelica::List<ArcStr>>;
    features = (::match_deref::match_deref! { match &(r#mod) {
        Some(Deref @ Absyn::Modification { elementArgLst: arglst, .. }) => {
            getFeaturesAnnotationList2(arglst.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(features)
}

fn getFeaturesAnnotationList2(mut eltArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(eltArgs) {
        Deref @ metamodelica::List::Nil => {
            return Ok(metamodelica::nil())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "features" }, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::ARRAY { arrayExp: expList }, .. }, .. }), .. }, tail: _ } => {
            let mut featuresList: Arc<metamodelica::List<ArcStr>>;
            return Ok(List::map(expList.clone(), (std::sync::Arc::new(fnptr!(expToString, Arc<Absyn::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>) -> Result<ArcStr> + 'static>))?)
        },
        Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
            let mut featuresList: Arc<metamodelica::List<ArcStr>>;
            { eltArgs = xs.clone(); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn expToString(mut inExp: Arc<Absyn::Exp>) -> ArcStr {
    let mut outExp: ArcStr;
    outExp = ((::match_deref::match_deref! { match &(inExp) {
        Deref @ Absyn::Exp::STRING { value: r#str } => {
            r#str.clone()
        },
        _ => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    outExp
}

