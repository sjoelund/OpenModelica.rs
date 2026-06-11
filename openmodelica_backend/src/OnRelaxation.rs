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

use crate::AdjacencyMatrix;
use crate::BackendDAETransform;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVariable;
use crate::Differentiate;
use crate::DumpGraphML;
use crate::Matching;
use crate::Sorting;
use crate::SymbolicJacobian;
use openmodelica_backend_types::BackendDAE;
use openmodelica_backend_util::BackendDAEEXT;
use openmodelica_frontend::HashSet;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashSet;
use openmodelica_util::BaseHashTable;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

/*
 * relaxation from gausian elemination
 *
 */
pub(crate) fn relaxSystem(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    (outDAE, _) = BackendDAEUtil::mapEqSystemAndFold(inDAE, (std::sync::Arc::new(relaxSystem0) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool)> + 'static>), false)?;
    Ok(outDAE)
}

fn relaxSystem0(mut isyst: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inChanged: bool) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool)> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut outShared: Arc<BackendDAE::Shared>;
    let mut outChanged: bool;
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let mut b2: bool;
    let __pa0 = ::match_deref::match_deref! { match &(isyst.clone()) {
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa0, .. }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    comps = __pa0.clone();
    (osyst, outShared, b2) = relaxSystem1(isyst, inShared, comps)?;
    outChanged = inChanged || b2;
    Ok((osyst, outShared, outChanged))
}

fn relaxSystem1(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut inComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool)> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut outRunMatching: bool;
    (osyst, oshared, outRunMatching) = 'mc: {
        let __mc_input = (ishared.clone(), inComps);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil) => {
                    Ok((isyst.clone(), ishared.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (shared @ Deref @ BackendDAE::Shared { functionTree: funcs, .. }, Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: eindex, vars: vindx, jac: Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: Some(jac) }, jacType: BackendDAE::JacobianType::JAC_LINEAR { .. }, .. }, tail: comps }) => {
                    let mut eorphans: Arc<metamodelica::List<i32>>;
                    let mut vorphans: Arc<metamodelica::List<i32>>;
                    let mut unassigned: Arc<metamodelica::List<i32>>;
                    let mut otherorphans: Arc<metamodelica::List<i32>>;
                    let mut roots: Arc<metamodelica::List<i32>>;
                    let mut constraints: Arc<metamodelica::List<i32>>;
                    let mut constraintresidual: Arc<metamodelica::List<i32>>;
                    let mut syst: Arc<BackendDAE::EqSystem>;
                    let mut subsyst: Arc<BackendDAE::EqSystem>;
                    let mut ass1: metamodelica::Array<i32>;
                    let mut ass2: metamodelica::Array<i32>;
                    let mut vec2: metamodelica::Array<i32>;
                    let mut rowmarks: metamodelica::Array<i32>;
                    let mut colummarks: metamodelica::Array<i32>;
                    let mut mapIncRowEqn: metamodelica::Array<i32>;
                    let mut orowmarks: metamodelica::Array<i32>;
                    let mut ocolummarks: metamodelica::Array<i32>;
                    let mut size: i32;
                    let mut mark: i32;
                    let mut esize: i32;
                    let mut eqn_lst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut var_lst: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut vars: BackendDAE::Variables;
                    let mut tvars: BackendDAE::Variables;
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut teqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut m1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut mc: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut mct: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut beqs: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut matrix: metamodelica::Array<Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>>;
                    let mut crefexps: metamodelica::Array<Arc<DAE::Exp>>;
                    let mut crefexplst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut vorphansarray1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut ass22: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut vec1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut neweqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut shared = (*shared).clone();
                    let mut jac = (*jac).clone();
                    metamodelica::print((literal!("try to relax\n")).clone());
                    Util::profilerinit()?;
                    Util::profilerstart2()?;
                    Util::profilerstart1()?;
                    size = (vindx.clone().len() as i32);
                    esize = (eindex.clone().len() as i32);
                    ass1 = arrayCreate(size.clone(), -1);
                    ass2 = arrayCreate(size.clone(), -1);
                    eqn_lst = BackendEquation::getList(eindex.clone(), BackendEquation::getEqnsFromEqSystem(isyst.clone()))?;
                    eqns = BackendEquation::listEquation(eqn_lst.clone())?;
                    var_lst = List::map1r(vindx.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), BackendVariable::daeVars(isyst.clone()))?;
                    vars = BackendVariable::listVar1(var_lst.clone())?;
                    subsyst = BackendDAEUtil::createEqSystem(vars.clone(), eqns.clone(), metamodelica::nil(), openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
                    (subsyst, m, mt, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::getAdjacencyMatrixScalar(subsyst.clone(), openmodelica_backend_types::BackendDAE::IndexType::ABSOLUTE, Some(funcs.clone()), BackendDAEUtil::isInitializationDAE(ishared.clone()))?;
                    (_, ass1, ass2) = List::fold1(eqn_lst.clone(), (std::sync::Arc::new(vectorMatching) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, BackendDAE::Variables, (i32, metamodelica::Array<i32>, metamodelica::Array<i32>)) -> Result<(i32, metamodelica::Array<i32>, metamodelica::Array<i32>)> + 'static>), vars.clone(), (1, ass1.clone(), ass2.clone()))?;
                    (_, ass1, ass2) = List::fold1(eqn_lst.clone(), (std::sync::Arc::new(aliasMatching) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, BackendDAE::Variables, (i32, metamodelica::Array<i32>, metamodelica::Array<i32>)) -> Result<(i32, metamodelica::Array<i32>, metamodelica::Array<i32>)> + 'static>), vars.clone(), (1, ass1.clone(), ass2.clone()))?;
                    m1 = arrayCreate(size.clone(), metamodelica::nil());
                    transformJacToAdjacencyMatrix2(jac.clone(), m1.clone(), mapIncRowEqn.clone(), eqns.clone(), ass1.clone(), ass2.clone(), (std::sync::Arc::new(fnptr!(isConstOneMinusOne, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>))?;
                    Matching::matchingExternalsetAdjacencyMatrix(size.clone(), size.clone(), m1.clone());
                    let true = (BackendDAEEXT::setAssignment(size.clone(), size.clone(), ass2.clone(), ass1.clone())) else { bail!("pattern mismatch") };
                    BackendDAEEXT::matching(size.clone(), size.clone(), 5, -1, metamodelica::OrderedFloat(1.0_f64), 0);
                    BackendDAEEXT::getAssignment(ass2.clone(), ass1.clone())?;
                    m1 = arrayCreate(size.clone(), metamodelica::nil());
                    transformJacToAdjacencyMatrix1(jac.clone(), m1.clone(), ass1.clone(), ass2.clone(), (std::sync::Arc::new(fnptr!(isConstOneMinusOne, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>))?;
                    Matching::matchingExternalsetAdjacencyMatrix(size.clone(), size.clone(), m1.clone());
                    let true = (BackendDAEEXT::setAssignment(size.clone(), size.clone(), ass2.clone(), ass1.clone())) else { bail!("pattern mismatch") };
                    BackendDAEEXT::matching(size.clone(), size.clone(), 1, -1, metamodelica::OrderedFloat(1.0_f64), 0);
                    BackendDAEEXT::getAssignment(ass2.clone(), ass1.clone())?;
                    unassigned = Matching::getUnassigned(size.clone(), ass2.clone(), metamodelica::nil());
                    colummarks = arrayCreate(size.clone(), -1);
                    onefreeMatchingBFS(unassigned.clone(), m.clone(), mt.clone(), size.clone(), ass1.clone(), ass2.clone(), colummarks.clone(), 1, metamodelica::nil())?;
                    Util::profilerstop1()?;
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Matching  time: ")); __mm_s.push_str(&*realString(Util::profilertime1())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Util::profilerreset1();
                    Util::profilerstart1()?;
                    vorphans = getOrphans(1, size.clone(), ass1.clone(), metamodelica::nil());
                    eorphans = getOrphans(1, size.clone(), ass2.clone(), metamodelica::nil());
                    ass1 = BackendDAETransform::varAssignmentNonScalar(ass1.clone(), mapIncRowEqn.clone());
                    ass22 = BackendDAETransform::eqnAssignmentNonScalar(mapEqnIncRow.clone(), ass2.clone())?;
                    eorphans = List::uniqueIntN(List::map1r(eorphans.clone(), (std::sync::Arc::new(arrayGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), mapIncRowEqn.clone())?, metamodelica::arrayLength(mapIncRowEqn.clone()))?;
                    (subsyst, m, mt) = BackendDAEUtil::getAdjacencyMatrix(subsyst.clone(), openmodelica_backend_types::BackendDAE::IndexType::ABSOLUTE, Some(funcs.clone()), BackendDAEUtil::isInitializationDAE(ishared.clone()))?;
                    rowmarks = arrayCreate(size.clone(), -1);
                    colummarks = arrayCreate(size.clone(), -1);
                    orowmarks = arrayCreate(size.clone(), -1);
                    ocolummarks = arrayCreate(size.clone(), -1);
                    vorphansarray1 = arrayCreate(size.clone(), metamodelica::nil());
                    mc = arrayCreate(esize.clone(), metamodelica::nil());
                    mct = arrayCreate(size.clone(), metamodelica::nil());
                    mc = Array::copy(m.clone(), mc.clone())?;
                    mct = Array::copy(mt.clone(), mct.clone())?;
                    mark = 1;
                    (mark, constraintresidual) = generateCliquesResidual(eorphans.clone(), ass1.clone(), ass22.clone(), mc.clone(), mct.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), vars.clone(), metamodelica::nil());
                    (mark, roots, constraints) = prepairOrphansOrder(vorphans.clone(), ass1.clone(), ass22.clone(), mc.clone(), mct.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), vorphansarray1.clone(), vars.clone(), metamodelica::nil(), metamodelica::nil())?;
                    mark = prepairOrphansOrder2(vorphans.clone(), ass1.clone(), ass22.clone(), mc.clone(), mct.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), vorphansarray1.clone());
                    Util::profilerstop1()?;
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Identifikation  time: ")); __mm_s.push_str(&*realString(Util::profilertime1())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Util::profilerreset1();
                    Util::profilerstart1()?;
                    vorphansarray1 = arrayCreate(size.clone(), metamodelica::nil());
                    List::map2_0(roots.clone(), (std::sync::Arc::new(doMark) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<()> + 'static>), rowmarks.clone(), mark.clone())?;
                    List::map2_0(constraints.clone(), (std::sync::Arc::new(doMark) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<()> + 'static>), rowmarks.clone(), mark.clone())?;
                    otherorphans = List::select2(vorphans.clone(), (std::sync::Arc::new(fnptr!(unmarked, i32, metamodelica::Array<i32>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), rowmarks.clone(), mark.clone())?;
                    mark = getOrphansOrderEdvanced(otherorphans.clone(), ass1.clone(), ass22.clone(), m.clone(), mt.clone(), mc.clone(), mct.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), vorphansarray1.clone());
                    List::map2_0(otherorphans.clone(), (std::sync::Arc::new(fnptr!(removeRootConnections, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>) -> Result<()> + 'static>), vorphansarray1.clone(), roots.clone())?;
                    mark = getConstraintesOrphansOrderEdvanced(constraints.clone(), ass1.clone(), ass22.clone(), m.clone(), mt.clone(), mc.clone(), mct.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), vorphansarray1.clone());
                    (vorphans, mark) = getOrphansOrderEdvanced3(roots.clone(), otherorphans.clone(), constraints.clone(), vorphans.clone(), vorphansarray1.clone(), mark.clone(), rowmarks.clone())?;
                    Util::profilerstop1()?;
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Reihenfolge  time: ")); __mm_s.push_str(&*realString(Util::profilertime1())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Util::profilerreset1();
                    Util::profilerstart1()?;
                    List::map2_0(constraints.clone(), (std::sync::Arc::new(doMark) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<()> + 'static>), rowmarks.clone(), mark.clone())?;
                    otherorphans = List::select2(vorphans.clone(), (std::sync::Arc::new(fnptr!(unmarked, i32, metamodelica::Array<i32>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), rowmarks.clone(), mark.clone())?;
                    List::map2_0(constraintresidual.clone(), (std::sync::Arc::new(doAssign) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>) -> Result<()> + 'static>), ass22.clone(), list![-1])?;
                    mark = getOrphansPairs(otherorphans.clone(), ass1.clone(), ass22.clone(), m.clone(), mt.clone(), mark.clone() + 1, rowmarks.clone(), colummarks.clone());
                    List::map2_0(constraintresidual.clone(), (std::sync::Arc::new(doAssign) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>) -> Result<()> + 'static>), ass22.clone(), metamodelica::nil())?;
                    mark = getOrphansPairsConstraints(constraints.clone(), ass1.clone(), ass22.clone(), mc.clone(), mct.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), eqns.clone());
                    Util::profilerstop1()?;
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Paarung  time: ")); __mm_s.push_str(&*realString(Util::profilertime1())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Util::profilerreset1();
                    Util::profilerstart1()?;
                    vec1 = arrayCreate(esize.clone(), metamodelica::nil());
                    vec2 = arrayCreate(esize.clone(), -1);
                    orowmarks = List::fold1(vorphans.clone(), (std::sync::Arc::new(markOrphans) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> + 'static>), 1, orowmarks.clone())?;
                    ocolummarks = List::fold1(eorphans.clone(), (std::sync::Arc::new(markOrphans) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> + 'static>), 1, ocolummarks.clone())?;
                    mark = getIndexesForEqnsAdvanced(vorphans.clone(), 1, m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), orowmarks.clone(), ocolummarks.clone(), ass1.clone(), ass22.clone(), vec1.clone(), vec2.clone(), arrayCreate(esize.clone(), false), vars.clone(), eqns.clone(), shared.clone(), size.clone());
                    (_, _, _, eqns, vars) = Array::fold(vec2.clone(), (std::sync::Arc::new(getEqnsinOrder) as std::sync::Arc<dyn ::std::ops::Fn(i32, (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables)) -> Result<(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables)> + 'static>), (eqns.clone(), vars.clone(), ass22.clone(), BackendEquation::listEquation(metamodelica::nil())?, BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())))?;
                    Util::profilerstop1()?;
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Indizierung  time: ")); __mm_s.push_str(&*realString(Util::profilertime1())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Util::profilerreset1();
                    Util::profilerstart1()?;
                    subsyst = BackendDAEUtil::createEqSystem(vars.clone(), eqns.clone(), metamodelica::nil(), openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
                    (subsyst, m, _) = BackendDAEUtil::getAdjacencyMatrix(subsyst.clone(), openmodelica_backend_types::BackendDAE::IndexType::ABSOLUTE, Some(funcs.clone()), BackendDAEUtil::isInitializationDAE(ishared.clone()))?;
                    let __pa0 = ::match_deref::match_deref! { match &(SymbolicJacobian::calculateJacobian(vars.clone(), eqns.clone(), m.clone(), true, ishared.clone())) {
                        (Some(__pa0), _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    jac = __pa0.clone();
                    (beqs, _) = BackendDAEUtil::getEqnSysRhs(eqns.clone(), vars.clone(), Some(funcs.clone()))?;
                    beqs = beqs.clone().reverse();
                    matrix = arrayCreate(size.clone(), metamodelica::nil());
                    transformJacToMatrix(jac.clone(), 1, 1, size.clone(), beqs.clone(), matrix.clone())?;
                    (tvars, teqns) = gaussElimination(1, size.clone(), matrix.clone(), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()), BackendEquation::listEquation(metamodelica::nil())?, (1, 1))?;
                    eqn_lst = BackendEquation::equationList(teqns.clone())?;
                    var_lst = BackendVariable::varList(tvars.clone())?;
                    syst = List::fold(eqn_lst.clone(), (std::sync::Arc::new(BackendEquation::equationAddDAE) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> + 'static>), isyst.clone())?;
                    syst = List::fold(var_lst.clone(), (std::sync::Arc::new(BackendVariable::addVarDAE) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> + 'static>), syst.clone())?;
                    crefexplst = List::map(BackendVariable::varList(vars.clone())?, (std::sync::Arc::new(makeCrefExps) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<Arc<DAE::Exp>> + 'static>))?;
                    crefexps = metamodelica::arrayFromVec(crefexplst.clone().into_iter().cloned().collect());
                    neweqns = makeGausElimination(1, size.clone(), matrix.clone(), crefexps.clone(), metamodelica::nil())?;
                    Util::profilerstop1()?;
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Gaus Elimination time: ")); __mm_s.push_str(&*realString(Util::profilertime1())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Util::profilerreset1();
                    Util::profilerstart1()?;
                    syst = replaceEquationsAddNew(eindex.clone(), neweqns.clone(), syst.clone())?;
                    Util::profilerstop2()?;
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Gesamt  time: ")); __mm_s.push_str(&*realString(Util::profilertime2())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Util::profilerreset1();
                    Util::profilerstart1()?;
                    metamodelica::print((literal!("Ok system relaxed\n")).clone());
                    (syst, shared, _) = relaxSystem1(syst.clone(), shared.clone(), comps.clone())?;
                    Ok((syst.clone(), shared.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: _, tail: comps }) => {
                    let mut b: bool;
                    let mut syst: Arc<BackendDAE::EqSystem>;
                    let mut shared: Arc<BackendDAE::Shared>;
                    (syst, shared, b) = relaxSystem1(isyst.clone(), ishared.clone(), comps.clone())?;
                    Ok((syst.clone(), shared.clone(), b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((osyst, oshared, outRunMatching))
}

fn removeRootConnections(mut orphan: i32, mut orphansarray: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut roots: Arc<metamodelica::List<i32>>) -> () {
    let () = 'mc: {
        let __mc_input = roots.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut lst: Arc<metamodelica::List<i32>>;
                    lst = ({let __elt = orphansarray.borrow()[(orphan-1) as usize].clone(); __elt});
                    let true = (intGt((lst.clone().len() as i32), 1)) else { bail!("pattern mismatch") };
                    lst = List::fold1(roots.clone(), (std::sync::Arc::new(List::removeOnTrue) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), lst.clone())?;
                    metamodelica::arrayUpdate(orphansarray.clone(), orphan, lst.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    ()
}

fn replaceFinalParameter(mut itpl: (Arc<DAE::Exp>, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, BackendDAE::Variables)> {
    let mut outTpl: (Arc<DAE::Exp>, BackendDAE::Variables);
    let mut e: Arc<DAE::Exp>;
    let mut knvars: BackendDAE::Variables;
    let mut b: bool;
    (e, knvars) = itpl;
    let (__pa0, (__pa1, __pa2)) = Expression::traverseExpBottomUp(e, (std::sync::Arc::new(fnptr!(traverserExpreplaceFinalParameter, Arc<DAE::Exp>, (BackendDAE::Variables, bool))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, bool)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, bool))> + 'static>), (knvars, false))?;
    e = __pa0.clone();
    knvars = __pa1.clone();
    b = __pa2.clone();
    (e, _) = ExpressionSimplify::condsimplify(b, e)?;
    outTpl = (e, knvars);
    Ok(outTpl)
}

fn traverserExpreplaceFinalParameter(mut inExp: Arc<DAE::Exp>, mut tpl: (BackendDAE::Variables, bool)) -> (Arc<DAE::Exp>, (BackendDAE::Variables, bool)) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (BackendDAE::Variables, bool);
    (outExp, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), tpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (knvars, _)) => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut v: BackendDAE::Var;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), knvars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: _ }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    v = __pa0.clone();
                    let true = (BackendVariable::isFinalVar(v.clone())) else { bail!("pattern mismatch") };
                    e1 = BackendVariable::varBindExpStartValue(v.clone())?;
                    Ok((e1.clone(), (knvars.clone(), true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outTpl)
}

fn replaceEquationsAddNew(mut inEqnIndxes: Arc<metamodelica::List<i32>>, mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inEqSystem: Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inEqnIndxes, inEqns.clone(), inEqSystem.clone())) {
        (Deref @ metamodelica::List::Nil, _, _) => {
            return Ok(BackendEquation::equationsAddDAE(inEqns, inEqSystem)?)
        },
        (Deref @ metamodelica::List::Cons { head: index, tail: indices }, Deref @ metamodelica::List::Cons { head: eqn, tail: eqns }, Deref @ BackendDAE::EqSystem { orderedEqs, .. }) => {
            let mut eqSystem: Arc<BackendDAE::EqSystem>;
            eqSystem = BackendDAEUtil::setEqSystEqs(inEqSystem, BackendEquation::setAtIndex(orderedEqs.clone(), index.clone(), eqn.clone())?);
            { (inEqnIndxes, inEqns, inEqSystem) = (indices.clone(), eqns.clone(), eqSystem); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn dumpVar(mut id: i32, mut vars: BackendDAE::Variables) -> Result<()> {
    let mut v: BackendDAE::Var;
    v = BackendVariable::getVarAt(vars, id)?;
    metamodelica::print((ComponentReferenceBasics::printComponentRefStr(BackendVariable::varCref(v)?)?).clone());
    metamodelica::print((literal!("\n")).clone());
    Ok(())
}

fn transposeOrphanVec(mut c: i32, mut vec3: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inId: i32) -> i32 {
    let mut outId: i32;
    outId = 'mc: {
        let __mc_input = inId;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut lst: Arc<metamodelica::List<i32>>;
            let true = (intGt(c, 0)) else { bail!("pattern mismatch") };
            lst = ({let __elt = vec3.borrow()[(c-1) as usize].clone(); __elt});
            metamodelica::arrayUpdate(vec3.clone(), c, metamodelica::cons(inId, lst.clone()))?;
            Ok(inId + 1)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(inId + 1)
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outId
}

fn markOrphans(mut o: i32, mut mark: i32, mut rowmark: metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> {
    let mut orowmark: metamodelica::Array<i32>;
    orowmark = metamodelica::arrayUpdate(rowmark.clone(), o, mark)?;
    Ok(orowmark)
}

fn generateCliquesResidual(mut inOrphans: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>, mut vars: BackendDAE::Variables, mut iconstraints: Arc<metamodelica::List<i32>>) -> (i32, Arc<metamodelica::List<i32>>) {
    let mut omark: i32 = 0;
    let mut oconstraints: Arc<metamodelica::List<i32>>;
    (omark, oconstraints) = 'mc: {
        let __mc_input = inOrphans;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((mark + 2, iconstraints.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: o, tail: rest } => {
                    let mut constraints: Arc<metamodelica::List<i32>>;
                    let mut rlst: Arc<metamodelica::List<i32>>;
                    let mut elst: Arc<metamodelica::List<i32>>;
                    let mut partner: Arc<metamodelica::List<i32>>;
                    let mut foundflow: bool;
                    let mut blst: Arc<metamodelica::List<bool>>;
                    let mut vlst: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut omark: i32 = omark.clone();
                    let false = (intEq(({let __elt = colummarks.borrow()[(o.clone()-1) as usize].clone(); __elt}), mark)) else { bail!("pattern mismatch") };
                    metamodelica::arrayUpdate(colummarks.clone(), o.clone(), mark)?;
                    rlst = ({let __elt = m.borrow()[(o.clone()-1) as usize].clone(); __elt});
                    elst = List::select1(List::flatten(List::map1r(rlst.clone(), (std::sync::Arc::new(arrayGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), mt.clone())?)?, (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                    partner = List::select1(elst.clone(), (std::sync::Arc::new(fnptr!(isResOrphan, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<bool> + 'static>), ass2.clone())?;
                    partner = List::uniqueIntN(List::removeOnTrue(o.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), partner.clone())?, metamodelica::arrayLength(colummarks.clone()))?;
                    List::map2_0(partner.clone(), (std::sync::Arc::new(doMark) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<()> + 'static>), colummarks.clone(), mark)?;
                    vlst = List::map1r(rlst.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone())?;
                    blst = List::map(vlst.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::isFlowVar, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>))?;
                    foundflow = List::any(blst.clone(), std::sync::Arc::new(fnptr!(Util::id, _)))?;
                    rlst = selectNonFlows(rlst.clone(), blst.clone())?;
                    foundflow = generateCliquesResidual1(rlst.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), foundflow.clone(), vars.clone())?;
                    generateCliquesResidual2(rlst.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark + 1, rowmarks.clone(), colummarks.clone(), metamodelica::cons(o.clone(), partner.clone()))?;
                    constraints = if (!(foundflow.clone())) {listAppend(metamodelica::cons(o.clone(), partner.clone()), iconstraints.clone())} else {iconstraints.clone()};
                    (omark, constraints) = generateCliquesResidual(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), vars.clone(), constraints.clone());
                    Ok(((omark, constraints.clone()), omark.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { omark = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    let mut constraints: Arc<metamodelica::List<i32>>;
                    let mut omark: i32 = omark.clone();
                    (omark, constraints) = generateCliquesResidual(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), vars.clone(), iconstraints.clone());
                    Ok(((omark, constraints.clone()), omark.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { omark = __wb0; break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (omark, oconstraints)
}

fn generateCliquesResidual1(mut rows: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>, mut ifoundFlow: bool, mut vars: BackendDAE::Variables) -> Result<bool> {
    let mut ofoundFlow: bool = ifoundFlow;
    let mut e: i32;
    let mut next: Arc<metamodelica::List<i32>>;
    let mut rlst: Arc<metamodelica::List<i32>>;
    let mut b1: bool;
    let mut blst: Arc<metamodelica::List<bool>>;
    let mut vlst: Arc<metamodelica::List<BackendDAE::Var>>;
    for mut r in &*rows {
        let mut r = r.clone();
        if !(intEq(({let __elt = rowmarks.borrow()[(r.clone()-1) as usize].clone(); __elt}), mark)) {
            next = List::select1(({let __elt = mt.borrow()[(r.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(isNoResOrphan, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<bool> + 'static>), ass2.clone())?;
            next = List::select2(next.clone(), (std::sync::Arc::new(fnptr!(unmarked, i32, metamodelica::Array<i32>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), colummarks.clone(), mark)?;
            next = List::removeOnTrue(({let __elt = ass1.borrow()[(r.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), next.clone())?;
            if next.clone().is_empty() {
                metamodelica::arrayUpdate(rowmarks.clone(), r.clone(), mark)?;
                e = ({let __elt = ass1.borrow()[(r.clone()-1) as usize].clone(); __elt});
                metamodelica::arrayUpdate(colummarks.clone(), e, mark)?;
                rlst = ({let __elt = ass2.borrow()[(e-1) as usize].clone(); __elt});
                next = List::fold1(rlst.clone(), (std::sync::Arc::new(List::removeOnTrue) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), ({let __elt = m.borrow()[(e-1) as usize].clone(); __elt}))?;
                vlst = List::map1r(next.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone())?;
                blst = List::map(vlst.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::isFlowVar, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>))?;
                b1 = List::any(blst.clone(), std::sync::Arc::new(fnptr!(Util::id, _)))?;
                next = selectNonFlows(next.clone(), blst.clone())?;
                ofoundFlow = generateCliquesResidual1(next.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), b1 || ofoundFlow, vars.clone())?;
            }
        }
    }
    Ok(ofoundFlow)
}

fn selectNonFlows(mut rows: Arc<metamodelica::List<i32>>, mut flowFlag: Arc<metamodelica::List<bool>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut oAcc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut brest: Arc<metamodelica::List<bool>> = flowFlag.clone();
    let mut b: bool;
    for mut r in &*rows {
        let mut r = r.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(brest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        b = __pa0.clone();
        brest = __pa1.clone();
        if !(b) {
            oAcc = metamodelica::cons(r.clone(), oAcc.clone());
        }
    }
    Ok(oAcc)
}

fn generateCliquesResidual2(mut eqns: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>, mut orphan: Arc<metamodelica::List<i32>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(eqns) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: r, tail: rest } if (!(intEq(({let __elt = rowmarks.borrow()[(r.clone()-1) as usize].clone(); __elt}), mark))) => {
            let mut e: i32;
            let mut lst: Arc<metamodelica::List<i32>>;
            let mut rlst: Arc<metamodelica::List<i32>>;
            let mut lst1: Arc<metamodelica::List<i32>>;
            e = ({let __elt = ass1.borrow()[(r.clone()-1) as usize].clone(); __elt});
            rlst = ({let __elt = ass2.borrow()[(e-1) as usize].clone(); __elt});
            lst = List::fold1(rlst.clone(), (std::sync::Arc::new(List::removeOnTrue) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), ({let __elt = m.borrow()[(e-1) as usize].clone(); __elt}))?;
            let __pa0 = ::match_deref::match_deref! { match &(List::select2(lst.clone(), (std::sync::Arc::new(fnptr!(unmarked, i32, metamodelica::Array<i32>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), rowmarks.clone(), mark - 1)?) {
                __pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: _ } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            lst1 = __pa0.clone();
            List::map4_0(lst1, (std::sync::Arc::new(generateResidualClique) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>, i32) -> Result<()> + 'static>), m.clone(), mt.clone(), orphan.clone(), e)?;
            List::map2_0(rlst, (std::sync::Arc::new(doMark) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<()> + 'static>), rowmarks.clone(), mark)?;
            lst = List::select2(lst, (std::sync::Arc::new(fnptr!(marked, i32, metamodelica::Array<i32>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), rowmarks.clone(), mark - 1)?;
            metamodelica::arrayUpdate(colummarks.clone(), e, mark)?;
            generateCliquesResidual2(lst, ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), orphan.clone())?;
            generateCliquesResidual2(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), orphan)?;
            ()
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
            generateCliquesResidual2(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), orphan)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn prepairOrphansOrder(mut inOrphans: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>, mut orphans: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut vars: BackendDAE::Variables, mut iroots: Arc<metamodelica::List<i32>>, mut iconstraints: Arc<metamodelica::List<i32>>) -> Result<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut omark: i32 = 0;
    let mut oroots: Arc<metamodelica::List<i32>>;
    let mut oconstraints: Arc<metamodelica::List<i32>>;
    (omark, oroots, oconstraints) = (::match_deref::match_deref! { match &(inOrphans) {
        Deref @ metamodelica::List::Nil => {
            (mark, iroots, iconstraints)
        },
        Deref @ metamodelica::List::Cons { head: o, tail: rest } if (!(intEq(({let __elt = rowmarks.borrow()[(o.clone()-1) as usize].clone(); __elt}), mark))) => {
            let mut roots: Arc<metamodelica::List<i32>>;
            let mut constraints: Arc<metamodelica::List<i32>>;
            let mut elst: Arc<metamodelica::List<i32>>;
            let mut rlst: Arc<metamodelica::List<i32>>;
            let mut foundflow: bool;
            let mut constr: bool;
            let mut vlst: Arc<metamodelica::List<BackendDAE::Var>>;
            metamodelica::arrayUpdate(rowmarks.clone(), o.clone(), mark)?;
            elst = ({let __elt = mt.borrow()[(o.clone()-1) as usize].clone(); __elt});
            rlst = List::flatten(List::map1r(elst, (std::sync::Arc::new(arrayGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), ass2.clone())?)?;
            vlst = List::map1r(rlst, (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone())?;
            constr = List::all(vlst, (std::sync::Arc::new(fnptr!(BackendVariable::isFlowVar, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>))?;
            constraints = List::consOnTrue(constr, o.clone(), iconstraints);
            foundflow = prepairOrphansOrder1(({let __elt = mt.borrow()[(o.clone()-1) as usize].clone(); __elt}), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), o.clone(), orphans.clone(), list![o.clone()], false, vars.clone())?;
            roots = List::consOnTrue(foundflow && !(constr), o.clone(), iroots);
            (omark, roots, constraints) = prepairOrphansOrder(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark + 1, rowmarks.clone(), colummarks.clone(), orphans.clone(), vars, roots, constraints)?;
            (omark, roots, constraints)
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
            let mut roots: Arc<metamodelica::List<i32>>;
            let mut constraints: Arc<metamodelica::List<i32>>;
            (omark, roots, constraints) = prepairOrphansOrder(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), orphans.clone(), vars, iroots, iconstraints)?;
            (omark, roots, constraints)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((omark, oroots, oconstraints))
}

fn prepairOrphansOrder1(mut eqns: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>, mut preorphan: i32, mut orphans: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut prer: Arc<metamodelica::List<i32>>, mut ifoundFlow: bool, mut vars: BackendDAE::Variables) -> Result<bool> {
    let mut ofoundFlow: bool = ifoundFlow;
    let mut next: Arc<metamodelica::List<i32>>;
    let mut r: Arc<metamodelica::List<i32>>;
    let mut elst: Arc<metamodelica::List<i32>>;
    let mut b1: bool;
    let mut vlst: Arc<metamodelica::List<BackendDAE::Var>>;
    for mut e in &*eqns {
        let mut e = e.clone();
        if !(intEq(({let __elt = colummarks.borrow()[(e.clone()-1) as usize].clone(); __elt}), mark)) {
            next = List::select1(({let __elt = m.borrow()[(e.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(isNoOrphan, i32, metamodelica::Array<i32>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>), ass1.clone())?;
            next = List::select2(next.clone(), (std::sync::Arc::new(fnptr!(unmarked, i32, metamodelica::Array<i32>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), rowmarks.clone(), mark)?;
            next = List::fold1(({let __elt = ass2.borrow()[(e.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(List::removeOnTrue) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), next.clone())?;
            if next.clone().is_empty() {
                metamodelica::arrayUpdate(colummarks.clone(), e.clone(), mark)?;
                r = ({let __elt = ass2.borrow()[(e.clone()-1) as usize].clone(); __elt});
                List::map2_0(r.clone(), (std::sync::Arc::new(doMark) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<()> + 'static>), rowmarks.clone(), mark)?;
                elst = List::select1(List::map1r(r.clone(), (std::sync::Arc::new(arrayGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), ass1.clone())?, (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                next = List::flatten(List::map1r(r.clone(), (std::sync::Arc::new(arrayGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), mt.clone())?)?;
                next = List::fold1(elst.clone(), (std::sync::Arc::new(List::removeOnTrue) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), next.clone())?;
                List::map2_0(r.clone(), (std::sync::Arc::new(addPreOrphan) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> + 'static>), preorphan, orphans.clone())?;
                vlst = List::map1r(r.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone())?;
                b1 = List::any(vlst.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::isFlowVar, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>))?;
                ofoundFlow = prepairOrphansOrder1(next.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), preorphan, orphans.clone(), r.clone(), b1 || ofoundFlow, vars.clone())?;
            }
        }
    }
    Ok(ofoundFlow)
}

fn prepairOrphansOrder2(mut inOrphans: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut imark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>, mut orphans: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> i32 {
    let mut omark: i32;
    omark = 'mc: {
        let __mc_input = inOrphans;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(imark + 1)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: o, tail: rest } => {
                    let mut elst: Arc<metamodelica::List<i32>>;
                    let mut rlst: Arc<metamodelica::List<i32>>;
                    let mut partner: Arc<metamodelica::List<i32>>;
                    let false = (intEq(({let __elt = rowmarks.borrow()[(o.clone()-1) as usize].clone(); __elt}), imark)) else { bail!("pattern mismatch") };
                    metamodelica::arrayUpdate(rowmarks.clone(), o.clone(), imark)?;
                    elst = List::select1(({let __elt = mt.borrow()[(o.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                    rlst = List::select1(List::flatten(List::map1r(elst.clone(), (std::sync::Arc::new(arrayGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), m.clone())?)?, (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                    partner = List::select1(rlst.clone(), (std::sync::Arc::new(fnptr!(isOrphan, i32, metamodelica::Array<i32>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>), ass1.clone())?;
                    partner = List::unique(partner.clone());
                    List::map2_0(partner.clone(), (std::sync::Arc::new(doMark) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<()> + 'static>), rowmarks.clone(), imark)?;
                    prepairOrphansOrder3(({let __elt = mt.borrow()[(o.clone()-1) as usize].clone(); __elt}), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), imark, rowmarks.clone(), colummarks.clone(), o.clone(), partner.clone(), orphans.clone(), list![o.clone()]);
                    Ok(prepairOrphansOrder2(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), imark, rowmarks.clone(), colummarks.clone(), orphans.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(prepairOrphansOrder2(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), imark, rowmarks.clone(), colummarks.clone(), orphans.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    omark
}

fn prepairOrphansOrder3(mut eqns: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>, mut preorphan: i32, mut partner: Arc<metamodelica::List<i32>>, mut orphans: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut prer: Arc<metamodelica::List<i32>>) -> () {
    let () = 'mc: {
        let __mc_input = eqns;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: e, tail: rest } => {
                    let mut next: Arc<metamodelica::List<i32>>;
                    let mut r: Arc<metamodelica::List<i32>>;
                    let mut elst: Arc<metamodelica::List<i32>>;
                    let mut lst: Arc<metamodelica::List<i32>>;
                    let false = (intEq(({let __elt = colummarks.borrow()[(e.clone()-1) as usize].clone(); __elt}), mark)) else { bail!("pattern mismatch") };
                    r = ({let __elt = ass2.borrow()[(e.clone()-1) as usize].clone(); __elt});
                    lst = List::unique(List::flatten(List::map1r(r.clone(), (std::sync::Arc::new(arrayGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), orphans.clone())?)?);
                    let true = (listMember(preorphan, lst.clone())) else { bail!("pattern mismatch") };
                    metamodelica::arrayUpdate(colummarks.clone(), e.clone(), mark)?;
                    List::map2_0(r.clone(), (std::sync::Arc::new(doMark) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<()> + 'static>), rowmarks.clone(), mark)?;
                    elst = List::select1(List::map1r(r.clone(), (std::sync::Arc::new(arrayGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), ass1.clone())?, (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                    next = List::flatten(List::map1r(r.clone(), (std::sync::Arc::new(arrayGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), mt.clone())?)?;
                    next = List::fold1(elst.clone(), (std::sync::Arc::new(List::removeOnTrue) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), next.clone())?;
                    prepairOrphansOrder3(next.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), preorphan, partner.clone(), orphans.clone(), r.clone());
                    prepairOrphansOrder3(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), preorphan, partner.clone(), orphans.clone(), prer.clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: e, tail: rest } => {
                    let false = (intEq(({let __elt = colummarks.borrow()[(e.clone()-1) as usize].clone(); __elt}), mark)) else { bail!("pattern mismatch") };
                    List::map4_0(prer.clone(), (std::sync::Arc::new(generateClique) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>, i32) -> Result<()> + 'static>), m.clone(), mt.clone(), partner.clone(), e.clone())?;
                    prepairOrphansOrder3(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), preorphan, partner.clone(), orphans.clone(), prer.clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    prepairOrphansOrder3(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), preorphan, partner.clone(), orphans.clone(), prer.clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    ()
}

fn generateClique(mut r: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut orphans: Arc<metamodelica::List<i32>>, mut e: i32) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(orphans) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: orphan, tail: rest } => {
            let mut lst: Arc<metamodelica::List<i32>>;
            lst = ({let __elt = mt.borrow()[(r-1) as usize].clone(); __elt});
            lst = List::removeOnTrue(e, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), lst)?;
            metamodelica::arrayUpdate(mt.clone(), r, lst)?;
            lst = ({let __elt = mt.borrow()[(orphan.clone()-1) as usize].clone(); __elt});
            lst = List::unique(metamodelica::cons(e, lst));
            metamodelica::arrayUpdate(mt.clone(), orphan.clone(), lst)?;
            lst = ({let __elt = m.borrow()[(e-1) as usize].clone(); __elt});
            lst = List::removeOnTrue(r, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), lst)?;
            lst = List::unique(metamodelica::cons(orphan.clone(), lst));
            metamodelica::arrayUpdate(m.clone(), e, lst)?;
            generateClique(r, m.clone(), mt.clone(), rest.clone(), e)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn generateResidualClique(mut r: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut orphans: Arc<metamodelica::List<i32>>, mut e: i32) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(orphans) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: orphan, tail: rest } => {
            let mut lst: Arc<metamodelica::List<i32>>;
            lst = ({let __elt = m.borrow()[(e-1) as usize].clone(); __elt});
            lst = List::removeOnTrue(r, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), lst)?;
            metamodelica::arrayUpdate(m.clone(), e, lst)?;
            lst = ({let __elt = m.borrow()[(orphan.clone()-1) as usize].clone(); __elt});
            lst = List::unique(metamodelica::cons(r, lst));
            metamodelica::arrayUpdate(m.clone(), orphan.clone(), lst)?;
            lst = ({let __elt = mt.borrow()[(r-1) as usize].clone(); __elt});
            lst = List::removeOnTrue(e, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), lst)?;
            lst = List::unique(metamodelica::cons(orphan.clone(), lst));
            metamodelica::arrayUpdate(mt.clone(), r, lst)?;
            generateResidualClique(r, m.clone(), mt.clone(), rest.clone(), e)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn getOrphansOrderEdvanced(mut inOrphans: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mc: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mct: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>, mut orphans: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> i32 {
    let mut omark: i32;
    omark = 'mc: {
        let __mc_input = inOrphans;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(mark)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: o, tail: rest } => {
                    let false = (intEq(({let __elt = rowmarks.borrow()[(o.clone()-1) as usize].clone(); __elt}), mark)) else { bail!("pattern mismatch") };
                    metamodelica::arrayUpdate(rowmarks.clone(), o.clone(), mark)?;
                    getOrphansOrderEdvanced1(({let __elt = mct.borrow()[(o.clone()-1) as usize].clone(); __elt}), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), o.clone(), orphans.clone(), metamodelica::nil())?;
                    Ok(getOrphansOrderEdvanced(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mc.clone(), mct.clone(), mark + 1, rowmarks.clone(), colummarks.clone(), orphans.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(getOrphansOrderEdvanced(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mc.clone(), mct.clone(), mark, rowmarks.clone(), colummarks.clone(), orphans.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    omark
}

fn hasOrphanAdvanced(mut rows: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut iAcc: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((rows, iAcc.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => {
            return Ok(iAcc)
        },
        (Deref @ metamodelica::List::Cons { head: r, tail: rest }, _) => {
            if (!(intGt(({let __elt = ass1.borrow()[(r.clone()-1) as usize].clone(); __elt}), 0))) {{ (rows, ass1, iAcc) = (rest.clone(), ass1.clone(), metamodelica::cons(r.clone(), iAcc)); continue '__tco; }} else {{ (rows, ass1, iAcc) = (rest.clone(), ass1.clone(), iAcc); continue '__tco; }}
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn addPreOrphan(mut orphan: i32, mut preorphan: i32, mut arr: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    let mut olst: Arc<metamodelica::List<i32>>;
    olst = ({let __elt = arr.borrow()[(orphan-1) as usize].clone(); __elt});
    olst = List::unionElt(preorphan, olst);
    metamodelica::arrayUpdate(arr.clone(), orphan, olst)?;
    Ok(())
}

fn addPreOrphans(mut orphan: i32, mut preorphans: Arc<metamodelica::List<i32>>, mut arr: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(preorphans) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: o, tail: rest } => {
            addPreOrphan(orphan, o.clone(), arr.clone())?;
            addPreOrphans(orphan, rest.clone(), arr.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn getOrphansOrderEdvanced1(mut eqns: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>, mut preorphan: i32, mut orphans: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut nextQueue: Arc<metamodelica::List<i32>>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (eqns, nextQueue.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    getOrphansOrderEdvanced1(nextQueue.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), preorphan, orphans.clone(), metamodelica::nil())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: e, tail: _ }, _) => {
                    let mut r: Arc<metamodelica::List<i32>>;
                    let mut olst: Arc<metamodelica::List<i32>>;
                    let false = (intEq(({let __elt = colummarks.borrow()[(e.clone()-1) as usize].clone(); __elt}), mark)) else { bail!("pattern mismatch") };
                    r = List::removeOnTrue(preorphan, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), ({let __elt = m.borrow()[(e.clone()-1) as usize].clone(); __elt}))?;
                    olst = hasOrphanAdvanced(r.clone(), ass1.clone(), metamodelica::nil())?;
                    metamodelica::arrayUpdate(colummarks.clone(), e.clone(), mark)?;
                    addPreOrphans(preorphan, olst.clone(), orphans.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: e, tail: rest }, _) => {
                    let mut next: Arc<metamodelica::List<i32>>;
                    let mut r: Arc<metamodelica::List<i32>>;
                    let mut r1: Arc<metamodelica::List<i32>>;
                    let mut elst: Arc<metamodelica::List<i32>>;
                    let false = (intEq(({let __elt = colummarks.borrow()[(e.clone()-1) as usize].clone(); __elt}), mark)) else { bail!("pattern mismatch") };
                    r = List::removeOnTrue(preorphan, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), ({let __elt = m.borrow()[(e.clone()-1) as usize].clone(); __elt}))?;
                    r1 = List::select1(({let __elt = ass2.borrow()[(e.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                    r = List::fold1(r1.clone(), (std::sync::Arc::new(List::removeOnTrue) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), r.clone())?;
                    elst = List::select1(List::map1r(r.clone(), (std::sync::Arc::new(arrayGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), ass1.clone())?, (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                    next = listAppend(nextQueue.clone(), elst.clone());
                    metamodelica::arrayUpdate(colummarks.clone(), e.clone(), mark)?;
                    getOrphansOrderEdvanced1(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), preorphan, orphans.clone(), next.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _) => {
                    getOrphansOrderEdvanced1(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), preorphan, orphans.clone(), nextQueue.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn getConstraintesOrphansOrderEdvanced(mut inOrphans: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mc: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mct: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>, mut orphans: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> i32 {
    let mut omark: i32;
    omark = 'mc: {
        let __mc_input = inOrphans;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(mark)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: o, tail: rest } => {
                    let false = (intEq(({let __elt = rowmarks.borrow()[(o.clone()-1) as usize].clone(); __elt}), mark)) else { bail!("pattern mismatch") };
                    metamodelica::arrayUpdate(rowmarks.clone(), o.clone(), mark)?;
                    getConstraintesOrphansOrderEdvanced1(({let __elt = mct.borrow()[(o.clone()-1) as usize].clone(); __elt}), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), o.clone(), orphans.clone(), metamodelica::nil())?;
                    Ok(getConstraintesOrphansOrderEdvanced(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mc.clone(), mct.clone(), mark + 1, rowmarks.clone(), colummarks.clone(), orphans.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(getConstraintesOrphansOrderEdvanced(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mc.clone(), mct.clone(), mark, rowmarks.clone(), colummarks.clone(), orphans.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    omark
}

fn getConstraintesOrphansOrderEdvanced1(mut eqns: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>, mut preorphan: i32, mut orphans: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut nextQueue: Arc<metamodelica::List<i32>>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (eqns, nextQueue.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    getConstraintesOrphansOrderEdvanced1(nextQueue.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), preorphan, orphans.clone(), metamodelica::nil())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: e, tail: rest }, _) => {
                    let mut next: Arc<metamodelica::List<i32>>;
                    let mut r: Arc<metamodelica::List<i32>>;
                    let mut r1: Arc<metamodelica::List<i32>>;
                    let mut elst: Arc<metamodelica::List<i32>>;
                    let mut olst: Arc<metamodelica::List<i32>>;
                    let false = (intEq(({let __elt = colummarks.borrow()[(e.clone()-1) as usize].clone(); __elt}), mark)) else { bail!("pattern mismatch") };
                    r = List::removeOnTrue(preorphan, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), ({let __elt = m.borrow()[(e.clone()-1) as usize].clone(); __elt}))?;
                    olst = hasOrphanAdvanced(r.clone(), ass1.clone(), metamodelica::nil())?;
                    metamodelica::arrayUpdate(colummarks.clone(), e.clone(), mark)?;
                    addPreOrphans(preorphan, olst.clone(), orphans.clone())?;
                    r1 = ({let __elt = ass2.borrow()[(e.clone()-1) as usize].clone(); __elt});
                    r = List::fold1(r1.clone(), (std::sync::Arc::new(List::removeOnTrue) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), r.clone())?;
                    elst = List::select1(List::map1r(r.clone(), (std::sync::Arc::new(arrayGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), ass1.clone())?, (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                    next = listAppend(nextQueue.clone(), elst.clone());
                    getConstraintesOrphansOrderEdvanced1(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), preorphan, orphans.clone(), next.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: e, tail: rest }, _) => {
                    let mut next: Arc<metamodelica::List<i32>>;
                    let mut r: Arc<metamodelica::List<i32>>;
                    let mut r1: Arc<metamodelica::List<i32>>;
                    let mut elst: Arc<metamodelica::List<i32>>;
                    let false = (intEq(({let __elt = colummarks.borrow()[(e.clone()-1) as usize].clone(); __elt}), mark)) else { bail!("pattern mismatch") };
                    r = List::removeOnTrue(preorphan, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), ({let __elt = m.borrow()[(e.clone()-1) as usize].clone(); __elt}))?;
                    r1 = ({let __elt = ass2.borrow()[(e.clone()-1) as usize].clone(); __elt});
                    r = List::fold1(r1.clone(), (std::sync::Arc::new(List::removeOnTrue) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), r.clone())?;
                    elst = List::select1(List::map1r(r.clone(), (std::sync::Arc::new(arrayGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), ass1.clone())?, (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                    next = listAppend(nextQueue.clone(), elst.clone());
                    metamodelica::arrayUpdate(colummarks.clone(), e.clone(), mark)?;
                    getConstraintesOrphansOrderEdvanced1(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), preorphan, orphans.clone(), next.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _) => {
                    getConstraintesOrphansOrderEdvanced1(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), preorphan, orphans.clone(), nextQueue.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn mergeOrphanParents(mut links: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iAcc: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<i32>> {
    let mut oAcc: Arc<metamodelica::List<i32>>;
    oAcc = 'mc: {
        let __mc_input = links;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(iAcc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: l, tail: rest } => {
                    ::match_deref::match_deref! { match &(({let __elt = m.borrow()[(l.clone()-1) as usize].clone(); __elt})) {
                        Deref @ metamodelica::List::Nil => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok(mergeOrphanParents(rest.clone(), m.clone(), iAcc.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: l, tail: rest } => {
                    let mut lst: Arc<metamodelica::List<i32>>;
                    lst = ({let __elt = m.borrow()[(l.clone()-1) as usize].clone(); __elt});
                    Ok(mergeOrphanParents(rest.clone(), m.clone(), listAppend(lst.clone(), iAcc.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    oAcc
}

fn getLinkPosition(mut orphans: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut iAcc: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<i32>> {
    let mut ochilds: Arc<metamodelica::List<i32>>;
    ochilds = 'mc: {
        let __mc_input = orphans;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(iAcc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: o, tail: rest } => {
                    let mut childs: Arc<metamodelica::List<i32>>;
                    let false = (intEq(({let __elt = rowmarks.borrow()[(o.clone()-1) as usize].clone(); __elt}), mark)) else { bail!("pattern mismatch") };
                    metamodelica::arrayUpdate(rowmarks.clone(), o.clone(), mark)?;
                    childs = getLinkPosition1(({let __elt = m.borrow()[(o.clone()-1) as usize].clone(); __elt}), m.clone(), mt.clone(), mark, rowmarks.clone(), o.clone(), iAcc.clone())?;
                    Ok(getLinkPosition(rest.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), childs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(getLinkPosition(rest.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), iAcc.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    ochilds
}

fn getLinkPosition1(mut orphans: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut preorphan: i32, mut iAcc: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut childs: Arc<metamodelica::List<i32>>;
    childs = 'mc: {
        let __mc_input = orphans.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(metamodelica::cons(preorphan, iAcc.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: o, tail: Deref @ metamodelica::List::Nil } => {
                    let false = (intEq(({let __elt = rowmarks.borrow()[(o.clone()-1) as usize].clone(); __elt}), mark)) else { bail!("pattern mismatch") };
                    metamodelica::arrayUpdate(rowmarks.clone(), o.clone(), mark)?;
                    Ok(getLinkPosition1(({let __elt = m.borrow()[(o.clone()-1) as usize].clone(); __elt}), m.clone(), mt.clone(), mark, rowmarks.clone(), o.clone(), iAcc.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: o, tail: Deref @ metamodelica::List::Nil } => {
                    let mut lst: Arc<metamodelica::List<i32>>;
                    let true = (intEq(({let __elt = rowmarks.borrow()[(o.clone()-1) as usize].clone(); __elt}), mark)) else { bail!("pattern mismatch") };
                    lst = listAppend(({let __elt = mt.borrow()[(0-1) as usize].clone(); __elt}), iAcc.clone());
                    Ok(lst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error in getLinkPosition1! Found Orphan with more than one parents ")); __mm_s.push_str(&*stringDelimitList(List::map(orphans.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(childs)
}

fn getOrphansOrderEdvanced5(mut linklst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut imark: i32, mut rowmarks: metamodelica::Array<i32>, mut iAcc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> (Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32) {
    '__tco: loop {
        ::match_deref::match_deref! { match &(linklst) {
        Deref @ metamodelica::List::Nil => {
            return (iAcc.reverse(), imark)
        },
        Deref @ metamodelica::List::Cons { head: links, tail: rest } => {
            let mut mark: i32;
            let mut lst: Arc<metamodelica::List<i32>>;
            let mut childs: Arc<metamodelica::List<i32>>;
            let mut acc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            lst = mergeOrphanParents(links.clone(), m.clone(), metamodelica::nil());
            childs = getLinkPosition(lst, m.clone(), mt.clone(), imark, rowmarks.clone(), metamodelica::nil());
            { (linklst, m, mt, imark, rowmarks, iAcc) = (rest.clone(), m.clone(), mt.clone(), imark + 1, rowmarks.clone(), metamodelica::cons(childs, iAcc)); continue '__tco; }
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

fn getOrphansOrderEdvanced6(mut linklst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut childslst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((linklst, childslst)) {
        (Deref @ metamodelica::List::Nil, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: links, tail: rest }, Deref @ metamodelica::List::Cons { head: childs, tail: acc }) => {
            let mut lst: Arc<metamodelica::List<i32>>;
            lst = List::unique(List::flatten(List::map1r(childs.clone(), (std::sync::Arc::new(arrayGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), m.clone())?)?);
            List::map2_0(links.clone(), (std::sync::Arc::new(doAssign) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>) -> Result<()> + 'static>), m.clone(), lst)?;
            List::map2_0(childs.clone(), (std::sync::Arc::new(doAssign) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>) -> Result<()> + 'static>), m.clone(), links.clone())?;
            getOrphansOrderEdvanced6(rest.clone(), acc.clone(), m.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn getOrphansOrderEdvanced4(mut linklst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut imark: i32, mut rowmarks: metamodelica::Array<i32>, mut iorder: Arc<metamodelica::List<i32>>, mut iAcc: Arc<metamodelica::List<i32>>) -> Result<i32> {
    let mut omark: i32;
    let mut childs: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    (childs, omark) = getOrphansOrderEdvanced5(linklst.clone(), m.clone(), mt.clone(), imark, rowmarks.clone(), metamodelica::nil());
    getOrphansOrderEdvanced6(linklst, childs, m.clone())?;
    Ok(omark)
}

fn getInvMap(mut orphan: i32, mut invmap: metamodelica::Array<i32>, mut index: i32) -> Result<i32> {
    let mut oindex: i32;
    metamodelica::arrayUpdate(invmap.clone(), orphan, index)?;
    oindex = index + 1;
    Ok(oindex)
}

fn getOrphansAdjacencyMatrix(mut orphans: Arc<metamodelica::List<i32>>, mut invmap: metamodelica::Array<i32>, mut vorphansarray: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut addself: bool) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>)> {
    let mut outM: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut outMT: metamodelica::Array<Arc<metamodelica::List<i32>>> = mT.clone();
    let mut m: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut lst: Arc<metamodelica::List<i32>>;
    let mut i: i32;
    for mut o in &*orphans {
        let mut o = o.clone();
        lst = List::map1r(({let __elt = vorphansarray.borrow()[(o.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(arrayGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), invmap.clone())?;
        i = ({let __elt = invmap.borrow()[(o.clone()-1) as usize].clone(); __elt});
        lst = List::consOnTrue(addself, i, lst.clone());
        outMT = List::fold1(lst.clone(), (std::sync::Arc::new(Array::consToElement) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<_> + 'static>), i, outMT.clone())?;
        m = metamodelica::cons(lst.clone(), m.clone());
    }
    outM = List::listArrayReverse(m)?;
    outMT = mT.clone();
    Ok((outM, outMT))
}

fn getOrder(mut comp: Arc<metamodelica::List<i32>>, mut inorder: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)) -> (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) {
    let mut outorder: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>);
    outorder = (::match_deref::match_deref! { match &((comp.clone(), inorder)) {
        (Deref @ metamodelica::List::Cons { head: o, tail: Deref @ metamodelica::List::Nil }, (order, links)) => {
            (metamodelica::cons(o.clone(), order.clone()), links.clone())
        },
        (_, (order, links)) => {
            (order.clone(), metamodelica::cons(comp, links.clone()))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outorder
}

fn getOrphansOrderEdvanced3(mut roots: Arc<metamodelica::List<i32>>, mut otherorphans: Arc<metamodelica::List<i32>>, mut constraints: Arc<metamodelica::List<i32>>, mut vorphans: Arc<metamodelica::List<i32>>, mut vorphansarray: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>) -> Result<(Arc<metamodelica::List<i32>>, i32)> {
    let mut sortvorphans: Arc<metamodelica::List<i32>>;
    let mut omark: i32;
    let mut order: Arc<metamodelica::List<i32>>;
    let mut size: i32;
    let mut map: metamodelica::Array<i32>;
    let mut ass: metamodelica::Array<i32>;
    let mut invmap: metamodelica::Array<i32>;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut range: Arc<metamodelica::List<i32>>;
    let mut comps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut linkslst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    map = metamodelica::arrayFromVec(vorphans.clone().into_iter().cloned().collect());
    size = metamodelica::arrayLength(map.clone());
    invmap = arrayCreate(metamodelica::arrayLength(vorphansarray.clone()), 0);
    List::fold1(vorphans.clone(), (std::sync::Arc::new(getInvMap) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<i32> + 'static>), invmap.clone(), 1)?;
    range = List::intRange(size);
    (m, mt) = getOrphansAdjacencyMatrix(vorphans.clone(), invmap.clone(), vorphansarray.clone(), arrayCreate(size, metamodelica::nil()), true)?;
    ass = metamodelica::arrayFromVec(range.into_iter().cloned().collect());
    comps = Sorting::TarjanTransposed(mt.clone(), ass.clone())?;
    (order, linkslst) = List::fold(comps.clone(), (std::sync::Arc::new(fnptr!(getOrder, Arc<metamodelica::List<i32>>, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)> + 'static>), (metamodelica::nil(), metamodelica::nil()))?;
    (m, mt) = getOrphansAdjacencyMatrix(vorphans, invmap.clone(), vorphansarray.clone(), arrayCreate(size, metamodelica::nil()), false)?;
    reduceOrphancMatrix(comps.reverse(), m.clone())?;
    omark = getOrphansOrderEdvanced4(linkslst, m.clone(), mt.clone(), mark, rowmarks.clone(), order, metamodelica::nil())?;
    mt = AdjacencyMatrix::transposeAdjacencyMatrix(m.clone(), metamodelica::arrayLength(mt.clone()))?;
    comps = Sorting::TarjanTransposed(mt.clone(), ass.clone())?;
    sortvorphans = List::flattenReverse(comps)?;
    sortvorphans = List::map1r(sortvorphans, (std::sync::Arc::new(arrayGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), map.clone())?;
    Ok((sortvorphans, omark))
}

fn reduceOrphancMatrix(mut comps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(comps) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, tail: rest } => {
            reduceOrphancMatrix(rest.clone(), m.clone())?;
            ()
        },
        Deref @ metamodelica::List::Cons { head: comp, tail: rest } => {
            reduceOrphancMatrix1(comp.clone(), comp.clone(), m.clone())?;
            reduceOrphancMatrix(rest.clone(), m.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn reduceOrphancMatrix1(mut comps: Arc<metamodelica::List<i32>>, mut comps1: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(comps) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: c, tail: rest } => {
            let mut lst: Arc<metamodelica::List<i32>>;
            lst = ({let __elt = m.borrow()[(c.clone()-1) as usize].clone(); __elt});
            lst = List::setDifference(lst, comps1.clone())?;
            metamodelica::arrayUpdate(m.clone(), c.clone(), lst.reverse())?;
            reduceOrphancMatrix1(rest.clone(), comps1, m.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn hasResidualOrphan1(mut eqns: Arc<metamodelica::List<i32>>, mut ass: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut eqnsarr: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<i32> {
    let mut Orphan: i32;
    Orphan = 'mc: {
        let __mc_input = eqns;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: e, tail: _ } => {
                    let mut len: i32;
                    let mut size: i32;
                    len = (({let __elt = ass.borrow()[(e.clone()-1) as usize].clone(); __elt}).len() as i32);
                    size = BackendEquation::equationSize(BackendEquation::get(eqnsarr.clone(), e.clone())?)?;
                    let true = (intLt(len.clone(), size.clone())) else { bail!("pattern mismatch") };
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(hasResidualOrphan1(rest.clone(), ass.clone(), eqnsarr.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(Orphan)
}

fn hasResidualOrphan(mut eqns: Arc<metamodelica::List<i32>>, mut ass: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<i32> {
    let mut Orphan: i32;
    Orphan = 'mc: {
        let __mc_input = eqns;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: e, tail: _ } => {
                    ::match_deref::match_deref! { match &(({let __elt = ass.borrow()[(e.clone()-1) as usize].clone(); __elt})) {
                        Deref @ metamodelica::List::Nil => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(hasResidualOrphan(rest.clone(), ass.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(Orphan)
}

fn makeCrefExps(mut v: BackendDAE::Var) -> Result<Arc<DAE::Exp>> {
    let mut e: Arc<DAE::Exp>;
    e = Expression::crefExp(BackendVariable::varCref(v)?)?;
    Ok(e)
}

fn makeGausEliminationRow(mut lst: Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>, mut size: i32, mut vars: metamodelica::Array<Arc<DAE::Exp>>, mut inExp: Arc<DAE::Exp>) -> Result<(Arc<DAE::Exp>, Arc<DAE::Exp>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outExp1: Arc<DAE::Exp>;
    (outExp, outExp1) = 'mc: {
        let __mc_input = lst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((inExp.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (c, e), tail: _ } => {
                    let true = (intGt(c.clone(), size)) else { bail!("pattern mismatch") };
                    Ok((inExp.clone(), e.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (c, e), tail: rest } => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut b: Arc<DAE::Exp>;
                    e1 = Expression::expMul(e.clone(), ({let __elt = vars.borrow()[(c.clone()-1) as usize].clone(); __elt}))?;
                    e1 = Expression::expAdd(e1.clone(), inExp.clone())?;
                    (e1, b) = makeGausEliminationRow(rest.clone(), size, vars.clone(), e1.clone())?;
                    Ok((e1.clone(), b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outExp1))
}

fn makeGausElimination(mut row: i32, mut size: i32, mut matrix: metamodelica::Array<Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>>, mut vars: metamodelica::Array<Arc<DAE::Exp>>, mut iAcc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut oAcc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    oAcc = 'mc: {
        let __mc_input = iAcc.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (intGt(row, size)) else { bail!("pattern mismatch") };
                    Ok(iAcc.clone().reverse())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut e: Arc<DAE::Exp>;
                    let mut b: Arc<DAE::Exp>;
                    let mut eqn: Arc<BackendDAE::Equation>;
                    (e, b) = makeGausEliminationRow(({let __elt = matrix.borrow()[(row-1) as usize].clone(); __elt}), size, vars.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }))?;
                    eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: e.clone(), scalar: b.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() });
                    Ok(makeGausElimination(row + 1, size, matrix.clone(), vars.clone(), metamodelica::cons(eqn.clone(), iAcc.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oAcc)
}

fn dumpMatrix(mut row: i32, mut size: i32, mut matrix: metamodelica::Array<Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = matrix.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (intGt(row, size)) else { bail!("pattern mismatch") };
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(row)); __mm_s.push_str(&*literal!(": ")); ArcStr::from(__mm_s) }).clone());
            BackendDump::debuglst(({let __elt = matrix.borrow()[(row-1) as usize].clone(); __elt}), (std::sync::Arc::new(dumpMatrix1) as std::sync::Arc<dyn ::std::ops::Fn((i32, Arc<DAE::Exp>)) -> Result<ArcStr> + 'static>), (literal!(", ")).clone(), (literal!("\n")).clone())?;
            dumpMatrix(row + 1, size, matrix.clone())?;
            Ok(())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn dumpMatrix1(mut inTpl: (i32, Arc<DAE::Exp>)) -> Result<ArcStr> {
    let mut s: ArcStr;
    let mut c: i32;
    let mut e: Arc<DAE::Exp>;
    let mut cs: ArcStr;
    let mut es: ArcStr;
    (c, e) = inTpl;
    cs = (intString(c)).clone();
    es = (ExpressionBasics::printExpStr(e)?).clone();
    s = stringAppendList(list![(cs).clone(), (literal!(":")).clone(), (es).clone()]);
    Ok(s)
}

fn addRows(mut inA: Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>, mut inB: Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>, mut col: i32, mut inVars: BackendDAE::Variables, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inTpl: (i32, i32), mut inElst: Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>) -> Result<(Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, (i32, i32))> {
    let mut outElst: Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>;
    let mut outVars: BackendDAE::Variables;
    let mut outEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut outTpl: (i32, i32);
    (outElst, outVars, outEqns, outTpl) = 'mc: {
        let __mc_input = (inA.clone(), inB.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    Ok((inElst.clone().reverse(), inVars.clone(), inEqns.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok((List::append_reverse(inElst.clone(), inB.clone()), inVars.clone(), inEqns.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil) => {
                    Ok((List::append_reverse(inElst.clone(), inA.clone()), inVars.clone(), inEqns.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (ca, _), tail: resta }, Deref @ metamodelica::List::Cons { head: (cb, _), tail: restb }) => {
                    let mut elst: Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>;
                    let mut vars: BackendDAE::Variables;
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut tpl: (i32, i32);
                    let true = (intEq(ca.clone(), cb.clone())) else { bail!("pattern mismatch") };
                    let true = (intEq(ca.clone(), col)) else { bail!("pattern mismatch") };
                    (elst, vars, eqns, tpl) = addRows(resta.clone(), restb.clone(), col, inVars.clone(), inEqns.clone(), inTpl.clone(), inElst.clone())?;
                    Ok((elst.clone(), vars.clone(), eqns.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (ca, ea), tail: resta }, Deref @ metamodelica::List::Cons { head: (cb, eb), tail: restb }) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut elst: Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>;
                    let mut vars: BackendDAE::Variables;
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut tpl: (i32, i32);
                    let true = (intEq(ca.clone(), cb.clone())) else { bail!("pattern mismatch") };
                    e = Expression::expAdd(ea.clone(), eb.clone())?;
                    (e, _) = ExpressionSimplify::simplify(e.clone())?;
                    (vars, eqns, e, tpl) = makeDummyVar(inTpl.clone(), e.clone(), inVars.clone(), inEqns.clone())?;
                    (elst, vars, eqns, tpl) = addRows(resta.clone(), restb.clone(), col, vars.clone(), eqns.clone(), tpl.clone(), metamodelica::cons((ca.clone(), e.clone()), inElst.clone()))?;
                    Ok((elst.clone(), vars.clone(), eqns.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (ca, _), tail: _ }, Deref @ metamodelica::List::Cons { head: (cb, _), tail: restb }) => {
                    let mut elst: Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>;
                    let mut vars: BackendDAE::Variables;
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut tpl: (i32, i32);
                    let true = (intGt(ca.clone(), cb.clone())) else { bail!("pattern mismatch") };
                    let true = (intEq(cb.clone(), col)) else { bail!("pattern mismatch") };
                    (elst, vars, eqns, tpl) = addRows(inA.clone(), restb.clone(), col, inVars.clone(), inEqns.clone(), inTpl.clone(), inElst.clone())?;
                    Ok((elst.clone(), vars.clone(), eqns.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (ca, _), tail: _ }, Deref @ metamodelica::List::Cons { head: (cb, eb), tail: restb }) => {
                    let mut elst: Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>;
                    let mut vars: BackendDAE::Variables;
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut tpl: (i32, i32);
                    let true = (intGt(ca.clone(), cb.clone())) else { bail!("pattern mismatch") };
                    (elst, vars, eqns, tpl) = addRows(inA.clone(), restb.clone(), col, inVars.clone(), inEqns.clone(), inTpl.clone(), metamodelica::cons((cb.clone(), eb.clone()), inElst.clone()))?;
                    Ok((elst.clone(), vars.clone(), eqns.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (ca, _), tail: resta }, Deref @ metamodelica::List::Cons { head: (cb, _), tail: _ }) => {
                    let mut elst: Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>;
                    let mut vars: BackendDAE::Variables;
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut tpl: (i32, i32);
                    let true = (intLt(ca.clone(), cb.clone())) else { bail!("pattern mismatch") };
                    let true = (intEq(ca.clone(), col)) else { bail!("pattern mismatch") };
                    (elst, vars, eqns, tpl) = addRows(resta.clone(), inB.clone(), col, inVars.clone(), inEqns.clone(), inTpl.clone(), inElst.clone())?;
                    Ok((elst.clone(), vars.clone(), eqns.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (ca, ea), tail: resta }, Deref @ metamodelica::List::Cons { head: (cb, _), tail: _ }) => {
                    let mut elst: Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>;
                    let mut vars: BackendDAE::Variables;
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut tpl: (i32, i32);
                    let true = (intLt(ca.clone(), cb.clone())) else { bail!("pattern mismatch") };
                    (elst, vars, eqns, tpl) = addRows(resta.clone(), inB.clone(), col, inVars.clone(), inEqns.clone(), inTpl.clone(), metamodelica::cons((ca.clone(), ea.clone()), inElst.clone()))?;
                    Ok((elst.clone(), vars.clone(), eqns.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outElst, outVars, outEqns, outTpl))
}

fn mulRow(mut inTpl: (i32, Arc<DAE::Exp>), mut e1: Arc<DAE::Exp>) -> Result<(i32, Arc<DAE::Exp>)> {
    let mut outTpl: (i32, Arc<DAE::Exp>);
    let mut e: Arc<DAE::Exp>;
    let mut c: i32;
    (c, e) = inTpl;
    e = Expression::negate(Expression::expMul(e, e1)?)?;
    outTpl = (c, e);
    Ok(outTpl)
}

fn removeFromCol(mut i: i32, mut inTpl: Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>, mut inAcc: Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>) -> Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inTpl) {
        Deref @ metamodelica::List::Nil => {
            return inAcc.reverse()
        },
        Deref @ metamodelica::List::Cons { head: (c, _), tail: rest } if (intEq(i, c.clone())) => {
            return listAppend(inAcc.reverse(), rest.clone())
        },
        Deref @ metamodelica::List::Cons { head: (c, e), tail: rest } => {
            { (i, inTpl, inAcc) = (i, rest.clone(), metamodelica::cons((c.clone(), e.clone()), inAcc)); continue '__tco; }
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

fn makeDummyVar(mut inTpl: (i32, i32), mut e: Arc<DAE::Exp>, mut inVars: BackendDAE::Variables, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<(BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<DAE::Exp>, (i32, i32))> {
    let mut outVars: BackendDAE::Variables;
    let mut outEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (i32, i32);
    (outVars, outEqns, outExp, outTpl) = 'mc: {
        let __mc_input = (inTpl.clone(), e.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::CREF { .. }) => {
                    Ok((inVars.clone(), inEqns.clone(), e.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CREF { .. }, .. }) => {
                    Ok((inVars.clone(), inEqns.clone(), e.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::RCONST { .. }) => {
                    Ok((inVars.clone(), inEqns.clone(), e.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let true = (Expression::isConst(e.clone())?) else { bail!("pattern mismatch") };
                    Ok((inVars.clone(), inEqns.clone(), e.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((a, b), _) => {
                    let mut cr: Arc<DAE::ComponentRef>;
                    let mut v: BackendDAE::Var;
                    let mut sa: ArcStr;
                    let mut sb: ArcStr;
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut vars: BackendDAE::Variables;
                    let mut cexp: Arc<DAE::Exp>;
                    sa = (intString(a.clone())).clone();
                    sb = (intString(b.clone())).clone();
                    cr = ComponentReferenceBasics::makeCrefIdent(stringAppendList(list![(literal!("$tmp")).clone(), (sa.clone()).clone(), (literal!("_")).clone(), (sb.clone()).clone()]), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil());
                    cexp = Expression::crefExp(cr.clone())?;
                    eqns = BackendEquation::add(Arc::new(BackendDAE::Equation::EQUATION { exp: cexp.clone(), scalar: e.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() }), inEqns.clone())?;
                    v = BackendDAE::Var { varName: cr.clone(), varKind: openmodelica_backend_types::BackendDAE::VarKind::VARIABLE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_REAL_DEFAULT().clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
                    vars = BackendVariable::addVar(v.clone(), inVars.clone())?;
                    Ok((vars.clone(), eqns.clone(), cexp.clone(), (a.clone(), b.clone() + 1)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVars, outEqns, outExp, outTpl))
}

fn gaussElimination1(mut col: i32, mut row: i32, mut size: i32, mut ce: Arc<DAE::Exp>, mut matrix: metamodelica::Array<Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>>, mut inVars: BackendDAE::Variables, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inTpl: (i32, i32)) -> (BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, (i32, i32)) {
    let mut outVars: BackendDAE::Variables;
    let mut outEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut outTpl: (i32, i32);
    (outVars, outEqns, outTpl) = 'mc: {
        let __mc_input = inTpl.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (intGt(row, size)) else { bail!("pattern mismatch") };
            Ok((inVars.clone(), inEqns.clone(), inTpl.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut vars: BackendDAE::Variables;
            let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
            let mut e: Arc<DAE::Exp>;
            let mut e1: Arc<DAE::Exp>;
            let mut cexp: Arc<DAE::Exp>;
            let mut elst: Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>;
            let mut tpl: (i32, i32);
            let __pa0 = ::match_deref::match_deref! { match &(diagonalEntry(col, ({let __elt = matrix.borrow()[(row-1) as usize].clone(); __elt}))?) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            e = __pa0.clone();
            e1 = Expression::expDiv(e.clone(), ce.clone())?;
            (e1, _) = ExpressionSimplify::simplify(e1.clone())?;
            (vars, eqns, cexp, tpl) = makeDummyVar(inTpl.clone(), e1.clone(), inVars.clone(), inEqns.clone())?;
            elst = ({let __elt = matrix.borrow()[(col-1) as usize].clone(); __elt});
            elst = List::map1(elst.clone(), (std::sync::Arc::new(mulRow) as std::sync::Arc<dyn ::std::ops::Fn((i32, Arc<DAE::Exp>), Arc<DAE::Exp>) -> Result<(i32, Arc<DAE::Exp>)> + 'static>), cexp.clone())?;
            (elst, vars, eqns, tpl) = addRows(({let __elt = matrix.borrow()[(row-1) as usize].clone(); __elt}), elst.clone(), col, vars.clone(), eqns.clone(), tpl.clone(), metamodelica::nil())?;
            metamodelica::arrayUpdate(matrix.clone(), row, elst.clone())?;
            (vars, eqns, tpl) = gaussElimination1(col, row + 1, size, ce.clone(), matrix.clone(), vars.clone(), eqns.clone(), tpl.clone());
            Ok((vars.clone(), eqns.clone(), tpl.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut vars: BackendDAE::Variables;
            let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
            let mut tpl: (i32, i32);
            (vars, eqns, tpl) = gaussElimination1(col, row + 1, size, ce.clone(), matrix.clone(), inVars.clone(), inEqns.clone(), inTpl.clone());
            Ok((vars.clone(), eqns.clone(), tpl.clone()))
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outVars, outEqns, outTpl)
}

fn gaussElimination(mut col: i32, mut size: i32, mut matrix: metamodelica::Array<Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>>, mut inVars: BackendDAE::Variables, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inTpl: (i32, i32)) -> Result<(BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)> {
    let mut outVars: BackendDAE::Variables;
    let mut outEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    (outVars, outEqns) = 'mc: {
        let __mc_input = inTpl.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (intGt(col, size)) else { bail!("pattern mismatch") };
            Ok((inVars.clone(), inEqns.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut vars: BackendDAE::Variables;
            let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
            let mut e: Arc<DAE::Exp>;
            let mut tpl: (i32, i32);
            let __pa0 = ::match_deref::match_deref! { match &(diagonalEntry(col, ({let __elt = matrix.borrow()[(col-1) as usize].clone(); __elt}))?) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            e = __pa0.clone();
            (vars, eqns, tpl) = gaussElimination1(col, col + 1, size, e.clone(), matrix.clone(), inVars.clone(), inEqns.clone(), inTpl.clone());
            (vars, eqns) = gaussElimination(col + 1, size, matrix.clone(), vars.clone(), eqns.clone(), tpl.clone())?;
            Ok((vars.clone(), eqns.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            ::match_deref::match_deref! { match &(diagonalEntry(col, ({let __elt = matrix.borrow()[(col-1) as usize].clone(); __elt}))?) {
                None => (),
                _ => bail!("pattern mismatch"),
            } };
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("gaussElimination failt because of non diagonal Entry for col ")); __mm_s.push_str(&*intString(col)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVars, outEqns))
}

fn diagonalEntry(mut col: i32, mut row: Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>) -> Result<Option<Arc<DAE::Exp>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(row) {
        Deref @ metamodelica::List::Cons { head: (r, e), tail: rest } => {
            if (intEq(r.clone(), col) && !(Expression::isZero(e.clone())?)) {return Ok(Some(e.clone()))} else {if (intGt(r.clone(), col)) {return Ok(None)} else {{ (col, row) = (col, rest.clone()); continue '__tco; }}}
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn isConstOneMinusOne(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut b: bool;
    b = Expression::isConstOne(inExp.clone()) || Expression::isConstMinusOne(inExp);
    b
}

fn transformJacToAdjacencyMatrix2(mut jac: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>, mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>) -> Result<()> {
    pub type CompareFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>;

    let () = (::match_deref::match_deref! { match &(jac) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: (r, c, Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. }), tail: rest } => {
            let mut i: i32;
            let mut b: bool;
            let mut b1: bool;
            let mut lst: Arc<metamodelica::List<i32>>;
            let mut eqn: Arc<BackendDAE::Equation>;
            i = ({let __elt = mapIncRowEqn.borrow()[(r.clone()-1) as usize].clone(); __elt});
            eqn = BackendEquation::get(eqns.clone(), i)?;
            b1 = BackendEquation::isArrayEquation(eqn);
            b = func(e.clone())?;
            lst = List::consOnTrue(b && b1, c.clone(), ({let __elt = m.borrow()[(r.clone()-1) as usize].clone(); __elt}));
            metamodelica::arrayUpdate(m.clone(), r.clone(), lst)?;
            transformJacToAdjacencyMatrix2(rest.clone(), m.clone(), mapIncRowEqn.clone(), eqns, ass1.clone(), ass2.clone(), func.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn transformJacToAdjacencyMatrix1(mut jac: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>) -> Result<()> {
    pub type CompareFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>;

    let () = (::match_deref::match_deref! { match &(jac) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: (r, c, Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. }), tail: rest } => {
            let mut b: bool;
            let mut b1: bool;
            let mut lst: Arc<metamodelica::List<i32>>;
            b1 = intLt(({let __elt = ass1.borrow()[(c.clone()-1) as usize].clone(); __elt}), 1);
            b = func(e.clone())?;
            lst = List::consOnTrue(b && b1, c.clone(), ({let __elt = m.borrow()[(r.clone()-1) as usize].clone(); __elt}));
            metamodelica::arrayUpdate(m.clone(), r.clone(), lst)?;
            transformJacToAdjacencyMatrix1(rest.clone(), m.clone(), ass1.clone(), ass2.clone(), func.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn transformJacToAdjacencyMatrix(mut jac: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>) -> Result<()> {
    pub type CompareFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>;

    let () = (::match_deref::match_deref! { match &(jac.clone()) {
        Deref @ metamodelica::List::Nil => {
            transformJacToAdjacencyMatrix(jac, m.clone(), mT.clone(), func.clone())?;
            ()
        },
        Deref @ metamodelica::List::Cons { head: (r, c, Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. }), tail: rest } => {
            let mut b: bool;
            let mut lst: Arc<metamodelica::List<i32>>;
            let mut lst1: Arc<metamodelica::List<i32>>;
            b = func(e.clone())?;
            lst = List::consOnTrue(b, c.clone(), ({let __elt = m.borrow()[(r.clone()-1) as usize].clone(); __elt}));
            lst1 = List::consOnTrue(b, r.clone(), ({let __elt = mT.borrow()[(c.clone()-1) as usize].clone(); __elt}));
            metamodelica::arrayUpdate(m.clone(), r.clone(), lst)?;
            metamodelica::arrayUpdate(mT.clone(), c.clone(), lst1)?;
            transformJacToAdjacencyMatrix(rest.clone(), m.clone(), mT.clone(), func.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn transformJacToMatrix(mut jac: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>, mut row: i32, mut col: i32, mut size: i32, mut b: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut matrix: metamodelica::Array<Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = jac.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (intGt(row, size)) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut be: Arc<DAE::Exp>;
                    let mut b1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut lst: Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>;
                    let true = (intGt(col, size)) else { bail!("pattern mismatch") };
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(b.clone()) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    be = __pa0.clone();
                    b1 = __pa1.clone();
                    lst = ({let __elt = matrix.borrow()[(row-1) as usize].clone(); __elt});
                    lst = List::consOnTrue(!(Expression::isZero(be.clone())?), (col, be.clone()), lst.clone());
                    lst = lst.clone().reverse();
                    metamodelica::arrayUpdate(matrix.clone(), row, lst.clone())?;
                    transformJacToMatrix(jac.clone(), row + 1, 1, size, b1.clone(), matrix.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    transformJacToMatrix(jac.clone(), row, col + 1, size, b.clone(), matrix.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (r, c, Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. }), tail: rest } => {
                    let mut lst: Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>;
                    let true = (intEq(r.clone(), row)) else { bail!("pattern mismatch") };
                    let true = (intEq(c.clone(), col)) else { bail!("pattern mismatch") };
                    lst = ({let __elt = matrix.borrow()[(r.clone()-1) as usize].clone(); __elt});
                    lst = metamodelica::cons((c.clone(), e.clone()), lst.clone());
                    metamodelica::arrayUpdate(matrix.clone(), row, lst.clone())?;
                    transformJacToMatrix(rest.clone(), row, col + 1, size, b.clone(), matrix.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (r, c, _), tail: _ } => {
                    let true = (intEq(r.clone(), row)) else { bail!("pattern mismatch") };
                    let true = (intLt(col, c.clone())) else { bail!("pattern mismatch") };
                    transformJacToMatrix(jac.clone(), row, col + 1, size, b.clone(), matrix.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (r, _, _), tail: _ } => {
                    let true = (intGe(r.clone(), row)) else { bail!("pattern mismatch") };
                    transformJacToMatrix(jac.clone(), row, col + 1, size, b.clone(), matrix.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn dumpJacMatrix(mut jac: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>, mut row: i32, mut col: i32, mut size: i32, mut vars: BackendDAE::Variables) -> Result<()> {
    let () = 'mc: {
        let __mc_input = jac.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (intGt(row, size)) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut v: BackendDAE::Var;
                    let mut cr: Arc<DAE::ComponentRef>;
                    let true = (intGt(col, size)) else { bail!("pattern mismatch") };
                    v = BackendVariable::getVarAt(vars.clone(), row)?;
                    cr = BackendVariable::varCref(v.clone())?;
                    metamodelica::print((literal!(";... % ")).clone());
                    metamodelica::print((intString(row)).clone());
                    metamodelica::print((literal!(" ")).clone());
                    metamodelica::print((ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone());
                    metamodelica::print((literal!("\n")).clone());
                    dumpJacMatrix(jac.clone(), row + 1, 1, size, vars.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    metamodelica::print((literal!("0, ")).clone());
                    dumpJacMatrix(jac.clone(), row, col + 1, size, vars.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (r, c, Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. }), tail: rest } => {
                    let mut estr: ArcStr;
                    let true = (intEq(r.clone(), row)) else { bail!("pattern mismatch") };
                    let true = (intEq(c.clone(), col)) else { bail!("pattern mismatch") };
                    estr = (ExpressionBasics::printExpStr(e.clone())?).clone();
                    metamodelica::print((estr.clone()).clone());
                    metamodelica::print((literal!(", ")).clone());
                    dumpJacMatrix(rest.clone(), row, col + 1, size, vars.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (r, c, _), tail: _ } => {
                    let true = (intEq(r.clone(), row)) else { bail!("pattern mismatch") };
                    let true = (intLt(col, c.clone())) else { bail!("pattern mismatch") };
                    metamodelica::print((literal!("0, ")).clone());
                    dumpJacMatrix(jac.clone(), row, col + 1, size, vars.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (r, _, _), tail: _ } => {
                    let false = (intEq(r.clone(), row)) else { bail!("pattern mismatch") };
                    metamodelica::print((literal!("0, ")).clone());
                    dumpJacMatrix(jac.clone(), row, col + 1, size, vars.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn getEqnsinOrder(mut indx: i32, mut inTpl: (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables)) -> Result<(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables)> {
    let mut outTpl: (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables);
    let mut e: Arc<BackendDAE::Equation>;
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut eqnssort: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut vlst: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut vars: BackendDAE::Variables;
    let mut varssort: BackendDAE::Variables;
    let mut vindxs: Arc<metamodelica::List<i32>>;
    (eqns, vars, ass2, eqnssort, varssort) = inTpl;
    e = BackendEquation::get(eqns.clone(), indx)?;
    eqnssort = BackendEquation::add(e.clone(), eqnssort)?;
    vindxs = ({let __elt = ass2.borrow()[(indx-1) as usize].clone(); __elt});
    vlst = List::map1r(vindxs.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone())?;
    vlst = sortVarsforOrder(e, vlst, vindxs, vars.clone())?;
    varssort = BackendVariable::addVars(vlst, varssort)?;
    outTpl = (eqns, vars, ass2.clone(), eqnssort, varssort);
    Ok(outTpl)
}

fn sortVarsforOrder(mut inEqn: Arc<BackendDAE::Equation>, mut inVarLst: Arc<metamodelica::List<BackendDAE::Var>>, mut vindxs: Arc<metamodelica::List<i32>>, mut vars: BackendDAE::Variables) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut outVarLst: Arc<metamodelica::List<BackendDAE::Var>>;
    outVarLst = 'mc: {
        let __mc_input = inEqn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: e1, .. } => {
                    let mut vlst: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    let mut elst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    elst = Expression::flattenArrayExpToList(e1.clone());
                    crlst = List::map(elst.clone(), (std::sync::Arc::new(Expression::expCrefNegCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::ComponentRef>> + 'static>))?;
                    vlst = sortVarsforOrder1(crlst.clone(), 1, inVarLst.clone(), vindxs.clone(), arrayCreate((vindxs.clone().len() as i32), None), vars.clone())?;
                    Ok(vlst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::ARRAY_EQUATION { right: e1, .. } => {
                    let mut vlst: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    let mut elst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    elst = Expression::flattenArrayExpToList(e1.clone());
                    crlst = List::map(elst.clone(), (std::sync::Arc::new(Expression::expCrefNegCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::ComponentRef>> + 'static>))?;
                    vlst = sortVarsforOrder1(crlst.clone(), 1, inVarLst.clone(), vindxs.clone(), arrayCreate((vindxs.clone().len() as i32), None), vars.clone())?;
                    Ok(vlst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut vlst: Arc<metamodelica::List<BackendDAE::Var>>;
                    vlst = List::sort(inVarLst.clone(), (std::sync::Arc::new(BackendVariable::varSortFunc) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendDAE::Var) -> Result<bool> + 'static>))?;
                    Ok(vlst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVarLst)
}

fn sortVarsforOrder1(mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut index: i32, mut inVarLst: Arc<metamodelica::List<BackendDAE::Var>>, mut vindxs: Arc<metamodelica::List<i32>>, mut vararray: metamodelica::Array<Option<BackendDAE::Var>>, mut vars: BackendDAE::Variables) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut outVarLst: Arc<metamodelica::List<BackendDAE::Var>>;
    outVarLst = 'mc: {
        let __mc_input = crlst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    let mut vlst: Arc<metamodelica::List<BackendDAE::Var>>;
                    vlst = List::sort(inVarLst.clone(), (std::sync::Arc::new(BackendVariable::varSortFunc) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendDAE::Var) -> Result<bool> + 'static>))?;
                    vlst = sortVarsforOrder2(1, vlst.clone(), vararray.clone(), metamodelica::nil())?;
                    Ok(vlst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: cr, tail: rest } => {
                    let mut i: i32;
                    let mut p: i32;
                    let mut ilst: Arc<metamodelica::List<i32>>;
                    let mut v: BackendDAE::Var;
                    let mut vlst: Arc<metamodelica::List<BackendDAE::Var>>;
                    (v, i) = BackendVariable::getVarSingle(cr.clone(), vars.clone())?;
                    p = List::position(i.clone(), vindxs.clone())?;
                    ilst = listDelete(vindxs.clone(), p.clone())?;
                    vlst = listDelete(inVarLst.clone(), p.clone())?;
                    metamodelica::arrayUpdate(vararray.clone(), index, Some(v.clone()))?;
                    Ok(sortVarsforOrder1(rest.clone(), index + 1, vlst.clone(), ilst.clone(), vararray.clone(), vars.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(sortVarsforOrder1(rest.clone(), index + 1, inVarLst.clone(), vindxs.clone(), vararray.clone(), vars.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVarLst)
}

fn sortVarsforOrder2(mut index: i32, mut inVarLst: Arc<metamodelica::List<BackendDAE::Var>>, mut vararray: metamodelica::Array<Option<BackendDAE::Var>>, mut iAcc: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut outVarLst: Arc<metamodelica::List<BackendDAE::Var>>;
    outVarLst = 'mc: {
        let __mc_input = inVarLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (intGt(index, metamodelica::arrayLength(vararray.clone()))) else { bail!("pattern mismatch") };
                    Ok(iAcc.clone().reverse())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut v: BackendDAE::Var;
                    let __pa0 = ::match_deref::match_deref! { match &(({let __elt = vararray.borrow()[(index-1) as usize].clone(); __elt})) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    v = __pa0.clone();
                    Ok(sortVarsforOrder2(index + 1, inVarLst.clone(), vararray.clone(), metamodelica::cons(v.clone(), iAcc.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: v, tail: vlst } => {
                    Ok(sortVarsforOrder2(index + 1, vlst.clone(), vararray.clone(), metamodelica::cons(v.clone(), iAcc.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVarLst)
}

fn getOrphansPairs(mut inOrphans: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>) -> i32 {
    let mut omark: i32;
    omark = 'mc: {
        let __mc_input = inOrphans;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(mark)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: o, tail: rest } => {
                    let false = (intEq(({let __elt = rowmarks.borrow()[(o.clone()-1) as usize].clone(); __elt}), mark)) else { bail!("pattern mismatch") };
                    getOrphansPairs1(list![o.clone()], ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), o.clone(), metamodelica::nil())?;
                    Ok(getOrphansPairs(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark + 1, rowmarks.clone(), colummarks.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(getOrphansPairs(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    omark
}

fn getOrphansPairs1(mut rows: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>, mut orphan: i32, mut nextQueue: Arc<metamodelica::List<i32>>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (rows, nextQueue.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    getOrphansPairs1(nextQueue.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), orphan, metamodelica::nil())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: r, tail: _ }, _) => {
                    let mut o: i32;
                    let mut elst: Arc<metamodelica::List<i32>>;
                    let false = (intEq(({let __elt = rowmarks.borrow()[(r.clone()-1) as usize].clone(); __elt}), mark)) else { bail!("pattern mismatch") };
                    elst = List::select1(({let __elt = mt.borrow()[(r.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                    o = hasResidualOrphan(elst.clone(), ass2.clone())?;
                    metamodelica::arrayUpdate(ass1.clone(), orphan, o.clone())?;
                    metamodelica::arrayUpdate(ass2.clone(), o.clone(), list![orphan])?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: r, tail: rest }, _) => {
                    let mut next: Arc<metamodelica::List<i32>>;
                    let mut elst: Arc<metamodelica::List<i32>>;
                    let false = (intEq(({let __elt = rowmarks.borrow()[(r.clone()-1) as usize].clone(); __elt}), mark)) else { bail!("pattern mismatch") };
                    elst = List::select1(({let __elt = mt.borrow()[(r.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                    next = List::select1(List::flatten(List::map1r(elst.clone(), (std::sync::Arc::new(arrayGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), ass2.clone())?)?, (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                    next = listAppend(nextQueue.clone(), next.clone());
                    metamodelica::arrayUpdate(rowmarks.clone(), r.clone(), mark)?;
                    getOrphansPairs1(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), orphan, next.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _) => {
                    getOrphansPairs1(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), orphan, nextQueue.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn getOrphansPairsConstraints(mut inOrphans: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>, mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> i32 {
    let mut omark: i32;
    omark = 'mc: {
        let __mc_input = inOrphans;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(mark)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: o, tail: rest } => {
                    let false = (intEq(({let __elt = colummarks.borrow()[(o.clone()-1) as usize].clone(); __elt}), mark)) else { bail!("pattern mismatch") };
                    metamodelica::arrayUpdate(colummarks.clone(), o.clone(), mark)?;
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getOrphansPairsConstraints Process Orphan ")); __mm_s.push_str(&*intString(o.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    getOrphansPairsConstraints1(({let __elt = mt.borrow()[(o.clone()-1) as usize].clone(); __elt}), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), eqns.clone(), o.clone(), metamodelica::nil())?;
                    Ok(getOrphansPairsConstraints(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark + 1, rowmarks.clone(), colummarks.clone(), eqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(getOrphansPairsConstraints(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), eqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    omark
}

fn getOrphansPairsConstraints1(mut eqns: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>, mut eqnsarr: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut orphan: i32, mut nextQueue: Arc<metamodelica::List<i32>>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (eqns, nextQueue.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    getOrphansPairsConstraints1(nextQueue.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), eqnsarr.clone(), orphan, metamodelica::nil())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: e, tail: _ }, _) => {
                    let mut o: i32;
                    let mut next: Arc<metamodelica::List<i32>>;
                    let mut rlst: Arc<metamodelica::List<i32>>;
                    let mut ass2lst: Arc<metamodelica::List<i32>>;
                    let false = (intEq(({let __elt = colummarks.borrow()[(e.clone()-1) as usize].clone(); __elt}), mark)) else { bail!("pattern mismatch") };
                    rlst = List::select1(({let __elt = m.borrow()[(e.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                    rlst = List::fold1(({let __elt = ass2.borrow()[(e.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(List::removeOnTrue) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), rlst.clone())?;
                    next = List::select1(List::flatten(List::map1r(rlst.clone(), (std::sync::Arc::new(arrayGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), mt.clone())?)?, (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                    o = hasResidualOrphan1(next.clone(), ass2.clone(), eqnsarr.clone())?;
                    metamodelica::arrayUpdate(ass1.clone(), orphan, o.clone())?;
                    ass2lst = ({let __elt = ass2.borrow()[(o.clone()-1) as usize].clone(); __elt});
                    ass2lst = metamodelica::cons(orphan, ass2lst.clone());
                    metamodelica::arrayUpdate(ass2.clone(), o.clone(), ass2lst.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: e, tail: rest }, _) => {
                    let mut next: Arc<metamodelica::List<i32>>;
                    let mut rlst: Arc<metamodelica::List<i32>>;
                    let mut lst: Arc<metamodelica::List<i32>>;
                    let false = (intEq(({let __elt = colummarks.borrow()[(e.clone()-1) as usize].clone(); __elt}), mark)) else { bail!("pattern mismatch") };
                    rlst = List::select1(({let __elt = m.borrow()[(e.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                    lst = List::select1(List::map1r(rlst.clone(), (std::sync::Arc::new(arrayGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), ass1.clone())?, (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                    rlst = List::fold1(lst.clone(), (std::sync::Arc::new(List::removeOnTrue) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), rlst.clone())?;
                    next = List::select1(List::map1r(rlst.clone(), (std::sync::Arc::new(arrayGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), ass1.clone())?, (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                    next = listAppend(nextQueue.clone(), next.clone());
                    metamodelica::arrayUpdate(colummarks.clone(), e.clone(), mark)?;
                    getOrphansPairsConstraints1(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), eqnsarr.clone(), orphan, next.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _) => {
                    getOrphansPairsConstraints1(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), eqnsarr.clone(), orphan, nextQueue.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn getIndexesForEqnsAdvanced(mut orphans: Arc<metamodelica::List<i32>>, mut index: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut imark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>, mut orowmarks: metamodelica::Array<i32>, mut ocolummarks: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut vec1: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut vec2: metamodelica::Array<i32>, mut queuemark: metamodelica::Array<bool>, mut vars: BackendDAE::Variables, mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut shared: Arc<BackendDAE::Shared>, mut size: i32) -> i32 {
    let mut outMark: i32;
    outMark = 'mc: {
        let __mc_input = orphans;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(imark)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: vorphan, tail: rest } => {
                    let mut eorphan: i32;
                    let mut index1: i32;
                    let mut mark: i32;
                    let mut rows: Arc<metamodelica::List<i32>>;
                    let mut queue: Arc<metamodelica::List<i32>>;
                    let mut rqueue: Arc<metamodelica::List<i32>>;
                    let mut bvars: Arc<metamodelica::List<i32>>;
                    let mut beqns: Arc<metamodelica::List<i32>>;
                    let mut lst: Arc<metamodelica::List<i32>>;
                    let mut vorphans: Arc<metamodelica::List<i32>>;
                    let mut vorphanseqns: Arc<metamodelica::List<i32>>;
                    let mut queuelst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let true = (intEq(({let __elt = orowmarks.borrow()[(vorphan.clone()-1) as usize].clone(); __elt}), 1)) else { bail!("pattern mismatch") };
                    eorphan = ({let __elt = ass1.borrow()[(vorphan.clone()-1) as usize].clone(); __elt});
                    vorphans = ({let __elt = ass2.borrow()[(eorphan.clone()-1) as usize].clone(); __elt});
                    rows = List::select(({let __elt = m.borrow()[(eorphan.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
                    rows = List::fold1(({let __elt = ass2.borrow()[(eorphan.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(List::removeOnTrue) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), rows.clone())?;
                    getIndexSubGraph(rows.clone(), vorphans.clone(), m.clone(), mT.clone(), imark, rowmarks.clone(), colummarks.clone(), orowmarks.clone(), ocolummarks.clone(), ass1.clone(), ass2.clone(), false);
                    vorphanseqns = List::unique(List::flatten(List::map1r(vorphans.clone(), (std::sync::Arc::new(arrayGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), mT.clone())?)?);
                    queuelst = getIndexQueque(vorphanseqns.clone(), m.clone(), mT.clone(), imark, rowmarks.clone(), colummarks.clone(), ass1.clone(), ass2.clone(), vec2.clone(), queuemark.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil())?;
                    queue = List::flatten(queuelst.clone())?;
                    mark = imark + 2;
                    (index1, queue, rqueue) = List::fold1(queue.clone(), (std::sync::Arc::new(fnptr!(setIndexQueue, i32, (metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<bool>, metamodelica::Array<i32>, i32), (i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>))) as std::sync::Arc<dyn ::std::ops::Fn(i32, (metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<bool>, metamodelica::Array<i32>, i32), (i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)) -> Result<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> + 'static>), (vec1.clone(), vec2.clone(), ass2.clone(), queuemark.clone(), colummarks.clone(), mark.clone()), (index, metamodelica::nil(), metamodelica::nil()))?;
                    metamodelica::arrayUpdate(vec1.clone(), index1.clone(), vorphans.clone())?;
                    metamodelica::arrayUpdate(vec2.clone(), index1.clone(), eorphan.clone())?;
                    metamodelica::arrayUpdate(queuemark.clone(), eorphan.clone(), true)?;
                    mark = mark.clone() + 1;
                    List::map2_0(rqueue.clone(), (std::sync::Arc::new(doMark) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<()> + 'static>), rowmarks.clone(), mark.clone())?;
                    List::map2_0(queue.clone(), (std::sync::Arc::new(doMark) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<()> + 'static>), colummarks.clone(), mark.clone())?;
                    bvars = getBorderElements(queue.clone(), m.clone(), mark.clone(), rowmarks.clone(), metamodelica::nil())?;
                    bvars = List::fold1(vorphans.clone(), (std::sync::Arc::new(List::removeOnTrue) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), bvars.clone())?;
                    beqns = getBorderElements(rqueue.clone(), mT.clone(), mark.clone(), colummarks.clone(), metamodelica::nil())?;
                    beqns = List::removeOnTrue(eorphan.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), beqns.clone())?;
                    lst = List::select2(({let __elt = m.borrow()[(eorphan.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(unmarked, i32, metamodelica::Array<i32>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), rowmarks.clone(), mark.clone())?;
                    lst = listAppend(vorphans.clone(), listAppend(lst.clone(), bvars.clone()));
                    metamodelica::arrayUpdate(m.clone(), eorphan.clone(), lst.clone())?;
                    lst = List::select2(vorphanseqns.clone(), (std::sync::Arc::new(fnptr!(unmarked, i32, metamodelica::Array<i32>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), colummarks.clone(), mark.clone())?;
                    lst = listAppend(metamodelica::cons(eorphan.clone(), lst.clone()), beqns.clone());
                    metamodelica::arrayUpdate(mT.clone(), vorphan.clone(), lst.clone())?;
                    setBoarderElemts(bvars.clone(), mT.clone(), mark.clone(), colummarks.clone(), eorphan.clone())?;
                    setBoarderElemts(beqns.clone(), m.clone(), mark.clone(), rowmarks.clone(), vorphan.clone())?;
                    List::fold1(vorphans.clone(), (std::sync::Arc::new(markOrphans) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> + 'static>), -1, orowmarks.clone())?;
                    metamodelica::arrayUpdate(ocolummarks.clone(), eorphan.clone(), -1)?;
                    vorphans = List::removeOnTrue(vorphan.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), vorphans.clone())?;
                    List::fold1(vorphans.clone(), (std::sync::Arc::new(markOrphans) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> + 'static>), -1, orowmarks.clone())?;
                    List::fold1r(vorphans.clone(), Arc::new(arrayUpdate.clone()), metamodelica::nil(), mT.clone())?;
                    Ok(getIndexesForEqnsAdvanced(rest.clone(), index1.clone() + 1, m.clone(), mT.clone(), mark.clone() + 2, rowmarks.clone(), colummarks.clone(), orowmarks.clone(), ocolummarks.clone(), ass1.clone(), ass2.clone(), vec1.clone(), vec2.clone(), queuemark.clone(), vars.clone(), eqns.clone(), shared.clone(), size))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(getIndexesForEqnsAdvanced(rest.clone(), index, m.clone(), mT.clone(), imark, rowmarks.clone(), colummarks.clone(), orowmarks.clone(), ocolummarks.clone(), ass1.clone(), ass2.clone(), vec1.clone(), vec2.clone(), queuemark.clone(), vars.clone(), eqns.clone(), shared.clone(), size))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outMark
}

fn getBorderElements(mut elements: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut arr: metamodelica::Array<i32>, mut iAcc: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(elements) {
        Deref @ metamodelica::List::Nil => {
            return Ok(iAcc)
        },
        Deref @ metamodelica::List::Cons { head: elem, tail: rest } => {
            let mut lst: Arc<metamodelica::List<i32>>;
            let mut lst1: Arc<metamodelica::List<i32>>;
            (lst, lst1) = List::split2OnTrue(({let __elt = m.borrow()[(elem.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(unmarked, i32, metamodelica::Array<i32>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), arr.clone(), mark)?;
            metamodelica::arrayUpdate(m.clone(), elem.clone(), lst1)?;
            lst = List::select2(lst, (std::sync::Arc::new(fnptr!(unmarked, i32, metamodelica::Array<i32>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), arr.clone(), mark + 1)?;
            List::map2_0(lst.clone(), (std::sync::Arc::new(doMark) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<()> + 'static>), arr.clone(), mark + 1)?;
            { (elements, m, mark, arr, iAcc) = (rest.clone(), m.clone(), mark, arr.clone(), listAppend(lst, iAcc)); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn setBoarderElemts(mut elements: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut arr: metamodelica::Array<i32>, mut orphan: i32) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(elements) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: elem, tail: rest } => {
            let mut lst: Arc<metamodelica::List<i32>>;
            lst = List::select2(({let __elt = m.borrow()[(elem.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(unmarked, i32, metamodelica::Array<i32>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), arr.clone(), mark)?;
            metamodelica::arrayUpdate(m.clone(), elem.clone(), metamodelica::cons(orphan, lst))?;
            setBoarderElemts(rest.clone(), m.clone(), mark, arr.clone(), orphan)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn setIndexQueue(mut col: i32, mut tpl: (metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<bool>, metamodelica::Array<i32>, i32), mut itpl: (i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)) -> (i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) {
    let mut otpl: (i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
    otpl = 'mc: {
        let __mc_input = (tpl, itpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((vec1, vec2, ass2, queuemark, colummark, mark), (index, elst, rlst)) => {
                    let mut r: Arc<metamodelica::List<i32>>;
                    r = ({let __elt = ass2.borrow()[(col-1) as usize].clone(); __elt});
                    let false = (({let __elt = queuemark.borrow()[(col-1) as usize].clone(); __elt})) else { bail!("pattern mismatch") };
                    metamodelica::arrayUpdate(vec1.clone(), index.clone(), r.clone())?;
                    metamodelica::arrayUpdate(vec2.clone(), index.clone(), col)?;
                    metamodelica::arrayUpdate(queuemark.clone(), col, true)?;
                    metamodelica::arrayUpdate(colummark.clone(), col, mark.clone())?;
                    Ok((index.clone() + 1, metamodelica::cons(col, elst.clone()), listAppend(r.clone(), rlst.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((_, _, ass2, _, colummark, mark), (index, elst, rlst)) => {
                    let mut r: Arc<metamodelica::List<i32>>;
                    r = ({let __elt = ass2.borrow()[(col-1) as usize].clone(); __elt});
                    let false = (intEq(({let __elt = colummark.borrow()[(col-1) as usize].clone(); __elt}), mark.clone())) else { bail!("pattern mismatch") };
                    metamodelica::arrayUpdate(colummark.clone(), col, mark.clone())?;
                    Ok((index.clone(), metamodelica::cons(col, elst.clone()), listAppend(r.clone(), rlst.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(itpl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    otpl
}

fn getIndexQueque(mut colums: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut vec2: metamodelica::Array<i32>, mut queuemark: metamodelica::Array<bool>, mut nextqueue: Arc<metamodelica::List<i32>>, mut iqueue: Arc<metamodelica::List<i32>>, mut iqueue1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((colums.clone(), nextqueue.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            return Ok(iqueue1)
        },
        (Deref @ metamodelica::List::Nil, _) => {
            let mut queue: Arc<metamodelica::List<i32>>;
            queue = List::unique(iqueue);
            { (colums, m, mT, mark, rowmarks, colummarks, ass1, ass2, vec2, queuemark, nextqueue, iqueue, iqueue1) = (nextqueue, m.clone(), mT.clone(), mark, rowmarks.clone(), colummarks.clone(), ass1.clone(), ass2.clone(), vec2.clone(), queuemark.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::cons(queue, iqueue1)); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: c, tail: rest }, _) => {
            let mut queue: Arc<metamodelica::List<i32>>;
            let mut r: Arc<metamodelica::List<i32>>;
            let mut queue1: Arc<metamodelica::List<i32>>;
            let mut colums1: Arc<metamodelica::List<i32>>;
            let mut b1: bool;
            let mut b2: bool;
            r = ({let __elt = ass2.borrow()[(c.clone()-1) as usize].clone(); __elt});
            (colums1, b2) = getIndexQueque1(r, c.clone(), mT.clone(), mark, rowmarks.clone())?;
            b1 = !(colums.is_empty());
            queue = if (b1) {List::unionOnTrue(colums1, nextqueue, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?} else {nextqueue};
            queue1 = List::consOnTrue(b2, c.clone(), iqueue);
            { (colums, m, mT, mark, rowmarks, colummarks, ass1, ass2, vec2, queuemark, nextqueue, iqueue, iqueue1) = (rest.clone(), m.clone(), mT.clone(), mark, rowmarks.clone(), colummarks.clone(), ass1.clone(), ass2.clone(), vec2.clone(), queuemark.clone(), queue, queue1, iqueue1); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn getIndexQueque1(mut rows: Arc<metamodelica::List<i32>>, mut c: i32, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>) -> Result<(Arc<metamodelica::List<i32>>, bool)> {
    let mut ocolums: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut ob: bool = false;
    let mut colums: Arc<metamodelica::List<i32>>;
    for mut r in &*rows {
        let mut r = r.clone();
        if intEq(({let __elt = rowmarks.borrow()[(r.clone()-1) as usize].clone(); __elt}), mark) {
            ob = true;
            colums = List::select(({let __elt = mT.borrow()[(r.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
            colums = List::removeOnTrue(c, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), colums.clone())?;
            ocolums = listAppend(colums.clone(), ocolums.clone());
        }
    }
    ocolums = List::unique(ocolums);
    Ok((ocolums, ob))
}

fn unmarked(mut indx: i32, mut markarray: metamodelica::Array<i32>, mut mark: i32) -> bool {
    let mut b: bool;
    b = intNe(({let __elt = markarray.borrow()[(indx-1) as usize].clone(); __elt}), mark);
    b
}

fn marked(mut indx: i32, mut markarray: metamodelica::Array<i32>, mut mark: i32) -> bool {
    let mut b: bool;
    b = intEq(({let __elt = markarray.borrow()[(indx-1) as usize].clone(); __elt}), mark);
    b
}

fn isOrphan(mut indx: i32, mut ass: metamodelica::Array<i32>) -> bool {
    let mut b: bool;
    b = intLt(({let __elt = ass.borrow()[(indx-1) as usize].clone(); __elt}), 1);
    b
}

fn isNoOrphan(mut indx: i32, mut ass: metamodelica::Array<i32>) -> bool {
    let mut b: bool;
    b = intGt(({let __elt = ass.borrow()[(indx-1) as usize].clone(); __elt}), 0);
    b
}

fn isResOrphan(mut indx: i32, mut ass: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> bool {
    let mut b: bool;
    b = ({let __elt = ass.borrow()[(indx-1) as usize].clone(); __elt}).is_empty();
    b
}

fn isNoResOrphan(mut indx: i32, mut ass: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> bool {
    let mut b: bool;
    b = !(({let __elt = ass.borrow()[(indx-1) as usize].clone(); __elt}).is_empty());
    b
}

fn doAssign(mut index: i32, mut arr: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut assign: Arc<metamodelica::List<i32>>) -> Result<()> {
    metamodelica::arrayUpdate(arr.clone(), index, assign)?;
    Ok(())
}

fn doMark(mut index: i32, mut arr: metamodelica::Array<i32>, mut mark: i32) -> Result<()> {
    metamodelica::arrayUpdate(arr.clone(), index, mark)?;
    Ok(())
}

fn getIndexSubGraph(mut rows: Arc<metamodelica::List<i32>>, mut vorphan: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>, mut orowmarks: metamodelica::Array<i32>, mut ocolummarks: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ifound: bool) -> bool {
    let mut found: bool;
    found = 'mc: {
        let __mc_input = rows;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(ifound)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r, tail: rest } => {
                    let true = (listMember(r.clone(), vorphan.clone())) else { bail!("pattern mismatch") };
                    getIndexSubGraph(rest.clone(), vorphan.clone(), m.clone(), mT.clone(), mark, rowmarks.clone(), colummarks.clone(), orowmarks.clone(), ocolummarks.clone(), ass1.clone(), ass2.clone(), false);
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r, tail: rest } => {
                    let mut e: i32;
                    let false = (listMember(r.clone(), vorphan.clone())) else { bail!("pattern mismatch") };
                    let false = (intEq(({let __elt = orowmarks.borrow()[(r.clone()-1) as usize].clone(); __elt}), 1)) else { bail!("pattern mismatch") };
                    let true = (intEq(({let __elt = rowmarks.borrow()[(r.clone()-1) as usize].clone(); __elt}), mark)) else { bail!("pattern mismatch") };
                    e = ({let __elt = ass1.borrow()[(r.clone()-1) as usize].clone(); __elt});
                    List::map2_0(({let __elt = ass2.borrow()[(e.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(doMark) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<()> + 'static>), rowmarks.clone(), mark)?;
                    Ok(getIndexSubGraph(rest.clone(), vorphan.clone(), m.clone(), mT.clone(), mark, rowmarks.clone(), colummarks.clone(), orowmarks.clone(), ocolummarks.clone(), ass1.clone(), ass2.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r, tail: rest } => {
                    let mut e: i32;
                    let mut nextrows: Arc<metamodelica::List<i32>>;
                    let mut b: bool;
                    let false = (listMember(r.clone(), vorphan.clone())) else { bail!("pattern mismatch") };
                    let false = (intEq(({let __elt = orowmarks.borrow()[(r.clone()-1) as usize].clone(); __elt}), 1)) else { bail!("pattern mismatch") };
                    e = ({let __elt = ass1.borrow()[(r.clone()-1) as usize].clone(); __elt});
                    let false = (intEq(({let __elt = ocolummarks.borrow()[(e.clone()-1) as usize].clone(); __elt}), 1)) else { bail!("pattern mismatch") };
                    let false = (intEq(({let __elt = colummarks.borrow()[(e.clone()-1) as usize].clone(); __elt}), mark)) else { bail!("pattern mismatch") };
                    nextrows = List::select(({let __elt = m.borrow()[(e.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
                    nextrows = List::setDifferenceOnTrue(nextrows.clone(), ({let __elt = ass2.borrow()[(e.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    metamodelica::arrayUpdate(colummarks.clone(), e.clone(), mark)?;
                    b = getIndexSubGraph(nextrows.clone(), vorphan.clone(), m.clone(), mT.clone(), mark, rowmarks.clone(), colummarks.clone(), orowmarks.clone(), ocolummarks.clone(), ass1.clone(), ass2.clone(), false);
                    markIndexSubgraph(b.clone(), ({let __elt = ass2.borrow()[(e.clone()-1) as usize].clone(); __elt}), mark, rowmarks.clone())?;
                    Ok(getIndexSubGraph(rest.clone(), vorphan.clone(), m.clone(), mT.clone(), mark, rowmarks.clone(), colummarks.clone(), orowmarks.clone(), ocolummarks.clone(), ass1.clone(), ass2.clone(), b.clone() || ifound))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(getIndexSubGraph(rest.clone(), vorphan.clone(), m.clone(), mT.clone(), mark, rowmarks.clone(), colummarks.clone(), orowmarks.clone(), ocolummarks.clone(), ass1.clone(), ass2.clone(), ifound))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    found
}

fn markIndexSubgraph(mut b: bool, mut r: Arc<metamodelica::List<i32>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>) -> Result<()> {
    let () = (match b {
        false => (),
        true => {
            List::map2_0(r, (std::sync::Arc::new(doMark) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<()> + 'static>), rowmarks.clone(), mark)?;
            ()
        },
    });
    Ok(())
}

fn getIndexesForEqnsRest(mut i: i32, mut size: i32, mut id: i32, mut mark: i32, mut colummarks: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut vec1: metamodelica::Array<i32>, mut vec2: metamodelica::Array<i32>) -> () {
    let () = 'mc: {
        let __mc_input = vec2.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let false = (intGt(i, size)) else { bail!("pattern mismatch") };
            let true = (intEq(mark, ({let __elt = colummarks.borrow()[(i-1) as usize].clone(); __elt}))) else { bail!("pattern mismatch") };
            getIndexesForEqnsRest(i + 1, size, id, mark, colummarks.clone(), ass1.clone(), ass2.clone(), vec1.clone(), vec2.clone());
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let false = (intGt(i, size)) else { bail!("pattern mismatch") };
            metamodelica::arrayUpdate(vec1.clone(), id, ({let __elt = ass2.borrow()[(i-1) as usize].clone(); __elt}))?;
            metamodelica::arrayUpdate(vec2.clone(), id, i)?;
            getIndexesForEqnsRest(i + 1, size, id + 1, mark, colummarks.clone(), ass1.clone(), ass2.clone(), vec1.clone(), vec2.clone());
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(())
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    ()
}

fn markIndexdColums(mut i: i32, mut size: i32, mut mark: i32, mut colummarks: metamodelica::Array<i32>, mut vec2: metamodelica::Array<i32>) -> () {
    let () = 'mc: {
        let __mc_input = vec2.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let false = (intGt(i, size)) else { bail!("pattern mismatch") };
            let true = (intGt(({let __elt = vec2.borrow()[(i-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
            metamodelica::arrayUpdate(colummarks.clone(), ({let __elt = vec2.borrow()[(i-1) as usize].clone(); __elt}), mark)?;
            markIndexdColums(i + 1, size, mark, colummarks.clone(), vec2.clone());
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let false = (intGt(i, size)) else { bail!("pattern mismatch") };
            markIndexdColums(i + 1, size, mark, colummarks.clone(), vec2.clone());
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(())
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    ()
}

fn getOrphans(mut indx: i32, mut size: i32, mut ass: metamodelica::Array<i32>, mut inOrphans: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<i32>> {
    let mut outOrphans: Arc<metamodelica::List<i32>>;
    outOrphans = 'mc: {
        let __mc_input = inOrphans.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (intGt(indx, size)) else { bail!("pattern mismatch") };
                    Ok(inOrphans.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut orphans: Arc<metamodelica::List<i32>>;
                    orphans = List::consOnTrue(intLt(({let __elt = ass.borrow()[(indx-1) as usize].clone(); __elt}), 1), indx, inOrphans.clone());
                    Ok(getOrphans(indx + 1, size, ass.clone(), orphans.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outOrphans
}

fn expHasCref(mut inExp: Arc<DAE::Exp>, mut cr: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut isthere: bool;
    let mut set: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    set = HashSet::emptyHashSet();
    set = addCrefandParentsToSet(cr, set, None)?;
    let (_, (_, __pa0)) = Expression::traverseExpTopDown(inExp, (std::sync::Arc::new(fnptr!(expHasCreftraverser, Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), bool))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), bool)) -> Result<(Arc<DAE::Exp>, bool, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), bool))> + 'static>), (set, false))?;
    isthere = __pa0.clone();
    Ok(isthere)
}

fn addCrefandParentsToSet(mut inCref: Arc<DAE::ComponentRef>, mut ihs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), mut oprecr: Option<Arc<DAE::ComponentRef>>) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inCref, oprecr)) {
        (cr @ Deref @ DAE::ComponentRef::CREF_IDENT { .. }, None) => {
            let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
            let mut set: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            crlst = ComponentReference::expandCref(cr.clone(), true)?;
            return Ok(List::fold(metamodelica::cons(cr.clone(), crlst), (std::sync::Arc::new(BaseHashSet::add) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), ihs)?)
        },
        (cr @ Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Some(precr)) => {
            let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
            let mut set: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            crlst = ComponentReference::expandCref(cr.clone(), true)?;
            crlst = List::map1r(metamodelica::cons(cr.clone(), crlst), (std::sync::Arc::new(ComponentReference::joinCrefs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> + 'static>), precr.clone())?;
            return Ok(List::fold(crlst, (std::sync::Arc::new(BaseHashSet::add) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), ihs)?)
        },
        (Deref @ DAE::ComponentRef::CREF_QUAL { ident, identType: ty, subscriptLst, componentRef: subcr }, None) => {
            let mut idcr: Arc<DAE::ComponentRef>;
            let mut set: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            idcr = ComponentReferenceBasics::makeCrefIdent((ident.clone()).clone(), ty.clone(), metamodelica::nil());
            set = BaseHashSet::add(idcr, ihs)?;
            idcr = ComponentReferenceBasics::makeCrefIdent((ident.clone()).clone(), ty.clone(), subscriptLst.clone());
            set = BaseHashSet::add(idcr.clone(), set)?;
            { (inCref, ihs, oprecr) = (subcr.clone(), set, Some(idcr)); continue '__tco; }
        },
        (Deref @ DAE::ComponentRef::CREF_QUAL { ident, identType: ty, subscriptLst, componentRef: subcr }, Some(precr)) => {
            let mut idcr: Arc<DAE::ComponentRef>;
            let mut set: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut precr = (*precr).clone();
            idcr = ComponentReferenceBasics::makeCrefIdent((ident.clone()).clone(), ty.clone(), metamodelica::nil());
            idcr = ComponentReference::joinCrefs(precr.clone(), idcr)?;
            set = BaseHashSet::add(idcr, ihs.clone())?;
            idcr = ComponentReferenceBasics::makeCrefIdent((ident.clone()).clone(), ty.clone(), subscriptLst.clone());
            precr = ComponentReference::joinCrefs(precr.clone(), idcr)?;
            set = BaseHashSet::add(precr.clone(), ihs)?;
            { (inCref, ihs, oprecr) = (subcr.clone(), set, Some(precr.clone())); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn expHasCreftraverser(mut e: Arc<DAE::Exp>, mut inTpl: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), bool)) -> (Arc<DAE::Exp>, bool, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), bool)) {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool;
    let mut outTpl: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), bool);
    (outExp, cont, outTpl) = 'mc: {
        let __mc_input = (e.clone(), inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (set, false)) => {
                    let mut b: bool;
                    b = BaseHashSet::has(cr.clone(), set.clone())?;
                    Ok((e.clone(), !(b.clone()), (set.clone(), b.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (set, b)) => {
                    Ok((e.clone(), !(b.clone()), (set.clone(), b.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, cont, outTpl)
}

fn assignLst(mut vlst: Arc<metamodelica::List<i32>>, mut e: i32, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(vlst) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: v, tail: rest } => {
            metamodelica::arrayUpdate(ass1.clone(), v.clone(), e)?;
            metamodelica::arrayUpdate(ass2.clone(), e, v.clone())?;
            assignLst(rest.clone(), e + 1, ass1.clone(), ass2.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn unassignedLst(mut vlst: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(vlst) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: v, tail: rest } => {
            let false = (intGt(({let __elt = ass1.borrow()[(v.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
            unassignedLst(rest.clone(), ass1.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn onefreeMatchingBFS(mut queue: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut size: i32, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut columark: metamodelica::Array<i32>, mut mark: i32, mut nextQeue: Arc<metamodelica::List<i32>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((queue, nextQeue.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            ()
        },
        (Deref @ metamodelica::List::Nil, _) => {
            onefreeMatchingBFS(nextQeue, m.clone(), mt.clone(), size, ass1.clone(), ass2.clone(), columark.clone(), mark, metamodelica::nil())?;
            ()
        },
        (Deref @ metamodelica::List::Cons { head: c, tail: rest }, _) => {
            let mut newqueue: Arc<metamodelica::List<i32>>;
            let mut rows: Arc<metamodelica::List<i32>>;
            rows = List::removeOnTrue(ass1.clone(), (std::sync::Arc::new(fnptr!(isAssignedSaveEnhanced, metamodelica::Array<i32>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), ({let __elt = m.borrow()[(c.clone()-1) as usize].clone(); __elt}))?;
            newqueue = onefreeMatchingBFS1(rows, c.clone(), mt.clone(), ass1.clone(), ass2.clone(), columark.clone(), mark, nextQeue);
            onefreeMatchingBFS(rest.clone(), m.clone(), mt.clone(), size, ass1.clone(), ass2.clone(), columark.clone(), mark, newqueue)?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn isAssignedSaveEnhanced(mut ass: metamodelica::Array<i32>, mut inTpl: i32) -> bool {
    let mut outB: bool;
    outB = if (intGt(inTpl, 0)) {intGt(({let __elt = ass.borrow()[(inTpl-1) as usize].clone(); __elt}), 0)} else {true};
    outB
}

fn onefreeMatchingBFS1(mut rows: Arc<metamodelica::List<i32>>, mut c: i32, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut columark: metamodelica::Array<i32>, mut mark: i32, mut inNextQeue: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<i32>> {
    let mut outNextQeue: Arc<metamodelica::List<i32>>;
    outNextQeue = 'mc: {
        let __mc_input = rows;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r, tail: Deref @ metamodelica::List::Nil } => {
                    let mut vareqns: Arc<metamodelica::List<i32>>;
                    metamodelica::arrayUpdate(ass1.clone(), r.clone(), c)?;
                    metamodelica::arrayUpdate(ass2.clone(), c, r.clone())?;
                    vareqns = List::removeOnTrue(ass2.clone(), (std::sync::Arc::new(fnptr!(isAssignedSaveEnhanced, metamodelica::Array<i32>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), ({let __elt = mt.borrow()[(r.clone()-1) as usize].clone(); __elt}))?;
                    Ok(listAppend(inNextQeue.clone(), vareqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inNextQeue.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outNextQeue
}

fn vectorMatching(mut eqn: Arc<BackendDAE::Equation>, mut vars: BackendDAE::Variables, mut inTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>)) -> Result<(i32, metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut outTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>);
    outTpl = 'mc: {
        let __mc_input = (eqn.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::ARRAY_EQUATION { dimSize: ds, left: e1, right: e2, .. }, _) => {
                    let mut size: i32;
                    let mut tpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>);
                    size = List::fold(ds.clone(), (std::sync::Arc::new(fnptr!(intMul, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), 1)?;
                    tpl = vectorMatching1(e1.clone(), e2.clone(), size.clone(), vars.clone(), inTpl.clone())?;
                    Ok(tpl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::ARRAY_EQUATION { dimSize: ds, left: e2, right: e1, .. }, _) => {
                    let mut size: i32;
                    let mut tpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>);
                    size = List::fold(ds.clone(), (std::sync::Arc::new(fnptr!(intMul, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), 1)?;
                    tpl = vectorMatching1(e2.clone(), e1.clone(), size.clone(), vars.clone(), inTpl.clone())?;
                    Ok(tpl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::COMPLEX_EQUATION { size, left: e1, right: e2, .. }, _) => {
                    let mut tpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>);
                    tpl = vectorMatching1(e1.clone(), e2.clone(), size.clone(), vars.clone(), inTpl.clone())?;
                    Ok(tpl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::COMPLEX_EQUATION { size, left: e2, right: e1, .. }, _) => {
                    let mut tpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>);
                    tpl = vectorMatching1(e2.clone(), e1.clone(), size.clone(), vars.clone(), inTpl.clone())?;
                    Ok(tpl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (id, vec1, vec2)) => {
                    let mut size: i32;
                    size = BackendEquation::equationSize(eqn.clone())?;
                    Ok((id.clone() + size.clone(), vec1.clone(), vec2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTpl)
}

fn vectorMatching1(mut e1: Arc<DAE::Exp>, mut e2: Arc<DAE::Exp>, mut size: i32, mut vars: BackendDAE::Variables, mut inTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>)) -> Result<(i32, metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut outTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>);
    outTpl = 'mc: {
        let __mc_input = (e1.clone(), e2.clone(), inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, _, (id, vec1, vec2)) => {
                    let mut ilst: Arc<metamodelica::List<i32>>;
                    let false = (expHasCref(e2.clone(), cr.clone())?) else { bail!("pattern mismatch") };
                    (_, ilst) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    let true = (intEq(size, (ilst.clone().len() as i32))) else { bail!("pattern mismatch") };
                    unassignedLst(ilst.clone(), vec1.clone())?;
                    assignLst(ilst.clone(), id.clone(), vec1.clone(), vec2.clone())?;
                    Ok((id.clone() + size, vec1.clone(), vec2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (id, vec1, vec2)) => {
                    let mut ilst: Arc<metamodelica::List<i32>>;
                    let false = (expHasCref(e1.clone(), cr.clone())?) else { bail!("pattern mismatch") };
                    (_, ilst) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    let true = (intEq(size, (ilst.clone().len() as i32))) else { bail!("pattern mismatch") };
                    unassignedLst(ilst.clone(), vec1.clone())?;
                    assignLst(ilst.clone(), id.clone(), vec1.clone(), vec2.clone())?;
                    Ok((id.clone() + size, vec1.clone(), vec2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. } }, _, (id, vec1, vec2)) => {
                    let mut ilst: Arc<metamodelica::List<i32>>;
                    let false = (expHasCref(e2.clone(), cr.clone())?) else { bail!("pattern mismatch") };
                    (_, ilst) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    let true = (intEq(size, (ilst.clone().len() as i32))) else { bail!("pattern mismatch") };
                    unassignedLst(ilst.clone(), vec1.clone())?;
                    assignLst(ilst.clone(), id.clone(), vec1.clone(), vec2.clone())?;
                    Ok((id.clone() + size, vec1.clone(), vec2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. } }, (id, vec1, vec2)) => {
                    let mut ilst: Arc<metamodelica::List<i32>>;
                    let false = (expHasCref(e1.clone(), cr.clone())?) else { bail!("pattern mismatch") };
                    (_, ilst) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    let true = (intEq(size, (ilst.clone().len() as i32))) else { bail!("pattern mismatch") };
                    unassignedLst(ilst.clone(), vec1.clone())?;
                    assignLst(ilst.clone(), id.clone(), vec1.clone(), vec2.clone())?;
                    Ok((id.clone() + size, vec1.clone(), vec2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, (id, vec1, vec2)) => {
                    let mut cr: Arc<DAE::ComponentRef>;
                    let mut crnosubs: Arc<DAE::ComponentRef>;
                    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    let mut crlst1: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    let mut elst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut ilst: Arc<metamodelica::List<i32>>;
                    let mut set: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    elst = Expression::flattenArrayExpToList(e1.clone());
                    crlst = List::map(elst.clone(), (std::sync::Arc::new(Expression::expCrefNegCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::ComponentRef>> + 'static>))?;
                    crlst = List::uniqueOnTrue(crlst.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefEqualNoStringCompare) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>))?;
                    let true = (intEq(size, (crlst.clone().len() as i32))) else { bail!("pattern mismatch") };
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(crlst.clone()) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cr = __pa0.clone();
                    crlst1 = __pa1.clone();
                    let true = (List::all(crlst1.clone(), (std::sync::Arc::new({ let __pe_b1 = cr.clone(); move |__pe_a0| ComponentReferenceBasics::crefEqualWithoutLastSubs(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<bool> + 'static>))?) else { bail!("pattern mismatch") };
                    set = HashSet::emptyHashSet();
                    crnosubs = ComponentReferenceBasics::crefStripLastSubs(cr.clone())?;
                    set = addCrefandParentsToSet(crnosubs.clone(), set.clone(), None)?;
                    set = List::fold(crlst.clone(), (std::sync::Arc::new(BaseHashSet::add) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), set.clone())?;
                    ::match_deref::match_deref! { match &(Expression::traverseExpTopDown(e2.clone(), (std::sync::Arc::new(fnptr!(expHasCreftraverser, Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), bool))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), bool)) -> Result<(Arc<DAE::Exp>, bool, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), bool))> + 'static>), (set.clone(), false))?) {
                        (_, (_, false)) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    (_, ilst) = BackendVariable::getVarLst(crlst.clone(), vars.clone());
                    unassignedLst(ilst.clone(), vec1.clone())?;
                    assignLst(ilst.clone(), id.clone(), vec1.clone(), vec2.clone())?;
                    Ok((id.clone() + size, vec1.clone(), vec2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, (id, vec1, vec2)) => {
                    let mut cr: Arc<DAE::ComponentRef>;
                    let mut crnosubs: Arc<DAE::ComponentRef>;
                    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    let mut crlst1: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    let mut elst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut ilst: Arc<metamodelica::List<i32>>;
                    let mut set: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    elst = Expression::flattenArrayExpToList(e2.clone());
                    crlst = List::map(elst.clone(), (std::sync::Arc::new(Expression::expCrefNegCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::ComponentRef>> + 'static>))?;
                    crlst = List::uniqueOnTrue(crlst.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefEqualNoStringCompare) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>))?;
                    let true = (intEq(size, (crlst.clone().len() as i32))) else { bail!("pattern mismatch") };
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(crlst.clone()) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cr = __pa0.clone();
                    crlst1 = __pa1.clone();
                    let true = (List::all(crlst1.clone(), (std::sync::Arc::new({ let __pe_b1 = cr.clone(); move |__pe_a0| ComponentReferenceBasics::crefEqualWithoutLastSubs(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<bool> + 'static>))?) else { bail!("pattern mismatch") };
                    set = HashSet::emptyHashSet();
                    crnosubs = ComponentReferenceBasics::crefStripLastSubs(cr.clone())?;
                    set = addCrefandParentsToSet(crnosubs.clone(), set.clone(), None)?;
                    set = List::fold(crlst.clone(), (std::sync::Arc::new(BaseHashSet::add) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), set.clone())?;
                    ::match_deref::match_deref! { match &(Expression::traverseExpTopDown(e1.clone(), (std::sync::Arc::new(fnptr!(expHasCreftraverser, Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), bool))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), bool)) -> Result<(Arc<DAE::Exp>, bool, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), bool))> + 'static>), (set.clone(), false))?) {
                        (_, (_, false)) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    (_, ilst) = BackendVariable::getVarLst(crlst.clone(), vars.clone());
                    unassignedLst(ilst.clone(), vec1.clone())?;
                    assignLst(ilst.clone(), id.clone(), vec1.clone(), vec2.clone())?;
                    Ok((id.clone() + size, vec1.clone(), vec2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTpl)
}

fn aliasMatching(mut eqn: Arc<BackendDAE::Equation>, mut vars: BackendDAE::Variables, mut inTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>)) -> Result<(i32, metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut outTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>);
    outTpl = 'mc: {
        let __mc_input = (eqn.clone(), inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, scalar: Deref @ DAE::Exp::CREF { componentRef: cr2, .. }, .. }, (id, vec1, vec2)) => {
                    let mut i: i32;
                    let mut i1: i32;
                    let mut i2: i32;
                    let mut vec1 = (*vec1).clone();
                    let mut vec2 = (*vec2).clone();
                    let false = (intGt(({let __elt = vec2.borrow()[(id.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
                    (_, i1) = BackendVariable::getVarSingle(cr1.clone(), vars.clone())?;
                    (_, i2) = BackendVariable::getVarSingle(cr2.clone(), vars.clone())?;
                    i = aliasMatching1(i1.clone(), i2.clone(), intGt(({let __elt = vec1.borrow()[(i1.clone()-1) as usize].clone(); __elt}), 0), intGt(({let __elt = vec1.borrow()[(i2.clone()-1) as usize].clone(); __elt}), 0))?;
                    vec1 = metamodelica::arrayUpdate(vec1.clone(), i.clone(), id.clone())?;
                    vec2 = metamodelica::arrayUpdate(vec2.clone(), id.clone(), i.clone())?;
                    Ok((id.clone() + 1, vec1.clone(), vec2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (id, vec1, vec2)) => {
                    let mut size: i32;
                    size = BackendEquation::equationSize(eqn.clone())?;
                    Ok((id.clone() + size.clone(), vec1.clone(), vec2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTpl)
}

fn aliasMatching1(mut i1: i32, mut i2: i32, mut b1: bool, mut b2: bool) -> Result<i32> {
    let mut i: i32;
    i = (match (b1, b2) {
        (false, true) => i1,
        (true, false) => i2,
        _ => bail!("match: no arm matched"),
    });
    Ok(i)
}

fn naturalMatching(mut eqn: Arc<BackendDAE::Equation>, mut vars: BackendDAE::Variables, mut inTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>)) -> (i32, metamodelica::Array<i32>, metamodelica::Array<i32>) {
    let mut outTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>);
    outTpl = 'mc: {
        let __mc_input = (eqn, inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. }, (id, vec1, vec2)) => {
                    let mut i: i32;
                    let mut vec1 = (*vec1).clone();
                    let mut vec2 = (*vec2).clone();
                    let false = (intGt(({let __elt = vec2.borrow()[(id.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (_, Deref @ metamodelica::List::Cons { head: __pa0, tail: _ }) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    i = __pa0.clone();
                    let false = (intGt(({let __elt = vec1.borrow()[(i.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
                    vec1 = metamodelica::arrayUpdate(vec1.clone(), i.clone(), id.clone())?;
                    vec2 = metamodelica::arrayUpdate(vec2.clone(), id.clone(), i.clone())?;
                    Ok((id.clone() + 1, vec1.clone(), vec2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (id, vec1, vec2)) => {
                    Ok((id.clone() + 1, vec1.clone(), vec2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outTpl
}

fn naturalMatching1(mut eqn: Arc<BackendDAE::Equation>, mut vars: BackendDAE::Variables, mut inTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>)) -> (i32, metamodelica::Array<i32>, metamodelica::Array<i32>) {
    let mut outTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>);
    outTpl = 'mc: {
        let __mc_input = (eqn, inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. }, (id, vec1, vec2)) => {
                    let mut i: i32;
                    let mut vec1 = (*vec1).clone();
                    let mut vec2 = (*vec2).clone();
                    let false = (intGt(({let __elt = vec2.borrow()[(id.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (_, Deref @ metamodelica::List::Cons { head: __pa0, tail: _ }) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    i = __pa0.clone();
                    let false = (intGt(({let __elt = vec1.borrow()[(i.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
                    vec1 = metamodelica::arrayUpdate(vec1.clone(), i.clone(), id.clone())?;
                    vec2 = metamodelica::arrayUpdate(vec2.clone(), id.clone(), i.clone())?;
                    Ok((id.clone() + 1, vec1.clone(), vec2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (id, vec1, vec2)) => {
                    Ok((id.clone() + 1, vec1.clone(), vec2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outTpl
}

fn naturalMatching2(mut eqn: Arc<BackendDAE::Equation>, mut vars: BackendDAE::Variables, mut inTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>)) -> (i32, metamodelica::Array<i32>, metamodelica::Array<i32>) {
    let mut outTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>);
    outTpl = 'mc: {
        let __mc_input = (eqn.clone(), inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, .. }, (id, vec1, vec2)) => {
                    let mut i: i32;
                    let mut e: Arc<DAE::Exp>;
                    let mut vlst: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut vec1 = (*vec1).clone();
                    let mut vec2 = (*vec2).clone();
                    let false = (intGt(({let __elt = vec2.borrow()[(id.clone()-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
                    e = Expression::expSub(e1.clone(), e2.clone())?;
                    vlst = BackendEquation::equationVars(eqn.clone(), vars.clone())?;
                    (_, i) = getConstOneVariable(vlst.clone(), e.clone(), vec1.clone(), vars.clone())?;
                    vec1 = metamodelica::arrayUpdate(vec1.clone(), i.clone(), id.clone())?;
                    vec2 = metamodelica::arrayUpdate(vec2.clone(), id.clone(), i.clone())?;
                    Ok((id.clone() + 1, vec1.clone(), vec2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (id, vec1, vec2)) => {
                    Ok((id.clone() + 1, vec1.clone(), vec2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outTpl
}

fn getConstOneVariable(mut vlst: Arc<metamodelica::List<BackendDAE::Var>>, mut e: Arc<DAE::Exp>, mut vec1: metamodelica::Array<i32>, mut vars: BackendDAE::Variables) -> Result<(Arc<DAE::ComponentRef>, i32)> {
    let mut outCr: Arc<DAE::ComponentRef>;
    let mut i: i32 = 0;
    (outCr, i) = 'mc: {
        let __mc_input = vlst;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: v, tail: _ } => {
                    let mut cr: Arc<DAE::ComponentRef>;
                    let mut e1: Arc<DAE::Exp>;
                    let mut e2: Arc<DAE::Exp>;
                    let mut i: i32 = i.clone();
                    cr = BackendVariable::varCref(v.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (_, Deref @ metamodelica::List::Cons { head: __pa0, tail: _ }) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    i = __pa0.clone();
                    let false = (intGt(({let __elt = vec1.borrow()[(i-1) as usize].clone(); __elt}), 0)) else { bail!("pattern mismatch") };
                    e1 = Differentiate::differentiateExpSolve(e.clone(), cr.clone(), None)?;
                    (e2, _) = ExpressionSimplify::simplify(e1.clone())?;
                    let true = (Expression::isConstOne(e2.clone()) || Expression::isConstMinusOne(e2.clone())) else { bail!("pattern mismatch") };
                    Ok(((cr.clone(), i), i.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { i = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    let mut cr: Arc<DAE::ComponentRef>;
                    let mut i: i32 = i.clone();
                    (cr, i) = getConstOneVariable(rest.clone(), e.clone(), vec1.clone(), vars.clone())?;
                    Ok(((cr.clone(), i), i.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { i = __wb0; break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCr, i))
}

