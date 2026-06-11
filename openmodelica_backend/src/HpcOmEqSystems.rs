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

use crate::BackendDAETransform;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVarTransform;
use crate::BackendVariable;
use crate::ExpressionSolve;
use crate::HpcOmScheduler;
use crate::HpcOmSimCodeMain;
use crate::HpcOmTaskGraph;
use crate::IndexReduction;
use crate::Matching;
use crate::Tearing;
use openmodelica_backend_types::BackendDAE;
use openmodelica_backend_util::BackendDAEEXT;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_simcode_types::HpcOmSimCode;
use openmodelica_simcode_types::SimCodeVar;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

// public imports
// protected imports
//--------------------------------------------------//
// matrix type
//-------------------------------------------------//
#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct EqSys {
    pub dim: i32,
    pub matrixA: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>,
    pub vectorB: metamodelica::Array<Arc<DAE::Exp>>,
    pub vectorX: metamodelica::Array<BackendDAE::Var>,
}

impl metamodelica::gc::MMTrace for EqSys {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.dim, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.matrixA, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.vectorB, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.vectorX, __mmv)?;
        Ok(())
    }
}
impl Default for EqSys {
    fn default() -> Self {
        Self {
            dim: Default::default(),
            matrixA: Default::default(),
            vectorB: Default::default(),
            vectorX: Default::default(),
        }
    }
}

pub type LINSYS = EqSys;


//--------------------------------------------------//
// start functions for handling linearTornSystems from here
//-------------------------------------------------//
pub(crate) fn partitionLinearTornSystem(mut daeIn: Arc<BackendDAE::BackendDAE>) -> Arc<BackendDAE::BackendDAE> {
    let mut daeOut: Arc<BackendDAE::BackendDAE>;
    daeOut = 'mc: {
        let __mc_input = daeIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::BackendDAE { eqs, shared } => {
                    let mut eqs = (*eqs).clone();
                    let true = (intGt(Flags::getConfigInt(Flags::PARTLINTORN.clone())?, 0)) else { bail!("pattern mismatch") };
                    (eqs, _) = List::map1Fold(eqs.clone(), (std::sync::Arc::new(reduceLinearTornSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32) -> Result<(Arc<BackendDAE::EqSystem>, i32)> + 'static>), shared.clone(), 1)?;
                    Ok(Arc::new(BackendDAE::BackendDAE { eqs: eqs.clone(), shared: shared.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(daeIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    daeOut
}

fn reduceLinearTornSystem(mut systIn: Arc<BackendDAE::EqSystem>, mut sharedIn: Arc<BackendDAE::Shared>, mut tornSysIdxIn: i32) -> Result<(Arc<BackendDAE::EqSystem>, i32)> {
    let mut systOut: Arc<BackendDAE::EqSystem>;
    let mut tornSysIdxOut: i32;
    (systOut, tornSysIdxOut) = 'mc: {
        let __mc_input = tornSysIdxIn;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut tornSysIdx: i32;
            let mut ass1: metamodelica::Array<i32>;
            let mut ass2: metamodelica::Array<i32>;
            let mut systTmp: Arc<BackendDAE::EqSystem>;
            let mut allComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(systIn.clone()) {
                Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { ass1: __pa0, ass2: __pa1, comps: __pa2 }, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            ass1 = __pa0.clone();
            ass2 = __pa1.clone();
            allComps = __pa2.clone();
            (systTmp, tornSysIdx) = reduceLinearTornSystem1(1, allComps.clone(), ass1.clone(), ass2.clone(), systIn.clone(), sharedIn.clone(), tornSysIdxIn);
            Ok((systTmp.clone(), tornSysIdx.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            metamodelica::print((literal!("reduceLinearTornSystem failed!")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((systOut, tornSysIdxOut))
}

fn reduceLinearTornSystem1(mut compIdx: i32, mut compsIn: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut systIn: Arc<BackendDAE::EqSystem>, mut sharedIn: Arc<BackendDAE::Shared>, mut tornSysIdxIn: i32) -> (Arc<BackendDAE::EqSystem>, i32) {
    let mut systOut: Arc<BackendDAE::EqSystem>;
    let mut tornSysIdxOut: i32;
    (systOut, tornSysIdxOut) = 'mc: {
        let __mc_input = systIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = ((compsIn.clone().len() as i32) < compIdx) else { bail!("pattern mismatch") };
                    Ok((systIn.clone(), tornSysIdxIn))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                syst => {
                    let mut numNewSingleEqs: i32;
                    let mut tornSysIdx: i32;
                    let mut linear: bool;
                    let mut ass1New: metamodelica::Array<i32>;
                    let mut ass2New: metamodelica::Array<i32>;
                    let mut ass1All: metamodelica::Array<i32>;
                    let mut ass2All: metamodelica::Array<i32>;
                    let mut tvarIdcs: Arc<metamodelica::List<i32>>;
                    let mut resEqIdcs: Arc<metamodelica::List<i32>>;
                    let mut innerEquations: Arc<metamodelica::List<BackendDAE::InnerEquation>>;
                    let mut matchingNew: Arc<BackendDAE::Matching>;
                    let mut matchingOther: Arc<BackendDAE::Matching>;
                    let mut comp: Arc<BackendDAE::StrongComponent>;
                    let mut compsNew: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
                    let mut compsTmp: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
                    let mut otherComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
                    let mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut eqsNew: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut eqsOld: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut resEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut varsNew: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut varsOld: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut syst = (*syst).clone();
                    let true = ((compsIn.clone().len() as i32) >= compIdx) else { bail!("pattern mismatch") };
                    comp = (compsIn.clone()).get(compIdx)?;
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(comp.clone()) {
                        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: __pa0, residualequations: __pa1, innerEquations: __pa2, .. }, linear: __pa3, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    tvarIdcs = __pa0.clone();
                    resEqIdcs = __pa1.clone();
                    innerEquations = __pa2.clone();
                    linear = __pa3.clone();
                    let true = (linear.clone()) else { bail!("pattern mismatch") };
                    let true = (intLe((tvarIdcs.clone().len() as i32), Flags::getConfigInt(Flags::PARTLINTORN.clone())?)) else { bail!("pattern mismatch") };
                    (varsNew, eqsNew, _, resEqs, matchingNew) = reduceLinearTornSystem2(systIn.clone(), sharedIn.clone(), tvarIdcs.clone(), resEqIdcs.clone(), innerEquations.clone(), tornSysIdxIn)?;
                    let (__pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(matchingNew.clone()) {
                        Deref @ BackendDAE::Matching::MATCHING { ass1: __pa4, ass2: __pa5, comps: __pa6 } => (__pa4.clone(), __pa5.clone(), __pa6.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    ass1New = __pa4.clone();
                    ass2New = __pa5.clone();
                    compsNew = __pa6.clone();
                    varsOld = BackendVariable::varList(syst.orderedVars.clone())?;
                    eqsOld = BackendEquation::equationList(syst.orderedEqs.clone())?;
                    varLst = listAppend(varsOld.clone(), varsNew.clone());
                    eqLst = listAppend(eqsOld.clone(), eqsNew.clone());
                    eqLst = List::fold2(List::intRange((resEqIdcs.clone().len() as i32)), (std::sync::Arc::new(replaceAtPositionFromList) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, Arc<metamodelica::List<i32>>, _) -> Result<_> + 'static>), resEqs.clone(), resEqIdcs.clone(), eqLst.clone())?;
                    assign_field!(
                        syst.orderedVars = BackendVariable::listVar1(varLst.clone())?,
                        syst.orderedEqs = BackendEquation::listEquation(eqLst.clone())?
                    );
                    ass1All = arrayCreate((varLst.clone().len() as i32), -1);
                    ass2All = arrayCreate((varLst.clone().len() as i32), -1);
                    ass1All = Array::copy(ass1.clone(), ass1All.clone())?;
                    ass2All = Array::copy(ass2.clone(), ass2All.clone())?;
                    (ass1All, ass2All) = List::fold2(List::intRange((tvarIdcs.clone().len() as i32)), (std::sync::Arc::new(updateResidualMatching) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, (metamodelica::Array<i32>, metamodelica::Array<i32>)) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>)> + 'static>), tvarIdcs.clone(), resEqIdcs.clone(), (ass1All.clone(), ass2All.clone()))?;
                    matchingOther = getOtherComps(innerEquations.clone(), ass1All.clone(), ass2All.clone())?;
                    let __pa7 = ::match_deref::match_deref! { match &(matchingOther.clone()) {
                        Deref @ BackendDAE::Matching::MATCHING { comps: __pa7, .. } => __pa7.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    otherComps = __pa7.clone();
                    numNewSingleEqs = (compsNew.clone().len() as i32) - (tvarIdcs.clone().len() as i32);
                    compsTmp = List::replaceAtWithList(listAppend(compsNew.clone(), otherComps.clone()), compIdx - 1, compsIn.clone())?;
                    (ass1All, ass2All) = List::fold2(List::intRange(metamodelica::arrayLength(ass1New.clone())), (std::sync::Arc::new(updateMatching) as std::sync::Arc<dyn ::std::ops::Fn(i32, (i32, i32), (metamodelica::Array<i32>, metamodelica::Array<i32>), (metamodelica::Array<i32>, metamodelica::Array<i32>)) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>)> + 'static>), ((eqsOld.clone().len() as i32), (varsOld.clone().len() as i32)), (ass1New.clone(), ass2New.clone()), (ass1All.clone(), ass2All.clone()))?;
                    assign_field!(syst.matching = Arc::new(BackendDAE::Matching::MATCHING { ass1: ass1All.clone(), ass2: ass2All.clone(), comps: compsTmp.clone() }));
                    syst = BackendDAEUtil::setEqSystMatrices(syst.clone(), None, None, None)?;
                    (syst, _, _) = BackendDAEUtil::getAdjacencyMatrix(syst.clone(), openmodelica_backend_types::BackendDAE::IndexType::NORMAL, None, BackendDAEUtil::isInitializationDAE(sharedIn.clone()))?;
                    (syst, tornSysIdx) = reduceLinearTornSystem1(compIdx + 1 + numNewSingleEqs.clone(), compsTmp.clone(), ass1All.clone(), ass2All.clone(), syst.clone(), sharedIn.clone(), tornSysIdxIn + 1);
                    Ok((syst.clone(), tornSysIdx.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                syst @ Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqs, .. } => {
                    let mut tornSysIdx: i32;
                    let mut ass1All: metamodelica::Array<i32>;
                    let mut ass2All: metamodelica::Array<i32>;
                    let mut eqIdcs: Arc<metamodelica::List<i32>>;
                    let mut varIdcs: Arc<metamodelica::List<i32>>;
                    let mut hpcSyst: EqSys;
                    let mut comp: Arc<BackendDAE::StrongComponent>;
                    let mut compsNew: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
                    let mut compsTmp: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
                    let mut otherComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
                    let mut derRepl: BackendVarTransform::VariableReplacements;
                    let mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut eqsNew: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut eqsOld: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut addEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut varLstRepl: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut varsOld: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut addVars: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut syst = (*syst).clone();
                    let true = ((compsIn.clone().len() as i32) >= compIdx) else { bail!("pattern mismatch") };
                    comp = (compsIn.clone()).get(compIdx)?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(comp.clone()) {
                        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { vars: __pa0, eqns: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    varIdcs = __pa0.clone();
                    eqIdcs = __pa1.clone();
                    let true = (intLe((varIdcs.clone().len() as i32), 2)) else { bail!("pattern mismatch") };
                    eqLst = BackendEquation::getList(eqIdcs.clone(), eqs.clone())?;
                    eqLst = BackendEquation::replaceDerOpInEquationList(eqLst.clone())?;
                    varLst = List::map1r(varIdcs.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone())?;
                    varLstRepl = List::map(varLst.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::transformXToXd, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<BackendDAE::Var> + 'static>))?;
                    derRepl = BackendVarTransform::emptyReplacements();
                    derRepl = List::threadFold(varLst.clone(), varLstRepl.clone(), (std::sync::Arc::new(addDerReplacement) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendDAE::Var, BackendVarTransform::VariableReplacements) -> Result<BackendVarTransform::VariableReplacements> + 'static>), derRepl.clone())?;
                    hpcSyst = getEqSystem(eqLst.clone(), varLstRepl.clone())?;
                    (eqsNew, addEqs, addVars) = CramerRule(hpcSyst.clone());
                    (eqsNew, _) = BackendVarTransform::replaceEquations(eqsNew.clone(), derRepl.clone(), None)?;
                    varsOld = BackendVariable::varList(vars.clone())?;
                    eqsOld = BackendEquation::equationList(eqs.clone())?;
                    compsNew = matchComponent(eqsNew.clone(), varLstRepl.clone(), eqIdcs.clone(), varIdcs.clone(), sharedIn.clone())?;
                    otherComps = matchComponent(addEqs.clone(), addVars.clone(), List::intRange2((eqsOld.clone().len() as i32) + 1, (eqsOld.clone().len() as i32) + 1 + (addEqs.clone().len() as i32)), List::intRange2((varsOld.clone().len() as i32) + 1, (varsOld.clone().len() as i32) + 1 + (addVars.clone().len() as i32)), sharedIn.clone())?;
                    compsNew = listAppend(otherComps.clone(), compsNew.clone());
                    compsTmp = List::replaceAtWithList(compsNew.clone(), compIdx - 1, compsIn.clone())?;
                    eqLst = listAppend(eqsOld.clone(), addEqs.clone());
                    varLst = listAppend(varsOld.clone(), addVars.clone());
                    eqLst = List::fold2(List::intRange((eqsNew.clone().len() as i32)), (std::sync::Arc::new(replaceAtPositionFromList) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, Arc<metamodelica::List<i32>>, _) -> Result<_> + 'static>), eqsNew.clone(), eqIdcs.clone(), eqLst.clone())?;
                    assign_field!(
                        syst.orderedEqs = BackendEquation::listEquation(eqLst.clone())?,
                        syst.orderedVars = BackendVariable::listVar1(varLst.clone())?
                    );
                    ass1All = arrayCreate((varLst.clone().len() as i32), -1);
                    ass2All = arrayCreate((varLst.clone().len() as i32), -1);
                    ass1All = Array::copy(ass1.clone(), ass1All.clone())?;
                    ass2All = Array::copy(ass2.clone(), ass2All.clone())?;
                    List::map2_0(compsNew.clone(), (std::sync::Arc::new(updateAssignmentsByComp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::StrongComponent>, metamodelica::Array<i32>, metamodelica::Array<i32>) -> Result<()> + 'static>), ass1All.clone(), ass2All.clone())?;
                    assign_field!(syst.matching = Arc::new(BackendDAE::Matching::MATCHING { ass1: ass1All.clone(), ass2: ass2All.clone(), comps: compsTmp.clone() }));
                    syst = BackendDAEUtil::setEqSystMatrices(syst.clone(), None, None, None)?;
                    (syst, tornSysIdx) = reduceLinearTornSystem1(compIdx + 1, compsTmp.clone(), ass1All.clone(), ass2All.clone(), syst.clone(), sharedIn.clone(), tornSysIdxIn + 1);
                    Ok((syst.clone(), tornSysIdx.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut tornSysIdx: i32;
                    let mut syst: Arc<BackendDAE::EqSystem>;
                    (syst, tornSysIdx) = reduceLinearTornSystem1(compIdx + 1, compsIn.clone(), ass1.clone(), ass2.clone(), systIn.clone(), sharedIn.clone(), tornSysIdxIn);
                    Ok((syst.clone(), tornSysIdx.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (systOut, tornSysIdxOut)
}

fn compHasDummyState(mut comp: Arc<BackendDAE::StrongComponent>, mut syst: Arc<BackendDAE::EqSystem>) -> Result<bool> {
    let mut hasDummy: bool;
    hasDummy = (::match_deref::match_deref! { match &((comp, syst)) {
        (Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: varIdcs, .. }, .. }, Deref @ BackendDAE::EqSystem { orderedVars: vars, .. }) => {
            let mut b: bool;
            let mut varLst: Arc<metamodelica::List<BackendDAE::Var>>;
            varLst = List::map1(varIdcs.clone(), (std::sync::Arc::new(BackendVariable::getVarAtIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, BackendDAE::Variables) -> Result<BackendDAE::Var> + 'static>), vars.clone())?;
            b = List::fold(List::map(varLst.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::isDummyStateVar, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>))?, (std::sync::Arc::new(fnptr!(boolOr, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>), false)?;
            b = b.clone() && intGt((varIdcs.clone().len() as i32), 1);
            b.clone()
        },
        (Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { vars: varIdcs, .. }, Deref @ BackendDAE::EqSystem { orderedVars: vars, .. }) => {
            let mut b: bool;
            let mut varLst: Arc<metamodelica::List<BackendDAE::Var>>;
            varLst = List::map1(varIdcs.clone(), (std::sync::Arc::new(BackendVariable::getVarAtIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, BackendDAE::Variables) -> Result<BackendDAE::Var> + 'static>), vars.clone())?;
            b = List::fold(List::map(varLst.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::isDummyStateVar, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>))?, (std::sync::Arc::new(fnptr!(boolOr, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>), false)?;
            b.clone()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(hasDummy)
}

fn updateAssignmentsByComp(mut comp: Arc<BackendDAE::StrongComponent>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<()> {
    let mut eqn: i32;
    let mut var: i32;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(comp) {
        Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { eqn: __pa0, var: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    eqn = __pa0.clone();
    var = __pa1.clone();
    metamodelica::arrayUpdate(ass2.clone(), eqn, var)?;
    metamodelica::arrayUpdate(ass1.clone(), var, eqn)?;
    Ok(())
}

fn matchComponent(mut eqLstIn: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut varLstIn: Arc<metamodelica::List<BackendDAE::Var>>, mut eqIdcs: Arc<metamodelica::List<i32>>, mut varIdcs: Arc<metamodelica::List<i32>>, mut sharedIn: Arc<BackendDAE::Shared>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>> {
    let mut compsOut: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let mut matching: Arc<BackendDAE::Matching>;
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    matching = buildSingleEquationSystem((eqLstIn.clone().len() as i32), eqLstIn, varLstIn, sharedIn, metamodelica::nil())?;
    let __pa0 = ::match_deref::match_deref! { match &(matching) {
        Deref @ BackendDAE::Matching::MATCHING { comps: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    comps = __pa0.clone();
    compsOut = List::map2(comps, (std::sync::Arc::new(replaceIndecesInComp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::StrongComponent>, metamodelica::Array<i32>, metamodelica::Array<i32>) -> Result<Arc<BackendDAE::StrongComponent>> + 'static>), metamodelica::arrayFromVec(eqIdcs.into_iter().cloned().collect()), metamodelica::arrayFromVec(varIdcs.into_iter().cloned().collect()))?;
    Ok(compsOut)
}

fn replaceIndecesInComp(mut comp: Arc<BackendDAE::StrongComponent>, mut eqMap: metamodelica::Array<i32>, mut varMap: metamodelica::Array<i32>) -> Result<Arc<BackendDAE::StrongComponent>> {
    let mut compOut: Arc<BackendDAE::StrongComponent>;
    compOut = (::match_deref::match_deref! { match &(comp) {
        Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { eqn, var } => {
            let mut eqn = (*eqn).clone();
            let mut var = (*var).clone();
            eqn = metamodelica::arrayGet(eqMap.clone(), eqn.clone())?;
            var = metamodelica::arrayGet(varMap.clone(), var.clone())?;
            Arc::new(BackendDAE::StrongComponent::SINGLEEQUATION { eqn: eqn.clone(), var: var.clone() })
        },
        _ => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(compOut)
}

fn reduceLinearTornSystem2(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut tVarIdcs0: Arc<metamodelica::List<i32>>, mut resEqIdcs0: Arc<metamodelica::List<i32>>, mut innerEquations: Arc<metamodelica::List<BackendDAE::InnerEquation>>, mut tornSysIdx: i32) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Matching>)> {
    let mut varsNewOut: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut eqsNewOut: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut tVarsOut: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut resEqsOut: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut matchingOut: Arc<BackendDAE::Matching>;
    let mut ass1New: metamodelica::Array<i32>;
    let mut ass2New: metamodelica::Array<i32>;
    let mut size: i32;
    let mut otherEqSize: i32;
    let mut compSize: i32;
    let mut otherEqnsInts: Arc<metamodelica::List<i32>>;
    let mut otherVarsInts: Arc<metamodelica::List<i32>>;
    let mut tVarRange: Arc<metamodelica::List<i32>>;
    let mut otherVarsIntsLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut oeqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut matchingNew: Arc<BackendDAE::Matching>;
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let mut compsNew: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let mut oComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let mut compsEqSys: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let mut vars: BackendDAE::Variables;
    let mut ovars: BackendDAE::Variables;
    let mut derRepl: BackendVarTransform::VariableReplacements;
    let mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut reqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut otherEqnsLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut otherEqnsLstReplaced: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut hs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut addEqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut tvars: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut tvarsReplaced: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut ovarsLst: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut a_0: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut addVarLst: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut hs_i_lst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>;
    let mut a_i_lst: Arc<metamodelica::List<Arc<metamodelica::List<BackendDAE::Var>>>>;
    let mut a_i_lst1: Arc<metamodelica::List<Arc<metamodelica::List<BackendDAE::Var>>>>;
    let mut g_iArr: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>;
    let mut hs_iArr: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>;
    let mut h_iArr: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>;
    let mut xa_iArr: metamodelica::Array<Arc<metamodelica::List<BackendDAE::Var>>>;
    let mut a_iArr: metamodelica::Array<Arc<metamodelica::List<BackendDAE::Var>>>;
    let mut replArr: metamodelica::Array<BackendVarTransform::VariableReplacements>;
    let mut tcrs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut ovcrs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(isyst) {
        Deref @ BackendDAE::EqSystem { orderedVars: __pa0, orderedEqs: __pa1, matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa2, .. }, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    vars = __pa0.clone();
    eqns = __pa1.clone();
    comps = __pa2.clone();
    eqLst = BackendEquation::equationList(eqns.clone())?;
    varLst = BackendVariable::varList(vars.clone())?;
    tvars = List::map1r(tVarIdcs0.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone())?;
    tvarsReplaced = List::map(tvars.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::transformXToXd, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<BackendDAE::Var> + 'static>))?;
    tcrs = List::map(tvarsReplaced.clone(), (std::sync::Arc::new(BackendVariable::varCref) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<Arc<DAE::ComponentRef>> + 'static>))?;
    derRepl = BackendVarTransform::emptyReplacements();
    derRepl = List::threadFold(tvars.clone(), tvarsReplaced.clone(), (std::sync::Arc::new(addDerReplacement) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendDAE::Var, BackendVarTransform::VariableReplacements) -> Result<BackendVarTransform::VariableReplacements> + 'static>), derRepl)?;
    reqns = BackendEquation::getList(resEqIdcs0.clone(), eqns.clone())?;
    reqns = BackendEquation::replaceDerOpInEquationList(reqns)?;
    (otherEqnsInts, otherVarsIntsLst, _) = List::map_3(innerEquations, (std::sync::Arc::new(BackendDAEUtil::getEqnAndVarsFromInnerEquation) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::InnerEquation) -> Result<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<DAE::Constraint>>>)> + 'static>))?;
    otherEqnsLst = BackendEquation::getList(otherEqnsInts, eqns)?;
    oeqns = BackendEquation::listEquation(otherEqnsLst.clone())?;
    otherEqnsLstReplaced = BackendEquation::replaceDerOpInEquationList(otherEqnsLst.clone())?;
    otherVarsInts = List::unionList(otherVarsIntsLst)?;
    ovarsLst = List::map1r(otherVarsInts, (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars)?;
    ovarsLst = List::map(ovarsLst, (std::sync::Arc::new(fnptr!(BackendVariable::transformXToXd, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<BackendDAE::Var> + 'static>))?;
    ovars = BackendVariable::listVar1(ovarsLst.clone())?;
    ovcrs = List::map(ovarsLst.clone(), (std::sync::Arc::new(BackendVariable::varCref) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<Arc<DAE::ComponentRef>> + 'static>))?;
    size = (tvars.len() as i32);
    otherEqSize = (otherEqnsLst.len() as i32);
    compSize = (comps.len() as i32);
    tVarRange = List::intRange2(0, size);
    replArr = arrayCreate(size + 1, BackendVarTransform::emptyReplacements());
    g_iArr = arrayCreate(size + 1, metamodelica::nil());
    h_iArr = arrayCreate(size + 1, metamodelica::nil());
    hs_iArr = arrayCreate(size + 1, metamodelica::nil());
    xa_iArr = arrayCreate(size + 1, metamodelica::nil());
    a_iArr = arrayCreate(size + 1, metamodelica::nil());
    (g_iArr, xa_iArr, replArr) = getAlgebraicEquationsForEI(tVarRange.clone(), size, otherEqnsLstReplaced, tvarsReplaced.clone(), tcrs, ovarsLst, ovcrs, g_iArr.clone(), xa_iArr.clone(), replArr.clone(), tornSysIdx)?;
    h_iArr = getResidualExpressions(tVarRange.clone(), reqns, replArr.clone(), h_iArr.clone())?;
    (hs_iArr, a_iArr) = getTornSystemCoefficients(tVarRange, size, tornSysIdx, h_iArr.clone(), hs_iArr.clone(), a_iArr.clone())?;
    a_i_lst = Arc::new(a_iArr.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
    hs_i_lst = Arc::new(hs_iArr.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
    eqsNewOut = List::flatten(listAppend(Arc::new(g_iArr.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), hs_i_lst))?;
    varsNewOut = List::flatten(listAppend(Arc::new(xa_iArr.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), a_i_lst.clone()))?;
    let (__pa4, __pa5) = ::match_deref::match_deref! { match &(a_i_lst) {
        Deref @ metamodelica::List::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
        _ => bail!("pattern mismatch"),
    } };
    a_0 = __pa4.clone();
    a_i_lst1 = __pa5.clone();
    hs = buildNewResidualEquation(1, a_i_lst1, a_0, tvarsReplaced.clone(), metamodelica::nil())?;
    tVarsOut = tvarsReplaced;
    resEqsOut = hs;
    (eqsNewOut, varsNewOut, resEqsOut) = simplifyNewEquations(eqsNewOut, varsNewOut, resEqsOut, ({
        let mut __acc: i32 = 0;
        for mut l in (xa_iArr.clone()).borrow().iter() {
            let __x = (l.clone().len() as i32);
            __acc += __x;
        }
        __acc
    }), 2, ishared.clone())?;
    (compsEqSys, resEqsOut, tVarsOut, addEqLst, addVarLst) = buildEqSystemComponent(resEqIdcs0, tVarIdcs0, resEqsOut, tVarsOut, a_iArr.clone(), ishared.clone())?;
    (resEqsOut, _) = BackendVarTransform::replaceEquations(resEqsOut, derRepl, None)?;
    eqsNewOut = listAppend(eqsNewOut, addEqLst);
    varsNewOut = listAppend(varsNewOut, addVarLst);
    matchingNew = buildSingleEquationSystem(compSize, eqsNewOut.clone(), varsNewOut.clone(), ishared, metamodelica::nil())?;
    let (__pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &(matchingNew) {
        Deref @ BackendDAE::Matching::MATCHING { ass1: __pa6, ass2: __pa7, comps: __pa8 } => (__pa6.clone(), __pa7.clone(), __pa8.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ass1New = __pa6.clone();
    ass2New = __pa7.clone();
    compsNew = __pa8.clone();
    compsNew = List::map2(compsNew, (std::sync::Arc::new(updateIndicesInComp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::StrongComponent>, i32, i32) -> Result<Arc<BackendDAE::StrongComponent>> + 'static>), (varLst.len() as i32), (eqLst.len() as i32))?;
    oComps = listAppend(compsNew, compsEqSys);
    matchingOut = Arc::new(BackendDAE::Matching::MATCHING { ass1: ass1New.clone(), ass2: ass2New.clone(), comps: oComps });
    Ok((varsNewOut, eqsNewOut, tVarsOut, resEqsOut, matchingOut))
}

fn addDerReplacement(mut var1: BackendDAE::Var, mut var2: BackendDAE::Var, mut replIn: BackendVarTransform::VariableReplacements) -> Result<BackendVarTransform::VariableReplacements> {
    let mut replOut: BackendVarTransform::VariableReplacements;
    replOut = (match var1.clone() {
        BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, .. } => {
            let mut dest: Arc<DAE::Exp>;
            let mut source: Arc<DAE::ComponentRef>;
            let mut repl: BackendVarTransform::VariableReplacements;
            source = BackendVariable::varCref(var2)?;
            dest = BackendVariable::varExp(var1)?;
            dest = IndexReduction::makeder(dest.clone())?;
            repl = BackendVarTransform::addReplacement(replIn, source.clone(), dest.clone(), None)?;
            repl.clone()
        },
        _ => {
            replIn
        },
    });
    Ok(replOut)
}

fn simplifyNewEquations(mut eqsIn: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut varsIn: Arc<metamodelica::List<BackendDAE::Var>>, mut resEqsIn: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut numAuxiliaryVars: i32, mut numIter: i32, mut shared: Arc<BackendDAE::Shared>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut eqsOut: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut varsOut: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut resEqsOut: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut eqArr: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut varArr: BackendDAE::Variables;
    let mut eqSys: Arc<BackendDAE::EqSystem>;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut size: i32;
    let mut numIterNew: i32;
    let mut numAux: i32;
    let mut varIdcs: Arc<metamodelica::List<i32>>;
    let mut eqIdcs: Arc<metamodelica::List<i32>>;
    eqArr = BackendEquation::listEquation(eqsIn.clone())?;
    varArr = BackendVariable::listVar1(varsIn)?;
    eqSys = BackendDAEUtil::createEqSystem(varArr.clone(), eqArr.clone(), metamodelica::nil(), openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
    (m, mT) = BackendDAEUtil::adjacencyMatrix(eqSys, openmodelica_backend_types::BackendDAE::IndexType::ABSOLUTE, None, BackendDAEUtil::isInitializationDAE(shared.clone()))?;
    size = (eqsIn.len() as i32);
    (eqIdcs, varIdcs, resEqsOut) = List::fold(List::intRange(size), (std::sync::Arc::new({ let __pe_b1 = eqArr.clone(); let __pe_b2 = varArr.clone(); let __pe_b3 = m.clone(); let __pe_b4 = mT.clone(); let __pe_b5 = numAuxiliaryVars; let __pe_b6 = shared.clone(); move |__pe_a0, __pe_a7| Ok(simplifyNewEquations1(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone(), __pe_a7)) }) as std::sync::Arc<dyn ::std::ops::Fn(i32, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> + 'static>), (metamodelica::nil(), metamodelica::nil(), resEqsIn))?;
    numAux = numAuxiliaryVars - (varIdcs.clone().len() as i32);
    if varIdcs.clone().is_empty() {
        numIterNew = 0;
    } else {
        numIterNew = numIter;
    }
    (_, varIdcs, _) = List::intersection1OnTrue(List::intRange(size), varIdcs, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    (_, eqIdcs, _) = List::intersection1OnTrue(List::intRange(size), eqIdcs, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    eqsOut = BackendEquation::getList(eqIdcs, eqArr)?;
    varsOut = List::map1(varIdcs, (std::sync::Arc::new(BackendVariable::getVarAtIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, BackendDAE::Variables) -> Result<BackendDAE::Var> + 'static>), varArr)?;
    if numIterNew != 0 {
        (eqsOut, varsOut, resEqsOut) = simplifyNewEquations(eqsOut, varsOut, resEqsOut, numAux, numIterNew - 1, shared)?;
    } else {
        (eqsOut, varsOut, resEqsOut) = (eqsOut, varsOut, resEqsOut);
    }
    Ok((eqsOut, varsOut, resEqsOut))
}

fn simplifyNewEquations1(mut eqIdx: i32, mut eqArr: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut varArr: BackendDAE::Variables, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut numAuxiliaryVars: i32, mut shared: Arc<BackendDAE::Shared>, mut tplIn: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)) -> (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) {
    let mut tplOut: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>);
    tplOut = 'mc: {
        let __mc_input = tplIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut varIdx: i32;
                    let mut varIdcs: Arc<metamodelica::List<i32>>;
                    let mut eqIdcs: Arc<metamodelica::List<i32>>;
                    let mut updEqIdcs: Arc<metamodelica::List<i32>>;
                    let mut eq: Arc<BackendDAE::Equation>;
                    let mut var: BackendDAE::Var;
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut varCref: Arc<DAE::ComponentRef>;
                    let mut varExp: Arc<DAE::Exp>;
                    let mut rhs: Arc<DAE::Exp>;
                    let mut lhs: Arc<DAE::Exp>;
                    let mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut resEqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    (eqIdcs, varIdcs, resEqLst) = tplIn.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(metamodelica::arrayGet(m.clone(), eqIdx)?) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    varIdx = __pa0.clone();
                    let true = (varIdx.clone() <= numAuxiliaryVars) else { bail!("pattern mismatch") };
                    var = BackendVariable::getVarAt(varArr.clone(), varIdx.clone())?;
                    eq = BackendEquation::get(eqArr.clone(), eqIdx)?;
                    varCref = BackendVariable::varCref(var.clone())?;
                    varExp = Expression::crefExp(varCref.clone())?;
                    rhs = BackendEquation::getEquationRHS(eq.clone())?;
                    lhs = BackendEquation::getEquationLHS(eq.clone())?;
                    (rhs, _) = ExpressionSolve::solve(lhs.clone(), rhs.clone(), varExp.clone(), None)?;
                    if Expression::isAsubExp(rhs.clone()) {
                        rhs = List::fold1(Expression::allTerms(rhs.clone()), (std::sync::Arc::new(fnptr!(Expression::makeBinaryExp, Arc<DAE::Exp>, DAE::Operator, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, DAE::Operator, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), DAE::Operator::ADD { ty: Expression::r#typeof(varExp.clone())? }, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }))?;
                    }
                    (rhs, _) = ExpressionSimplify::simplify(rhs.clone())?;
                    repl = BackendVarTransform::emptyReplacements();
                    repl = BackendVarTransform::addReplacement(repl.clone(), varCref.clone(), rhs.clone(), None)?;
                    updEqIdcs = metamodelica::arrayGet(mt.clone(), varIdx.clone())?;
                    eqLst = BackendEquation::getList(updEqIdcs.clone(), eqArr.clone())?;
                    (eqLst, _) = BackendVarTransform::replaceEquations(eqLst.clone(), repl.clone(), None)?;
                    (resEqLst, _) = BackendVarTransform::replaceEquations(resEqLst.clone(), repl.clone(), None)?;
                    List::threadFold(updEqIdcs.clone(), eqLst.clone(), (std::sync::Arc::new(BackendEquation::setAtIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<BackendDAE::Equation>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> + 'static>), eqArr.clone())?;
                    varIdcs = metamodelica::cons(varIdx.clone(), varIdcs.clone());
                    eqIdcs = metamodelica::cons(eqIdx, eqIdcs.clone());
                    Ok((eqIdcs.clone(), varIdcs.clone(), resEqLst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(tplIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    tplOut
}

fn buildEqSystemComponent(mut eqIdcsIn: Arc<metamodelica::List<i32>>, mut varIdcsIn: Arc<metamodelica::List<i32>>, mut resEqsIn: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut tVarsIn: Arc<metamodelica::List<BackendDAE::Var>>, mut jacValuesIn: metamodelica::Array<Arc<metamodelica::List<BackendDAE::Var>>>, mut shared: Arc<BackendDAE::Shared>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)> {
    let mut outComp: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let mut resEqsOut: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut tVarsOut: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut addEqsOut: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut addVarsOut: Arc<metamodelica::List<BackendDAE::Var>>;
    (outComp, resEqsOut, tVarsOut, addEqsOut, addVarsOut) = 'mc: {
        let __mc_input = (eqIdcsIn.clone(), varIdcsIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: eqIdx, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: varIdx, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut comp: Arc<BackendDAE::StrongComponent>;
                    let true = (intEq((eqIdcsIn.clone().len() as i32), 1)) else { bail!("pattern mismatch") };
                    comp = Arc::new(BackendDAE::StrongComponent::SINGLEEQUATION { eqn: eqIdx.clone(), var: varIdx.clone() });
                    Ok((list![comp.clone()], resEqsIn.clone(), tVarsIn.clone(), metamodelica::nil(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
                    let mut resEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut addEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut addVars: Arc<metamodelica::List<BackendDAE::Var>>;
                    let true = (intLe((tVarsIn.clone().len() as i32), 3)) else { bail!("pattern mismatch") };
                    (resEqs, _, addEqs, addVars) = applyCramerRule(jacValuesIn.clone(), tVarsIn.clone())?;
                    comps = List::threadMap(eqIdcsIn.clone(), varIdcsIn.clone(), (std::sync::Arc::new(fnptr!(BackendDAEUtil::makeSingleEquationComp, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<Arc<BackendDAE::StrongComponent>> + 'static>))?;
                    Ok((comps.clone(), resEqs.clone(), tVarsIn.clone(), addEqs.clone(), addVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut jac: Option<Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>>;
                    let mut comp: Arc<BackendDAE::StrongComponent>;
                    let mut jacValues: Arc<metamodelica::List<Arc<metamodelica::List<BackendDAE::Var>>>>;
                    let mut mixedSystem: bool;
                    let __pa0 = ::match_deref::match_deref! { match &(Arc::new(jacValuesIn.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())) {
                        Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    jacValues = __pa0.clone();
                    jac = buildLinearJacobian(jacValues.clone(), List::intRange((resEqsIn.clone().len() as i32)), List::intRange((tVarsIn.clone().len() as i32)))?;
                    mixedSystem = BackendVariable::hasDiscreteVar(tVarsIn.clone());
                    comp = Arc::new(BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: eqIdcsIn.clone(), vars: varIdcsIn.clone(), jac: Arc::new(BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: jac.clone() }), jacType: openmodelica_backend_types::BackendDAE::JacobianType::JAC_LINEAR, mixedSystem: mixedSystem.clone() });
                    Ok((list![comp.clone()], resEqsIn.clone(), tVarsIn.clone(), metamodelica::nil(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outComp, resEqsOut, tVarsOut, addEqsOut, addVarsOut))
}

fn buildLinearJacobian(mut inElements: Arc<metamodelica::List<Arc<metamodelica::List<BackendDAE::Var>>>>, mut eqIdcs: Arc<metamodelica::List<i32>>, mut varIdcs: Arc<metamodelica::List<i32>>) -> Result<Option<Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>>> {
    let mut outJac: Option<Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>>;
    let mut jac: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>;
    jac = List::fold2(eqIdcs, (std::sync::Arc::new(buildLinearJacobian1) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<metamodelica::List<BackendDAE::Var>>>>, Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>) -> Result<Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>> + 'static>), varIdcs, inElements, metamodelica::nil())?;
    jac = jac.reverse();
    outJac = Some(jac);
    Ok(outJac)
}

fn buildLinearJacobian1(mut rowIdx: i32, mut columns: Arc<metamodelica::List<i32>>, mut inElements: Arc<metamodelica::List<Arc<metamodelica::List<BackendDAE::Var>>>>, mut inJac: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>) -> Result<Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>> {
    let mut outJac: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>;
    let mut elements: Arc<metamodelica::List<BackendDAE::Var>>;
    elements = (inElements).get(rowIdx)?;
    elements = List::map1(columns.clone(), (std::sync::Arc::new(List::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), elements)?;
    outJac = List::fold2(columns, (std::sync::Arc::new(buildLinearJacobian2) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<BackendDAE::Var>>, i32, Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>) -> Result<Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>> + 'static>), elements, rowIdx, inJac)?;
    Ok(outJac)
}

fn buildLinearJacobian2(mut colIdx: i32, mut inElements: Arc<metamodelica::List<BackendDAE::Var>>, mut rowIdx: i32, mut inJac: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>) -> Result<Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>> {
    let mut outJac: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>;
    let mut cref: Arc<DAE::ComponentRef>;
    let mut exp: Arc<DAE::Exp>;
    let mut eq: Arc<BackendDAE::Equation>;
    let mut elem: BackendDAE::Var;
    let mut entry: (i32, i32, Arc<BackendDAE::Equation>);
    elem = (inElements).get(colIdx)?;
    cref = BackendVariable::varCref(elem)?;
    exp = Arc::new(DAE::Exp::CREF { componentRef: cref, ty: DAE::T_REAL_DEFAULT().clone() });
    exp = Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: DAE::T_REAL_DEFAULT().clone() }, exp: exp });
    eq = Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: exp, source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
    entry = (colIdx, rowIdx, eq);
    outJac = metamodelica::cons(entry, inJac);
    Ok(outJac)
}

fn updateMatching(mut idx: i32, mut offsetTpl: (i32, i32), mut matching2: (metamodelica::Array<i32>, metamodelica::Array<i32>), mut matching1In: (metamodelica::Array<i32>, metamodelica::Array<i32>)) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut matching1Out: (metamodelica::Array<i32>, metamodelica::Array<i32>);
    let mut eqOffset: i32;
    let mut varOffset: i32;
    let mut eqValue: i32;
    let mut varValue: i32;
    let mut ass11: metamodelica::Array<i32>;
    let mut ass21: metamodelica::Array<i32>;
    let mut ass12: metamodelica::Array<i32>;
    let mut ass22: metamodelica::Array<i32>;
    (eqOffset, varOffset) = offsetTpl;
    (ass12, ass22) = matching2;
    (ass11, ass21) = matching1In;
    eqValue = idx + eqOffset;
    varValue = metamodelica::arrayGet(ass22.clone(), idx)? + varOffset;
    ass11 = metamodelica::arrayUpdate(ass11.clone(), varValue, eqValue)?;
    ass21 = metamodelica::arrayUpdate(ass21.clone(), eqValue, varValue)?;
    matching1Out = (ass11.clone(), ass21.clone());
    Ok(matching1Out)
}

fn updateResidualMatching(mut idx: i32, mut tvars: Arc<metamodelica::List<i32>>, mut resEqs: Arc<metamodelica::List<i32>>, mut tplIn: (metamodelica::Array<i32>, metamodelica::Array<i32>)) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut tplOut: (metamodelica::Array<i32>, metamodelica::Array<i32>);
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut eqIdx: i32;
    let mut varIdx: i32;
    (ass1, ass2) = tplIn;
    eqIdx = (resEqs).get(idx)?;
    varIdx = (tvars).get(idx)?;
    ass1 = metamodelica::arrayUpdate(ass1.clone(), varIdx, eqIdx)?;
    ass2 = metamodelica::arrayUpdate(ass2.clone(), eqIdx, varIdx)?;
    tplOut = (ass1.clone(), ass2.clone());
    Ok(tplOut)
}

fn getOtherComps(mut innerEquations: Arc<metamodelica::List<BackendDAE::InnerEquation>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<Arc<BackendDAE::Matching>> {
    let mut matchingOut: Arc<BackendDAE::Matching>;
    let mut ass1Tmp: metamodelica::Array<i32>;
    let mut ass2Tmp: metamodelica::Array<i32>;
    let mut compsTmp: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    (ass1Tmp, ass2Tmp, compsTmp) = List::fold(innerEquations, (std::sync::Arc::new(getOtherComps1) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::InnerEquation, (metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>)) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>)> + 'static>), (ass1.clone(), ass2.clone(), metamodelica::nil()))?;
    compsTmp = compsTmp.reverse();
    matchingOut = Arc::new(BackendDAE::Matching::MATCHING { ass1: ass1Tmp.clone(), ass2: ass2Tmp.clone(), comps: compsTmp });
    Ok(matchingOut)
}

fn getOtherComps1(mut innerEquation: BackendDAE::InnerEquation, mut tplIn: (metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>)) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>)> {
    let mut tplOut: (metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>);
    tplOut = 'mc: {
        let __mc_input = tplIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ass1, ass2, compsIn) => {
                    let mut eqIdx: i32;
                    let mut varIdx: i32;
                    let mut varIdcs: Arc<metamodelica::List<i32>>;
                    let mut comp: Arc<BackendDAE::StrongComponent>;
                    let mut compsTmp: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
                    let mut ass1 = (*ass1).clone();
                    let mut ass2 = (*ass2).clone();
                    (eqIdx, varIdcs, _) = BackendDAEUtil::getEqnAndVarsFromInnerEquation(innerEquation.clone())?;
                    let true = ((varIdcs.clone().len() as i32) == 1) else { bail!("pattern mismatch") };
                    varIdx = (varIdcs.clone()).get(1)?;
                    comp = Arc::new(BackendDAE::StrongComponent::SINGLEEQUATION { eqn: eqIdx.clone(), var: varIdx.clone() });
                    ass1 = metamodelica::arrayUpdate(ass1.clone(), varIdx.clone(), eqIdx.clone())?;
                    ass2 = metamodelica::arrayUpdate(ass2.clone(), eqIdx.clone(), varIdx.clone())?;
                    compsTmp = metamodelica::cons(comp.clone(), compsIn.clone());
                    Ok((ass1.clone(), ass2.clone(), compsTmp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("getOtherComps failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(tplOut)
}

fn replaceAtPositionFromList<ElementType: Clone + 'static + metamodelica::gc::MMTrace>(mut n: i32, mut replacingLst: Arc<metamodelica::List<ElementType>>, mut positionLst: Arc<metamodelica::List<i32>>, mut inLst: Arc<metamodelica::List<ElementType>>) -> Result<Arc<metamodelica::List<ElementType>>> {
    let mut outLst: Arc<metamodelica::List<ElementType>>;
    let mut idx: i32;
    let mut entry: ElementType;
    idx = (positionLst).get(n)?;
    entry = (replacingLst).get(n)?;
    outLst = List::replaceAt(entry, idx, inLst)?;
    Ok(outLst)
}

fn updateIndicesInComp(mut compIn: Arc<BackendDAE::StrongComponent>, mut varOffset: i32, mut eqOffset: i32) -> Result<Arc<BackendDAE::StrongComponent>> {
    let mut compOut: Arc<BackendDAE::StrongComponent>;
    compOut = (::match_deref::match_deref! { match &(compIn) {
        Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { eqn: eqIdx, var: varIdx } => {
            let mut compTmp: Arc<BackendDAE::StrongComponent>;
            let mut eqIdx = (*eqIdx).clone();
            let mut varIdx = (*varIdx).clone();
            varIdx = varIdx.clone() + varOffset;
            eqIdx = eqIdx.clone() + eqOffset;
            compTmp = Arc::new(BackendDAE::StrongComponent::SINGLEEQUATION { eqn: eqIdx.clone(), var: varIdx.clone() });
            compTmp.clone()
        },
        _ => {
            metamodelica::print((literal!("updateVarEqIndices failed\n")).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(compOut)
}

fn buildNewResidualEquation(mut resIdx: i32, mut aCoeffLst: Arc<metamodelica::List<Arc<metamodelica::List<BackendDAE::Var>>>>, mut a0CoeffLst: Arc<metamodelica::List<BackendDAE::Var>>, mut tvars: Arc<metamodelica::List<BackendDAE::Var>>, mut resEqsIn: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut resEqsOut: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    resEqsOut = 'mc: {
        let __mc_input = resEqsIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut eqLstTmp: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let true = (resIdx > (tvars.clone().len() as i32)) else { bail!("pattern mismatch") };
                    eqLstTmp = resEqsIn.clone().reverse();
                    Ok(eqLstTmp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut eqLstTmp: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut aCoeffs: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut hs: Arc<BackendDAE::Equation>;
                    let mut a0Coeff: BackendDAE::Var;
                    let mut lhs: Arc<DAE::Exp>;
                    let mut rhs: Arc<DAE::Exp>;
                    let mut a0Exp: Arc<DAE::Exp>;
                    let mut ty: Arc<DAE::Type>;
                    let true = (resIdx <= (tvars.clone().len() as i32)) else { bail!("pattern mismatch") };
                    aCoeffs = List::map1(aCoeffLst.clone(), (std::sync::Arc::new(listGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), resIdx)?;
                    a0Coeff = (a0CoeffLst.clone()).get(resIdx)?;
                    a0Exp = varExp(a0Coeff.clone())?;
                    ty = DAE::T_REAL_DEFAULT().clone();
                    rhs = buildNewResidualEquation2(1, aCoeffs.clone(), tvars.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }))?;
                    rhs = Arc::new(DAE::Exp::BINARY { exp1: rhs.clone(), operator: DAE::Operator::ADD { ty: ty.clone() }, exp2: a0Exp.clone() });
                    lhs = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) });
                    hs = Arc::new(BackendDAE::Equation::EQUATION { exp: lhs.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                    eqLstTmp = metamodelica::cons(hs.clone(), resEqsIn.clone());
                    eqLstTmp = buildNewResidualEquation(resIdx + 1, aCoeffLst.clone(), a0CoeffLst.clone(), tvars.clone(), eqLstTmp.clone())?;
                    Ok(eqLstTmp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("buildNewResidualEquation failed")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(resEqsOut)
}

fn buildNewResidualEquation2(mut idx: i32, mut coeffs: Arc<metamodelica::List<BackendDAE::Var>>, mut tVars: Arc<metamodelica::List<BackendDAE::Var>>, mut expIn: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut expOut: Arc<DAE::Exp>;
    expOut = 'mc: {
        let __mc_input = expIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut coeff: BackendDAE::Var;
                    let mut tVar: BackendDAE::Var;
                    let mut coeffExp: Arc<DAE::Exp>;
                    let mut tVarExp: Arc<DAE::Exp>;
                    let mut expTmp: Arc<DAE::Exp>;
                    let mut ty: Arc<DAE::Type>;
                    let true = (idx == 1) else { bail!("pattern mismatch") };
                    coeff = (coeffs.clone()).get(idx)?;
                    coeffExp = varExp(coeff.clone())?;
                    tVar = (tVars.clone()).get(idx)?;
                    tVarExp = varExp(tVar.clone())?;
                    tVarExp = if (BackendVariable::isStateVar(tVar.clone())) {Expression::expDer(tVarExp.clone())} else {tVarExp.clone()};
                    ty = DAE::T_REAL_DEFAULT().clone();
                    expTmp = Arc::new(DAE::Exp::BINARY { exp1: coeffExp.clone(), operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: tVarExp.clone() });
                    expTmp = buildNewResidualEquation2(idx + 1, coeffs.clone(), tVars.clone(), expTmp.clone())?;
                    Ok(expTmp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut coeff: BackendDAE::Var;
                    let mut tVar: BackendDAE::Var;
                    let mut expTmp: Arc<DAE::Exp>;
                    let true = (idx <= (tVars.clone().len() as i32)) else { bail!("pattern mismatch") };
                    coeff = (coeffs.clone()).get(idx)?;
                    tVar = (tVars.clone()).get(idx)?;
                    expTmp = addProductToExp(coeff.clone(), tVar.clone(), expIn.clone())?;
                    expTmp = buildNewResidualEquation2(idx + 1, coeffs.clone(), tVars.clone(), expTmp.clone())?;
                    Ok(expTmp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (idx > (tVars.clone().len() as i32)) else { bail!("pattern mismatch") };
                    Ok(expIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("buildNewResidualEquation2 failed!\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(expOut)
}

fn addProductToExp(mut var1: BackendDAE::Var, mut var2: BackendDAE::Var, mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut expOut: Arc<DAE::Exp>;
    let mut fac1: Arc<DAE::Exp>;
    let mut fac2: Arc<DAE::Exp>;
    let mut prod: Arc<DAE::Exp>;
    let mut ty: Arc<DAE::Type>;
    fac1 = varExp(var1)?;
    fac2 = varExp(var2.clone())?;
    fac2 = if (BackendVariable::isStateVar(var2)) {Expression::expDer(fac2)} else {fac2};
    ty = DAE::T_REAL_DEFAULT().clone();
    prod = Arc::new(DAE::Exp::BINARY { exp1: fac1, operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: fac2 });
    expOut = Arc::new(DAE::Exp::BINARY { exp1: inExp, operator: DAE::Operator::ADD { ty: ty }, exp2: prod });
    Ok(expOut)
}

fn buildSingleEquationSystem(mut eqSizeOrig: i32, mut inEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inVars: Arc<metamodelica::List<BackendDAE::Var>>, mut shared: Arc<BackendDAE::Shared>, mut compsIn: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>) -> Result<Arc<BackendDAE::Matching>> {
    let mut matchingOut: Arc<BackendDAE::Matching>;
    matchingOut = 'mc: {
        let __mc_input = compsIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut ass1: metamodelica::Array<i32>;
                    let mut ass2: metamodelica::Array<i32>;
                    let mut mapIncRowEqn: metamodelica::Array<i32>;
                    let mut nVars: i32;
                    let mut nEqs: i32;
                    let mut eqArr: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut sysTmp: Arc<BackendDAE::EqSystem>;
                    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut matching: Arc<BackendDAE::Matching>;
                    let mut matchingTmp: Arc<BackendDAE::Matching>;
                    let mut compsTmp: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
                    let mut vars: BackendDAE::Variables;
                    vars = BackendVariable::listVar1(inVars.clone())?;
                    eqArr = BackendEquation::listEquation(inEqs.clone())?;
                    sysTmp = BackendDAEUtil::createEqSystem(vars.clone(), eqArr.clone(), metamodelica::nil(), openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
                    (sysTmp, m, _) = BackendDAEUtil::getAdjacencyMatrix(sysTmp.clone(), openmodelica_backend_types::BackendDAE::IndexType::NORMAL, None, BackendDAEUtil::isInitializationDAE(shared.clone()))?;
                    nVars = (inVars.clone().len() as i32);
                    nEqs = (inEqs.clone().len() as i32);
                    ass1 = arrayCreate(nVars.clone(), -1);
                    ass2 = arrayCreate(nEqs.clone(), -1);
                    Matching::matchingExternalsetAdjacencyMatrix(nVars.clone(), nEqs.clone(), m.clone());
                    BackendDAEEXT::matching(nVars.clone(), nEqs.clone(), 5, -1, metamodelica::OrderedFloat(0.0_f64), 1);
                    BackendDAEEXT::getAssignment(ass2.clone(), ass1.clone())?;
                    matching = Arc::new(BackendDAE::Matching::MATCHING { ass1: ass1.clone(), ass2: ass2.clone(), comps: metamodelica::nil() });
                    sysTmp = BackendDAEUtil::createEqSystem(vars.clone(), eqArr.clone(), metamodelica::nil(), openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
                    (sysTmp, _, _) = BackendDAEUtil::getAdjacencyMatrix(sysTmp.clone(), openmodelica_backend_types::BackendDAE::IndexType::ABSOLUTE, None, BackendDAEUtil::isInitializationDAE(shared.clone()))?;
                    sysTmp = BackendDAEUtil::setEqSystMatching(sysTmp.clone(), matching.clone())?;
                    mapIncRowEqn = Array::createIntRange(nEqs.clone());
                    mapEqnIncRow = Array::map(mapIncRowEqn.clone(), std::sync::Arc::new(fnptr!(List::create, _)))?;
                    (sysTmp, compsTmp) = BackendDAETransform::strongComponentsScalar(sysTmp.clone(), shared.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
                    compsTmp = listAppend(compsIn.clone(), compsTmp.clone());
                    matchingTmp = Arc::new(BackendDAE::Matching::MATCHING { ass1: ass1.clone(), ass2: ass2.clone(), comps: compsTmp.clone() });
                    Ok(matchingTmp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("buildSingleEquationSystem failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(matchingOut)
}

fn getTornSystemCoefficients(mut iValueRange: Arc<metamodelica::List<i32>>, mut numTVars: i32, mut tornSysIdx: i32, mut h_iArr: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>, mut hs_iArrIn: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, mut a_iArrIn: metamodelica::Array<Arc<metamodelica::List<BackendDAE::Var>>>) -> Result<(metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<BackendDAE::Var>>>)> {
    let mut hs_iArrOut: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>;
    let mut a_iArrOut: metamodelica::Array<Arc<metamodelica::List<BackendDAE::Var>>>;
    (hs_iArrOut, a_iArrOut) = 'mc: {
        let __mc_input = iValueRange;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((hs_iArrIn.clone(), a_iArrIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: iValue, tail: iLstRest } => {
                    let mut hs_iArrTmp: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>;
                    let mut a_iArrTmp: metamodelica::Array<Arc<metamodelica::List<BackendDAE::Var>>>;
                    (hs_iArrTmp, a_iArrTmp) = getTornSystemCoefficients1(List::intRange(numTVars).reverse(), iValue.clone(), h_iArr.clone(), hs_iArrIn.clone(), a_iArrIn.clone(), tornSysIdx)?;
                    (hs_iArrTmp, a_iArrTmp) = getTornSystemCoefficients(iLstRest.clone(), numTVars, tornSysIdx, h_iArr.clone(), hs_iArrTmp.clone(), a_iArrTmp.clone())?;
                    Ok((hs_iArrTmp.clone(), a_iArrTmp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("getTornSystemCoefficients failed!\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((hs_iArrOut, a_iArrOut))
}

fn getTornSystemCoefficients1(mut resIdxLst: Arc<metamodelica::List<i32>>, mut iIdx: i32, mut h_iArr: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>, mut hs_iArrIn: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, mut a_iArrIn: metamodelica::Array<Arc<metamodelica::List<BackendDAE::Var>>>, mut tornSysIdx: i32) -> Result<(metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<BackendDAE::Var>>>)> {
    let mut hs_iArrOut: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>;
    let mut a_iArrOut: metamodelica::Array<Arc<metamodelica::List<BackendDAE::Var>>>;
    (hs_iArrOut, a_iArrOut) = 'mc: {
        let __mc_input = resIdxLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((hs_iArrIn.clone(), a_iArrIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: resIdx, tail: resIdxRest } => {
                    let mut aName: ArcStr;
                    let mut hs_iArrTmp: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>;
                    let mut a_iArrTmp: metamodelica::Array<Arc<metamodelica::List<BackendDAE::Var>>>;
                    let mut hs_iTmp: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut a_iTmp: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut hs_ii: Arc<BackendDAE::Equation>;
                    let mut a_ii: BackendDAE::Var;
                    let mut aCRef: Arc<DAE::ComponentRef>;
                    let mut lhs: Arc<DAE::Exp>;
                    let mut rhs: Arc<DAE::Exp>;
                    let mut ty: Arc<DAE::Type>;
                    let true = (intEq(0, iIdx)) else { bail!("pattern mismatch") };
                    aName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$a")); __mm_s.push_str(&*intString(tornSysIdx)); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(resIdx.clone())); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(iIdx)); ArcStr::from(__mm_s) }).clone();
                    ty = DAE::T_REAL_DEFAULT().clone();
                    aCRef = ComponentReferenceBasics::makeCrefIdent((aName.clone()).clone(), ty.clone(), metamodelica::nil());
                    a_ii = BackendDAE::Var { varName: aCRef.clone(), varKind: openmodelica_backend_types::BackendDAE::VarKind::VARIABLE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: ty.clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
                    a_ii = BackendVariable::setVarStartValue(a_ii.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }))?;
                    lhs = varExp(a_ii.clone())?;
                    rhs = (metamodelica::arrayGet(h_iArr.clone(), iIdx + 1)?).get(resIdx.clone())?;
                    (rhs, _) = ExpressionSimplify::simplify(rhs.clone())?;
                    hs_ii = Arc::new(BackendDAE::Equation::EQUATION { exp: lhs.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                    hs_iTmp = metamodelica::arrayGet(hs_iArrIn.clone(), iIdx + 1)?;
                    hs_iTmp = metamodelica::cons(hs_ii.clone(), hs_iTmp.clone());
                    hs_iArrTmp = metamodelica::arrayUpdate(hs_iArrIn.clone(), iIdx + 1, hs_iTmp.clone())?;
                    a_iArrTmp = a_iArrIn.clone();
                    a_iTmp = metamodelica::arrayGet(a_iArrIn.clone(), iIdx + 1)?;
                    a_iTmp = metamodelica::cons(a_ii.clone(), a_iTmp.clone());
                    a_iArrTmp = metamodelica::arrayUpdate(a_iArrIn.clone(), iIdx + 1, a_iTmp.clone())?;
                    (hs_iArrTmp, a_iArrTmp) = getTornSystemCoefficients1(resIdxRest.clone(), iIdx, h_iArr.clone(), hs_iArrTmp.clone(), a_iArrTmp.clone(), tornSysIdx)?;
                    Ok((hs_iArrTmp.clone(), a_iArrTmp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: resIdx, tail: resIdxRest } => {
                    let mut aName: ArcStr;
                    let mut hs_iArrTmp: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>;
                    let mut a_iArrTmp: metamodelica::Array<Arc<metamodelica::List<BackendDAE::Var>>>;
                    let mut hs_iTmp: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut a_iTmp: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut d_lst: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut hs_ii: Arc<BackendDAE::Equation>;
                    let mut a_ii: BackendDAE::Var;
                    let mut dVar: BackendDAE::Var;
                    let mut aCRef: Arc<DAE::ComponentRef>;
                    let mut lhs: Arc<DAE::Exp>;
                    let mut rhs: Arc<DAE::Exp>;
                    let mut dExp: Arc<DAE::Exp>;
                    let mut ty: Arc<DAE::Type>;
                    let true = (iIdx > 0) else { bail!("pattern mismatch") };
                    aName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$a")); __mm_s.push_str(&*intString(tornSysIdx)); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(resIdx.clone())); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(iIdx)); ArcStr::from(__mm_s) }).clone();
                    ty = DAE::T_REAL_DEFAULT().clone();
                    aCRef = ComponentReferenceBasics::makeCrefIdent((aName.clone()).clone(), ty.clone(), metamodelica::nil());
                    a_ii = BackendDAE::Var { varName: aCRef.clone(), varKind: openmodelica_backend_types::BackendDAE::VarKind::VARIABLE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: ty.clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
                    a_ii = BackendVariable::setVarStartValue(a_ii.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }))?;
                    d_lst = metamodelica::arrayGet(a_iArrIn.clone(), 1)?;
                    dVar = (d_lst.clone()).get(resIdx.clone())?;
                    dExp = varExp(dVar.clone())?;
                    lhs = varExp(a_ii.clone())?;
                    rhs = (metamodelica::arrayGet(h_iArr.clone(), iIdx + 1)?).get(resIdx.clone())?;
                    rhs = Arc::new(DAE::Exp::BINARY { exp1: rhs.clone(), operator: DAE::Operator::SUB { ty: ty.clone() }, exp2: dExp.clone() });
                    (rhs, _) = ExpressionSimplify::simplify(rhs.clone())?;
                    hs_ii = Arc::new(BackendDAE::Equation::EQUATION { exp: lhs.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                    hs_iTmp = metamodelica::arrayGet(hs_iArrIn.clone(), iIdx + 1)?;
                    hs_iTmp = metamodelica::cons(hs_ii.clone(), hs_iTmp.clone());
                    hs_iArrTmp = metamodelica::arrayUpdate(hs_iArrIn.clone(), iIdx + 1, hs_iTmp.clone())?;
                    a_iArrTmp = a_iArrIn.clone();
                    a_iTmp = metamodelica::arrayGet(a_iArrIn.clone(), iIdx + 1)?;
                    a_iTmp = metamodelica::cons(a_ii.clone(), a_iTmp.clone());
                    a_iArrTmp = metamodelica::arrayUpdate(a_iArrIn.clone(), iIdx + 1, a_iTmp.clone())?;
                    (hs_iArrTmp, a_iArrTmp) = getTornSystemCoefficients1(resIdxRest.clone(), iIdx, h_iArr.clone(), hs_iArrTmp.clone(), a_iArrTmp.clone(), tornSysIdx)?;
                    Ok((hs_iArrTmp.clone(), a_iArrTmp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("getTornSystemCoefficients1 failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((hs_iArrOut, a_iArrOut))
}

fn varExp(mut varIn: BackendDAE::Var) -> Result<Arc<DAE::Exp>> {
    let mut expOut: Arc<DAE::Exp>;
    let mut cr: Arc<DAE::ComponentRef>;
    let mut ty: Arc<DAE::Type>;
    ty = BackendVariable::varType(varIn.clone())?;
    cr = BackendVariable::varCref(varIn)?;
    expOut = Arc::new(DAE::Exp::CREF { componentRef: cr, ty: ty });
    Ok(expOut)
}

fn getResidualExpressions(mut iIn: Arc<metamodelica::List<i32>>, mut resEqLstIn: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut replArrIn: metamodelica::Array<BackendVarTransform::VariableReplacements>, mut h_iArrIn: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>> {
    let mut h_iArrOut: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>;
    let mut resExps: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    resExps = List::map(resEqLstIn, (std::sync::Arc::new(getResidualExpressionForEquation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<Arc<DAE::Exp>> + 'static>))?;
    h_iArrOut = List::fold2(iIn, (std::sync::Arc::new(getResidualExpressions1) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<Arc<DAE::Exp>>>, metamodelica::Array<BackendVarTransform::VariableReplacements>, metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>> + 'static>), resExps, replArrIn.clone(), h_iArrIn.clone())?;
    Ok(h_iArrOut)
}

fn getResidualExpressions1(mut i: i32, mut resExpsIn: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut replArr: metamodelica::Array<BackendVarTransform::VariableReplacements>, mut h_iArrIn: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>> {
    let mut h_iArrOut: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>;
    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    let mut h_i: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut h_iArr: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>> = Default::default();
    h_iArrOut = 'mc: {
        let __mc_input = h_iArrIn.clone();
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut h_i: Arc<metamodelica::List<Arc<DAE::Exp>>> = h_i.clone();
            let mut h_iArr: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>> = h_iArr.clone();
            let mut repl: BackendVarTransform::VariableReplacements = repl.clone();
            repl = metamodelica::arrayGet(replArr.clone(), i + 1)?;
            (h_i, _) = BackendVarTransform::replaceExpList1(resExpsIn.clone(), repl.clone(), None);
            h_iArr = metamodelica::arrayUpdate(h_iArrIn.clone(), i + 1, h_i.clone())?;
            Ok((h_iArr.clone(), h_i.clone(), h_iArr.clone(), repl.clone()))
        })() { h_i = __wb0; h_iArr = __wb1; repl = __wb2; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            metamodelica::print((literal!("getResidualExpressions failed \n")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(h_iArrOut)
}

fn getResidualExpressionForEquation(mut eq: Arc<BackendDAE::Equation>) -> Result<Arc<DAE::Exp>> {
    let mut exp: Arc<DAE::Exp>;
    exp = (::match_deref::match_deref! { match &(eq) {
        Deref @ BackendDAE::Equation::EQUATION { exp: lhs, scalar: rhs, .. } => {
            let mut ty: Arc<DAE::Type>;
            let mut rhs = (*rhs).clone();
            ty = Expression::r#typeof(lhs.clone())?;
            rhs = Arc::new(DAE::Exp::BINARY { exp1: rhs.clone(), operator: DAE::Operator::SUB { ty: ty.clone() }, exp2: lhs.clone() });
            (rhs, _) = ExpressionSimplify::simplify(rhs.clone())?;
            rhs.clone()
        },
        _ => {
            metamodelica::print((literal!("getResidualExpressionForEquation failed\n")).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn varInFrontList(mut varIn: BackendDAE::Var, mut lstLstIn: Arc<metamodelica::List<Arc<metamodelica::List<BackendDAE::Var>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<BackendDAE::Var>>>>> {
    let mut lstLstOut: Arc<metamodelica::List<Arc<metamodelica::List<BackendDAE::Var>>>> = metamodelica::nil();
    lstLstOut = (::match_deref::match_deref! { match &(lstLstIn.clone()) {
        Deref @ metamodelica::List::Nil => {
            lstLstIn
        },
        _ => {
            let mut varLst: Arc<metamodelica::List<BackendDAE::Var>>;
            varLst = listHead(lstLstIn.clone())?;
            varLst = metamodelica::cons(varIn, varLst.clone());
            lstLstOut = List::replaceAt(varLst.clone(), 1, lstLstIn)?;
            lstLstOut
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(lstLstOut)
}

fn eqInFrontList(mut eqIn: Arc<BackendDAE::Equation>, mut lstLstIn: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>> {
    let mut lstLstOut: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>> = metamodelica::nil();
    lstLstOut = (::match_deref::match_deref! { match &(lstLstIn.clone()) {
        Deref @ metamodelica::List::Nil => {
            lstLstIn
        },
        _ => {
            let mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            eqLst = listHead(lstLstIn.clone())?;
            eqLst = metamodelica::cons(eqIn, eqLst.clone());
            lstLstOut = List::replaceAt(eqLst.clone(), 1, lstLstIn)?;
            lstLstOut
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(lstLstOut)
}

fn getAlgebraicEquationsForEI(mut iIn: Arc<metamodelica::List<i32>>, mut size: i32, mut otherEqLstIn: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut tvarLstIn: Arc<metamodelica::List<BackendDAE::Var>>, mut tVarCRefLstIn: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut otherVarLstIn: Arc<metamodelica::List<BackendDAE::Var>>, mut oVarCRefLstIn: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut g_iArrIn: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, mut xa_iArrIn: metamodelica::Array<Arc<metamodelica::List<BackendDAE::Var>>>, mut replacementArrIn: metamodelica::Array<BackendVarTransform::VariableReplacements>, mut tornSysIdx: i32) -> Result<(metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<BackendDAE::Var>>>, metamodelica::Array<BackendVarTransform::VariableReplacements>)> {
    let mut g_i_Out: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>;
    let mut xa_i_Out: metamodelica::Array<Arc<metamodelica::List<BackendDAE::Var>>>;
    let mut replacementArrOut: metamodelica::Array<BackendVarTransform::VariableReplacements>;
    (g_i_Out, xa_i_Out, replacementArrOut) = 'mc: {
        let __mc_input = iIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((g_iArrIn.clone(), xa_iArrIn.clone(), replacementArrIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: iValue, tail: iLstRest } => {
                    let mut gEqLstTmp: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut xaVarLstTmp: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut replArrTmp: metamodelica::Array<BackendVarTransform::VariableReplacements>;
                    let mut g_iArrTmp: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>;
                    let mut xa_iArrTmp: metamodelica::Array<Arc<metamodelica::List<BackendDAE::Var>>>;
                    let mut replTmp: BackendVarTransform::VariableReplacements;
                    let true = (iValue.clone() == 0) else { bail!("pattern mismatch") };
                    replTmp = BackendVarTransform::emptyReplacementsSized(size);
                    replTmp = List::fold1(tVarCRefLstIn.clone(), (std::sync::Arc::new(replaceTVarWithReal) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, metamodelica::Real, BackendVarTransform::VariableReplacements) -> Result<BackendVarTransform::VariableReplacements> + 'static>), metamodelica::OrderedFloat(0.0_f64), replTmp.clone())?;
                    (xaVarLstTmp, replTmp) = List::fold2(List::intRange((oVarCRefLstIn.clone().len() as i32)), (std::sync::Arc::new(replaceOtherVarsWithPrefixCref) as std::sync::Arc<dyn ::std::ops::Fn(i32, ArcStr, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, (Arc<metamodelica::List<BackendDAE::Var>>, BackendVarTransform::VariableReplacements)) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, BackendVarTransform::VariableReplacements)> + 'static>), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$xa")); __mm_s.push_str(&*intString(tornSysIdx)); __mm_s.push_str(&*literal!("0")); ArcStr::from(__mm_s) }).clone(), oVarCRefLstIn.clone(), (metamodelica::nil(), replTmp.clone()))?;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVarTransform::replaceEquations(otherEqLstIn.clone(), replTmp.clone(), None)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    gEqLstTmp = __pa0.clone();
                    g_iArrTmp = metamodelica::arrayUpdate(g_iArrIn.clone(), iValue.clone() + 1, gEqLstTmp.clone())?;
                    xa_iArrTmp = metamodelica::arrayUpdate(xa_iArrIn.clone(), iValue.clone() + 1, xaVarLstTmp.clone())?;
                    replArrTmp = metamodelica::arrayUpdate(replacementArrIn.clone(), iValue.clone() + 1, replTmp.clone())?;
                    (g_iArrTmp, xa_iArrTmp, replArrTmp) = getAlgebraicEquationsForEI(iLstRest.clone(), size, otherEqLstIn.clone(), tvarLstIn.clone(), tVarCRefLstIn.clone(), otherVarLstIn.clone(), oVarCRefLstIn.clone(), g_iArrTmp.clone(), xa_iArrTmp.clone(), replArrTmp.clone(), tornSysIdx)?;
                    Ok((g_iArrTmp.clone(), xa_iArrTmp.clone(), replArrTmp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: iValue, tail: iLstRest } => {
                    let mut str1: ArcStr;
                    let mut gEqLstTmp: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut xaVarLstTmp: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut replArrTmp: metamodelica::Array<BackendVarTransform::VariableReplacements>;
                    let mut tVarCRefLst1: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    let mut g_iArrTmp: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>;
                    let mut xa_iArrTmp: metamodelica::Array<Arc<metamodelica::List<BackendDAE::Var>>>;
                    let mut replTmp: BackendVarTransform::VariableReplacements;
                    let mut tVarCRef: Arc<DAE::ComponentRef>;
                    let true = (iValue.clone() > 0) else { bail!("pattern mismatch") };
                    str1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$xa")); __mm_s.push_str(&*intString(tornSysIdx)); __mm_s.push_str(&*intString(iValue.clone())); ArcStr::from(__mm_s) }).clone();
                    tVarCRef = (tVarCRefLstIn.clone()).get(iValue.clone())?;
                    tVarCRefLst1 = listDelete(tVarCRefLstIn.clone(), iValue.clone())?;
                    replTmp = BackendVarTransform::emptyReplacementsSized(size);
                    replTmp = replaceTVarWithReal(tVarCRef.clone(), metamodelica::OrderedFloat(1.0_f64), replTmp.clone())?;
                    replTmp = List::fold1(tVarCRefLst1.clone(), (std::sync::Arc::new(replaceTVarWithReal) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, metamodelica::Real, BackendVarTransform::VariableReplacements) -> Result<BackendVarTransform::VariableReplacements> + 'static>), metamodelica::OrderedFloat(0.0_f64), replTmp.clone())?;
                    (xaVarLstTmp, replTmp) = List::fold2(List::intRange((oVarCRefLstIn.clone().len() as i32)), (std::sync::Arc::new(replaceOtherVarsWithPrefixCref) as std::sync::Arc<dyn ::std::ops::Fn(i32, ArcStr, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, (Arc<metamodelica::List<BackendDAE::Var>>, BackendVarTransform::VariableReplacements)) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, BackendVarTransform::VariableReplacements)> + 'static>), (str1.clone()).clone(), oVarCRefLstIn.clone(), (metamodelica::nil(), replTmp.clone()))?;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVarTransform::replaceEquations(otherEqLstIn.clone(), replTmp.clone(), None)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    gEqLstTmp = __pa0.clone();
                    g_iArrTmp = metamodelica::arrayUpdate(g_iArrIn.clone(), iValue.clone() + 1, gEqLstTmp.clone())?;
                    xa_iArrTmp = metamodelica::arrayUpdate(xa_iArrIn.clone(), iValue.clone() + 1, xaVarLstTmp.clone())?;
                    replArrTmp = metamodelica::arrayUpdate(replacementArrIn.clone(), iValue.clone() + 1, replTmp.clone())?;
                    (g_iArrTmp, xa_iArrTmp, replArrTmp) = getAlgebraicEquationsForEI(iLstRest.clone(), size, otherEqLstIn.clone(), tvarLstIn.clone(), tVarCRefLstIn.clone(), otherVarLstIn.clone(), oVarCRefLstIn.clone(), g_iArrTmp.clone(), xa_iArrTmp.clone(), replArrTmp.clone(), tornSysIdx)?;
                    Ok((g_iArrTmp.clone(), xa_iArrTmp.clone(), replArrTmp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("getAlgebraicEquationsForEI failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((g_i_Out, xa_i_Out, replacementArrOut))
}

fn replaceTVarWithReal(mut tVarCRefIn: Arc<DAE::ComponentRef>, mut realIn: metamodelica::Real, mut replacementIn: BackendVarTransform::VariableReplacements) -> Result<BackendVarTransform::VariableReplacements> {
    let mut replacementOut: BackendVarTransform::VariableReplacements;
    replacementOut = BackendVarTransform::addReplacement(replacementIn, tVarCRefIn, Arc::new(DAE::Exp::RCONST { real: realIn }), None)?;
    Ok(replacementOut)
}

fn replaceOtherVarsWithPrefixCref(mut indxIn: i32, mut prefix: ArcStr, mut oVarCRefLstIn: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut tplIn: (Arc<metamodelica::List<BackendDAE::Var>>, BackendVarTransform::VariableReplacements)) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, BackendVarTransform::VariableReplacements)> {
    let mut tplOut: (Arc<metamodelica::List<BackendDAE::Var>>, BackendVarTransform::VariableReplacements);
    let mut replVarLstIn: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut replVarLstOut: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut replVar: BackendDAE::Var;
    let mut replacementIn: BackendVarTransform::VariableReplacements;
    let mut replacementOut: BackendVarTransform::VariableReplacements;
    let mut cRef: Arc<DAE::ComponentRef>;
    let mut oVarCRef: Arc<DAE::ComponentRef>;
    let mut varExp: Arc<DAE::Exp>;
    let mut ty: Arc<DAE::Type>;
    (replVarLstIn, replacementIn) = tplIn;
    oVarCRef = (oVarCRefLstIn).get(indxIn)?;
    cRef = ComponentReferenceBasics::makeCrefQual((prefix).clone(), DAE::T_COMPLEX_DEFAULT().clone(), metamodelica::nil(), oVarCRef.clone());
    cRef = ComponentReference::replaceSubsWithString(cRef)?;
    cRef = ComponentReference::crefSetLastType(cRef, DAE::T_REAL_DEFAULT().clone())?;
    varExp = Expression::crefExp(cRef.clone())?;
    replacementOut = BackendVarTransform::addReplacement(replacementIn, oVarCRef, varExp, None)?;
    ty = ComponentReference::crefLastType(cRef.clone())?;
    replVar = BackendDAE::Var { varName: cRef, varKind: openmodelica_backend_types::BackendDAE::VarKind::VARIABLE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: ty, bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
    replVar = BackendVariable::setVarStartValue(replVar, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }))?;
    replVarLstOut = metamodelica::cons(replVar, replVarLstIn);
    tplOut = (replVarLstOut, replacementOut);
    Ok(tplOut)
}

//--------------------------------------------------//
// get EqSystem object
//-------------------------------------------------//
fn getEqSystem(mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut varLst: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<EqSys> {
    let mut syst: EqSys;
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    syst = createEqSystem(varLst.clone());
    crefs = List::map(varLst, (std::sync::Arc::new(BackendVariable::varCref) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<Arc<DAE::ComponentRef>> + 'static>))?;
    (syst, _) = List::fold1(eqLst, (std::sync::Arc::new(getEqSystem2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, (EqSys, i32)) -> Result<(EqSys, i32)> + 'static>), crefs, (syst, 1))?;
    Ok(syst)
}

fn createEqSystem(mut varLst: Arc<metamodelica::List<BackendDAE::Var>>) -> EqSys {
    let mut sys: EqSys;
    let mut dim: i32;
    let mut matrixA: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>;
    let mut vectorB: metamodelica::Array<Arc<DAE::Exp>>;
    dim = (varLst.clone().len() as i32);
    matrixA = arrayCreate(dim, metamodelica::nil());
    vectorB = arrayCreate(dim, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }));
    sys = EqSys { dim: dim, matrixA: matrixA.clone(), vectorB: vectorB.clone(), vectorX: metamodelica::arrayFromVec(varLst.into_iter().cloned().collect()) };
    sys
}

fn getEqSystem2(mut eq: Arc<BackendDAE::Equation>, mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut foldIn: (EqSys, i32)) -> Result<(EqSys, i32)> {
    let mut foldOut: (EqSys, i32);
    let mut idx: i32;
    let mut dim: i32;
    let mut summands: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut coeffs: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut offsetLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut offset: Arc<DAE::Exp>;
    let mut sys: EqSys;
    let mut matrixA: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>;
    let mut vectorB: metamodelica::Array<Arc<DAE::Exp>>;
    let mut vectorX: metamodelica::Array<BackendDAE::Var>;
    (sys, idx) = foldIn;
    summands = getSummands(eq)?;
    (summands, _) = List::map_2(summands, (std::sync::Arc::new(ExpressionSimplify::simplify) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<(Arc<DAE::Exp>, bool)> + 'static>))?;
    (offsetLst, coeffs) = List::fold(crefs, (std::sync::Arc::new(getEqSystem3) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<DAE::Exp>>>)) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<DAE::Exp>>>)> + 'static>), (summands, metamodelica::nil()))?;
    if offsetLst.clone().is_empty() {
        offset = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) });
    } else {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(offsetLst) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        offset = __pa0.clone();
        offsetLst = __pa1.clone();
    }
    offset = List::fold(offsetLst, (std::sync::Arc::new(Expression::expAdd) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), offset)?;
    offset = Expression::negate(offset)?;
    let EqSys { dim: __pa2, matrixA: __pa3, vectorB: __pa4, vectorX: __pa5 } = (sys) else { bail!("pattern mismatch") };
    dim = __pa2.clone();
    matrixA = __pa3.clone();
    vectorB = __pa4.clone();
    vectorX = __pa5.clone();
    matrixA = metamodelica::arrayUpdate(matrixA.clone(), idx, coeffs.reverse())?;
    vectorB = metamodelica::arrayUpdate(vectorB.clone(), idx, offset)?;
    sys = EqSys { dim: dim, matrixA: matrixA.clone(), vectorB: vectorB.clone(), vectorX: vectorX.clone() };
    foldOut = (sys, idx + 1);
    Ok(foldOut)
}

fn getEqSystem3(mut cref: Arc<DAE::ComponentRef>, mut foldIn: (Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<DAE::Exp>>>)) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<DAE::Exp>>>)> {
    let mut foldOut: (Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<DAE::Exp>>>);
    let mut coeff: Arc<DAE::Exp>;
    let mut allTerms: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut coeffs: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut coeffsIn: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    (allTerms, coeffsIn) = foldIn;
    (coeffs, allTerms) = List::extract1OnTrue(allTerms, (std::sync::Arc::new(Expression::expHasCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), cref.clone())?;
    coeff = List::fold(coeffs, (std::sync::Arc::new(Expression::expAdd) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((0) as f64) }))?;
    if containsFunctioncallOfCref(coeff.clone(), cref.clone())? {
        metamodelica::print((literal!("This system of equations cannot be decomposed because its actually not linear (the coeffs are function calls of x).\n")).clone());
        bail!("fail");
    }
    (coeff, _) = Expression::replaceExp(coeff, Expression::crefExp(cref)?, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }))?;
    (coeff, _) = ExpressionSimplify::simplify(coeff)?;
    foldOut = (allTerms, metamodelica::cons(coeff, coeffsIn));
    Ok(foldOut)
}

fn containsFunctioncallOfCref(mut expIn: Arc<DAE::Exp>, mut cref: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut hasCrefInCall: bool;
    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    if Expression::containFunctioncall(expIn.clone())? {
        (_, expLst) = Expression::traverseExpBottomUp(expIn, (std::sync::Arc::new(fnptr!(getCallExpLst, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Exp>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Exp>>>)> + 'static>), metamodelica::nil())?;
        hasCrefInCall = List::fold(List::map1(expLst, (std::sync::Arc::new(Expression::expHasCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), cref)?, (std::sync::Arc::new(fnptr!(boolOr, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>), false)?;
    } else {
        hasCrefInCall = false;
    }
    Ok(hasCrefInCall)
}

fn getCallExpLst(mut eIn: Arc<DAE::Exp>, mut eLstIn: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> (Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Exp>>>) {
    let mut eOut: Arc<DAE::Exp>;
    let mut eLstOut: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    (eOut, eLstOut) = (::match_deref::match_deref! { match &(eIn.clone()) {
        Deref @ DAE::Exp::CALL { expLst, .. } => {
            (eIn, listAppend(expLst.clone(), eLstIn))
        },
        _ => {
            (eIn, eLstIn)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (eOut, eLstOut)
}

fn getSummands(mut eq: Arc<BackendDAE::Equation>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    exps = 'mc: {
        let __mc_input = eq.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::EQUATION { exp: lhs, scalar: rhs, .. } => {
                    let mut expLst1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut expLst2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    expLst1 = Expression::allTerms(lhs.clone());
                    expLst1 = List::map(expLst1.clone(), (std::sync::Arc::new(Expression::negate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?;
                    expLst2 = Expression::allTerms(rhs.clone());
                    expLst2 = listAppend(expLst1.clone(), expLst2.clone());
                    Ok(expLst2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getSummands failed! for")); __mm_s.push_str(&*BackendDump::equationString(eq.clone())?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(exps)
}

//--------------------------------------------------//
// Chios Condensation
//-------------------------------------------------//
fn chiosCondensation(mut systemIn: EqSys) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)> {
    let mut newResEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut addEqsOut: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut addVarsOut: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut dim: i32;
    let mut vectorB: metamodelica::Array<Arc<DAE::Exp>>;
    let mut vectorX: metamodelica::Array<BackendDAE::Var>;
    let mut matrixA: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>;
    let EqSys { dim: __pa0, matrixA: __pa1, vectorB: __pa2, vectorX: __pa3 } = (systemIn.clone()) else { bail!("pattern mismatch") };
    dim = __pa0.clone();
    matrixA = __pa1.clone();
    vectorB = __pa2.clone();
    vectorX = __pa3.clone();
    (addEqsOut, addVarsOut) = ChiosCondensation2(systemIn, 1, metamodelica::nil(), metamodelica::nil())?;
    addEqsOut = addEqsOut.reverse();
    addVarsOut = addVarsOut.reverse();
    newResEqs = generateCramerEqs(List::intRange(dim).reverse(), dim, vectorX.clone(), vectorB.clone(), matrixA.clone(), metamodelica::nil())?;
    newResEqs = newResEqs.reverse();
    Ok((newResEqs, addEqsOut, addVarsOut))
}

fn ChiosCondensation2(mut systemIn: EqSys, mut iterIdx: i32, mut addEqsIn: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut addVarsIn: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)> {
    let mut addEqsOut: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut addVarsOut: Arc<metamodelica::List<BackendDAE::Var>>;
    (addEqsOut, addVarsOut) = 'mc: {
        let __mc_input = systemIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let EqSys { dim: mut dim, vectorX: mut vectorX, .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut syst: EqSys;
            let mut matrixB: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>;
            let mut vecAi: metamodelica::Array<Arc<DAE::Exp>>;
            let mut addEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut addVars: Arc<metamodelica::List<BackendDAE::Var>>;
            let true = (intGt(dim.clone(), 1)) else { bail!("pattern mismatch") };
            matrixB = arrayCreate(dim.clone() - 1, metamodelica::nil());
            vecAi = arrayCreate(dim.clone() - 1, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }));
            (matrixB, vecAi, addEqs, addVars) = List::fold(List::intRange2(2, dim.clone()), (std::sync::Arc::new({ let __pe_b1 = systemIn.clone(); let __pe_b2 = iterIdx; move |__pe_a0, __pe_a3| getNewChioRow(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }) as std::sync::Arc<dyn ::std::ops::Fn(i32, (metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>, metamodelica::Array<Arc<DAE::Exp>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)) -> Result<(metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>, metamodelica::Array<Arc<DAE::Exp>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)> + 'static>), (matrixB.clone(), vecAi.clone(), addEqsIn.clone(), addVarsIn.clone()))?;
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("matrixB")); __mm_s.push_str(&*intString(dim.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            dumpMatrix(matrixB.clone())?;
            metamodelica::print((literal!("vecAi\n")).clone());
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (vecAi.clone()).borrow().iter() {
            let __x = ExpressionDump::dumpExpStr(e.clone(), 0)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            BackendDump::dumpEquationList(addEqs.clone(), (literal!("new det eqs")).clone())?;
            syst = EqSys { dim: dim.clone() - 1, matrixA: matrixB.clone(), vectorB: vecAi.clone(), vectorX: vectorX.clone() };
            Ok(ChiosCondensation2(syst.clone(), iterIdx + 1, addEqs.clone(), addVars.clone())?)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let EqSys { dim: mut dim, matrixA: mut matrixA, vectorB: mut vecAi, .. } = __mc_input.clone() else { bail!("nomatch") };
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("end matrixB")); __mm_s.push_str(&*intString(dim.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            dumpMatrix(matrixA.clone())?;
            metamodelica::print((literal!("end vecAi\n")).clone());
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (vecAi.clone()).borrow().iter() {
            let __x = ExpressionDump::dumpExpStr(e.clone(), 0)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            BackendDump::dumpEquationList(addEqsIn.clone(), (literal!("new det eqs")).clone())?;
            Ok((addEqsIn.clone(), addVarsIn.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((addEqsOut, addVarsOut))
}

fn generateCramerEqs(mut varIdcs: Arc<metamodelica::List<i32>>, mut dim: i32, mut vectorX: metamodelica::Array<BackendDAE::Var>, mut vectorB: metamodelica::Array<Arc<DAE::Exp>>, mut matrixA: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>, mut eqsIn: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut eqsOut: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    eqsOut = 'mc: {
        let __mc_input = varIdcs;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(eqsIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: varIdx, tail: rest } => {
                    let mut rangeAi: Arc<metamodelica::List<i32>>;
                    let mut rangeX: Arc<metamodelica::List<i32>>;
                    let mut detAexp: Arc<DAE::Exp>;
                    let mut detAiexp: Arc<DAE::Exp>;
                    let mut xExp: Arc<DAE::Exp>;
                    let mut rhs: Arc<DAE::Exp>;
                    let mut detAiExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut xLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut ty: Arc<DAE::Type>;
                    let mut xEq: Arc<BackendDAE::Equation>;
                    let mut xVar: BackendDAE::Var;
                    let true = (intNe(varIdx.clone(), 1)) else { bail!("pattern mismatch") };
                    xVar = metamodelica::arrayGet(vectorX.clone(), varIdx.clone())?;
                    xExp = BackendVariable::varExp(xVar.clone())?;
                    ty = Expression::r#typeof(xExp.clone())?;
                    detAexp = makeDetExp(varIdx.clone() - 1, (literal!("a")).clone(), 1, 1, ty.clone())?;
                    if intNe(varIdx.clone(), dim) {
                        rangeAi = List::intRange2(2, 1 + dim - varIdx.clone());
                        rangeX = List::intRange2(varIdx.clone() + 1, dim);
                    } else {
                        rangeAi = metamodelica::nil();
                        rangeX = metamodelica::nil();
                    }
                    detAiexp = makeDetExp(varIdx.clone() - 1, (literal!("b")).clone(), 1, dim - varIdx.clone() + 1, ty.clone())?;
                    detAiExpLst = List::map(rangeAi.clone(), (std::sync::Arc::new({ let __pe_b0 = varIdx.clone() - 1; let __pe_b1 = (literal!("a")).clone(); let __pe_b2 = 1; let __pe_b4 = ty.clone(); move |__pe_a3| makeDetExp(__pe_b0.clone(), __pe_b1.clone(), __pe_b2.clone(), __pe_a3, __pe_b4.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<Arc<DAE::Exp>> + 'static>))?;
                    xLst = List::map(List::map1(rangeX.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), vectorX.clone())?, (std::sync::Arc::new(BackendVariable::varExp) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<Arc<DAE::Exp>> + 'static>))?;
                    detAiExpLst = List::threadMap(xLst.clone(), detAiExpLst.clone(), (std::sync::Arc::new({ let __pe_b1 = DAE::Operator::MUL { ty: ty.clone() }; move |__pe_a0, __pe_a2| Ok(Expression::makeBinaryExp(__pe_a0, __pe_b1.clone(), __pe_a2)) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?;
                    detAiexp = List::foldr(detAiExpLst.clone(), (std::sync::Arc::new({ let __pe_b1 = DAE::Operator::SUB { ty: ty.clone() }; move |__pe_a0, __pe_a2| Ok(Expression::makeBinaryExp(__pe_a0, __pe_b1.clone(), __pe_a2)) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), detAiexp.clone())?;
                    (detAiexp, _) = ExpressionSimplify::simplify(detAiexp.clone())?;
                    rhs = Arc::new(DAE::Exp::BINARY { exp1: detAiexp.clone(), operator: DAE::Operator::DIV { ty: ty.clone() }, exp2: detAexp.clone() });
                    xEq = Arc::new(BackendDAE::Equation::EQUATION { exp: xExp.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                    BackendDump::dumpEquationList(list![xEq.clone()], (literal!("the new equation to solve x")).clone())?;
                    Ok(generateCramerEqs(rest.clone(), dim, vectorX.clone(), vectorB.clone(), matrixA.clone(), metamodelica::cons(xEq.clone(), eqsIn.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: 1, tail: rest } => {
                    let mut varIdx: i32;
                    let mut rangeX: Arc<metamodelica::List<i32>>;
                    let mut detAexp: Arc<DAE::Exp>;
                    let mut detAiexp: Arc<DAE::Exp>;
                    let mut xExp: Arc<DAE::Exp>;
                    let mut rhs: Arc<DAE::Exp>;
                    let mut detAiExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut xLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut ty: Arc<DAE::Type>;
                    let mut xEq: Arc<BackendDAE::Equation>;
                    let mut xVar: BackendDAE::Var;
                    varIdx = 1;
                    xVar = metamodelica::arrayGet(vectorX.clone(), varIdx.clone())?;
                    xExp = BackendVariable::varExp(xVar.clone())?;
                    ty = Expression::r#typeof(xExp.clone())?;
                    detAexp = (metamodelica::arrayGet(matrixA.clone(), 1)?).get(1)?;
                    rangeX = List::intRange2(2, dim);
                    detAiexp = metamodelica::arrayGet(vectorB.clone(), 1)?;
                    detAiExpLst = List::map1(rangeX.clone(), (std::sync::Arc::new(List::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), metamodelica::arrayGet(matrixA.clone(), 1)?)?;
                    xLst = List::map(List::map1(rangeX.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), vectorX.clone())?, (std::sync::Arc::new(BackendVariable::varExp) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<Arc<DAE::Exp>> + 'static>))?;
                    detAiExpLst = List::threadMap(xLst.clone(), detAiExpLst.clone(), (std::sync::Arc::new({ let __pe_b1 = DAE::Operator::MUL { ty: ty.clone() }; move |__pe_a0, __pe_a2| Ok(Expression::makeBinaryExp(__pe_a0, __pe_b1.clone(), __pe_a2)) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?;
                    detAiexp = List::foldr(detAiExpLst.clone(), (std::sync::Arc::new({ let __pe_b1 = DAE::Operator::SUB { ty: ty.clone() }; move |__pe_a0, __pe_a2| Ok(Expression::makeBinaryExp(__pe_a0, __pe_b1.clone(), __pe_a2)) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), detAiexp.clone())?;
                    (detAiexp, _) = ExpressionSimplify::simplify(detAiexp.clone())?;
                    rhs = Arc::new(DAE::Exp::BINARY { exp1: detAiexp.clone(), operator: DAE::Operator::DIV { ty: ty.clone() }, exp2: detAexp.clone() });
                    xEq = Arc::new(BackendDAE::Equation::EQUATION { exp: xExp.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                    BackendDump::dumpEquationList(list![xEq.clone()], (literal!("the new equation to solve x")).clone())?;
                    Ok(generateCramerEqs(rest.clone(), dim, vectorX.clone(), vectorB.clone(), matrixA.clone(), metamodelica::cons(xEq.clone(), eqsIn.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(eqsOut)
}

fn makeDetExp(mut iterIdx: i32, mut ident: ArcStr, mut row: i32, mut col: i32, mut ty: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut detExp: Arc<DAE::Exp>;
    let mut cr: Arc<DAE::ComponentRef>;
    let mut name: ArcStr;
    name = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$det_")); __mm_s.push_str(&*ident); __mm_s.push_str(&*intString(iterIdx)); __mm_s.push_str(&*literal!("__")); __mm_s.push_str(&*intString(row)); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(col)); ArcStr::from(__mm_s) }).clone();
    cr = ComponentReferenceBasics::makeCrefIdent((name).clone(), ty.clone(), metamodelica::nil());
    detExp = Expression::makeCrefExp(cr, ty)?;
    Ok(detExp)
}

fn makeVarOfIdent(mut ident: ArcStr, mut ty: Arc<DAE::Type>) -> BackendDAE::Var {
    let mut var: BackendDAE::Var;
    let mut cr: Arc<DAE::ComponentRef>;
    cr = ComponentReferenceBasics::makeCrefIdent((ident).clone(), ty.clone(), metamodelica::nil());
    var = BackendDAE::Var { varName: cr, varKind: openmodelica_backend_types::BackendDAE::VarKind::VARIABLE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: ty, bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
    var
}

fn getNewChioRow(mut row: i32, mut systemIn: EqSys, mut iterIdx: i32, mut foldIn: (metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>, metamodelica::Array<Arc<DAE::Exp>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)) -> Result<(metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>, metamodelica::Array<Arc<DAE::Exp>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)> {
    let mut foldOut: (metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>, metamodelica::Array<Arc<DAE::Exp>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>);
    let mut dim: i32;
    let mut columns: Arc<metamodelica::List<i32>>;
    let EqSys { dim: __pa0, .. } = (systemIn.clone()) else { bail!("pattern mismatch") };
    dim = __pa0.clone();
    columns = List::intRange2(2, dim).reverse();
    foldOut = List::fold(columns, (std::sync::Arc::new({ let __pe_b1 = row; let __pe_b2 = systemIn; let __pe_b3 = iterIdx; move |__pe_a0, __pe_a4| getNewChioEntry(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_a4) }) as std::sync::Arc<dyn ::std::ops::Fn(i32, (metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>, metamodelica::Array<Arc<DAE::Exp>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)) -> Result<(metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>, metamodelica::Array<Arc<DAE::Exp>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)> + 'static>), foldIn)?;
    Ok(foldOut)
}

fn getNewChioEntry(mut col: i32, mut row: i32, mut syst: EqSys, mut iter: i32, mut foldIn: (metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>, metamodelica::Array<Arc<DAE::Exp>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)) -> Result<(metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>, metamodelica::Array<Arc<DAE::Exp>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)> {
    let mut foldOut: (metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>, metamodelica::Array<Arc<DAE::Exp>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>);
    let mut dim: i32;
    let mut a11: Arc<DAE::Exp>;
    let mut ar1: Arc<DAE::Exp>;
    let mut a1c: Arc<DAE::Exp>;
    let mut arc: Arc<DAE::Exp>;
    let mut br: Arc<DAE::Exp>;
    let mut b1: Arc<DAE::Exp>;
    let mut detExp: Arc<DAE::Exp>;
    let mut detVarExp: Arc<DAE::Exp>;
    let mut ty: Arc<DAE::Type>;
    let mut detCR: Arc<DAE::ComponentRef>;
    let mut detAeq: Arc<BackendDAE::Equation>;
    let mut detAieq: Arc<BackendDAE::Equation>;
    let mut detAVar: BackendDAE::Var;
    let mut detAiVar: BackendDAE::Var;
    let mut detVarName: ArcStr;
    let mut matrixA: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>;
    let mut matrixB: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>;
    let mut vectorB: metamodelica::Array<Arc<DAE::Exp>>;
    let mut vecAi: metamodelica::Array<Arc<DAE::Exp>>;
    let mut vectorX: metamodelica::Array<BackendDAE::Var>;
    let mut addEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut addVars: Arc<metamodelica::List<BackendDAE::Var>>;
    let EqSys { dim: __pa0, matrixA: __pa1, vectorB: __pa2, vectorX: __pa3 } = (syst) else { bail!("pattern mismatch") };
    dim = __pa0.clone();
    matrixA = __pa1.clone();
    vectorB = __pa2.clone();
    vectorX = __pa3.clone();
    (matrixB, vecAi, addEqs, addVars) = foldIn;
    a11 = (metamodelica::arrayGet(matrixA.clone(), 1)?).get(1)?;
    ar1 = (metamodelica::arrayGet(matrixA.clone(), row)?).get(1)?;
    a1c = (metamodelica::arrayGet(matrixA.clone(), 1)?).get(col)?;
    arc = (metamodelica::arrayGet(matrixA.clone(), row)?).get(col)?;
    ty = Expression::r#typeof(a11.clone())?;
    detExp = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: a11.clone(), operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: arc }), operator: DAE::Operator::SUB { ty: ty.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: ar1.clone(), operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: a1c }) });
    (detExp, _) = ExpressionSimplify::simplify(detExp)?;
    detVarName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$det_a")); __mm_s.push_str(&*intString(iter)); __mm_s.push_str(&*literal!("__")); __mm_s.push_str(&*intString(row - 1)); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(col - 1)); ArcStr::from(__mm_s) }).clone();
    detCR = ComponentReferenceBasics::makeCrefIdent((detVarName).clone(), ty.clone(), metamodelica::nil());
    detAVar = BackendDAE::Var { varName: detCR.clone(), varKind: openmodelica_backend_types::BackendDAE::VarKind::VARIABLE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: ty.clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
    detVarExp = Expression::crefExp(detCR)?;
    detAeq = Arc::new(BackendDAE::Equation::EQUATION { exp: detVarExp.clone(), scalar: detExp, source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
    matrixB = Array::consToElement(row - 1, detVarExp, matrixB.clone())?;
    addEqs = metamodelica::cons(detAeq, addEqs);
    addVars = metamodelica::cons(detAVar, addVars);
    if col == dim {
        b1 = metamodelica::arrayGet(vectorB.clone(), 1)?;
        br = metamodelica::arrayGet(vectorB.clone(), row)?;
        detExp = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: a11, operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: br }), operator: DAE::Operator::SUB { ty: ty.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: ar1, operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: b1 }) });
        (detExp, _) = ExpressionSimplify::simplify(detExp)?;
        detVarName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$det_b")); __mm_s.push_str(&*intString(iter)); __mm_s.push_str(&*literal!("__")); __mm_s.push_str(&*intString(row - 1)); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(col - 1)); ArcStr::from(__mm_s) }).clone();
        detCR = ComponentReferenceBasics::makeCrefIdent((detVarName).clone(), ty.clone(), metamodelica::nil());
        detAiVar = BackendDAE::Var { varName: detCR.clone(), varKind: openmodelica_backend_types::BackendDAE::VarKind::VARIABLE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: ty, bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
        detVarExp = Expression::crefExp(detCR)?;
        detAieq = Arc::new(BackendDAE::Equation::EQUATION { exp: detVarExp.clone(), scalar: detExp, source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
        metamodelica::arrayUpdate(vecAi.clone(), row - 1, detVarExp)?;
        addEqs = metamodelica::cons(detAieq, addEqs);
        addVars = metamodelica::cons(detAiVar, addVars);
    }
    foldOut = (matrixB.clone(), vecAi.clone(), addEqs, addVars);
    Ok(foldOut)
}

//--------------------------------------------------//
// Cramers Rule
//-------------------------------------------------//
fn applyCramerRule(mut jacValuesIn: metamodelica::Array<Arc<metamodelica::List<BackendDAE::Var>>>, mut varsIn: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)> {
    let mut resEqsOut: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut tvarsOut: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut addEqsOut: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut addVarsOut: Arc<metamodelica::List<BackendDAE::Var>>;
    (resEqsOut, tvarsOut, addEqsOut, addVarsOut) = (::match_deref::match_deref! { match &(varsIn.clone()) {
        _ => {
            let mut syst: EqSys;
            let mut addEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut resEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut addVars: Arc<metamodelica::List<BackendDAE::Var>>;
            syst = getMatrixFromJac(jacValuesIn.clone(), varsIn.clone())?;
            (resEqs, addEqs, addVars) = CramerRule(syst.clone());
            (resEqs.clone(), varsIn, addEqs.clone(), addVars.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((resEqsOut, tvarsOut, addEqsOut, addVarsOut))
}

fn CramerRule(mut system: EqSys) -> (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>) {
    let mut newResEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut otherEqsOut: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut otherVarsOut: Arc<metamodelica::List<BackendDAE::Var>>;
    (newResEqs, otherEqsOut, otherVarsOut) = 'mc: {
        let __mc_input = system.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let EqSys { dim: mut dim, matrixA: mut matrixA, vectorX: mut vectorX, .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut matrixAT: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>;
            let mut detA: Arc<DAE::Exp>;
            let mut detLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut varExp: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let true = (intEq(dim.clone(), 2)) else { bail!("pattern mismatch") };
            matrixAT = transposeMatrix(matrixA.clone())?;
            detA = determinant(matrixA.clone())?;
            detLst = List::map2(List::intRange(dim.clone()), (std::sync::Arc::new(CramerRule1) as std::sync::Arc<dyn ::std::ops::Fn(i32, EqSys, metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>) -> Result<Arc<DAE::Exp>> + 'static>), system.clone(), matrixAT.clone())?;
            varExp = List::mapArray(vectorX.clone(), (std::sync::Arc::new(BackendVariable::varExp) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<Arc<DAE::Exp>> + 'static>))?;
            detLst = List::map1(detLst.clone(), (std::sync::Arc::new({ let __pe_b1 = DAE::Operator::DIV { ty: DAE::T_ANYTYPE_DEFAULT().clone() }; move |__pe_a0, __pe_a2| Ok(Expression::makeBinaryExp(__pe_a0, __pe_b1.clone(), __pe_a2)) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), detA.clone())?;
            (detLst, _) = List::map_2(detLst.clone(), (std::sync::Arc::new(ExpressionSimplify::simplify) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<(Arc<DAE::Exp>, bool)> + 'static>))?;
            eqLst = List::threadMap2(varExp.clone(), detLst.clone(), (std::sync::Arc::new(BackendEquation::generateEquation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>, Arc<DAE::ElementSource>, BackendDAE::EquationAttributes) -> Result<Arc<BackendDAE::Equation>> + 'static>), DAE::emptyElementSource().clone(), BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone())?;
            Ok((eqLst.clone(), metamodelica::nil(), metamodelica::nil()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let EqSys { dim: mut dim, matrixA: mut matrixA, vectorX: mut vectorX, .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut matrixAT: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>;
            let mut detA: Arc<DAE::Exp>;
            let mut detLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut varExp: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let true = (intEq(dim.clone(), 3)) else { bail!("pattern mismatch") };
            matrixAT = transposeMatrix(matrixA.clone())?;
            detA = determinant(matrixA.clone())?;
            detLst = List::map2(List::intRange(dim.clone()), (std::sync::Arc::new(CramerRule1) as std::sync::Arc<dyn ::std::ops::Fn(i32, EqSys, metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>) -> Result<Arc<DAE::Exp>> + 'static>), system.clone(), matrixAT.clone())?;
            varExp = List::mapArray(vectorX.clone(), (std::sync::Arc::new(BackendVariable::varExp) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<Arc<DAE::Exp>> + 'static>))?;
            detLst = List::map1(detLst.clone(), (std::sync::Arc::new({ let __pe_b1 = DAE::Operator::DIV { ty: DAE::T_ANYTYPE_DEFAULT().clone() }; move |__pe_a0, __pe_a2| Ok(Expression::makeBinaryExp(__pe_a0, __pe_b1.clone(), __pe_a2)) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), detA.clone())?;
            (detLst, _) = List::map_2(detLst.clone(), (std::sync::Arc::new(ExpressionSimplify::simplify) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<(Arc<DAE::Exp>, bool)> + 'static>))?;
            eqLst = List::threadMap2(varExp.clone(), detLst.clone(), (std::sync::Arc::new(BackendEquation::generateEquation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>, Arc<DAE::ElementSource>, BackendDAE::EquationAttributes) -> Result<Arc<BackendDAE::Equation>> + 'static>), DAE::emptyElementSource().clone(), BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone())?;
            Ok((eqLst.clone(), metamodelica::nil(), metamodelica::nil()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let EqSys { dim: mut dim, .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut addEqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut addVarLst: Arc<metamodelica::List<BackendDAE::Var>>;
            let true = (intGt(dim.clone(), 3)) else { bail!("pattern mismatch") };
            (eqLst, addEqLst, addVarLst) = chiosCondensation(system.clone())?;
            Ok((eqLst.clone(), addEqLst.clone(), addVarLst.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok((metamodelica::nil(), metamodelica::nil(), metamodelica::nil()))
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (newResEqs, otherEqsOut, otherVarsOut)
}

fn CramerRule1(mut idx: i32, mut syst: EqSys, mut matrixAT: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>) -> Result<Arc<DAE::Exp>> {
    let mut det: Arc<DAE::Exp>;
    det = (match syst {
        EqSys { vectorB: mut vectorB, .. } => {
            let mut matrixA: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>;
            matrixA = metamodelica::arrayFromVec(matrixAT.clone().borrow().clone());
            matrixA = replaceColumnInMatrix(matrixA.clone(), idx, Arc::new(vectorB.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()))?;
            determinant(matrixA.clone())?
        },
    });
    Ok(det)
}

fn determinant(mut matrix: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>) -> Result<Arc<DAE::Exp>> {
    let mut detOut: Arc<DAE::Exp>;
    detOut = 'mc: {
        let __mc_input = matrix.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut a11: Arc<DAE::Exp>;
            let mut a12: Arc<DAE::Exp>;
            let mut a21: Arc<DAE::Exp>;
            let mut a22: Arc<DAE::Exp>;
            let mut det: Arc<DAE::Exp>;
            let mut ty: Arc<DAE::Type>;
            let true = (metamodelica::arrayLength(matrix.clone()) == 2) else { bail!("pattern mismatch") };
            a11 = (metamodelica::arrayGet(matrix.clone(), 1)?).get(1)?;
            a12 = (metamodelica::arrayGet(matrix.clone(), 1)?).get(2)?;
            a21 = (metamodelica::arrayGet(matrix.clone(), 2)?).get(1)?;
            a22 = (metamodelica::arrayGet(matrix.clone(), 2)?).get(2)?;
            ty = Expression::r#typeof(a11.clone())?;
            det = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: a11.clone(), operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: a22.clone() }), operator: DAE::Operator::SUB { ty: ty.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: a12.clone(), operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: a21.clone() }) });
            (det, _) = ExpressionSimplify::simplify(det.clone())?;
            Ok(det.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut a11: Arc<DAE::Exp>;
            let mut a12: Arc<DAE::Exp>;
            let mut a21: Arc<DAE::Exp>;
            let mut a22: Arc<DAE::Exp>;
            let mut a13: Arc<DAE::Exp>;
            let mut a23: Arc<DAE::Exp>;
            let mut a33: Arc<DAE::Exp>;
            let mut a31: Arc<DAE::Exp>;
            let mut a32: Arc<DAE::Exp>;
            let mut s1: Arc<DAE::Exp>;
            let mut s2: Arc<DAE::Exp>;
            let mut s3: Arc<DAE::Exp>;
            let mut s4: Arc<DAE::Exp>;
            let mut s5: Arc<DAE::Exp>;
            let mut s6: Arc<DAE::Exp>;
            let mut det: Arc<DAE::Exp>;
            let mut ty: Arc<DAE::Type>;
            let true = (metamodelica::arrayLength(matrix.clone()) == 3) else { bail!("pattern mismatch") };
            a11 = (metamodelica::arrayGet(matrix.clone(), 1)?).get(1)?;
            a12 = (metamodelica::arrayGet(matrix.clone(), 1)?).get(2)?;
            a13 = (metamodelica::arrayGet(matrix.clone(), 1)?).get(3)?;
            a21 = (metamodelica::arrayGet(matrix.clone(), 2)?).get(1)?;
            a22 = (metamodelica::arrayGet(matrix.clone(), 2)?).get(2)?;
            a23 = (metamodelica::arrayGet(matrix.clone(), 2)?).get(3)?;
            a31 = (metamodelica::arrayGet(matrix.clone(), 3)?).get(1)?;
            a32 = (metamodelica::arrayGet(matrix.clone(), 3)?).get(2)?;
            a33 = (metamodelica::arrayGet(matrix.clone(), 3)?).get(3)?;
            ty = Expression::r#typeof(a11.clone())?;
            s1 = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: a11.clone(), operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: a22.clone() }), operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: a33.clone() });
            s2 = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: a12.clone(), operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: a23.clone() }), operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: a31.clone() });
            s3 = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: a13.clone(), operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: a21.clone() }), operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: a32.clone() });
            s4 = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: a13.clone(), operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: a22.clone() }), operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: a31.clone() });
            s5 = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: a23.clone(), operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: a32.clone() }), operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: a11.clone() });
            s6 = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: a33.clone(), operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: a12.clone() }), operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: a21.clone() });
            det = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: s1.clone(), operator: DAE::Operator::ADD { ty: ty.clone() }, exp2: s2.clone() }), operator: DAE::Operator::ADD { ty: ty.clone() }, exp2: s3.clone() }), operator: DAE::Operator::SUB { ty: ty.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: s4.clone(), operator: DAE::Operator::ADD { ty: ty.clone() }, exp2: s5.clone() }), operator: DAE::Operator::ADD { ty: ty.clone() }, exp2: s6.clone() }) });
            (det, _) = ExpressionSimplify::simplify(det.clone())?;
            Ok(det.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            metamodelica::print((literal!("computation fo determinant failed!\n")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(detOut)
}

fn replaceColumnInMatrix(mut matrixT: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>, mut col: i32, mut vectorB: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>> {
    let mut matrixOut: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>;
    let mut matrix: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>;
    matrix = metamodelica::arrayUpdate(matrixT.clone(), col, vectorB)?;
    matrixOut = transposeMatrix(matrix.clone())?;
    Ok(matrixOut)
}

fn getMatrixFromJac(mut jacValuesIn: metamodelica::Array<Arc<metamodelica::List<BackendDAE::Var>>>, mut vars: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<EqSys> {
    let mut matrixOut: EqSys;
    let mut AVars: Arc<metamodelica::List<Arc<metamodelica::List<BackendDAE::Var>>>>;
    let mut bVars: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut matrixA: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>;
    let mut vectorB: metamodelica::Array<Arc<DAE::Exp>>;
    let mut vectorX: metamodelica::Array<BackendDAE::Var>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Arc::new(jacValuesIn.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    bVars = __pa0.clone();
    AVars = __pa1.clone();
    matrixA = metamodelica::arrayFromVec(List::mapList(AVars, (std::sync::Arc::new(BackendVariable::varExp) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<Arc<DAE::Exp>> + 'static>))?.into_iter().cloned().collect());
    matrixA = transposeMatrix(matrixA.clone())?;
    vectorB = metamodelica::arrayFromVec(List::mapMap(bVars.clone(), (std::sync::Arc::new(BackendVariable::varExp) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<Arc<DAE::Exp>> + 'static>), (std::sync::Arc::new(Expression::negate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?.into_iter().cloned().collect());
    vectorX = metamodelica::arrayFromVec(vars.into_iter().cloned().collect());
    matrixOut = EqSys { dim: (bVars.len() as i32), matrixA: matrixA.clone(), vectorB: vectorB.clone(), vectorX: vectorX.clone() };
    Ok(matrixOut)
}

fn transposeMatrix(mut matrixIn: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>> {
    let mut matrixOut: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>;
    let mut size: i32;
    size = metamodelica::arrayLength(matrixIn.clone());
    matrixOut = arrayCreate(size, metamodelica::nil());
    matrixOut = List::fold1(List::intRange(size).reverse(), (std::sync::Arc::new(transposeMatrix1) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>, metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>> + 'static>), matrixIn.clone(), matrixOut.clone())?;
    Ok(matrixOut)
}

fn transposeMatrix1(mut idx: i32, mut matrixOrig: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>, mut matrixIn: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>> {
    let mut matrixOut: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>;
    let mut row: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    row = metamodelica::arrayGet(matrixOrig.clone(), idx)?;
    matrixOut = List::threadFold(List::intRange(metamodelica::arrayLength(matrixOrig.clone())), row, (std::sync::Arc::new(Array::consToElement) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<_> + 'static>), matrixIn.clone())?;
    Ok(matrixOut)
}

//--------------------------------------------------//
// Printing stuff
//-------------------------------------------------//
fn dumpEqSys(mut matrix: EqSys) -> Result<()> {
    let mut dim: i32;
    let mut sLst: Arc<metamodelica::List<ArcStr>>;
    let mut matrixA: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>;
    let mut vectorB: metamodelica::Array<Arc<DAE::Exp>>;
    let mut vectorX: metamodelica::Array<BackendDAE::Var>;
    let EqSys { dim: __pa0, matrixA: __pa1, vectorB: __pa2, vectorX: __pa3 } = (matrix) else { bail!("pattern mismatch") };
    dim = __pa0.clone();
    matrixA = __pa1.clone();
    vectorB = __pa2.clone();
    vectorX = __pa3.clone();
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Matrix(")); __mm_s.push_str(&*intString(dim)); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
    sLst = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        let __thr_src0 = matrixA.clone();
        let __thr_borrow0 = __thr_src0.borrow();
        let mut __thr_it0 = __thr_borrow0.iter().cloned();
        let __thr_src1 = vectorX.clone();
        let __thr_borrow1 = __thr_src1.borrow();
        let mut __thr_it1 = __thr_borrow1.iter().cloned();
        let __thr_src2 = vectorB.clone();
        let __thr_borrow2 = __thr_src2.borrow();
        let mut __thr_it2 = __thr_borrow2.iter().cloned();
        loop {
            match (__thr_it0.next(), __thr_it1.next(), __thr_it2.next()) {
                (Some(Arow), Some(x), Some(b)) => {
                    let __x = EqSysRowString(Arow.clone(), x.clone(), b.clone())?;
                    __acc = cons(__x, __acc);
                }
                (None, None, None) => break,
                _ => bail!("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(sLst, (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

fn EqSysRowString(mut Arow: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut x: BackendDAE::Var, mut b: Arc<DAE::Exp>) -> Result<ArcStr> {
    let mut s: ArcStr;
    let mut s1: ArcStr;
    let mut s2: ArcStr;
    let mut s3: ArcStr;
    s1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{ ")); __mm_s.push_str(&*stringDelimitList(List::map(Arow, (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>))?, (literal!("  \t  ")).clone())); __mm_s.push_str(&*literal!("} ")); ArcStr::from(__mm_s) }).clone();
    s2 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{ ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(BackendVariable::varCref(x)?)?); __mm_s.push_str(&*literal!(" } ")); ArcStr::from(__mm_s) }).clone();
    s3 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" = { ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(b)?); __mm_s.push_str(&*literal!(" }")); ArcStr::from(__mm_s) }).clone();
    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s1); __mm_s.push_str(&*literal!(" * ")); __mm_s.push_str(&*s2); __mm_s.push_str(&*s3); ArcStr::from(__mm_s) }).clone();
    Ok(s)
}

fn dumpMatrix(mut matrix: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>) -> Result<()> {
    let mut sLst: Arc<metamodelica::List<ArcStr>>;
    let mut s: ArcStr;
    sLst = List::mapArray(matrix.clone(), (std::sync::Arc::new(ExpressionDump::printExpListStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<ArcStr> + 'static>))?;
    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{ ")); __mm_s.push_str(&*stringDelimitList(sLst, (literal!("  \n  ")).clone())); __mm_s.push_str(&*literal!("} \n")); ArcStr::from(__mm_s) }).clone();
    metamodelica::print((s).clone());
    Ok(())
}

fn dumpVarArrLst(mut inArrLst: metamodelica::Array<Arc<metamodelica::List<BackendDAE::Var>>>, mut heading: ArcStr) -> Result<()> {
    let mut r#str: ArcStr;
    let mut inLstLst: Arc<metamodelica::List<Arc<metamodelica::List<BackendDAE::Var>>>>;
    inLstLst = Arc::new(inArrLst.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("---------\n")); __mm_s.push_str(&*heading.clone()); __mm_s.push_str(&*literal!("-variables\n---------\n")); ArcStr::from(__mm_s) }).clone());
    r#str = (List::fold1(List::intRange(metamodelica::arrayLength(inArrLst.clone())), (std::sync::Arc::new(dumpVarArrLst1) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<Arc<metamodelica::List<BackendDAE::Var>>>>, ArcStr) -> Result<ArcStr> + 'static>), inLstLst, (heading).clone())?).clone();
    Ok(())
}

fn dumpVarArrLst1(mut lstIdx: i32, mut inLstLst: Arc<metamodelica::List<Arc<metamodelica::List<BackendDAE::Var>>>>, mut heading: ArcStr) -> Result<ArcStr> {
    let mut headingOut: ArcStr;
    let mut str1: ArcStr;
    let mut inLst: Arc<metamodelica::List<BackendDAE::Var>>;
    inLst = (inLstLst).get(lstIdx)?;
    str1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*heading.clone()); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(lstIdx - 1)); ArcStr::from(__mm_s) }).clone();
    BackendDump::dumpVarList(inLst, (str1).clone())?;
    headingOut = (heading).clone();
    Ok(headingOut)
}

fn dumpEqArrLst(mut inArrLst: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, mut heading: ArcStr) -> Result<()> {
    let mut r#str: ArcStr;
    let mut inLstLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>;
    inLstLst = Arc::new(inArrLst.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("---------\n")); __mm_s.push_str(&*heading.clone()); __mm_s.push_str(&*literal!("-equations\n---------\n")); ArcStr::from(__mm_s) }).clone());
    r#str = (List::fold1(List::intRange(metamodelica::arrayLength(inArrLst.clone())), (std::sync::Arc::new(dumpEqArrLst1) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, ArcStr) -> Result<ArcStr> + 'static>), inLstLst, (heading).clone())?).clone();
    Ok(())
}

fn dumpEqArrLst1(mut lstIdx: i32, mut inLstLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, mut heading: ArcStr) -> Result<ArcStr> {
    let mut headingOut: ArcStr;
    let mut str1: ArcStr;
    let mut inLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    inLst = (inLstLst).get(lstIdx)?;
    str1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*heading.clone()); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(lstIdx - 1)); ArcStr::from(__mm_s) }).clone();
    BackendDump::dumpEquationList(inLst, (str1).clone())?;
    headingOut = (heading).clone();
    Ok(headingOut)
}

//--------------------------------------------------//
// solve torn systems in parallel
//-------------------------------------------------//
pub(crate) fn parallelizeTornSystems(mut graphIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut metaIn: HpcOmTaskGraph::TaskGraphMeta, mut sccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut simVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut inDAE: Arc<BackendDAE::BackendDAE>) -> (Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, Arc<metamodelica::List<i32>>) {
    let mut scheduledTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    let mut daeNodeIdcs: Arc<metamodelica::List<i32>>;
    (scheduledTasks, daeNodeIdcs) = 'mc: {
        let __mc_input = inDAE.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut eqSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
                    let mut taskLst: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
                    let mut daeNodes: Arc<metamodelica::List<i32>>;
                    let true = (false) else { bail!("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(inDAE.clone()) {
                        Deref @ BackendDAE::BackendDAE { eqs: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    eqSysts = __pa0.clone();
                    (_, taskLst) = pts_traverseEqSystems(eqSysts.clone(), sccSimEqMapping.clone(), simVarMapping.clone(), 1, metamodelica::nil(), BackendDAEUtil::isInitializationDAE(inDAE.shared.clone()))?;
                    daeNodes = List::map(taskLst.clone(), (std::sync::Arc::new(getScheduledTaskCompIdx) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>) -> Result<i32> + 'static>))?;
                    Ok((taskLst.clone(), daeNodes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (scheduledTasks, daeNodeIdcs)
}

fn getScheduledTaskCompIdx(mut taskIn: Arc<HpcOmSimCode::Task>) -> Result<i32> {
    let mut compIdx: i32 = 0;
    compIdx = (::match_deref::match_deref! { match &(taskIn) {
        Deref @ HpcOmSimCode::Task::SCHEDULED_TASK { compIdx: __esc_compIdx, .. } => {
            compIdx = (*__esc_compIdx).clone();
            compIdx.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(compIdx)
}

fn pts_traverseEqSystems(mut eqSysIn: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut sccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut simVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut compIdxIn: i32, mut taskLstIn: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut isInitial: bool) -> Result<(i32, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)> {
    let mut compIdxOut: i32;
    let mut taskLstOut: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    (compIdxOut, taskLstOut) = 'mc: {
        let __mc_input = eqSysIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqs, matching: Deref @ BackendDAE::Matching::MATCHING { comps, .. }, .. }, tail: eqSysRest } => {
                    let mut compIdx: i32;
                    let mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut taskLst: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
                    eqLst = BackendEquation::equationList(eqs.clone())?;
                    varLst = BackendVariable::varList(vars.clone())?;
                    (compIdx, taskLst) = pts_traverseCompsAndParallelize(comps.clone(), eqLst.clone(), varLst.clone(), sccSimEqMapping.clone(), simVarMapping.clone(), compIdxIn, taskLstIn.clone(), isInitial);
                    (compIdx, taskLst) = pts_traverseEqSystems(eqSysRest.clone(), sccSimEqMapping.clone(), simVarMapping.clone(), compIdx.clone(), taskLst.clone(), isInitial)?;
                    Ok((compIdx.clone(), taskLst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((compIdxIn, taskLstIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("pts_traverseEqSystems failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((compIdxOut, taskLstOut))
}

fn pts_traverseCompsAndParallelize(mut inComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut eqsIn: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut varsIn: Arc<metamodelica::List<BackendDAE::Var>>, mut sccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut simVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut compIdxIn: i32, mut taskLstIn: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut isInitial: bool) -> (i32, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) {
    let mut compIdxOut: i32;
    let mut taskLstOut: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    (compIdxOut, taskLstOut) = 'mc: {
        let __mc_input = inComps;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((compIdxIn, taskLstIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: comp @ Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { residualequations: resEqs, innerEquations, .. }, .. }, tail: rest } => {
                    let mut numEqs: i32;
                    let mut numVars: i32;
                    let mut compIdx: i32;
                    let mut numResEqs: i32;
                    let mut eqIdcs: Arc<metamodelica::List<i32>>;
                    let mut varIdcs: Arc<metamodelica::List<i32>>;
                    let mut eqIdcsSys: Arc<metamodelica::List<i32>>;
                    let mut simEqSysIdcs: Arc<metamodelica::List<i32>>;
                    let mut resSimEqSysIdcs: Arc<metamodelica::List<i32>>;
                    let mut otherSimEqSysIdcs: Arc<metamodelica::List<i32>>;
                    let mut varIdcLstSys: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut varIdcsLsts: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut otherSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut otherEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut otherVars: BackendDAE::Variables;
                    let mut graph: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut graphMerged: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut meta: HpcOmTaskGraph::TaskGraphMeta;
                    let mut metaMerged: HpcOmTaskGraph::TaskGraphMeta;
                    let mut schedule: Arc<HpcOmSimCode::Schedule>;
                    let mut task: Arc<HpcOmSimCode::Task>;
                    let mut taskLst: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
                    let mut otherEqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut otherVarLst: Arc<metamodelica::List<BackendDAE::Var>>;
                    (eqIdcs, varIdcsLsts, _) = List::map_3(innerEquations.clone(), (std::sync::Arc::new(BackendDAEUtil::getEqnAndVarsFromInnerEquation) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::InnerEquation) -> Result<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<DAE::Constraint>>>)> + 'static>))?;
                    varIdcs = List::flatten(varIdcsLsts.clone())?;
                    numEqs = (eqIdcs.clone().len() as i32);
                    numVars = (varIdcs.clone().len() as i32);
                    numResEqs = (resEqs.clone().len() as i32);
                    eqIdcsSys = List::intRange(numEqs.clone());
                    (varIdcLstSys, _) = List::mapFold(varIdcsLsts.clone(), (std::sync::Arc::new(fnptr!(genSystemVarIdcs, Arc<metamodelica::List<i32>>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<(Arc<metamodelica::List<i32>>, i32)> + 'static>), 1)?;
                    otherEqLst = List::map1(eqIdcs.clone(), (std::sync::Arc::new(List::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), eqsIn.clone())?;
                    otherVarLst = List::map1(varIdcs.clone(), (std::sync::Arc::new(List::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), varsIn.clone())?;
                    otherVars = BackendVariable::listVar1(otherVarLst.clone())?;
                    otherEqs = BackendEquation::listEquation(otherEqLst.clone())?;
                    (m, mT) = BackendDAEUtil::adjacencyMatrixDispatch(otherVars.clone(), otherEqs.clone(), openmodelica_backend_types::BackendDAE::IndexType::ABSOLUTE, None, isInitial)?;
                    (graph, meta) = HpcOmTaskGraph::getEmptyTaskGraph(numEqs.clone(), numEqs.clone(), numVars.clone());
                    graph = buildMatchedGraphForTornSystem(1, eqIdcsSys.clone(), varIdcLstSys.clone(), m.clone(), mT.clone(), graph.clone())?;
                    meta = buildTaskgraphMetaForTornSystem(graph.clone(), otherEqLst.clone(), otherVarLst.clone(), meta.clone())?;
                    simEqSysIdcs = metamodelica::arrayGet(sccSimEqMapping.clone(), compIdxIn)?;
                    resSimEqSysIdcs = List::map1r(List::intRange(numResEqs.clone()), (std::sync::Arc::new(fnptr!(intSub, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), listHead(simEqSysIdcs.clone())?)?;
                    otherSimEqSysIdcs = List::map1r(List::intRange2(numResEqs.clone() + 1, numResEqs.clone() + numEqs.clone()), (std::sync::Arc::new(fnptr!(intSub, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), listHead(simEqSysIdcs.clone())?)?;
                    otherSimEqMapping = metamodelica::arrayFromVec(List::map(otherSimEqSysIdcs.clone(), std::sync::Arc::new(fnptr!(List::create, _)))?.into_iter().cloned().collect());
                    BackendDump::dumpBipartiteGraphStrongComponent1(comp.clone(), eqsIn.clone(), varsIn.clone(), None, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("tornSys_bipartite_")); __mm_s.push_str(&*intString(compIdxIn)); ArcStr::from(__mm_s) }).clone())?;
                    BackendDump::dumpDAGStrongComponent(graph.clone(), meta.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("tornSys_matched_")); __mm_s.push_str(&*intString(compIdxIn)); ArcStr::from(__mm_s) }).clone())?;
                    (graphMerged, metaMerged) = (graph.clone(), meta.clone());
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("function pts_traverseCompsAndParallelize failed. GRS is temporarily disabled.")).clone()])?;
                    BackendDump::dumpDAGStrongComponent(graphMerged.clone(), metaMerged.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("tornSys_matched2_")); __mm_s.push_str(&*intString(compIdxIn)); ArcStr::from(__mm_s) }).clone())?;
                    schedule = HpcOmScheduler::createListSchedule(graphMerged.clone(), metaMerged.clone(), 2, otherSimEqMapping.clone(), simVarMapping.clone())?;
                    HpcOmScheduler::printSchedule(schedule.clone())?;
                    task = pts_transformScheduleToTask(schedule.clone(), resSimEqSysIdcs.clone(), compIdxIn)?;
                    (compIdx, taskLst) = pts_traverseCompsAndParallelize(rest.clone(), eqsIn.clone(), varsIn.clone(), sccSimEqMapping.clone(), simVarMapping.clone(), compIdxIn + 1, metamodelica::cons(task.clone(), taskLstIn.clone()), isInitial);
                    Ok((compIdx.clone(), taskLst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    let mut compIdx: i32;
                    let mut taskLst: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
                    (compIdx, taskLst) = pts_traverseCompsAndParallelize(rest.clone(), eqsIn.clone(), varsIn.clone(), sccSimEqMapping.clone(), simVarMapping.clone(), compIdxIn + 1, taskLstIn.clone(), isInitial);
                    Ok((compIdx.clone(), taskLst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (compIdxOut, taskLstOut)
}

fn pts_transformScheduleToTask(mut otherEqSys: Arc<HpcOmSimCode::Schedule>, mut resSimEqs: Arc<metamodelica::List<i32>>, mut compIdx: i32) -> Result<Arc<HpcOmSimCode::Task>> {
    let mut task: Arc<HpcOmSimCode::Task>;
    task = 'mc: {
        let __mc_input = otherEqSys;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { .. } => {
                    metamodelica::print((literal!("levelScheduling is not supported for heterogenious scheduling\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks, outgoingDepTasks, allCalcTasks, .. } => {
                    let mut numThreads: i32;
                    let mut schedule: Arc<HpcOmSimCode::Schedule>;
                    numThreads = metamodelica::arrayLength(threadTasks.clone());
                    schedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
                    Ok(Arc::new(HpcOmSimCode::Task::SCHEDULED_TASK { compIdx: compIdx, numThreads: numThreads.clone(), taskSchedule: schedule.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("pts_transformScheduleToTask failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(task)
}

fn genSystemVarIdcs(mut idcsIn: Arc<metamodelica::List<i32>>, mut idx: i32) -> (Arc<metamodelica::List<i32>>, i32) {
    let mut idcsOut: Arc<metamodelica::List<i32>>;
    let mut idx2: i32;
    idx2 = (idcsIn.len() as i32) + idx;
    idcsOut = List::intRange2(idx, idx2 - 1);
    (idcsOut, idx2)
}

//05-09-2014 marcusw: Changed because of dependency-task restructuring for MPI
//protected function appendStringToLockIdcs "author: Waurich TUD 2014-07
//  appends the suffix to the lockIds of the given tasks
//"
//  input list<HpcOmSimCode.Task> taskLstIn;
//  input String suffix;
//  output list<HpcOmSimCode.Task> taskLstOut;
//algorithm
//  taskLstOut := List.map1(taskLstIn,appendStringToLockIdcs1,suffix);
//end appendStringToLockIdcs;
//
//protected function appendStringToLockIdcs1 "author: Waurich TUD 2014-07
//  appends the suffix to the lockIds of the given tasks
//"
//  input HpcOmSimCode.Task taskIn;
//  input String suffix;
//  output HpcOmSimCode.Task taskOut;
//algorithm
//  taskOut := match(taskIn,suffix)
//    local
//      String lockId;
//    case(HpcOmSimCode.ASSIGNLOCKTASK(lockId=lockId),_)
//      equation
//        lockId = stringAppend(lockId,suffix);
//    then HpcOmSimCode.ASSIGNLOCKTASK(lockId);
//     case(HpcOmSimCode.RELEASELOCKTASK(lockId=lockId),_)
//      equation
//        lockId = stringAppend(lockId,suffix);
//    then HpcOmSimCode.RELEASELOCKTASK(lockId);
//    else
//      then taskIn;
//  end match;
//end appendStringToLockIdcs1;
fn buildMatchedGraphForTornSystem(mut idx: i32, mut eqsIn: Arc<metamodelica::List<i32>>, mut varsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut graphIn: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut graphOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    graphOut = 'mc: {
        let __mc_input = graphIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut eq: i32;
            let mut vars: Arc<metamodelica::List<i32>>;
            let mut depEqs: Arc<metamodelica::List<i32>>;
            let mut graph: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let true = ((eqsIn.clone().len() as i32) >= idx) else { bail!("pattern mismatch") };
            vars = (varsIn.clone()).get(idx)?;
            eq = (eqsIn.clone()).get(idx)?;
            depEqs = List::flatten(List::map1(vars.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), mt.clone())?)?;
            (depEqs, _) = List::deleteMemberOnTrue(eq.clone(), depEqs.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            graph = metamodelica::arrayUpdate(graphIn.clone(), eq.clone(), depEqs.clone())?;
            graph = buildMatchedGraphForTornSystem(idx + 1, eqsIn.clone(), varsIn.clone(), m.clone(), mt.clone(), graph.clone())?;
            Ok(graph.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let false = ((eqsIn.clone().len() as i32) > idx) else { bail!("pattern mismatch") };
            Ok(graphIn.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(graphOut)
}

fn buildTaskgraphMetaForTornSystem(mut graph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut varLst: Arc<metamodelica::List<BackendDAE::Var>>, mut metaIn: HpcOmTaskGraph::TaskGraphMeta) -> Result<HpcOmTaskGraph::TaskGraphMeta> {
    let mut metaOut: HpcOmTaskGraph::TaskGraphMeta;
    let mut numNodes: i32;
    let mut eqStrings: Arc<metamodelica::List<ArcStr>>;
    let mut varStrings: Arc<metamodelica::List<ArcStr>>;
    let mut descLst: Arc<metamodelica::List<ArcStr>>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut compParamMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>;
    let mut compInformations: metamodelica::Array<HpcOmTaskGraph::ComponentInfo>;
    let HpcOmTaskGraph::TASKGRAPHMETA { varCompMapping: __pa0, eqCompMapping: __pa1, compParamMapping: __pa2, nodeMark: __pa3, compInformations: __pa4, .. } = (metaIn) else { bail!("pattern mismatch") };
    varCompMapping = __pa0.clone();
    eqCompMapping = __pa1.clone();
    compParamMapping = __pa2.clone();
    nodeMark = __pa3.clone();
    compInformations = __pa4.clone();
    numNodes = metamodelica::arrayLength(graph.clone());
    inComps = metamodelica::arrayFromVec(List::map(List::intRange(numNodes), std::sync::Arc::new(fnptr!(List::create, _)))?.into_iter().cloned().collect());
    compNames = metamodelica::arrayFromVec(List::map(List::intRange(numNodes), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?.into_iter().cloned().collect());
    exeCosts = arrayCreate(numNodes, (3, metamodelica::OrderedFloat(20.0_f64)));
    commCosts = Array::map(graph.clone(), (std::sync::Arc::new(buildDummyCommCosts) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>> + 'static>))?;
    eqStrings = List::map(eqLst, (std::sync::Arc::new(BackendDump::equationString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<ArcStr> + 'static>))?;
    varStrings = List::map(varLst, (std::sync::Arc::new(HpcOmTaskGraph::getVarString) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<ArcStr> + 'static>))?;
    descLst = List::map1(eqStrings, (std::sync::Arc::new(fnptr!(stringAppend, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<ArcStr> + 'static>), (literal!(" FOR ")).clone())?;
    descLst = List::threadMap(descLst, varStrings, (std::sync::Arc::new(fnptr!(stringAppend, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<ArcStr> + 'static>))?;
    compDescs = metamodelica::arrayFromVec(descLst.into_iter().cloned().collect());
    metaOut = HpcOmTaskGraph::TaskGraphMeta { inComps: inComps.clone(), varCompMapping: varCompMapping.clone(), eqCompMapping: eqCompMapping.clone(), compParamMapping: compParamMapping.clone(), compNames: compNames.clone(), compDescs: compDescs.clone(), exeCosts: exeCosts.clone(), commCosts: commCosts.clone(), nodeMark: nodeMark.clone(), compInformations: compInformations.clone() };
    Ok(metaOut)
}

fn buildDummyCommCosts(mut childNodes: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>> {
    let mut commCosts: Arc<metamodelica::List<HpcOmTaskGraph::Communication>>;
    commCosts = List::map(childNodes, (std::sync::Arc::new(fnptr!(buildDummyCommCost, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<HpcOmTaskGraph::Communication> + 'static>))?;
    Ok(commCosts)
}

fn buildDummyCommCost(mut iChildNodeIdx: i32) -> HpcOmTaskGraph::Communication {
    let mut oCommCost: HpcOmTaskGraph::Communication;
    oCommCost = HpcOmTaskGraph::Communication { numberOfVars: 1, integerVars: metamodelica::nil(), floatVars: list![-1], booleanVars: metamodelica::nil(), stringVars: metamodelica::nil(), childNode: iChildNodeIdx, requiredTime: metamodelica::OrderedFloat(70.0_f64) };
    oCommCost
}

pub(crate) fn createSingleBlockSchedule(mut graphIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut metaIn: HpcOmTaskGraph::TaskGraphMeta, mut scheduledTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut sccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<HpcOmSimCode::Schedule>> {
    let mut schedule: Arc<HpcOmSimCode::Schedule>;
    let mut nodes: Arc<metamodelica::List<i32>>;
    let mut comps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut simEqSys: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut thread1: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
    let HpcOmTaskGraph::TASKGRAPHMETA { inComps: __pa0, .. } = (metaIn) else { bail!("pattern mismatch") };
    inComps = __pa0.clone();
    nodes = List::intRange(metamodelica::arrayLength(graphIn.clone()));
    comps = List::map1(nodes.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), inComps.clone())?;
    simEqSys = HpcOmScheduler::getSimEqSysIdcsForNodeLst(comps, sccSimEqMapping.clone())?;
    simEqSys = List::map1(simEqSys, (std::sync::Arc::new(List::sort) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    thread1 = List::threadMap1(simEqSys, nodes, (std::sync::Arc::new(fnptr!(HpcOmScheduler::makeCalcTask, Arc<metamodelica::List<i32>>, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32, i32) -> Result<Arc<HpcOmSimCode::Task>> + 'static>), 1)?;
    threadTasks = arrayCreate(4, metamodelica::nil());
    threadTasks = metamodelica::arrayUpdate(threadTasks.clone(), 1, thread1.clone())?;
    allCalcTasks = arrayCreate((thread1.len() as i32), (openmodelica_simcode_types::HpcOmSimCode::Task::interned_TASKEMPTY(), 0));
    schedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: metamodelica::nil(), scheduledTasks: scheduledTasks, allCalcTasks: allCalcTasks.clone() });
    Ok(schedule)
}

