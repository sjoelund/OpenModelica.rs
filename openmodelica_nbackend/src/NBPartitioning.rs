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

use crate::NBAdjacency;
use crate::NBBackendUtil;
use crate::NBCausalize as Causalize;
use crate::NBEquation as BEquation;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationKind;
use crate::NBEquation::EquationPointer;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::WhenEquationBody;
use crate::NBEquation::WhenStatement;
use crate::NBMatching as Matching;
use crate::NBModule as Module;
use crate::NBPartition as Partition;
use crate::NBSorting as Sorting;
use crate::NBStrongComponent as StrongComponent;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointer;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use openmodelica_backend_types::BackendDAE as OldBackendDAE;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE as OldDAE;
use openmodelica_nf_frontend::NFBackendExtension::BackendInfo;
use openmodelica_nf_frontend::NFBackendExtension::VariableKind;
use openmodelica_nf_frontend::NFBuiltinFuncs;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFClockKind as ClockKind;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFPrefixes;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::Rational;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

// NF
// Backend
// Util
// Old imports
pub mod BClock {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum BClock {
        BASE_CLOCK {
            clock: Arc<ClockKind::NFClockKind>,
        },
        SUB_CLOCK {
            factor: Arc<Rational::Rational>,
            shift: Arc<Rational::Rational>,
            solver: Option<ArcStr>,
        },
        INFERRED_CLOCK {
            base_ref: Arc<ComponentRef::NFComponentRef>,
        },
    }
    impl metamodelica::gc::MMTrace for BClock {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                BClock::BASE_CLOCK { clock } => {
                    metamodelica::gc::MMTrace::mm_accept(clock, __mmv)?;
                    Ok(())
                }
                BClock::SUB_CLOCK { factor, shift, solver } => {
                    metamodelica::gc::MMTrace::mm_accept(factor, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(shift, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(solver, __mmv)?;
                    Ok(())
                }
                BClock::INFERRED_CLOCK { base_ref } => {
                    metamodelica::gc::MMTrace::mm_accept(base_ref, __mmv)?;
                    Ok(())
                }
            }
        }
    }
    impl Default for BClock {
        fn default() -> Self {
            Self::BASE_CLOCK {
                clock: Default::default(),
            }
        }
    }
    pub use self::BClock::{BASE_CLOCK,SUB_CLOCK,INFERRED_CLOCK};
    pub(crate) fn toString(mut clock: Arc<BClock>) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = ((::match_deref::match_deref! { match &(clock.clone()) {
        Deref @ BASE_CLOCK { .. } => ClockKind::toDebugString(var_field!((*clock).clock, BClock::BASE_CLOCK).clone())?,
        Deref @ SUB_CLOCK { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SUB_CLOCK(")); __mm_s.push_str(&*Rational::toString(var_field!((*clock).factor, BClock::SUB_CLOCK).clone())); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*Rational::toString(var_field!((*clock).shift, BClock::SUB_CLOCK).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
        Deref @ INFERRED_CLOCK { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("INFERRED_CLOCK(")); __mm_s.push_str(&*ComponentRef::toString(var_field!((*clock).base_ref, BClock::INFERRED_CLOCK).clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
        _ => literal!("UNKNOWN_CLOCK()"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        Ok(r#str)
    }

    pub(crate) fn hash(mut clock: Arc<BClock>) -> Result<i32> {
        let mut i: i32 = stringHashDjb2((toString(clock.clone())?).clone());
        Ok(i)
    }

    pub(crate) fn isEqual(mut clock1: Arc<BClock>, mut clock2: Arc<BClock>) -> Result<bool> {
        let mut b: bool;
        b = (::match_deref::match_deref! { match &((clock1.clone(), clock2.clone())) {
        (Deref @ BASE_CLOCK { .. }, Deref @ BASE_CLOCK { .. }) => ClockKind::compare(var_field!((*clock1).clock, BClock::BASE_CLOCK).clone(), var_field!((*clock2).clock, BClock::BASE_CLOCK).clone())? == 0,
        (Deref @ SUB_CLOCK { .. }, Deref @ SUB_CLOCK { .. }) => Rational::isEqual(var_field!((*clock1).factor, BClock::SUB_CLOCK).clone(), var_field!((*clock2).factor, BClock::SUB_CLOCK).clone()) && Rational::isEqual(var_field!((*clock1).shift, BClock::SUB_CLOCK).clone(), var_field!((*clock2).shift, BClock::SUB_CLOCK).clone()) && Util::optionEqual(var_field!((*clock1).solver, BClock::SUB_CLOCK).clone(), var_field!((*clock2).solver, BClock::SUB_CLOCK).clone(), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?,
        (Deref @ INFERRED_CLOCK { .. }, Deref @ INFERRED_CLOCK { .. }) => ComponentRef::isEqual(var_field!((*clock1).base_ref, BClock::INFERRED_CLOCK).clone(), var_field!((*clock2).base_ref, BClock::INFERRED_CLOCK).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(b)
    }

    pub(crate) fn add(mut eqn: Arc<Equation::Equation>, mut info: Arc<ClockedInfo::ClockedInfo>) -> Result<()> {
        let () = (::match_deref::match_deref! { match &((BEquation::Equation::getLHS(eqn.clone())?, BEquation::Equation::getRHS(eqn)?)) {
        (Some(Deref @ Expression::CREF { cref: clock_name, .. }), Some(exp)) if (Expression::isClockOrSampleFunction(exp.clone())?) => {
            create(clock_name.clone(), exp.clone(), info)?;
            ()
        },
        (Some(exp), Some(Deref @ Expression::CREF { cref: clock_name, .. })) if (Expression::isClockOrSampleFunction(exp.clone())?) => {
            create(clock_name.clone(), exp.clone(), info)?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(())
    }

    pub(crate) fn isBaseClock(mut clock: Arc<BClock>) -> bool {
        let mut b: bool;
        b = (::match_deref::match_deref! { match &(clock) {
        Deref @ BASE_CLOCK { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        b
    }

    pub(crate) fn isInferredClock(mut clock: Arc<BClock>) -> bool {
        let mut b: bool;
        b = (::match_deref::match_deref! { match &(clock) {
        Deref @ BASE_CLOCK { clock: Deref @ ClockKind::INFERRED_CLOCK { .. } } => true,
        Deref @ INFERRED_CLOCK { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        b
    }

    pub(crate) fn isEventClock(mut clock: Arc<BClock>) -> bool {
        let mut b: bool;
        b = (::match_deref::match_deref! { match &(clock) {
        Deref @ BASE_CLOCK { clock: Deref @ ClockKind::EVENT_CLOCK { .. } } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        b
    }

    pub(crate) fn baseClockInferrence(mut clock: Arc<BClock>, mut base_clock_inferrence: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<BClock>>>) -> Result<Arc<BClock>> {
        let mut clock: Arc<BClock> = clock;
        clock = (::match_deref::match_deref! { match &(clock.clone()) {
        Deref @ INFERRED_CLOCK { .. } => {
            let mut base_clock: Arc<BClock>;
            base_clock = UnorderedMap::getSafe(var_field!((*clock).base_ref, BClock::INFERRED_CLOCK).clone(), base_clock_inferrence.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo"))?;
            baseClockInferrence(base_clock.clone(), base_clock_inferrence)?
        },
        Deref @ BASE_CLOCK { clock: Deref @ ClockKind::INFERRED_CLOCK { .. } } => {
            DEFAULT_BASE_CLOCK().clone()
        },
        _ => {
            clock
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(clock)
    }

    pub(crate) fn convertBase(mut clock: Arc<BClock>) -> Result<Arc<OldDAE::ClockKind>> {
        let mut oldClock: Arc<OldDAE::ClockKind>;
        oldClock = (::match_deref::match_deref! { match &(clock.clone()) {
        Deref @ BASE_CLOCK { .. } => ClockKind::toDAE(var_field!((*clock).clock, BClock::BASE_CLOCK).clone())?,
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.BClock.convertBase")); __mm_s.push_str(&*literal!(" failed for non-base clock: ")); __mm_s.push_str(&*toString(clock)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(oldClock)
    }

    pub(crate) fn convertSub(mut clock: Arc<BClock>) -> Result<OldBackendDAE::SubClock> {
        let mut oldClock: OldBackendDAE::SubClock;
        oldClock = (::match_deref::match_deref! { match &(clock.clone()) {
        Deref @ SUB_CLOCK { .. } => OldBackendDAE::SubClock::SUBCLOCK { factor: NBBackendUtil::convertRational(var_field!((*clock).factor, BClock::SUB_CLOCK).clone()), shift: NBBackendUtil::convertRational(var_field!((*clock).shift, BClock::SUB_CLOCK).clone()), solver: var_field!((*clock).solver, BClock::SUB_CLOCK).clone() },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.BClock.convertSub")); __mm_s.push_str(&*literal!(" failed for non-sub clock: ")); __mm_s.push_str(&*toString(clock)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(oldClock)
    }

    pub(crate) fn toExp(mut clock: Arc<BClock>) -> Result<Arc<Expression::NFExpression>> {
        let mut exp: Arc<Expression::NFExpression>;
        exp = (::match_deref::match_deref! { match &(clock.clone()) {
        Deref @ BASE_CLOCK { .. } => Arc::new(Expression::NFExpression::CLKCONST { clk: var_field!((*clock).clock, BClock::BASE_CLOCK).clone() }),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.BClock.toExp")); __mm_s.push_str(&*literal!(" failed for non-base clock: ")); __mm_s.push_str(&*toString(clock)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(exp)
    }

    fn create(mut clock_name: Arc<ComponentRef::NFComponentRef>, mut exp: Arc<Expression::NFExpression>, mut info: Arc<ClockedInfo::ClockedInfo>) -> Result<()> {
        let mut clock: Arc<BClock>;
        let mut baseClock: Option<Arc<ComponentRef::NFComponentRef>>;
        let mut clock_var: Pointer::Pointer<Arc<Variable::NFVariable>>;
        match '__try0: {
            (clock, baseClock) = unwrap_break_err!(fromExp(exp.clone()), '__try0);
            if isSome(baseClock.clone()) {
                unwrap_break_err!(UnorderedMap::add(clock_name.clone(), clock.clone(), info.subClocks.clone()), '__try0);
                unwrap_break_err!(UnorderedMap::add(clock_name.clone(), unwrap_break_err!(Util::getOption(baseClock.clone()), '__try0), info.subToBase.clone()), '__try0);
            } else {
                unwrap_break_err!(UnorderedMap::add(clock_name.clone(), clock.clone(), info.baseClocks.clone()), '__try0);
            }
            clock_var = unwrap_break_err!(BVariable::getVarPointer(clock_name.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo")), '__try0);
            if !(BVariable::isClockOrClocked(clock_var.clone())) {
                BVariable::setVarKind(clock_var.clone(), openmodelica_nf_frontend::NFBackendExtension::VariableKind::interned_CLOCKED());
            }
            Ok::<_, anyhow::Error>((baseClock.clone(), clock.clone(), clock_var.clone()))
        } {
            Ok((__try0_o0, __try0_o1, __try0_o2)) => {
                baseClock = __try0_o0;
                clock = __try0_o1;
                clock_var = __try0_o2;
            }
            Err(__try0_err) => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.BClock.create")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*ComponentRef::toString(clock_name.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
                return Err(__try0_err);
            }
        }
        Ok(())
    }

    fn fromExp(mut exp: Arc<Expression::NFExpression>) -> Result<(Arc<BClock>, Option<Arc<ComponentRef::NFComponentRef>>)> {
        let mut subClock: Arc<BClock> = Arc::new(<BClock as ::std::default::Default>::default());
        let mut baseClock: Option<Arc<ComponentRef::NFComponentRef>> = None;
        (subClock, baseClock) = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CLKCONST { .. } => {
            (Arc::new(BClock::BASE_CLOCK { clock: var_field!((*exp).clk, Expression::NFExpression::CLKCONST).clone() }), None)
        },
        Deref @ Expression::CREF { .. } => {
            (DEFAULT_SUB_CLOCK().clone(), Some(var_field!((*exp).cref, Expression::NFExpression::CREF).clone()))
        },
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } => {
            (baseClock, subClock) = (::match_deref::match_deref! { match &((AbsynUtil::pathString(Function::nameConsiderBuiltin(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL).clone())?, (literal!(".")).clone(), true, false)?, Call::arguments(call.clone())?)) {
        (Deref @ "sample", Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil } }) => {
            (subClock, baseClock) = fromExp(e.clone())?;
            (baseClock, subClock)
        },
        (Deref @ "subSample", Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: i1 }, tail: Deref @ metamodelica::List::Nil } }) => {
            (subClock, baseClock) = fromExp(e.clone())?;
            subClock = updateSubClock(subClock, Arc::new(BClock::SUB_CLOCK { factor: Arc::new(Rational::Rational { n: i1.clone(), d: 1 }), shift: Rational::ZERO.clone(), solver: None }))?;
            (baseClock, subClock)
        },
        (Deref @ "superSample", Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: i1 }, tail: Deref @ metamodelica::List::Nil } }) => {
            (subClock, baseClock) = fromExp(e.clone())?;
            subClock = updateSubClock(subClock, Arc::new(BClock::SUB_CLOCK { factor: Arc::new(Rational::Rational { n: 1, d: i1.clone() }), shift: Rational::ZERO.clone(), solver: None }))?;
            (baseClock, subClock)
        },
        (Deref @ "shiftSample", Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: i1 }, tail: Deref @ metamodelica::List::Nil } }) => {
            (subClock, baseClock) = fromExp(e.clone())?;
            subClock = updateSubClock(subClock, Arc::new(BClock::SUB_CLOCK { factor: Rational::ONE.clone(), shift: Arc::new(Rational::Rational { n: i1.clone(), d: 1 }), solver: None }))?;
            (baseClock, subClock)
        },
        (Deref @ "shiftSample", Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: i1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: i2 }, tail: Deref @ metamodelica::List::Nil } } }) => {
            (subClock, baseClock) = fromExp(e.clone())?;
            subClock = updateSubClock(subClock, Arc::new(BClock::SUB_CLOCK { factor: Rational::ONE.clone(), shift: Arc::new(Rational::Rational { n: i1.clone(), d: i2.clone() }), solver: None }))?;
            (baseClock, subClock)
        },
        (Deref @ "backSample", Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: i1 }, tail: Deref @ metamodelica::List::Nil } }) => {
            (subClock, baseClock) = fromExp(e.clone())?;
            subClock = updateSubClock(subClock, Arc::new(BClock::SUB_CLOCK { factor: Rational::ONE.clone(), shift: Arc::new(Rational::Rational { n: -(i1.clone()), d: 1 }), solver: None }))?;
            (baseClock, subClock)
        },
        (Deref @ "backSample", Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: i1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: i2 }, tail: Deref @ metamodelica::List::Nil } } }) => {
            (subClock, baseClock) = fromExp(e.clone())?;
            subClock = updateSubClock(subClock, Arc::new(BClock::SUB_CLOCK { factor: Rational::ONE.clone(), shift: Arc::new(Rational::Rational { n: -(i1.clone()), d: i2.clone() }), solver: None }))?;
            (baseClock, subClock)
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.BClock.fromExp")); __mm_s.push_str(&*literal!(" failed for exp with unhandled call: ")); __mm_s.push_str(&*Expression::toString(exp)?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            (subClock, baseClock)
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.BClock.fromExp")); __mm_s.push_str(&*literal!(" failed for exp with unhandled expression kind: ")); __mm_s.push_str(&*Expression::toString(exp)?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok((subClock, baseClock))
    }

    pub(crate) fn updateSubClock(mut dest: Arc<BClock>, mut src: Arc<BClock>) -> Result<Arc<BClock>> {
        let mut dest: Arc<BClock> = dest;
        dest = (::match_deref::match_deref! { match &((dest.clone(), src.clone())) {
        (Deref @ SUB_CLOCK { .. }, Deref @ SUB_CLOCK { .. }) => {
            assign_variant_field!(dest => BClock::SUB_CLOCK;
                shift = Rational::add(var_field!((*dest).shift, BClock::SUB_CLOCK).clone(), Rational::mul(var_field!((*src).shift, BClock::SUB_CLOCK).clone(), var_field!((*dest).factor, BClock::SUB_CLOCK).clone())),
                factor = Rational::mul(var_field!((*dest).factor, BClock::SUB_CLOCK).clone(), var_field!((*src).factor, BClock::SUB_CLOCK).clone())
            );
            dest
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.BClock.updateSubClock")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*toString(dest)?); __mm_s.push_str(&*literal!(" and ")); __mm_s.push_str(&*toString(src)?); __mm_s.push_str(&*literal!(" because of incorrect clock types.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(dest)
    }

}

thread_local! { static __DEFAULT_BASE_CLOCK_TLS: Arc<BClock::BClock> = Arc::new(BClock::BClock::BASE_CLOCK { clock: Arc::new(ClockKind::NFClockKind::REAL_CLOCK { interval: Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(1.0_f64) }) }) }); }
pub(crate) fn DEFAULT_BASE_CLOCK() -> Arc<BClock::BClock> { __DEFAULT_BASE_CLOCK_TLS.with(|__t| __t.clone()) }

thread_local! { static __DEFAULT_SUB_CLOCK_TLS: Arc<BClock::BClock> = Arc::new(BClock::BClock::SUB_CLOCK { factor: Rational::ONE.clone(), shift: Rational::ZERO.clone(), solver: None }); }
pub(crate) fn DEFAULT_SUB_CLOCK() -> Arc<BClock::BClock> { __DEFAULT_SUB_CLOCK_TLS.with(|__t| __t.clone()) }

pub type CrefLst = Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;

pub mod ClockedInfo {
    use super::*;
#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct ClockedInfo {
        pub baseClocks: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<BClock::BClock>>>,
        pub subClocks: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<BClock::BClock>>>,
        pub subToBase: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>,
        pub baseToSub: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>,
    }

    impl metamodelica::gc::MMTrace for ClockedInfo {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.baseClocks, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.subClocks, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.subToBase, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.baseToSub, __mmv)?;
            Ok(())
        }
    }
    pub type CLOCKED_INFO = ClockedInfo;

    pub(crate) fn new() -> Arc<ClockedInfo> {
        let mut info: Arc<ClockedInfo> = Arc::new(ClockedInfo { baseClocks: UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1), subClocks: UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1), subToBase: UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1), baseToSub: UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1) });
        info
    }

    pub(crate) fn toString(mut info: Arc<ClockedInfo>) -> Result<ArcStr> {
        let mut r#str: ArcStr = literal!("");
        if !(isEmpty(info.clone())) {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_2((literal!("Clocked Info")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*StringUtil::headline_3((literal!("Base Clocks")).clone())?); __mm_s.push_str(&*UnorderedMap::toString(info.baseClocks.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (std::sync::Arc::new(BClock::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>) -> Result<ArcStr> + 'static>), (literal!("\n")).clone(), (literal!(", ")).clone())?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*StringUtil::headline_3((literal!("Sub Clocks")).clone())?); __mm_s.push_str(&*UnorderedMap::toString(info.subClocks.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (std::sync::Arc::new(BClock::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>) -> Result<ArcStr> + 'static>), (literal!("\n")).clone(), (literal!(", ")).clone())?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*StringUtil::headline_3((literal!("Sub to Base Clocks")).clone())?); __mm_s.push_str(&*UnorderedMap::toString(info.subToBase.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("\n")).clone(), (literal!(", ")).clone())?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*StringUtil::headline_3((literal!("Base to Sub Clocks")).clone())?); __mm_s.push_str(&*UnorderedMap::toString(info.baseToSub.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (std::sync::Arc::new(ComponentRef::listToString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>) -> Result<ArcStr> + 'static>), (literal!("\n")).clone(), (literal!(", ")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        }
        Ok(r#str)
    }

    pub(crate) fn isEmpty(mut info: Arc<ClockedInfo>) -> bool {
        let mut b: bool = UnorderedMap::isEmpty(info.baseClocks.clone());
        b
    }

    pub(crate) fn resolveSubClocks(mut info: Arc<ClockedInfo>, mut clock_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>) -> Result<()> {
        for mut cref in &*UnorderedMap::keyList(clock_map.clone()) {
            let mut cref = cref.clone();
            resolveImplicitSubClock(cref.clone(), info.clone(), clock_map.clone())?;
        }
        for mut sub_clock in &*UnorderedMap::keyList(info.subClocks.clone()) {
            let mut sub_clock = sub_clock.clone();
            resolveSubClock(sub_clock.clone(), info.clone(), clock_map.clone())?;
        }
        for mut sub_clock in &*UnorderedMap::keyList(info.subClocks.clone()) {
            let mut sub_clock = sub_clock.clone();
            addSubClock(sub_clock.clone(), info.clone())?;
        }
        Ok(())
    }

    pub(crate) fn baseClockCount(mut info: Arc<ClockedInfo>, mut countInferred: bool) -> Result<i32> {
        let mut count: i32 = UnorderedMap::size(info.baseClocks.clone());
        if !(countInferred) {
            count = count - List::count(UnorderedMap::valueList(info.baseClocks.clone()), (std::sync::Arc::new(fnptr!(BClock::isInferredClock, Arc<BClock::BClock>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>) -> Result<bool> + 'static>))?;
        }
        Ok(count)
    }

    pub(crate) fn subClockCount(mut info: Arc<ClockedInfo>) -> i32 {
        let mut count: i32 = UnorderedMap::size(info.subClocks.clone());
        count
    }

    fn resolveImplicitSubClock(mut key: Arc<ComponentRef::NFComponentRef>, mut info: Arc<ClockedInfo>, mut clock_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<ComponentRef::NFComponentRef>> {
        let mut clock: Arc<ComponentRef::NFComponentRef> = key.clone();
        if UnorderedMap::contains(key.clone(), clock_map.clone())? {
            clock = UnorderedMap::getSafe(key.clone(), clock_map.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo"))?;
            if !(UnorderedMap::contains(clock.clone(), info.subClocks.clone())? || UnorderedMap::contains(clock.clone(), info.baseClocks.clone())?) {
                clock = resolveImplicitSubClock(clock, info, clock_map.clone())?;
                UnorderedMap::add(key, clock.clone(), clock_map)?;
            }
        }
        Ok(clock)
    }

    fn resolveSubClock(mut clock_name: Arc<ComponentRef::NFComponentRef>, mut info: Arc<ClockedInfo>, mut clock_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<ComponentRef::NFComponentRef>> {
        let mut base_clock: Arc<ComponentRef::NFComponentRef>;
        let mut implicit_clock: Arc<ComponentRef::NFComponentRef>;
        let mut parent_clock: Arc<ComponentRef::NFComponentRef> = UnorderedMap::getSafe(clock_name.clone(), info.subToBase.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo"))?;
        let mut implicit_clock_opt: Option<Arc<ComponentRef::NFComponentRef>> = None;
        let mut dest: Arc<BClock::BClock>;
        let mut src: Arc<BClock::BClock>;
        if UnorderedMap::contains(parent_clock.clone(), info.baseClocks.clone())? {
            base_clock = parent_clock;
        } else {
            if !(UnorderedMap::contains(parent_clock.clone(), info.subClocks.clone())?) {
                implicit_clock_opt = Some(parent_clock.clone());
                parent_clock = UnorderedMap::getSafe(parent_clock, clock_map.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo"))?;
            }
            base_clock = resolveSubClock(parent_clock.clone(), info.clone(), clock_map)?;
            dest = UnorderedMap::getSafe(parent_clock, info.subClocks.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo"))?;
            src = UnorderedMap::getSafe(clock_name.clone(), info.subClocks.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo"))?;
            UnorderedMap::add(clock_name.clone(), BClock::updateSubClock(dest.clone(), src)?, info.subClocks.clone())?;
            UnorderedMap::add(clock_name, base_clock.clone(), info.subToBase.clone())?;
            if isSome(implicit_clock_opt.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(implicit_clock_opt) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                implicit_clock = __pa0.clone();
                UnorderedMap::add(implicit_clock.clone(), dest, info.subClocks.clone())?;
                UnorderedMap::add(implicit_clock, base_clock.clone(), info.subToBase.clone())?;
            }
        }
        Ok(base_clock)
    }

    fn addSubClock(mut clock_name: Arc<ComponentRef::NFComponentRef>, mut info: Arc<ClockedInfo>) -> Result<()> {
        let mut base_clock: Arc<ComponentRef::NFComponentRef> = UnorderedMap::getSafe(clock_name.clone(), info.subToBase.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo"))?;
        let mut current_clocks: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;
        current_clocks = UnorderedMap::getOrDefault(base_clock.clone(), info.baseToSub.clone(), metamodelica::nil())?;
        UnorderedMap::add(base_clock, metamodelica::cons(clock_name, current_clocks), info.baseToSub.clone())?;
        Ok(())
    }

}

// =========================================================================
//                      MAIN ROUTINE, PLEASE DO NOT CHANGE
// =========================================================================
pub(crate) fn main(mut bdae: Arc<BackendDAE::NBackendDAE>, mut kind: Partition::Kind) -> Result<Arc<BackendDAE::NBackendDAE>> {
    let mut bdae: Arc<BackendDAE::NBackendDAE> = bdae;
    let mut func: Module::partitioningInterface;
    func = getModule()?;
    bdae = (::match_deref::match_deref! { match &((kind, bdae.clone())) {
        (Partition::Kind::ODE, Deref @ BackendDAE::MAIN { varData: Deref @ BVariable::VarData::VAR_DATA_SIM { unknowns: variables, clocks, .. }, eqData: Deref @ BEquation::EqData::EQ_DATA_SIM { simulation: equations, clocked, .. }, .. }) => {
            assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN; ode = func(kind, variables.clone(), equations.clone(), clocks.clone(), clocked.clone(), var_field!((*bdae).clockedInfo, BackendDAE::NBackendDAE::MAIN).clone())?);
            assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN;
                ode = ({
        let mut __acc: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>> = metamodelica::nil();
        for mut sys in (var_field!((*bdae).ode, BackendDAE::NBackendDAE::MAIN).clone()).into_iter().cloned() {
            if !(!(Partition::Partition::isEmpty(sys.clone())?)) { continue; }
            let __x = sys.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
                varData = BVariable::VarData::removeTypedCheck(var_field!((*bdae).varData, BackendDAE::NBackendDAE::MAIN).clone(), (std::sync::Arc::new(fnptr!(BVariable::isClock, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), VarData::VarType::DISCRETE.clone())?,
                eqData = BEquation::EqData::removeTypedCheck(var_field!((*bdae).eqData, BackendDAE::NBackendDAE::MAIN).clone(), (std::sync::Arc::new(BEquation::Equation::isTypeClock) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>) -> Result<bool> + 'static>), EqData::EqType::DISCRETE.clone())?
            );
            bdae
        },
        (_, Deref @ BackendDAE::MAIN { varData: Deref @ BVariable::VarData::VAR_DATA_SIM { initials: variables, clocks, .. }, eqData: Deref @ BEquation::EqData::EQ_DATA_SIM { initials: equations, clocked, .. }, .. }) if (Partition::kindIsInitial(kind)) => {
            assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN; init = partitioningNone(kind, variables.clone(), equations.clone(), clocks.clone(), clocked.clone(), var_field!((*bdae).clockedInfo, BackendDAE::NBackendDAE::MAIN).clone())?);
            assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN; init = ({
        let mut __acc: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>> = metamodelica::nil();
        for mut sys in (var_field!((*bdae).init, BackendDAE::NBackendDAE::MAIN).clone()).into_iter().cloned() {
            if !(!(Partition::Partition::isEmpty(sys.clone())?)) { continue; }
            let __x = sys.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            bdae
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.main")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(bdae)
}

pub(crate) fn getModule() -> Result<Arc<dyn ::std::ops::Fn(Partition::Kind, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Arc<ClockedInfo::ClockedInfo>) -> Result<Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>> + 'static>> {
    let mut func: Module::partitioningInterface;
    let mut flag: ArcStr = literal!("clocked");
    func = (::match_deref::match_deref! { match &(flag) {
        Deref @ "default" => (std::sync::Arc::new(partitioningClocked) as std::sync::Arc<dyn ::std::ops::Fn(Partition::Kind, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Arc<ClockedInfo::ClockedInfo>) -> Result<Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>> + 'static>),
        Deref @ "clocked" => (std::sync::Arc::new(partitioningClocked) as std::sync::Arc<dyn ::std::ops::Fn(Partition::Kind, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Arc<ClockedInfo::ClockedInfo>) -> Result<Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>> + 'static>),
        Deref @ "none" => (std::sync::Arc::new(partitioningNone) as std::sync::Arc<dyn ::std::ops::Fn(Partition::Kind, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Arc<ClockedInfo::ClockedInfo>) -> Result<Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>> + 'static>),
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(func)
}

pub(crate) fn categorize(mut bdae: Arc<BackendDAE::NBackendDAE>) -> Result<Arc<BackendDAE::NBackendDAE>> {
    let mut bdae: Arc<BackendDAE::NBackendDAE> = bdae;
    bdae = ({
        let mut ode: DoubleEnded::MutableList<Arc<Partition::Partition::Partition>> = DoubleEnded::fromList(metamodelica::nil())?;
        let mut alg: DoubleEnded::MutableList<Arc<Partition::Partition::Partition>> = DoubleEnded::fromList(metamodelica::nil())?;
        let mut ode_evt: DoubleEnded::MutableList<Arc<Partition::Partition::Partition>> = DoubleEnded::fromList(metamodelica::nil())?;
        let mut alg_evt: DoubleEnded::MutableList<Arc<Partition::Partition::Partition>> = DoubleEnded::fromList(metamodelica::nil())?;
        let mut clocked: DoubleEnded::MutableList<Arc<Partition::Partition::Partition>> = DoubleEnded::fromList(metamodelica::nil())?;
        (::match_deref::match_deref! { match &(bdae.clone()) {
        Deref @ BackendDAE::MAIN { .. } => {
            for mut syst in &*var_field!((*bdae).ode, BackendDAE::NBackendDAE::MAIN).clone() {
                let mut syst = syst.clone();
                Partition::Partition::categorize(syst.clone(), ode.clone(), alg.clone(), ode_evt.clone(), alg_evt.clone(), clocked.clone())?;
            }
            assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN;
                ode = DoubleEnded::toListAndClear(ode.clone(), metamodelica::nil())?,
                algebraic = DoubleEnded::toListAndClear(alg.clone(), metamodelica::nil())?,
                ode_event = DoubleEnded::toListAndClear(ode_evt.clone(), metamodelica::nil())?,
                alg_event = DoubleEnded::toListAndClear(alg_evt.clone(), metamodelica::nil())?,
                clocked = DoubleEnded::toListAndClear(clocked.clone(), metamodelica::nil())?
            );
            bdae
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.categorize")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
    Ok(bdae)
}

pub(crate) fn extractClocksEqn(mut eqn: Arc<Equation::Equation>, mut clck_coll: Arc<UnorderedMap::UnorderedMap<Arc<BClock::BClock>, Arc<ComponentRef::NFComponentRef>>>, mut infr_coll: Arc<UnorderedMap::UnorderedMap<Arc<BClock::BClock>, Arc<ComponentRef::NFComponentRef>>>, mut new_clocks: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>, mut new_infers: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>, mut idx: Pointer::Pointer<i32>) -> Result<Arc<Equation::Equation>> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    eqn = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BEquation::Equation::WHEN_EQUATION { .. } => {
            assign_variant_field!(eqn => Equation::Equation::WHEN_EQUATION; body = Util::getOption(extractClocksWhenCond(Some(var_field!((*eqn).body, Equation::Equation::WHEN_EQUATION).clone()), clck_coll.clone(), infr_coll.clone(), new_clocks.clone(), new_infers.clone(), idx.clone())?)?);
            eqn
        },
        _ => eqn,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    eqn = BEquation::Equation::map(eqn, (std::sync::Arc::new({ let __pe_b1 = clck_coll; let __pe_b2 = infr_coll; let __pe_b3 = new_clocks; let __pe_b4 = new_infers; let __pe_b5 = idx; let __pe_b6 = false; move |__pe_a0| extractClocks(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    Ok(eqn)
}

pub(crate) fn extractClocksWhenCond(mut body_opt: Option<Arc<WhenEquationBody::WhenEquationBody>>, mut clck_coll: Arc<UnorderedMap::UnorderedMap<Arc<BClock::BClock>, Arc<ComponentRef::NFComponentRef>>>, mut infr_coll: Arc<UnorderedMap::UnorderedMap<Arc<BClock::BClock>, Arc<ComponentRef::NFComponentRef>>>, mut new_clocks: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>, mut new_infers: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>, mut idx: Pointer::Pointer<i32>) -> Result<Option<Arc<WhenEquationBody::WhenEquationBody>>> {
    let mut body_opt: Option<Arc<WhenEquationBody::WhenEquationBody>> = body_opt;
    body_opt = (::match_deref::match_deref! { match &(body_opt.clone()) {
        Some(body) => {
            let mut body = (*body).clone();
            assign_field!(
                body.condition = Expression::map(body.condition.clone(), (std::sync::Arc::new({ let __pe_b1 = clck_coll.clone(); let __pe_b2 = infr_coll.clone(); let __pe_b3 = new_clocks.clone(); let __pe_b4 = new_infers.clone(); let __pe_b5 = idx.clone(); let __pe_b6 = true; move |__pe_a0| extractClocks(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?,
                body.else_when = extractClocksWhenCond(body.else_when.clone(), clck_coll, infr_coll, new_clocks, new_infers, idx)?
            );
            Some(body.clone())
        },
        _ => {
            body_opt
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(body_opt)
}

pub(crate) fn extractClocks(mut exp: Arc<Expression::NFExpression>, mut clck_coll: Arc<UnorderedMap::UnorderedMap<Arc<BClock::BClock>, Arc<ComponentRef::NFComponentRef>>>, mut infr_coll: Arc<UnorderedMap::UnorderedMap<Arc<BClock::BClock>, Arc<ComponentRef::NFComponentRef>>>, mut new_clocks: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>, mut new_infers: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>, mut idx: Pointer::Pointer<i32>, mut when_cond: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CLKCONST { .. } if (when_cond || !(ClockKind::isInferred(var_field!((*exp).clk, Expression::NFExpression::CLKCONST).clone()))) => {
            let mut clock: Arc<BClock::BClock>;
            let mut clock_var: Pointer::Pointer<Arc<Variable::NFVariable>>;
            let mut clock_name: Arc<ComponentRef::NFComponentRef>;
            clock = Arc::new(BClock::BClock::BASE_CLOCK { clock: var_field!((*exp).clk, Expression::NFExpression::CLKCONST).clone() });
            if UnorderedMap::contains(clock.clone(), clck_coll.clone())? {
                clock_name = UnorderedMap::getSafe(clock.clone(), clck_coll, metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo"))?;
            } else if UnorderedMap::contains(clock.clone(), infr_coll.clone())? {
                clock_name = UnorderedMap::getSafe(clock.clone(), infr_coll, metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo"))?;
            } else {
                (clock_var, clock_name) = BVariable::makeClockVar(Pointer::access(idx.clone()), Expression::typeOf(exp.clone()))?;
                if BClock::isInferredClock(clock.clone()) {
                    UnorderedMap::add(clock.clone(), clock_name.clone(), infr_coll)?;
                    Pointer::update(new_infers.clone(), metamodelica::cons(clock_var.clone(), Pointer::access(new_infers)));
                } else {
                    UnorderedMap::add(clock.clone(), clock_name.clone(), clck_coll)?;
                    Pointer::update(new_clocks.clone(), metamodelica::cons(clock_var.clone(), Pointer::access(new_clocks)));
                }
                Pointer::update(idx.clone(), Pointer::access(idx) + 1);
            }
            Expression::fromCref(clock_name.clone(), false)?
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub(crate) enum ClusterElementType {
    EQUATION = 1,
    VARIABLE = 2,
}
impl PartialOrd for ClusterElementType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for ClusterElementType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}
impl metamodelica::gc::MMTrace for ClusterElementType {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> { Ok(()) }
}

pub mod Cluster {
    use super::*;
#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct Cluster {
        /// set of all variables in this cluster
        pub variables: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>,
        /// set of all equations in this cluster
        pub eqn_idnts: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>,
    }

    impl metamodelica::gc::MMTrace for Cluster {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.variables, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.eqn_idnts, __mmv)?;
            Ok(())
        }
    }
    impl Default for Cluster {
        fn default() -> Self {
            Self {
                variables: Default::default(),
                eqn_idnts: Default::default(),
            }
        }
    }

    pub type CLUSTER = Cluster;

    pub(crate) fn toString(mut cluster: Arc<Cluster>) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("### Cluster Variables:\n")); __mm_s.push_str(&*UnorderedSet::toString(cluster.variables.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("\n")).clone())?); __mm_s.push_str(&*literal!("\n### Cluster Equation Identifiers:\n")); __mm_s.push_str(&*UnorderedSet::toString(cluster.eqn_idnts.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("\n")).clone())?); ArcStr::from(__mm_s) }).clone();
        Ok(r#str)
    }

    pub(crate) fn addElement(mut cluster_opt: Option<Arc<Cluster>>, mut cref: Arc<ComponentRef::NFComponentRef>, mut ty: ClusterElementType) -> Result<Arc<Cluster>> {
        let mut cluster: Arc<Cluster> = Arc::new(<Cluster as ::std::default::Default>::default());
        cluster = (::match_deref::match_deref! { match &(cluster_opt) {
        Some(__esc_cluster) => {
            cluster = (*__esc_cluster).clone();
            cluster.clone()
        },
        _ => Arc::new(Cluster { variables: UnorderedSet::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13), eqn_idnts: UnorderedSet::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13) }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        cluster = (match ty {
        ClusterElementType::VARIABLE => {
            UnorderedSet::add(cref, cluster.variables.clone())?;
            cluster
        },
        ClusterElementType::EQUATION { .. } => {
            UnorderedSet::add(cref, cluster.eqn_idnts.clone())?;
            cluster
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.Cluster.addElement")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*ComponentRef::toString(cref)?); __mm_s.push_str(&*literal!(" because of unknown cluster element type.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
    });
        Ok(cluster)
    }

    pub(crate) fn addToClockMap(mut cluster: Arc<Cluster>, mut equations: Arc<EquationPointers::EquationPointers>, mut info: Arc<ClockedInfo::ClockedInfo>, mut clock_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>) -> Result<()> {
        fn findClock(mut exp: Arc<Expression::NFExpression>, mut info: Arc<ClockedInfo::ClockedInfo>, mut clock_ptr: Pointer::Pointer<Option<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<Expression::NFExpression>> {
            let mut exp: Arc<Expression::NFExpression> = exp;
            let mut clock_opt: Option<Arc<ComponentRef::NFComponentRef>> = Pointer::access(clock_ptr.clone());
            exp = (::match_deref::match_deref! { match &((exp.clone(), clock_opt)) {
        (_, Some(_)) => exp.clone(),
        (Deref @ Expression::CREF { .. }, None) if (BVariable::isClockOrClocked(BVariable::getVarPointer(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo"))?)) => {
            Pointer::update(clock_ptr, Some(var_field!((*exp).cref, Expression::NFExpression::CREF).clone()));
            exp.clone()
        },
        (Deref @ Expression::CALL { .. }, _) if (Expression::isClockOrSampleFunction(exp.clone())?) => exp.clone(),
        _ => Expression::mapShallow(exp.clone(), (std::sync::Arc::new({ let __pe_b1 = info; let __pe_b2 = clock_ptr; move |__pe_a0| findClock(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            Ok(exp)
        }

        let mut clock_ptr: Pointer::Pointer<Option<Arc<ComponentRef::NFComponentRef>>> = Pointer::create(None);
        let mut clock_opt: Option<Arc<ComponentRef::NFComponentRef>> = None;
        let mut clock: Arc<ComponentRef::NFComponentRef>;
        for mut eqn_name in &*UnorderedSet::toList(cluster.eqn_idnts.clone()) {
            let mut eqn_name = eqn_name.clone();
            BEquation::Equation::map(Pointer::access(BEquation::EquationPointers::getEqnByName(equations.clone(), eqn_name.clone())?), (std::sync::Arc::new({ let __pe_b1 = info.clone(); let __pe_b2 = clock_ptr.clone(); move |__pe_a0| findClock(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>), None, (std::sync::Arc::new(Expression::fakeMap) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            clock_opt = Pointer::access(clock_ptr.clone());
            if isSome(clock_opt.clone()) {
                break;
            }
        }
        if isSome(clock_opt.clone()) {
            let __pa0 = ::match_deref::match_deref! { match &(clock_opt) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            clock = __pa0.clone();
            for mut var_name in &*UnorderedSet::toList(cluster.variables.clone()) {
                let mut var_name = var_name.clone();
                UnorderedMap::add(var_name.clone(), clock.clone(), clock_map.clone())?;
            }
        }
        Ok(())
    }

    pub(crate) fn toPartition(mut cluster: Arc<Cluster>, mut variables: Arc<VariablePointers::VariablePointers>, mut equations: Arc<EquationPointers::EquationPointers>, mut kind: Partition::Kind, mut info: Arc<ClockedInfo::ClockedInfo>, mut held_crefs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut infer_del: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<Partition::Partition::Partition>> {
        let mut partition: Arc<Partition::Partition::Partition>;
        let mut cvars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = UnorderedSet::toList(cluster.variables.clone());
        let mut cidnt: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = UnorderedSet::toList(cluster.eqn_idnts.clone());
        let mut association: Arc<Partition::Association::Association>;
        let mut var_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
        let mut filtered_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
        let mut eqn_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>;
        let mut partVariables: Arc<VariablePointers::VariablePointers>;
        let mut partEquations: Arc<EquationPointers::EquationPointers>;
        let mut inferred_clocks: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = UnorderedSet::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
        var_lst = ({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut cref in (cvars).into_iter().cloned() {
            let __x = BVariable::getVarPointer(cref.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo"))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        filtered_vars = ({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut var in (var_lst).into_iter().cloned() {
            if !(BVariable::VariablePointers::contains(var.clone(), variables.clone())?) { continue; }
            let __x = var.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        eqn_lst = ({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
        for mut name in (cidnt).into_iter().cloned() {
            let __x = BEquation::EquationPointers::getEqnByName(equations.clone(), name.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        partVariables = BVariable::VariablePointers::fromList(filtered_vars, false)?;
        partEquations = BEquation::EquationPointers::fromList(eqn_lst)?;
        association = Partition::Association::create(partEquations.clone(), kind, info, infer_del.clone())?;
        partEquations = BEquation::EquationPointers::mapExp(partEquations, (std::sync::Arc::new({ let __pe_b1 = held_crefs; move |__pe_a0| replaceClockedFunctions(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
        if Partition::Association::isClocked(association.clone()) {
            partVariables = BVariable::VariablePointers::mapRemovePtr(partVariables, (std::sync::Arc::new({ let __pe_b1 = inferred_clocks.clone(); move |__pe_a0| collectInferredClock(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>))?;
            partEquations = BEquation::EquationPointers::mapRemovePtr(partEquations, (std::sync::Arc::new({ let __pe_b1 = inferred_clocks.clone(); move |__pe_a0| removeInferredClock(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>) -> Result<bool> + 'static>))?;
            partEquations = BEquation::EquationPointers::map(partEquations, (std::sync::Arc::new(replaceClockedWhen) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::Equation>) -> Result<Arc<Equation::Equation>> + 'static>))?;
            partVariables = BVariable::VariablePointers::mapPtr(partVariables, (std::sync::Arc::new({ let __pe_b1 = openmodelica_nf_frontend::NFBackendExtension::VariableKind::interned_CLOCKED(); move |__pe_a0| Ok(BVariable::setVarKind(__pe_a0, __pe_b1.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<()> + 'static>))?;
            if BEquation::EquationPointers::size(partEquations.clone()) == 0 {
                UnorderedSet::merge(infer_del, inferred_clocks)?;
            }
        }
        partition = Arc::new(Partition::Partition::Partition { index: 0, association: association, unknowns: partVariables, daeUnknowns: None, equations: partEquations, adjacencyMatrix: None, matching: None, strongComponents: None });
        Ok(partition)
    }

    fn collectInferredClock(mut var: Pointer::Pointer<Arc<Variable::NFVariable>>, mut inferred_clocks: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<bool> {
        let mut delete: bool = BVariable::isClock(var.clone());
        if delete {
            UnorderedSet::add(BVariable::getVarName(var), inferred_clocks)?;
        }
        Ok(delete)
    }

    fn removeInferredClock(mut eqn: Pointer::Pointer<Arc<Equation::Equation>>, mut inferred_clocks: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<bool> {
        let mut delete: bool;
        delete = (::match_deref::match_deref! { match &(Pointer::access(eqn)) {
        Deref @ BEquation::Equation::SCALAR_EQUATION { lhs: Deref @ Expression::CREF { cref: lhs, .. }, .. } => {
            UnorderedSet::contains(lhs.clone(), inferred_clocks)?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(delete)
    }

}

// Perhaps this deserves its own place in Util/*.mo
pub mod DisjointSetForest {
    use super::*;
    /// Custom implementation of disjoint-set data structure with constant number of elements.
#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct DisjointSetForest {
        pub parent: Pointer::Pointer<metamodelica::Array<i32>>,
        pub rank: Pointer::Pointer<metamodelica::Array<i32>>,
    }

    impl metamodelica::gc::MMTrace for DisjointSetForest {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.parent, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.rank, __mmv)?;
            Ok(())
        }
    }
    impl Default for DisjointSetForest {
        fn default() -> Self {
            Self {
                parent: Default::default(),
                rank: Default::default(),
            }
        }
    }

    pub type FOREST = DisjointSetForest;

    pub(crate) fn new(mut n: i32) -> Arc<DisjointSetForest> {
        let mut dsf: Arc<DisjointSetForest>;
        dsf = Arc::new(DisjointSetForest { parent: Pointer::create(metamodelica::arrayFromVec(({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (1..=n).into_iter() {
            let __x = i.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }).into_iter().cloned().collect())), rank: Pointer::create(arrayCreate(n, 0)) });
        dsf
    }

    pub(crate) fn find(mut dsf: Arc<DisjointSetForest>, mut index: i32) -> i32 {
        let mut index: i32 = index;
        let mut parent: metamodelica::Array<i32> = Pointer::access(dsf.parent.clone());
        while index != ({let __elt = parent.borrow()[(index-1) as usize].clone(); __elt}) {
            {
                let __cell0 = ({let __elt = parent.borrow()[(({let __elt = parent.borrow()[(index-1) as usize].clone(); __elt})-1) as usize].clone(); __elt});
                let __idx0 = index;
                parent.clone().borrow_mut()[(__idx0-1) as usize] = __cell0;
            }
            index = ({let __elt = parent.borrow()[(index-1) as usize].clone(); __elt});
        }
        Pointer::update(dsf.parent.clone(), parent.clone());
        index
    }

    pub(crate) fn unite(mut dsf: Arc<DisjointSetForest>, mut indices: Arc<metamodelica::List<i32>>) -> Result<i32> {
        let mut root: i32;
        let mut roots: Arc<metamodelica::List<i32>> = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (indices.clone()).into_iter().cloned() {
            let __x = find(dsf.clone(), i.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        let mut parent: metamodelica::Array<i32> = Pointer::access(dsf.parent.clone());
        let mut rank: metamodelica::Array<i32> = Pointer::access(dsf.rank.clone());
        let mut maxRank: i32;
        let mut tied: bool = false;
        root = listHead(roots.clone())?;
        maxRank = ({let __elt = rank.borrow()[(root-1) as usize].clone(); __elt});
        for mut r in &*listRest(roots.clone())? {
            let mut r = r.clone();
            if r.clone() != root {
                if ({let __elt = rank.borrow()[(r.clone()-1) as usize].clone(); __elt}) > maxRank {
                    root = r.clone();
                    maxRank = ({let __elt = rank.borrow()[(root-1) as usize].clone(); __elt});
                    tied = false;
                } else if ({let __elt = rank.borrow()[(r.clone()-1) as usize].clone(); __elt}) == maxRank {
                    tied = true;
                }
            }
        }
        for mut r in &*roots {
            let mut r = r.clone();
            {
                let __cell0 = root;
                let __idx0 = find(dsf.clone(), r.clone());
                parent.clone().borrow_mut()[(__idx0-1) as usize] = __cell0;
            }
        }
        if tied {
            {
                let __cell1 = ({let __elt = rank.borrow()[(root-1) as usize].clone(); __elt}) + 1;
                let __idx1 = root;
                rank.clone().borrow_mut()[(__idx1-1) as usize] = __cell1;
            }
        }
        Pointer::update(dsf.parent.clone(), parent.clone());
        Pointer::update(dsf.rank.clone(), rank.clone());
        Ok(root)
    }

}

fn partitioningNone(mut kind: Partition::Kind, mut variables: Arc<VariablePointers::VariablePointers>, mut equations: Arc<EquationPointers::EquationPointers>, mut clocks: Arc<VariablePointers::VariablePointers>, mut clocked: Arc<EquationPointers::EquationPointers>, mut info: Arc<ClockedInfo::ClockedInfo>) -> Result<Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>> {
    let mut partitions: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>;
    let mut clone_vars: Arc<VariablePointers::VariablePointers>;
    let mut clone_eqns: Arc<EquationPointers::EquationPointers>;
    clone_vars = BVariable::VariablePointers::clone(variables, true)?;
    clone_eqns = BEquation::EquationPointers::clone(equations, true)?;
    partitions = list![Arc::new(Partition::Partition::Partition { index: 1, association: Arc::new(Partition::Association::Association::CONTINUOUS { kind: kind, jacobian: None, jacobianAdjoint: None, LFG_jacobian: None, MRF_jacobian: None, R0_jacobian: None }), unknowns: clone_vars, daeUnknowns: None, equations: clone_eqns, adjacencyMatrix: None, matching: None, strongComponents: None })];
    Ok(partitions)
}

fn partitioningClocked(mut kind: Partition::Kind, mut variables: Arc<VariablePointers::VariablePointers>, mut equations: Arc<EquationPointers::EquationPointers>, mut clocks: Arc<VariablePointers::VariablePointers>, mut clocked: Arc<EquationPointers::EquationPointers>, mut info: Arc<ClockedInfo::ClockedInfo>) -> Result<Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>> {
    let mut partitions: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>;
    let mut eqn_dsf: Arc<DisjointSetForest::DisjointSetForest> = DisjointSetForest::new(ExpandableArray::getLastUsedIndex(equations.eqArr.clone()));
    let mut var_map: metamodelica::Array<i32> = arrayCreate(ExpandableArray::getLastUsedIndex(variables.varArr.clone()), -1);
    let mut eqn: Pointer::Pointer<Arc<Equation::Equation>>;
    let mut var: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut var_crefs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
    let mut var_indices: Arc<metamodelica::List<i32>>;
    let mut part_idx: i32;
    let mut cluster_map: Arc<UnorderedMap::UnorderedMap<i32, Arc<Cluster::Cluster>>> = UnorderedMap::new(std::sync::Arc::new(fnptr!(Util::id, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 1);
    let mut name_cref: Arc<ComponentRef::NFComponentRef>;
    let mut marked_vars: metamodelica::Array<bool>;
    let mut single_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    let mut held_crefs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = UnorderedSet::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
    let mut clock_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>> = UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
    let mut infer_del: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = UnorderedSet::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
    let mut index: Pointer::Pointer<i32> = Pointer::create(1);
    for mut eq_idx in &*UnorderedMap::valueList(clocked.map.clone()) {
        let mut eq_idx = eq_idx.clone();
        if eq_idx.clone() > 0 {
            eqn = BEquation::EquationPointers::getEqnAt(clocked.clone(), eq_idx.clone())?;
            BClock::add(Pointer::access(eqn.clone()), info.clone())?;
        }
    }
    for mut eq_idx in &*UnorderedMap::valueList(equations.map.clone()) {
        let mut eq_idx = eq_idx.clone();
        if eq_idx.clone() > 0 {
            eqn = BEquation::EquationPointers::getEqnAt(equations.clone(), eq_idx.clone())?;
            BClock::add(Pointer::access(eqn.clone()), info.clone())?;
            var_crefs = UnorderedSet::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
            BEquation::Equation::map(Pointer::access(eqn.clone()), (std::sync::Arc::new({ let __pe_b1 = var_crefs.clone(); move |__pe_a0| collectPartitioningCrefs(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>), None, (std::sync::Arc::new(Expression::fakeMap) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            var_indices = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut cref in (UnorderedSet::toList(var_crefs.clone())).into_iter().cloned() {
            let __x = BVariable::VariablePointers::getVarIndex(variables.clone(), cref.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            var_indices = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (var_indices.clone()).into_iter().cloned() {
            if !(i.clone() > 0) { continue; }
            let __x = i.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            part_idx = DisjointSetForest::unite(eqn_dsf.clone(), metamodelica::cons(eq_idx.clone(), ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut j in (var_indices.clone()).into_iter().cloned() {
            if !(({let __elt = var_map.borrow()[(j.clone()-1) as usize].clone(); __elt}) > 0) { continue; }
            let __x = ({let __elt = var_map.borrow()[(j.clone()-1) as usize].clone(); __elt});
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })))?;
            for mut i in &*var_indices.clone() {
                let mut i = i.clone();
                {
                    let __cell0 = part_idx;
                    let __idx0 = i.clone();
                    var_map.clone().borrow_mut()[(__idx0-1) as usize] = __cell0;
                }
            }
        }
    }
    marked_vars = metamodelica::arrayFromVec(({
        let mut __acc: Arc<metamodelica::List<bool>> = metamodelica::nil();
        for mut var_idx in (UnorderedMap::valueList(variables.map.clone())).into_iter().cloned() {
            let __x = ({let __elt = var_map.borrow()[(var_idx.clone()-1) as usize].clone(); __elt}) < 0;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }).into_iter().cloned().collect());
    single_vars = ({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut var_ptr in (BVariable::VariablePointers::getMarkedVars(variables.clone(), marked_vars.clone())?).into_iter().cloned() {
            let __x = var_ptr.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    if !(single_vars.clone().is_empty()) {
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.partitioningClocked")); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*Partition::Partition::kindToString(kind)?); __mm_s.push_str(&*literal!(") failed because the following variables could not be assigned to a partition:\n  {")); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut var_ptr in (single_vars).into_iter().cloned() {
            let __x = BVariable::toString(Pointer::access(var_ptr.clone()), (literal!("")).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone()])?;
        bail!("fail");
    }
    for mut eq_idx in &*UnorderedMap::valueList(equations.map.clone()) {
        let mut eq_idx = eq_idx.clone();
        if eq_idx.clone() > 0 {
            eqn = BEquation::EquationPointers::getEqnAt(equations.clone(), eq_idx.clone())?;
            name_cref = BEquation::Equation::getEqnName(eqn.clone())?;
            part_idx = DisjointSetForest::find(eqn_dsf.clone(), eq_idx.clone());
            UnorderedMap::addUpdate(part_idx, (std::sync::Arc::new({ let __pe_b1 = name_cref.clone(); let __pe_b2 = ClusterElementType::EQUATION.clone(); move |__pe_a0| Cluster::addElement(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<Cluster::Cluster>>) -> Result<Arc<Cluster::Cluster>> + 'static>), cluster_map.clone())?;
        }
    }
    for mut var_idx in &*UnorderedMap::valueList(variables.map.clone()) {
        let mut var_idx = var_idx.clone();
        if var_idx.clone() > 0 {
            var = BVariable::VariablePointers::getVarAt(variables.clone(), var_idx.clone())?;
            name_cref = BVariable::getVarName(var.clone());
            part_idx = DisjointSetForest::find(eqn_dsf.clone(), ({let __elt = var_map.borrow()[(var_idx.clone()-1) as usize].clone(); __elt}));
            UnorderedMap::addUpdate(part_idx, (std::sync::Arc::new({ let __pe_b1 = name_cref.clone(); let __pe_b2 = ClusterElementType::VARIABLE.clone(); move |__pe_a0| Cluster::addElement(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<Cluster::Cluster>>) -> Result<Arc<Cluster::Cluster>> + 'static>), cluster_map.clone())?;
        }
    }
    for mut cluster in &*UnorderedMap::valueList(cluster_map.clone()) {
        let mut cluster = cluster.clone();
        Cluster::addToClockMap(cluster.clone(), equations.clone(), info.clone(), clock_map.clone())?;
    }
    ClockedInfo::resolveSubClocks(info.clone(), clock_map)?;
    partitions = ({
        let mut __acc: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>> = metamodelica::nil();
        for mut cl in (UnorderedMap::valueList(cluster_map)).into_iter().cloned() {
            let __x = Cluster::toPartition(cl.clone(), variables.clone(), equations.clone(), kind, info.clone(), held_crefs.clone(), infer_del.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    partitions = ({
        let mut __acc: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>> = metamodelica::nil();
        for mut part in (partitions).into_iter().cloned() {
            let __x = Partition::Partition::updateHeldVars(part.clone(), held_crefs.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    partitions = ({
        let mut __acc: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>> = metamodelica::nil();
        for mut partition in (sortAndMergeClockedPartitions(partitions, info.clone())?).into_iter().cloned() {
            if !(!(Partition::Partition::isEmpty(partition.clone())?)) { continue; }
            let __x = Partition::Partition::setIndex(partition.clone(), index.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    for mut unused_infer in &*UnorderedSet::toList(infer_del) {
        let mut unused_infer = unused_infer.clone();
        UnorderedMap::remove(unused_infer.clone(), info.baseClocks.clone())?;
        UnorderedMap::remove(unused_infer.clone(), info.baseToSub.clone())?;
    }
    if Flags::isSet(Flags::DUMP_SYNCHRONOUS.clone())? {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_1((literal!("[dumpSynchronous] Partitioning result:")).clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*List::toString(partitions.clone(), (std::sync::Arc::new({ let __pe_b1 = 2; move |__pe_a0| Partition::Partition::toString(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Partition::Partition::Partition>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("")).clone(), (literal!("\n")).clone(), (literal!("\n")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone());
        metamodelica::print((ClockedInfo::toString(info)?).clone());
    }
    Ok(partitions)
}

fn sortAndMergeClockedPartitions(mut partitions: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>, mut info: Arc<ClockedInfo::ClockedInfo>) -> Result<Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>> {
    pub(crate) type SubMap = Arc<UnorderedMap::UnorderedMap<Arc<BClock::BClock>, Arc<Partition::Partition::Partition>>>;

    let mut partitions: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>> = partitions;
    let mut clocked_partitions: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>;
    let mut new_clocked: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>>> = metamodelica::nil();
    let mut clock_collector: Arc<UnorderedMap::UnorderedMap<Arc<BClock::BClock>, Arc<UnorderedMap::UnorderedMap<Arc<BClock::BClock>, Arc<Partition::Partition::Partition>>>>> = UnorderedMap::new((std::sync::Arc::new(BClock::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>) -> Result<i32> + 'static>), (std::sync::Arc::new(BClock::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>, Arc<BClock::BClock>) -> Result<bool> + 'static>), 1);
    let mut base_clock_inferrence: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<BClock::BClock>>> = UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
    let mut clock: Arc<BClock::BClock>;
    let mut baseClock: Arc<BClock::BClock> = Arc::new(<BClock::BClock as ::std::default::Default>::default());
    let mut subClock: Arc<BClock::BClock>;
    let mut baseClock_opt: Option<Arc<BClock::BClock>>;
    let mut subClockMap: Arc<UnorderedMap::UnorderedMap<Arc<BClock::BClock>, Arc<Partition::Partition::Partition>>>;
    let mut new_part: Arc<Partition::Partition::Partition> = Arc::new(<Partition::Partition::Partition as ::std::default::Default>::default());
    (clocked_partitions, partitions) = List::splitOnTrue(partitions, (std::sync::Arc::new(fnptr!(Partition::Partition::isClocked, Arc<Partition::Partition::Partition>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Partition::Partition::Partition>) -> Result<bool> + 'static>))?;
    for mut baseClock in &*UnorderedMap::valueList(info.baseClocks.clone()) {
        let mut baseClock = baseClock.clone();
        UnorderedMap::add(baseClock.clone(), UnorderedMap::new((std::sync::Arc::new(BClock::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>) -> Result<i32> + 'static>), (std::sync::Arc::new(BClock::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>, Arc<BClock::BClock>) -> Result<bool> + 'static>), 1), clock_collector.clone())?;
    }
    for mut partition in &*clocked_partitions.clone() {
        let mut partition = partition.clone();
        (clock, baseClock_opt, _) = Partition::Partition::getClocks(partition.clone())?;
        clock = (::match_deref::match_deref! { match &(baseClock_opt.clone()) {
        Some(__esc_clock) => {
            clock = (*__esc_clock).clone();
            clock.clone()
        },
        _ => clock.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        for mut var in &*BVariable::VariablePointers::toList(partition.unknowns.clone())? {
            let mut var = var.clone();
            UnorderedMap::add(BVariable::getVarName(var.clone()), clock.clone(), base_clock_inferrence.clone())?;
        }
    }
    for mut partition in &*clocked_partitions {
        let mut partition = partition.clone();
        (clock, baseClock_opt, _) = Partition::Partition::getClocks(partition.clone())?;
        if isSome(baseClock_opt.clone()) {
            let __pa0 = ::match_deref::match_deref! { match &(baseClock_opt.clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            baseClock = __pa0.clone();
            baseClock = BClock::baseClockInferrence(baseClock.clone(), base_clock_inferrence.clone())?;
            subClock = clock.clone();
        } else {
            baseClock = BClock::baseClockInferrence(clock.clone(), base_clock_inferrence.clone())?;
            subClock = DEFAULT_SUB_CLOCK().clone();
        }
        partition = Partition::Partition::setClocks(partition.clone(), subClock.clone(), Some(baseClock.clone()))?;
        subClockMap = UnorderedMap::getSafe(baseClock.clone(), clock_collector.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo"))?;
        new_part = (::match_deref::match_deref! { match &(UnorderedMap::get(subClock.clone(), subClockMap.clone())?) {
        Some(__esc_new_part) => {
            new_part = (*__esc_new_part).clone();
            Partition::Partition::merge(new_part.clone(), partition.clone(), true)?
        },
        _ => partition.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        UnorderedMap::add(subClock.clone(), new_part.clone(), subClockMap.clone())?;
    }
    for mut tpl in &*UnorderedMap::toList(clock_collector) {
        let mut tpl = tpl.clone();
        (baseClock, subClockMap) = tpl.clone();
        new_clocked = metamodelica::cons(sortClockedPartitions(UnorderedMap::valueList(subClockMap.clone()))?, new_clocked.clone());
    }
    partitions = ({
        let mut __acc: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>> = metamodelica::nil();
        for mut partition in (metamodelica::cons(partitions, new_clocked).reverse()).into_iter().cloned() {
            let __x = partition.clone();
            __acc = __x.append(&__acc);
        }
        __acc
    });
    Ok(partitions)
}

fn sortClockedPartitions(mut unsorted: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>) -> Result<Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>> {
    let mut sorted: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>> = metamodelica::nil();
    let mut n: i32 = (unsorted.clone().len() as i32);
    let mut partitions: metamodelica::Array<Arc<Partition::Partition::Partition>> = metamodelica::arrayFromVec(unsorted.clone().reverse().into_iter().cloned().collect());
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>> = arrayCreate(n, metamodelica::nil());
    let mut matching: Arc<Matching::NBMatching> = Matching::trivial(n);
    let mut index_map: Arc<UnorderedMap::UnorderedMap<Arc<BClock::BClock>, i32>> = UnorderedMap::new((std::sync::Arc::new(BClock::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>) -> Result<i32> + 'static>), (std::sync::Arc::new(BClock::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>, Arc<BClock::BClock>) -> Result<bool> + 'static>), 1);
    let mut partition_order: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut j: i32 = 0;
    for mut i in 1..=n {
        UnorderedMap::add((Partition::Partition::getClocks(({let __elt = partitions.borrow()[(i.clone()-1) as usize].clone(); __elt}))?).0, i.clone(), index_map.clone())?;
    }
    for mut i in 1..=n {
        let __range0 = &*UnorderedSet::toList(Partition::Partition::getClockDependencies(({let __elt = partitions.borrow()[(i.clone()-1) as usize].clone(); __elt}))?);
        for mut clock in __range0 {
            let mut clock = clock.clone();
            j = UnorderedMap::getSafe(clock.clone(), index_map.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo"))?;
            {
                let __cell1 = metamodelica::cons(j, ({let __elt = m.borrow()[(i.clone()-1) as usize].clone(); __elt}));
                let __idx1 = i.clone();
                m.clone().borrow_mut()[(__idx1-1) as usize] = __cell1;
            }
        }
    }
    partition_order = Sorting::tarjanScalar(m.clone(), matching)?;
    for mut comp in &*partition_order.reverse() {
        let mut comp = comp.clone();
        sorted = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ metamodelica::List::Cons { head: __esc_j, tail: Deref @ metamodelica::List::Nil } => {
            j = (*__esc_j).clone();
            metamodelica::cons(({let __elt = partitions.borrow()[(j.clone()-1) as usize].clone(); __elt}), sorted.clone())
        },
        _ => {
            let mut var_clock_map: Arc<UnorderedMap::UnorderedMap<Pointer::Pointer<Arc<Variable::NFVariable>>, Arc<BClock::BClock>>>;
            let mut part: Arc<Partition::Partition::Partition>;
            let mut sub_comps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>>;
            let mut sub_comp_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
            let mut sub_comp_eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>;
            let mut collector: Option<(Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>, Arc<BClock::BClock>)>;
            let mut var_clocks: Arc<UnorderedSet::UnorderedSet<Arc<BClock::BClock>>>;
            let mut baseClock: Option<Arc<BClock::BClock>>;
            let mut vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
            let mut eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
            let mut clock: Arc<BClock::BClock> = Arc::new(<BClock::BClock as ::std::default::Default>::default());
            let mut new_clock: Arc<BClock::BClock> = Arc::new(<BClock::BClock as ::std::default::Default>::default());
            var_clock_map = UnorderedMap::new((std::sync::Arc::new(BVariable::hash) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<i32> + 'static>), (std::sync::Arc::new(BVariable::equalName) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>, Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), 1);
            for mut i in &*comp.clone() {
                let mut i = i.clone();
                part = ({let __elt = partitions.borrow()[(i.clone()-1) as usize].clone(); __elt});
                for mut var in &*BVariable::VariablePointers::toList(part.unknowns.clone())? {
                    let mut var = var.clone();
                    UnorderedMap::add(var.clone(), (Partition::Partition::getClocks(part.clone())?).0, var_clock_map.clone())?;
                }
            }
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(comp.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            j = __pa0.clone();
            comp = __pa1.clone();
            part = ({let __elt = partitions.borrow()[(j-1) as usize].clone(); __elt});
            for mut i in &*comp.clone() {
                let mut i = i.clone();
                part = Partition::Partition::merge(part.clone(), ({let __elt = partitions.borrow()[(i.clone()-1) as usize].clone(); __elt}), false)?;
            }
            (_, baseClock, _) = Partition::Partition::getClocks(part.clone())?;
            (_, sub_comps) = Causalize::simple(part.unknowns.clone(), part.equations.clone(), Partition::Partition::getKind(part.clone()), NBAdjacency::MatrixStrictness::MATCHING.clone(), crate::NBEquation::Iterator::interned_EMPTY())?;
            collector = None;
            for mut sub_comp in &*sub_comps.clone().reverse() {
                let mut sub_comp = sub_comp.clone();
                sub_comp_vars = StrongComponent::getVariables(sub_comp.clone())?;
                sub_comp_eqns = StrongComponent::getEquations(sub_comp.clone())?;
                var_clocks = UnorderedSet::new((std::sync::Arc::new(BClock::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>) -> Result<i32> + 'static>), (std::sync::Arc::new(BClock::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>, Arc<BClock::BClock>) -> Result<bool> + 'static>), 13);
                for mut var in &*sub_comp_vars.clone() {
                    let mut var = var.clone();
                    UnorderedSet::add(UnorderedMap::getSafe(var.clone(), var_clock_map.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo"))?, var_clocks.clone())?;
                }
                collector = (::match_deref::match_deref! { match &((collector.clone(), UnorderedSet::toList(var_clocks.clone()))) {
        (None, Deref @ metamodelica::List::Cons { head: __esc_new_clock, tail: Deref @ metamodelica::List::Nil }) => {
            new_clock = (*__esc_new_clock).clone();
            Some((sub_comp_vars.clone(), sub_comp_eqns.clone(), new_clock.clone()))
        },
        (Some((__esc_vars, __esc_eqns, __esc_clock)), Deref @ metamodelica::List::Cons { head: __esc_new_clock, tail: Deref @ metamodelica::List::Nil }) => {
            vars = (*__esc_vars).clone();
            eqns = (*__esc_eqns).clone();
            clock = (*__esc_clock).clone();
            new_clock = (*__esc_new_clock).clone();
            if BClock::isEqual(clock.clone(), new_clock.clone())? {
                collector = Some((listAppend(sub_comp_vars.clone(), vars.clone()), listAppend(sub_comp_eqns.clone(), eqns.clone()), clock.clone()));
            } else {
                part = Arc::new(Partition::Partition::Partition { index: 0, association: Arc::new(Partition::Association::Association::CLOCKED { clock: clock.clone(), baseClock: baseClock.clone(), clock_deps: UnorderedSet::new((std::sync::Arc::new(BClock::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>) -> Result<i32> + 'static>), (std::sync::Arc::new(BClock::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>, Arc<BClock::BClock>) -> Result<bool> + 'static>), 13), holdEvents: false }), unknowns: BVariable::VariablePointers::fromList(vars.clone(), false)?, daeUnknowns: None, equations: BEquation::EquationPointers::fromList(eqns.clone())?, adjacencyMatrix: None, matching: None, strongComponents: None });
                sorted = metamodelica::cons(part.clone(), sorted.clone());
                collector = Some((sub_comp_vars.clone(), sub_comp_eqns.clone(), new_clock.clone()));
            }
            collector.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.sortClockedPartitions")); __mm_s.push_str(&*literal!(" failed for sub-partitions with cyclic dependency that could not be resolved:\n")); __mm_s.push_str(&*literal!("There are contradicting sub-clocks: ")); __mm_s.push_str(&*List::toString(UnorderedSet::toList(var_clocks.clone()), (std::sync::Arc::new(BClock::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*literal!(" in strong component:\n")); __mm_s.push_str(&*StrongComponent::toString(sub_comp.clone(), -1)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            if isSome(collector.clone()) {
                let (__pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(collector.clone()) {
                    Some((__pa2, __pa3, __pa4)) => (__pa2.clone(), __pa3.clone(), __pa4.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                vars = __pa2.clone();
                eqns = __pa3.clone();
                clock = __pa4.clone();
                part = Arc::new(Partition::Partition::Partition { index: 0, association: Arc::new(Partition::Association::Association::CLOCKED { clock: clock.clone(), baseClock: baseClock.clone(), clock_deps: UnorderedSet::new((std::sync::Arc::new(BClock::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>) -> Result<i32> + 'static>), (std::sync::Arc::new(BClock::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>, Arc<BClock::BClock>) -> Result<bool> + 'static>), 13), holdEvents: false }), unknowns: BVariable::VariablePointers::fromList(vars.clone(), false)?, daeUnknowns: None, equations: BEquation::EquationPointers::fromList(eqns.clone())?, adjacencyMatrix: None, matching: None, strongComponents: None });
                sorted = metamodelica::cons(part.clone(), sorted.clone());
            }
            sorted.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(sorted)
}

fn collectPartitioningCrefs(mut exp: Arc<Expression::NFExpression>, mut var_crefs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } => {
            let mut newExp: Arc<Expression::NFExpression>;
            let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            newExp = (::match_deref::match_deref! { match &(AbsynUtil::pathString(Function::nameConsiderBuiltin(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL).clone())?, (literal!(".")).clone(), true, false)?) {
        Deref @ "subSample" => exp,
        Deref @ "superSample" => exp,
        Deref @ "shiftSample" => exp,
        Deref @ "backSample" => exp,
        Deref @ "previous" => exp,
        Deref @ "hold" => exp,
        Deref @ "sample" => {
            arg = (::match_deref::match_deref! { match &(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?) {
        Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: __esc_arg, tail: Deref @ metamodelica::List::Nil } } => {
            arg = (*__esc_arg).clone();
            arg.clone()
        },
        Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: __esc_arg, tail: Deref @ metamodelica::List::Nil } } } => {
            arg = (*__esc_arg).clone();
            arg.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.collectPartitioningCrefs")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp)?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            Expression::mapShallow(arg.clone(), (std::sync::Arc::new({ let __pe_b1 = var_crefs; move |__pe_a0| collectPartitioningCrefs(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?
        },
        _ => Expression::mapShallow(exp, (std::sync::Arc::new({ let __pe_b1 = var_crefs; move |__pe_a0| collectPartitioningCrefs(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            newExp.clone()
        },
        Deref @ Expression::CREF { .. } => {
            let mut children: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;
            let mut stripped: Arc<ComponentRef::NFComponentRef>;
            children = (::match_deref::match_deref! { match &(BVariable::getVar(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo"))?) {
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendInfo::BACKEND_INFO { varKind: Deref @ VariableKind::RECORD { children: children_vars, .. }, .. }, .. } => {
            ({
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut var in (children_vars.clone()).into_iter().cloned() {
            let __x = BVariable::getVarName(var.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
        },
        _ => {
            list![var_field!((*exp).cref, Expression::NFExpression::CREF).clone()]
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            for mut child in &*children.clone() {
                let mut child = child.clone();
                stripped = ComponentRef::stripSubscriptsAll(child.clone());
                if !(BVariable::checkCref(stripped.clone(), (std::sync::Arc::new(fnptr!(BVariable::isParamOrConst, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo"))?) {
                    addCrefToSet(stripped.clone(), var_crefs.clone())?;
                }
            }
            exp
        },
        _ => {
            Expression::mapShallow(exp, (std::sync::Arc::new({ let __pe_b1 = var_crefs; move |__pe_a0| collectPartitioningCrefs(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn addCrefToSet(mut cref: Arc<ComponentRef::NFComponentRef>, mut set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<()> {
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>> = BVariable::getVarPointer(cref.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo"))?;
    if BVariable::isState(var_ptr.clone()) {
        UnorderedSet::add(BVariable::getPartnerCref(cref, (std::sync::Arc::new(fnptr!(BVariable::getVarDer, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<(Option<Pointer::Pointer<Arc<Variable::NFVariable>>>, ArcStr)> + 'static>), false)?, set)?;
    } else if BVariable::isPrevious(var_ptr) {
        UnorderedSet::add(BVariable::getPartnerCref(cref, (std::sync::Arc::new(fnptr!(BVariable::getVarPre, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<(Option<Pointer::Pointer<Arc<Variable::NFVariable>>>, ArcStr)> + 'static>), false)?, set)?;
    } else {
        UnorderedSet::add(cref, set)?;
    }
    Ok(())
}

fn replaceClockedFunctions(mut exp: Arc<Expression::NFExpression>, mut held_crefs: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<Expression::NFExpression>> {
    pub(crate) fn replaceSample(mut exp: Arc<Expression::NFExpression>, mut call: Arc<Call::NFCall>, mut basic: bool) -> Result<Arc<Expression::NFExpression>> {
        let mut exp: Arc<Expression::NFExpression> = exp;
        let mut arg1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut arg2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &((::match_deref::match_deref! { match &(Call::arguments(call)?) {
        Deref @ metamodelica::List::Cons { head: __esc_arg1, tail: Deref @ metamodelica::List::Cons { head: __esc_arg2, tail: Deref @ metamodelica::List::Nil } } => {
            arg1 = (*__esc_arg1).clone();
            arg2 = (*__esc_arg2).clone();
            list![arg1.clone(), arg2.clone()]
        },
        Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: __esc_arg1, tail: Deref @ metamodelica::List::Cons { head: __esc_arg2, tail: Deref @ metamodelica::List::Nil } } } if (basic) => {
            arg1 = (*__esc_arg1).clone();
            arg2 = (*__esc_arg2).clone();
            list![arg1.clone(), arg2.clone()]
        },
        Deref @ metamodelica::List::Cons { head: __esc_arg1, tail: Deref @ metamodelica::List::Cons { head: __esc_arg2, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } } => {
            arg1 = (*__esc_arg1).clone();
            arg2 = (*__esc_arg2).clone();
            list![arg1.clone(), arg2.clone()]
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.replaceClockedFunctions.replaceSample")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        arg1 = __pa0.clone();
        arg2 = __pa1.clone();
        if basic {
            exp = if (Type::isClock(Expression::typeOf(arg2))?) {replaceClockedFunctionExp(arg1)?} else {exp};
        } else {
            exp = replaceClockedFunctionExp(arg1)?;
        }
        Ok(exp)
    }

    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } => {
            let mut newExp: Arc<Expression::NFExpression>;
            let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            newExp = (::match_deref::match_deref! { match &(AbsynUtil::pathString(Function::nameConsiderBuiltin(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL).clone())?, (literal!(".")).clone(), true, false)?) {
        Deref @ "sample" => replaceSample(exp, call.clone(), true)?,
        Deref @ "subSample" => replaceSample(exp, call.clone(), false)?,
        Deref @ "superSample" => replaceSample(exp, call.clone(), false)?,
        Deref @ "shiftSample" => replaceSample(exp, call.clone(), false)?,
        Deref @ "backSample" => replaceSample(exp, call.clone(), false)?,
        Deref @ "hold" => {
            arg = (::match_deref::match_deref! { match &(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?) {
        Deref @ metamodelica::List::Cons { head: __esc_arg @ Deref @ Expression::CREF { .. }, tail: Deref @ metamodelica::List::Nil } => {
            arg = (*__esc_arg).clone();
            UnorderedSet::add(var_field!((*arg).cref, Expression::NFExpression::CREF).clone(), held_crefs.clone())?;
            arg.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.replaceClockedFunctions")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp)?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            replaceClockedFunctionExp(arg.clone())?
        },
        _ => exp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            newExp.clone()
        },
        _ => {
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn replaceClockedFunctionExp(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut func: Arc<Function::Function>;
    func = (::match_deref::match_deref! { match &(Expression::typeOf(exp.clone())) {
        Deref @ Type::REAL => NFBuiltinFuncs::GET_PART_REAL().clone(),
        Deref @ Type::INTEGER => NFBuiltinFuncs::GET_PART_INT().clone(),
        Deref @ Type::BOOLEAN => NFBuiltinFuncs::GET_PART_BOOL().clone(),
        Deref @ Type::CLOCK => NFBuiltinFuncs::GET_PART_CLOCK().clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.replaceClockedFunctionExp")); __mm_s.push_str(&*literal!(" failed. ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(" is of type ")); __mm_s.push_str(&*Type::toString(Expression::typeOf(exp.clone()))?); __mm_s.push_str(&*literal!(", only real, integer, boolean and clock are allowed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(func.clone(), list![exp.clone()], Expression::variability(exp)?, NFPrefixes::Purity::PURE.clone(), func.returnType.clone()) });
    Ok(exp)
}

fn replaceClockedWhen(mut eqn: Arc<Equation::Equation>) -> Result<Arc<Equation::Equation>> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    eqn = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BEquation::Equation::WHEN_EQUATION { body: Deref @ BEquation::WhenEquationBody::WHEN_EQUATION_BODY { condition: cond, when_stmts: Deref @ metamodelica::List::Cons { head: stmt, tail: Deref @ metamodelica::List::Nil }, else_when: None }, .. } if (Type::isClock(Expression::typeOf(cond.clone()))?) => {
            BEquation::WhenStatement::toEquation(stmt.clone(), var_field!((*eqn).attr, Equation::Equation::WHEN_EQUATION).clone(), false)?
        },
        _ => {
            eqn
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eqn)
}

