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

use crate::NBEquation as BEquation;
use crate::NBEquation::Equation;
use crate::NBEquation::Frame;
use crate::NBEquation::FrameLocation;
use crate::NBMatching as Matching;
use crate::NBVariable as BVariable;
use openmodelica_nf_frontend::NFBackendExtension::BackendInfo;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::MMath;
use openmodelica_util::Rational;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Mutable;
use openmodelica_util_datatypes_basic::Pointer;

// NF imports
// backend imports
// Util imports
// old imports
pub(crate) fn convertRational(mut r: Arc<Rational::Rational>) -> MMath::Rational {
    let mut oldR: MMath::Rational = MMath::Rational { nom: r.n.clone(), denom: r.d.clone() };
    oldR
}

pub(crate) fn findTrueIndices(mut arr: metamodelica::Array<bool>) -> Arc<metamodelica::List<i32>> {
    let mut indices: Arc<metamodelica::List<i32>> = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (({let __s=metamodelica::arrayLength(arr.clone()); let __e=1; (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)})).into_iter() {
            if !(({let __elt = arr.borrow()[(i.clone()-1) as usize].clone(); __elt})) { continue; }
            let __x = i.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    indices
}

pub(crate) fn countElem(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> i32 {
    let mut count: i32 = ({
        let mut __acc: i32 = 0;
        for mut lst in (m.clone()).borrow().iter() {
            let __x = (lst.clone().len() as i32);
            __acc += __x;
        }
        __acc
    });
    count
}

pub(crate) fn indexTplGt<T: Clone + 'static + metamodelica::gc::MMTrace>(mut tpl1: (i32, T), mut tpl2: (i32, T)) -> bool {
    let mut gt: bool;
    let mut i1: i32;
    let mut i2: i32;
    (i1, _) = tpl1;
    (i2, _) = tpl2;
    gt = i1 > i2;
    gt
}

pub(crate) fn noNameHashEq(mut eq: Arc<Equation::Equation>, mut r#mod: i32) -> Result<i32> {
    let mut hash: i32;
    hash = noNameHashExp(BEquation::Equation::getResidualExp(eq, true)?, r#mod)?;
    Ok(hash)
}

pub(crate) fn noNameHashExp(mut exp: Arc<Expression::NFExpression>, mut r#mod: i32) -> Result<i32> {
    let mut hash: i32 = 0;
    hash = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::INTEGER { .. } => {
            var_field!((*exp).value, Expression::NFExpression::INTEGER).clone()
        },
        Deref @ Expression::REAL { .. } => {
            ((var_field!((*exp).value, Expression::NFExpression::REAL).clone()).0.floor() as i32)
        },
        Deref @ Expression::STRING { .. } => {
            stringHashDjb2Mod((var_field!((*exp).value, Expression::NFExpression::STRING).clone()).clone(), r#mod)
        },
        Deref @ Expression::BOOLEAN { .. } => {
            Util::boolInt(var_field!((*exp).value, Expression::NFExpression::BOOLEAN).clone())
        },
        Deref @ Expression::ENUM_LITERAL { .. } => {
            var_field!((*exp).index, Expression::NFExpression::ENUM_LITERAL).clone()
        },
        Deref @ Expression::CLKCONST { .. } => {
            0
        },
        Deref @ Expression::CREF { .. } => {
            let mut var: Arc<Variable::NFVariable>;
            var = BVariable::getVar(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), metamodelica::sourceInfo!("NBackEnd/Util/NBBackendUtil.mo"))?;
            stringHashDjb2Mod((BackendInfo::toString(var.backendinfo.clone())?).clone(), r#mod)
        },
        Deref @ Expression::TYPENAME { .. } => {
            1
        },
        Deref @ Expression::ARRAY { .. } => {
            let __range0 = var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut elem in __range0 {
                hash = hash + noNameHashExp(elem.clone(), r#mod)?;
            }
            hash = hash + Util::boolInt(var_field!((*exp).literal, Expression::NFExpression::ARRAY).clone());
            hash
        },
        Deref @ Expression::MATRIX { .. } => {
            for mut lst in &*var_field!((*exp).elements, Expression::NFExpression::MATRIX).clone() {
                let mut lst = lst.clone();
                for mut elem in &*lst.clone() {
                    let mut elem = elem.clone();
                    hash = hash + noNameHashExp(elem.clone(), r#mod)?;
                }
            }
            hash
        },
        Deref @ Expression::RANGE { .. } => {
            if isSome(var_field!((*exp).step, Expression::NFExpression::RANGE).clone()) {
                hash = noNameHashExp(Util::getOption(var_field!((*exp).step, Expression::NFExpression::RANGE).clone())?, r#mod)?;
            }
            hash + noNameHashExp(var_field!((*exp).start, Expression::NFExpression::RANGE).clone(), r#mod)? + noNameHashExp(var_field!((*exp).stop, Expression::NFExpression::RANGE).clone(), r#mod)?
        },
        Deref @ Expression::TUPLE { .. } => {
            for mut elem in &*var_field!((*exp).elements, Expression::NFExpression::TUPLE).clone() {
                let mut elem = elem.clone();
                hash = hash + noNameHashExp(elem.clone(), r#mod)?;
            }
            hash
        },
        Deref @ Expression::RECORD { .. } => {
            for mut elem in &*var_field!((*exp).elements, Expression::NFExpression::RECORD).clone() {
                let mut elem = elem.clone();
                hash = hash + noNameHashExp(elem.clone(), r#mod)?;
            }
            hash
        },
        Deref @ Expression::CALL { .. } => {
            2
        },
        Deref @ Expression::SIZE { .. } => {
            if isSome(var_field!((*exp).dimIndex, Expression::NFExpression::SIZE).clone()) {
                hash = noNameHashExp(Util::getOption(var_field!((*exp).dimIndex, Expression::NFExpression::SIZE).clone())?, r#mod)?;
            }
            hash + noNameHashExp(var_field!((*exp).exp, Expression::NFExpression::SIZE).clone(), r#mod)?
        },
        Deref @ Expression::END => {
            stringHashDjb2Mod((literal!("end")).clone(), r#mod)
        },
        Deref @ Expression::BINARY { .. } => {
            let mut hash1: i32;
            let mut hash2: i32;
            hash1 = noNameHashExp(var_field!((*exp).exp1, Expression::NFExpression::BINARY).clone(), r#mod)?;
            hash2 = noNameHashExp(var_field!((*exp).exp2, Expression::NFExpression::BINARY).clone(), r#mod)?;
            hash = (match Operator::classify(var_field!((*exp).operator, Expression::NFExpression::BINARY).clone())? {
        (Operator::MathClassification::ADDITION, _) => hash1 + hash2,
        (Operator::MathClassification::SUBTRACTION, _) => hash1 - hash2,
        (Operator::MathClassification::MULTIPLICATION, _) => hash1 * hash2,
        (Operator::MathClassification::DIVISION, _) => ((metamodelica::OrderedFloat((hash1) as f64) / metamodelica::OrderedFloat((hash2) as f64)).0.floor() as i32),
        (Operator::MathClassification::POWER, _) => (((metamodelica::OrderedFloat((hash1) as f64)).powf(metamodelica::OrderedFloat((hash2) as f64))).0.floor() as i32),
        (Operator::MathClassification::LOGICAL, _) => -(hash1 + hash2),
        (Operator::MathClassification::RELATION, _) => hash2 - hash1,
        _ => hash2 - hash1,
    });
            hash
        },
        Deref @ Expression::UNARY { .. } => {
            -(noNameHashExp(var_field!((*exp).exp, Expression::NFExpression::UNARY).clone(), r#mod)?)
        },
        Deref @ Expression::LBINARY { .. } => {
            let mut hash1: i32;
            let mut hash2: i32;
            hash1 = noNameHashExp(var_field!((*exp).exp1, Expression::NFExpression::LBINARY).clone(), r#mod)?;
            hash2 = noNameHashExp(var_field!((*exp).exp2, Expression::NFExpression::LBINARY).clone(), r#mod)?;
            hash = (match var_field!((*exp).operator, Expression::NFExpression::LBINARY).op.clone() {
        Operator::Op::AND => hash1 + hash2,
        Operator::Op::OR => hash1 - hash2,
        _ => hash2 - hash1,
    });
            hash
        },
        Deref @ Expression::LUNARY { .. } => {
            -(noNameHashExp(var_field!((*exp).exp, Expression::NFExpression::LUNARY).clone(), r#mod)?)
        },
        Deref @ Expression::RELATION { .. } => {
            let mut hash1: i32;
            let mut hash2: i32;
            hash1 = noNameHashExp(var_field!((*exp).exp1, Expression::NFExpression::RELATION).clone(), r#mod)?;
            hash2 = noNameHashExp(var_field!((*exp).exp2, Expression::NFExpression::RELATION).clone(), r#mod)?;
            hash = (match var_field!((*exp).operator, Expression::NFExpression::RELATION).op.clone() {
        Operator::Op::LESS => hash1 + hash2,
        Operator::Op::LESSEQ => -(hash1 + hash2),
        Operator::Op::GREATER => hash1 - hash2,
        Operator::Op::GREATEREQ => hash2 - hash1,
        Operator::Op::EQUAL => hash1 * hash2,
        Operator::Op::NEQUAL => (((metamodelica::OrderedFloat((hash1) as f64)).powf(metamodelica::OrderedFloat((hash2) as f64))).0.floor() as i32),
        _ => hash2 - hash1,
    });
            hash
        },
        Deref @ Expression::IF { .. } => {
            noNameHashExp(var_field!((*exp).condition, Expression::NFExpression::IF).clone(), r#mod)? + noNameHashExp(var_field!((*exp).trueBranch, Expression::NFExpression::IF).clone(), r#mod)? + noNameHashExp(var_field!((*exp).falseBranch, Expression::NFExpression::IF).clone(), r#mod)?
        },
        Deref @ Expression::CAST { .. } => {
            noNameHashExp(var_field!((*exp).exp, Expression::NFExpression::CAST).clone(), r#mod)?
        },
        Deref @ Expression::BOX { .. } => {
            noNameHashExp(var_field!((*exp).exp, Expression::NFExpression::BOX).clone(), r#mod)?
        },
        Deref @ Expression::UNBOX { .. } => {
            noNameHashExp(var_field!((*exp).exp, Expression::NFExpression::UNBOX).clone(), r#mod)?
        },
        Deref @ Expression::SUBSCRIPTED_EXP { .. } => {
            noNameHashExp(var_field!((*exp).exp, Expression::NFExpression::SUBSCRIPTED_EXP).clone(), r#mod)?
        },
        Deref @ Expression::TUPLE_ELEMENT { .. } => {
            noNameHashExp(var_field!((*exp).tupleExp, Expression::NFExpression::TUPLE_ELEMENT).clone(), r#mod)? + var_field!((*exp).index, Expression::NFExpression::TUPLE_ELEMENT).clone()
        },
        Deref @ Expression::RECORD_ELEMENT { .. } => {
            noNameHashExp(var_field!((*exp).recordExp, Expression::NFExpression::RECORD_ELEMENT).clone(), r#mod)? + var_field!((*exp).index, Expression::NFExpression::RECORD_ELEMENT).clone()
        },
        Deref @ Expression::MUTABLE { .. } => {
            noNameHashExp(Mutable::access(var_field!((*exp).exp, Expression::NFExpression::MUTABLE).clone()), r#mod)?
        },
        Deref @ Expression::EMPTY { .. } => {
            stringHashDjb2Mod((literal!("empty")).clone(), r#mod)
        },
        Deref @ Expression::PARTIAL_FUNCTION_APPLICATION { .. } => {
            for mut arg in &*var_field!((*exp).args, Expression::NFExpression::PARTIAL_FUNCTION_APPLICATION).clone() {
                let mut arg = arg.clone();
                hash = hash + noNameHashExp(arg.clone(), r#mod)?;
            }
            hash
        },
        _ => {
            0
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hash = intMod(intAbs(hash), r#mod);
    Ok(hash)
}

pub(crate) fn isOnlyTimeDependent(mut exp: Arc<Expression::NFExpression>) -> Result<bool> {
    let mut b: bool;
    b = Expression::fold(exp, (std::sync::Arc::new(isOnlyTimeDependentFold) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, bool) -> Result<bool> + 'static>), true)?;
    Ok(b)
}

pub(crate) fn isOnlyTimeDependentFold(mut exp: Arc<Expression::NFExpression>, mut b: bool) -> Result<bool> {
    let mut b: bool = b;
    if b {
        b = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => ComponentRef::isTime(var_field!((*exp).cref, Expression::NFExpression::CREF).clone())? || BVariable::checkCref(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), (std::sync::Arc::new(fnptr!(BVariable::isParamOrConst, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), metamodelica::sourceInfo!("NBackEnd/Util/NBBackendUtil.mo"))?,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(b)
}

pub(crate) fn isContinuous(mut exp: Arc<Expression::NFExpression>, mut staticAsContinuous: bool) -> Result<bool> {
    let mut b: bool;
    b = Expression::fold(exp, (std::sync::Arc::new({ let __pe_b1 = staticAsContinuous; move |__pe_a0, __pe_a2| isContinuousFold(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, bool) -> Result<bool> + 'static>), true)?;
    Ok(b)
}

pub(crate) fn isContinuousFold(mut exp: Arc<Expression::NFExpression>, mut staticAsContinuous: bool, mut b: bool) -> Result<bool> {
    let mut b: bool = b;
    if b {
        b = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => BVariable::checkCref(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), (std::sync::Arc::new({ let __pe_b1 = staticAsContinuous; move |__pe_a0| BVariable::isContinuous(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), metamodelica::sourceInfo!("NBackEnd/Util/NBBackendUtil.mo"))?,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(b)
}

pub(crate) fn getLocalSystem(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut matching: Arc<Matching::NBMatching>, mut eqn_indices: Arc<metamodelica::List<i32>>) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<Matching::NBMatching>, metamodelica::Array<i32>)> {
    let mut m_loc: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut matching_loc: Arc<Matching::NBMatching>;
    let mut map_back: metamodelica::Array<i32>;
    let N: i32 = (eqn_indices.clone().len() as i32);
    let mut var_to_eqn: metamodelica::Array<i32> = arrayCreate(N, -1);
    let mut eqn_to_var: metamodelica::Array<i32> = arrayCreate(N, -1);
    let mut var_loc: Arc<UnorderedMap::UnorderedMap<i32, i32>> = UnorderedMap::new(std::sync::Arc::new(fnptr!(Util::id, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), N);
    let mut j: i32 = 1;
    map_back = arrayCreate(N, -1);
    for mut i in &*eqn_indices {
        let mut i = i.clone();
        {
            let __cell0 = i.clone();
            let __idx0 = j;
            map_back.clone().borrow_mut()[(__idx0-1) as usize] = __cell0;
        }
        UnorderedMap::addUnique(({let __elt = matching.eqn_to_var.borrow()[(i.clone()-1) as usize].clone(); __elt}), j, var_loc.clone())?;
        {
            let __cell1 = j;
            let __idx1 = j;
            eqn_to_var.clone().borrow_mut()[(__idx1-1) as usize] = __cell1;
        }
        {
            let __cell2 = j;
            let __idx2 = j;
            var_to_eqn.clone().borrow_mut()[(__idx2-1) as usize] = __cell2;
        }
        j = j + 1;
    }
    matching_loc = Arc::new(Matching::NBMatching { var_to_eqn: var_to_eqn.clone(), eqn_to_var: eqn_to_var.clone() });
    m_loc = arrayCreate(N, metamodelica::nil());
    for mut j in 1..=N {
        {
            let __cell3 = UnorderedMap::getList(({let __elt = m.borrow()[(({let __elt = map_back.borrow()[(j-1) as usize].clone(); __elt})-1) as usize].clone(); __elt}), var_loc.clone())?;
            let __idx3 = j;
            m_loc.clone().borrow_mut()[(__idx3-1) as usize] = __cell3;
        }
    }
    Ok((m_loc, matching_loc, map_back))
}

pub(crate) fn makeFDerString(mut r#str: ArcStr, mut i_opt: Option<i32>) -> Result<ArcStr> {
    let mut r#str: ArcStr = r#str;
    let mut i: ArcStr = if (isSome(i_opt.clone())) {intString(Util::getOption(i_opt.clone())?)} else {literal!("")};
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(BVariable::FUNCTION_DERIVATIVE_STR)); __mm_s.push_str(&*i); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*r#str); ArcStr::from(__mm_s) }).clone();
    Ok(r#str)
}

