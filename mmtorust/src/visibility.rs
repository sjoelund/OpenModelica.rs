//! Cross-crate visibility analysis.
//!
//! Decides which generated `pub fn`s must stay `pub` and which can be narrowed
//! to `pub(crate)`. Because `pub(crate)` is already visible from everywhere
//! inside the defining crate, a function needs full `pub` only when it is
//! reachable from a *different* crate. So this pass collects every cross-crate
//! function reference in the generated program; a function not referenced
//! across a crate boundary (and not in the hand-written-export allow-list) is
//! narrowed to `pub(crate)`.
//!
//! ## What counts as keeping a function `pub`
//!
//!  * A call / partial-application / function-value reference whose resolved
//!    target lives in another crate (the bulk — direct cross-crate calls).
//!  * A function used as the default value of an `input` parameter: codegen
//!    expands such defaults at the *call site*, which may be in any crate, so
//!    the referenced function is conservatively kept public.
//!  * The base of a `function Foo = Bar(...)` alias: the alias is emitted as a
//!    `pub use … as …;` re-export, and `pub use` of a `pub(crate)` item is
//!    rejected (E0365). Keeping the base public makes the re-export valid
//!    regardless of where the alias is consumed.
//!  * The hand-written allow-list ([`HANDWRITTEN_EXPORTS`]): generated items
//!    that hand-written `.rs` files reference across a crate boundary. Those
//!    references live outside the MetaModelica hierarchy this pass walks, so
//!    they cannot be discovered automatically and are listed explicitly.
//!
//! Soundness note: missing a real cross-crate reference would narrow a function
//! that is actually used elsewhere, surfacing as a hard `E0603` at build time
//! (never as silent miscompilation); over-approximating merely leaves a
//! function `pub` that could have been `pub(crate)`. The analysis therefore
//! errs toward keeping functions public when a reference cannot be resolved.

use std::collections::{BTreeMap, BTreeSet};

use openmodelica_ast::Absyn;

use crate::hierarchy::{extract_default_exp, InstanceHierarchy, NameNode, NodeKind, Ty};
use crate::typedexp::resolve_call_node;
use crate::unused_functions::RefScan;
use crate::MM;

/// Generated items (by fully-qualified MetaModelica name) that hand-written
/// Rust code references across a crate boundary, and which must therefore stay
/// `pub`. These references are invisible to the MetaModelica hierarchy, so they
/// are declared here as upstream knowledge rather than discovered. Add an entry
/// whenever a hand-written `.rs` file gains a cross-crate use of a generated
/// function (an omission shows up as an `E0603` for a `pub(crate)` item named
/// from another crate's hand-written source).
const HANDWRITTEN_EXPORTS: &[&str] = &[
    // openmodelica/src/main.rs → the program entry point.
    "Main.main",
    // openmodelica_script_util/src/{DynLoadExt,SimulationResults,UnitParserExt,Unzip}.rs,
    // openmodelica_backend/src/SerializeSparsityPattern.rs, …/Curl.rs → error reporting.
    "Error.addMessage",
    // openmodelica_script_util/src/DynLoadExt.rs → diagnostics / flag access.
    "Error.getCurrentComponent",
    "AbsynUtil.pathString",
    "Flags.getFlags",
    // openmodelica_frontend/src/Globals.rs → global-state initialisers.
    "Flags.getConfigInt",
    "BaseHashTable.emptyHashTableWork",
    // openmodelica_util/src/Globals.rs → global-state initialiser.
    "DoubleEnded.fromList",

    // ── Referenced by integration tests ──────────────────────────────────────
    // A crate's `tests/*.rs` files compile as a *separate* crate, so every
    // generated helper they exercise is a cross-crate reference and must stay
    // `pub`. (Listed because the MetaModelica hierarchy this pass walks does not
    // include the hand-written test crates.)
    "Array.findFirstOnTrue",
    "Array.heapSort",
    "Array.insertList",
    "Array.remove",
    "AvlSetString.intersection",
    "AvlSetString.keyCompare",
    "AvlSetString.keyStr",
    "AvlSetString.listKeysReverse",
    "AvlSetString.printNodeStr",
    "AvlSetString.printTreeStr",
    "AvlSetString.smallestKey",
    "AvlTreeString.add",
    "AvlTreeString.addConflictKeep",
    "AvlTreeString.addConflictReplace",
    "AvlTreeString.addList",
    "AvlTreeString.addUpdate",
    "AvlTreeString.fold",
    "AvlTreeString.forEach",
    "AvlTreeString.fromList",
    "AvlTreeString.get",
    "AvlTreeString.getOpt",
    "AvlTreeString.hasKey",
    "AvlTreeString.intersection",
    "AvlTreeString.isEmpty",
    "AvlTreeString.join",
    "AvlTreeString.keyCompare",
    "AvlTreeString.keyStr",
    "AvlTreeString.listKeys",
    "AvlTreeString.listValues",
    "AvlTreeString.map",
    "AvlTreeString.new",
    "AvlTreeString.printNodeStr",
    "AvlTreeString.printTreeStr",
    "AvlTreeString.valueStr",
    "BaseHashSet.get",
    "ComponentReferenceBasics.crefNotPrefixOf",
    "DoubleEnded.clear",
    "DoubleEnded.pop_front",
    "Dump.expPriority",
    "ExpressionBasics.subscriptInt",
    "ExpressionBasics.subscriptsInt",
    "HashSetString.emptyHashSetSized",
    "List.countingSort",
    "List.deletePositionsSorted",
    "List.filter",
    "List.isPrefixOnTrue",
    "List.keepPositionsSorted",
    "List.sortedUniqueAndDuplicates",
    "List.threadMapList",
    "List.threadMap_2",
    "List.unionIntN",
    "SBAtomicSet.cardinality",
    "SBAtomicSet.contains",
    "SBAtomicSet.copy",
    "SBAtomicSet.intersection",
    "SBAtomicSet.isEmpty",
    "SBAtomicSet.isEqual",
    "SBAtomicSet.ndim",
    "SBAtomicSet.newEmpty",
    "SBAtomicSet.replace",
    "SBAtomicSet.toString",
    "SBInterval.cardinality",
    "SBInterval.contains",
    "SBInterval.intersection",
    "SBInterval.isEqual",
    "SBInterval.newFull",
    "SBInterval.newUnit",
    "SBInterval.toString",
    "SBMultiInterval.cardinality",
    "SBMultiInterval.contains",
    "SBMultiInterval.crossProd",
    "SBMultiInterval.fromList",
    "SBMultiInterval.isEqual",
    "StringUtil.equalIgnoreSpace",
    "StringUtil.findCharNot",
    "StringUtil.isAlpha",
    "StringUtil.rfindChar",
    "StringUtil.rfindCharNot",
    "UnorderedMap.first",
    "UnorderedSet.any",
    "UnorderedSet.get",
    "UnorderedSet.getOrFail",
    "UnorderedSet.none",
    "UnorderedSet.rehash",
    "Util.anyToEmptyString",
    "Util.gcd",
    "Util.intBool",
    "Util.intGreaterZero",
    "Util.intNegative",
    "Util.intProduct",
    "Util.isRealGreater",
    "Util.lcm",
    "Util.makeOptionOnTrue",
    "Util.makeTupleR",
    "Util.msb",
    "Util.mulListIntegerOpt",
    "Util.nextPowerOf2",
    "Util.realNegative",
    "Util.selectFirstNonEmptyString",
    "Util.stringContainsChar",
];

/// Result of [`analyze`]: the set of function FQNs that must keep full `pub`
/// visibility. Every other `pub` function is narrowed to `pub(crate)`.
#[derive(Debug, Default, Clone)]
pub struct VisibilityInfo {
    pub keep_public: BTreeSet<String>,
}

impl VisibilityInfo {
    /// Whether the (public) function `qname` may be narrowed to `pub(crate)` —
    /// i.e. it is not reachable from another crate.
    pub fn fn_is_crate_local(&self, qname: &str) -> bool {
        !self.keep_public.contains(qname)
    }
}

fn is_function_node(node: &NameNode<'_>) -> bool {
    matches!(&node.kind, NodeKind::Class(c) if matches!(c.restriction, Absyn::Restriction::R_FUNCTION { .. }))
}

fn path_to_dotted(p: &Absyn::Path) -> String {
    match p {
        Absyn::Path::IDENT { name } => name.to_string(),
        Absyn::Path::QUALIFIED { name, path } => format!("{}.{}", name, path_to_dotted(path)),
        Absyn::Path::FULLYQUALIFIED { path } => path_to_dotted(path),
    }
}

/// The fully-qualified item paths a *specific-item* import brings into scope.
/// Codegen lowers each to a `use …::Item;` statement, so each is a real
/// reference even when the imported name is never otherwise used (e.g. an
/// import whose only uses sit in commented-out source). A whole-package
/// `import Pkg;` / wildcard `import Pkg.*;` names no specific item — its members
/// are reached through ordinary `Pkg.foo` calls that the body scan already
/// sees — so it contributes nothing here.
fn import_item_targets(import: &Absyn::Import) -> Vec<String> {
    match import {
        Absyn::Import::QUAL_IMPORT { path } | Absyn::Import::NAMED_IMPORT { path, .. } => {
            vec![path_to_dotted(path)]
        }
        Absyn::Import::GROUP_IMPORT { prefix, groups } => {
            let pfx = path_to_dotted(prefix);
            (&**groups).into_iter().map(|g| {
                let name = match g {
                    Absyn::GroupImport::GROUP_IMPORT_NAME { name }
                    | Absyn::GroupImport::GROUP_IMPORT_RENAME { name, .. } => name,
                };
                format!("{pfx}.{name}")
            }).collect()
        }
        // `import Pkg.*;` — a glob, not a specific item.
        Absyn::Import::UNQUAL_IMPORT { .. } => Vec::new(),
    }
}

/// Compute the cross-crate visibility classification for every function.
pub fn analyze(hier: &InstanceHierarchy<'_>) -> VisibilityInfo {
    let top_level = &hier.top_level;

    // top-level package name → Rust crate name (mirrors the `crate_map` built
    // in `codegen::generate_all`). The crate owning any FQN is the crate of its
    // first dotted segment.
    let crate_map: BTreeMap<&str, &str> = top_level.iter()
        .filter_map(|(name, node)| match &node.kind {
            NodeKind::Class(c) => c.crate_name.as_deref().map(|cn| (name.as_str(), cn)),
            _ => None,
        })
        .collect();
    let crate_of = |qname: &str| -> Option<&str> {
        crate_map.get(qname.split('.').next().unwrap_or(qname)).copied()
    };

    let mut functions: Vec<(String, &NameNode<'_>)> = Vec::new();
    crate::codegen::collect_all_function_nodes(top_level, "", &mut functions);

    let mut keep_public: BTreeSet<String> = BTreeSet::new();

    for (qname, node) in &functions {
        let NodeKind::Class(class) = &node.kind else { continue };
        let Some(ref_crate) = crate_of(qname) else { continue };

        // Direct references in the body and in component default expressions: a
        // target function resolving into another crate keeps that function pub.
        for raw in &RefScan::scan_class(class).refs {
            if let Some((target, n)) = resolve_call_node(raw, top_level, qname)
                && is_function_node(n)
                && crate_of(&target).is_some_and(|def_crate| def_crate != ref_crate)
            {
                keep_public.insert(target);
            }
        }

        // `function Foo = Bar(...)` alias: its base is re-exported via `pub use`
        // and must stay `pub` (E0365). Resolve and keep it regardless of crate —
        // a same-crate base only ends up redundantly present, never wrongly so.
        if let Ty::FunctionAlias { base, .. } = &node.ty
            && let Some((target, n)) = resolve_call_node(base, top_level, qname)
            && is_function_node(n)
        {
            keep_public.insert(target);
        }

        // `input <FuncT> f = <default>` — the default function value is expanded
        // at the caller's site, which may be in any crate. Keep it public.
        let members = match &class.body {
            MM::ClassDef::Parts { members, .. } | MM::ClassDef::ClassExtends { members, .. } => members.as_slice(),
            _ => &[],
        };
        for m in members {
            let MM::ClassMember::Component(cm) = m else { continue };
            if cm.direction != Absyn::Direction::INPUT { continue; }
            let Some(default) = extract_default_exp(&cm.modification) else { continue };
            let mut scan = RefScan::default();
            scan.scan_exp(default);
            for raw in &scan.refs {
                if let Some((target, n)) = resolve_call_node(raw, top_level, qname)
                    && is_function_node(n)
                {
                    keep_public.insert(target);
                }
            }
        }
    }

    // Specific-item imports (`import Pkg.{a,b}`, `import Pkg.X`, `import N = Pkg.X`),
    // declared at package or function scope, each lower to a `use …::Item;`
    // statement — a cross-crate reference even when the name is never otherwise
    // used. Walk every import and keep cross-crate function targets public.
    scan_imports(top_level, top_level, "", &crate_map, &mut keep_public);

    for &q in HANDWRITTEN_EXPORTS {
        keep_public.insert(q.to_owned());
    }

    VisibilityInfo { keep_public }
}

fn scan_imports<'a>(
    nodes: &BTreeMap<String, NameNode<'a>>,
    top_level: &BTreeMap<String, NameNode<'a>>,
    prefix: &str,
    crate_map: &BTreeMap<&str, &str>,
    keep_public: &mut BTreeSet<String>,
) {
    let crate_of = |qname: &str| -> Option<&str> {
        crate_map.get(qname.split('.').next().unwrap_or(qname)).copied()
    };
    for (name, node) in nodes {
        let qname = if prefix.is_empty() { name.clone() } else { format!("{prefix}.{name}") };
        if let NodeKind::Import(m) = &node.kind
            && let Some(ref_crate) = crate_of(prefix)
        {
            for target in import_item_targets(&m.import) {
                // Resolve in the importing scope (handles relative imports and
                // import-alias chains, same as codegen's `use`-line emission).
                if let Some((q, n)) = resolve_call_node(&target, top_level, prefix)
                    && is_function_node(n)
                    && crate_of(&q).is_some_and(|def_crate| def_crate != ref_crate)
                {
                    keep_public.insert(q);
                }
            }
        }
        scan_imports(&node.children, top_level, &qname, crate_map, keep_public);
    }
}
