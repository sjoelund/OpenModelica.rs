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

use crate::AbsynUtil;
use crate::ClassInfUtil;
use crate::Dump;
use crate::ExpressionBasics;
use crate::SCodeDump;
use crate::ValuesDump;
use openmodelica_ast::Absyn;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::Print;
use openmodelica_util_datatypes_basic::List;

pub type Binding = Arc<DAE::Binding>;

pub type Const = DAE::Const;

pub type EqualityConstraint = Option<(Arc<Absyn::Path>, i32, DAE::InlineType)>;

pub type FuncArg = Arc<DAE::FuncArg>;

pub type Properties = DAE::Properties;

pub type TupleConst = Arc<DAE::TupleConst>;

pub type Type = Arc<DAE::Type>;

pub type Var = Arc<DAE::Var>;

pub type EqMod = DAE::EqMod;

pub fn unparseEqMod(mut eq: DAE::EqMod) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match eq {
        DAE::EqMod::TYPED { modifierAsExp: ref e, .. } => {
            r#str = (ExpressionBasics::printExpStr(e.clone())?).clone();
            r#str
        },
        DAE::EqMod::UNTYPED { exp: ref e2 } => {
            r#str = (Dump::printExpStr(e2.clone())?).clone();
            r#str
        },
    })).clone();
    Ok(r#str)
}

pub fn unparseOptionEqMod(mut eq: Option<DAE::EqMod>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = ((match eq {
        None => {
            literal!("NONE()")
        },
        Some(mut e) => {
            unparseEqMod(e.clone())?
        },
    })).clone();
    Ok(r#str)
}

pub fn unparseType(mut inType: Arc<DAE::Type>) -> Result<ArcStr> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_INTEGER { varLst: Deref @ metamodelica::List::Nil } => {
            return Ok(literal!("Integer"))
        },
        Deref @ DAE::Type::T_REAL { varLst: Deref @ metamodelica::List::Nil } => {
            return Ok(literal!("Real"))
        },
        Deref @ DAE::Type::T_STRING { varLst: Deref @ metamodelica::List::Nil } => {
            return Ok(literal!("String"))
        },
        Deref @ DAE::Type::T_BOOL { varLst: Deref @ metamodelica::List::Nil } => {
            return Ok(literal!("Boolean"))
        },
        Deref @ DAE::Type::T_CLOCK { .. } => {
            return Ok(literal!("Clock"))
        },
        Deref @ DAE::Type::T_INTEGER { varLst: vs } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            s1 = stringDelimitList(List::map(vs.clone(), (std::sync::Arc::new(fnptr!(unparseVarAttr, Arc<DAE::Var>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone());
            return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Integer(")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) })
        },
        Deref @ DAE::Type::T_REAL { varLst: vs } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            s1 = stringDelimitList(List::map(vs.clone(), (std::sync::Arc::new(fnptr!(unparseVarAttr, Arc<DAE::Var>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone());
            return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Real(")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) })
        },
        Deref @ DAE::Type::T_STRING { varLst: vs } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            s1 = stringDelimitList(List::map(vs.clone(), (std::sync::Arc::new(fnptr!(unparseVarAttr, Arc<DAE::Var>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone());
            return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("String(")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) })
        },
        Deref @ DAE::Type::T_BOOL { varLst: vs } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            s1 = stringDelimitList(List::map(vs.clone(), (std::sync::Arc::new(fnptr!(unparseVarAttr, Arc<DAE::Var>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone());
            return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Boolean(")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) })
        },
        Deref @ DAE::Type::T_ENUMERATION { path, names: l, .. } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut r#str: ArcStr;
            s1 = (if (Config::typeinfo()?) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" /*")); __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!("*/ (")); ArcStr::from(__mm_s) }} else {literal!("(")}).clone();
            s2 = stringDelimitList(l.clone(), (literal!(", ")).clone());
            return Ok(stringAppendList(list![(literal!("enumeration")).clone(), (s1.clone()).clone(), (s2.clone()).clone(), (literal!(")")).clone()]))
        },
        ty @ Deref @ DAE::Type::T_ARRAY { .. } => {
            let mut dims: ArcStr;
            let mut res: ArcStr;
            let mut tystr: ArcStr;
            let mut dimlst: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
            let mut ty = (*ty).clone();
            (ty, dimlst) = flattenArrayType(ty.clone());
            tystr = (unparseType(ty.clone())?).clone();
            dims = (printDimensionsStr(dimlst.clone())?).clone();
            return Ok(stringAppendList(list![(tystr.clone()).clone(), (literal!("[")).clone(), (dims.clone()).clone(), (literal!("]")).clone()]))
        },
        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path }, varLst: vs, .. } => {
            let mut res: ArcStr;
            let mut vstr: ArcStr;
            let mut name: ArcStr;
            let mut vars: Arc<metamodelica::List<ArcStr>>;
            name = AbsynUtil::pathStringNoQual(path.clone(), (literal!(".")).clone(), false, false)?;
            vars = List::map(vs.clone(), (std::sync::Arc::new(unparseVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>))?;
            vstr = stringAppendList(vars.clone());
            return Ok(stringAppendList(list![(literal!("record ")).clone(), (name.clone()).clone(), (literal!("\n")).clone(), (vstr.clone()).clone(), (literal!("end ")).clone(), (name.clone()).clone(), (literal!(";")).clone()]))
        },
        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::CONNECTOR { path, isExpandable: b }, varLst: vs, .. } => {
            let mut r#str: ArcStr;
            let mut res: ArcStr;
            let mut vstr: ArcStr;
            let mut name: ArcStr;
            let mut vars: Arc<metamodelica::List<ArcStr>>;
            name = AbsynUtil::pathStringNoQual(path.clone(), (literal!(".")).clone(), false, false)?;
            vars = List::map(vs.clone(), (std::sync::Arc::new(unparseVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>))?;
            vstr = stringAppendList(vars.clone());
            r#str = (if (b.clone()) {literal!("expandable ")} else {literal!("")}).clone();
            return Ok(stringAppendList(list![(r#str.clone()).clone(), (literal!("connector ")).clone(), (name.clone()).clone(), (literal!("\n")).clone(), (vstr.clone()).clone(), (literal!("end ")).clone(), (name.clone()).clone(), (literal!(";")).clone()]))
        },
        Deref @ DAE::Type::T_SUBTYPE_BASIC { complexClassType: ci_state, complexType: bc_tp, .. } => {
            let mut res: ArcStr;
            let mut st_str: ArcStr;
            let mut bc_tp_str: ArcStr;
            st_str = (AbsynUtil::pathString(ClassInfUtil::getStateName(ci_state.clone()), (literal!(".")).clone(), true, false)?).clone();
            res = (ClassInfUtil::printStateStr(ci_state.clone())).clone();
            bc_tp_str = (unparseType(bc_tp.clone())?).clone();
            return Ok(stringAppendList(list![(literal!("(")).clone(), (res.clone()).clone(), (literal!(" ")).clone(), (st_str.clone()).clone(), (literal!(" bc:")).clone(), (bc_tp_str.clone()).clone(), (literal!(")")).clone()]))
        },
        Deref @ DAE::Type::T_COMPLEX { complexClassType: ci_state, .. } => {
            let mut res: ArcStr;
            let mut st_str: ArcStr;
            st_str = (AbsynUtil::pathString(ClassInfUtil::getStateName(ci_state.clone()), (literal!(".")).clone(), true, false)?).clone();
            res = (ClassInfUtil::printStateStr(ci_state.clone())).clone();
            return Ok(stringAppendList(list![(res.clone()).clone(), (literal!(" ")).clone(), (st_str.clone()).clone()]))
        },
        Deref @ DAE::Type::T_FUNCTION { funcArg: params, funcResultType: restype, path, .. } => {
            let mut res: ArcStr;
            let mut paramstr: ArcStr;
            let mut restypestr: ArcStr;
            let mut funcstr: ArcStr;
            let mut paramstrs: Arc<metamodelica::List<ArcStr>>;
            funcstr = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
            paramstrs = List::map(params.clone(), (std::sync::Arc::new(unparseParam) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>) -> Result<ArcStr> + 'static>))?;
            paramstr = stringDelimitList(paramstrs.clone(), (literal!(", ")).clone());
            restypestr = (unparseType(restype.clone())?).clone();
            return Ok(stringAppendList(list![(funcstr.clone()).clone(), (literal!("<function>(")).clone(), (paramstr.clone()).clone(), (literal!(") => ")).clone(), (restypestr.clone()).clone()]))
        },
        Deref @ DAE::Type::T_TUPLE { types: tys, .. } => {
            let mut res: ArcStr;
            let mut tystr: ArcStr;
            let mut tystrs: Arc<metamodelica::List<ArcStr>>;
            tystrs = (::match_deref::match_deref! { match &(var_field!((*inType).names, DAE::Type::T_TUPLE).clone()) {
        Some(names) => {
            ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        let __thr_src0 = tys.clone();
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = names.clone();
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(t), Some(n)) => {
                    let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*unparseType(t.clone())?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*n.clone()); ArcStr::from(__mm_s) };
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => bail!("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    })
        },
        _ => {
            ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut t in (tys.clone()).into_iter().cloned() {
            let __x = unparseType(t.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } });
            tystr = stringDelimitList(tystrs.clone(), (literal!(", ")).clone());
            return Ok(stringAppendList(list![(literal!("(")).clone(), (tystr.clone()).clone(), (literal!(")")).clone()]))
        },
        Deref @ DAE::Type::T_METATUPLE { types: tys } => {
            let mut res: ArcStr;
            let mut tystr: ArcStr;
            let mut tystrs: Arc<metamodelica::List<ArcStr>>;
            tystrs = List::map(tys.clone(), (std::sync::Arc::new(unparseType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<ArcStr> + 'static>))?;
            tystr = stringDelimitList(tystrs.clone(), (literal!(", ")).clone());
            return Ok(stringAppendList(list![(literal!("tuple<")).clone(), (tystr.clone()).clone(), (literal!(">")).clone()]))
        },
        Deref @ DAE::Type::T_METALIST { ty } => {
            let mut res: ArcStr;
            let mut tystr: ArcStr;
            tystr = (unparseType(ty.clone())?).clone();
            return Ok(stringAppendList(list![(literal!("list<")).clone(), (tystr.clone()).clone(), (literal!(">")).clone()]))
        },
        Deref @ DAE::Type::T_METAARRAY { ty } => {
            let mut res: ArcStr;
            let mut tystr: ArcStr;
            tystr = (unparseType(ty.clone())?).clone();
            return Ok(stringAppendList(list![(literal!("array<")).clone(), (tystr.clone()).clone(), (literal!(">")).clone()]))
        },
        Deref @ DAE::Type::T_METAPOLYMORPHIC { name: tystr } => {
            let mut res: ArcStr;
            return Ok(stringAppendList(list![(literal!("polymorphic<")).clone(), (tystr.clone()).clone(), (literal!(">")).clone()]))
        },
        Deref @ DAE::Type::T_METAUNIONTYPE { .. } => {
            let mut res: ArcStr;
            res = AbsynUtil::pathStringNoQual(var_field!((*inType).path, DAE::Type::T_METAUNIONTYPE).clone(), (literal!(".")).clone(), false, false)?;
            if (var_field!((*inType).typeVars, DAE::Type::T_METAUNIONTYPE).clone().is_empty()) {return Ok(res.clone())} else {return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*res.clone()); __mm_s.push_str(&*literal!("<")); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut tv in (var_field!((*inType).typeVars, DAE::Type::T_METAUNIONTYPE).clone()).into_iter().cloned() {
            let __x = unparseType(tv.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!(",")).clone())); __mm_s.push_str(&*literal!(">")); ArcStr::from(__mm_s) })}
        },
        Deref @ DAE::Type::T_METARECORD { .. } => {
            let mut res: ArcStr;
            res = AbsynUtil::pathStringNoQual(var_field!((*inType).path, DAE::Type::T_METARECORD).clone(), (literal!(".")).clone(), false, false)?;
            if (var_field!((*inType).typeVars, DAE::Type::T_METARECORD).clone().is_empty()) {return Ok(res.clone())} else {return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*res.clone()); __mm_s.push_str(&*literal!("<")); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut tv in (var_field!((*inType).typeVars, DAE::Type::T_METARECORD).clone()).into_iter().cloned() {
            let __x = unparseType(tv.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!(",")).clone())); __mm_s.push_str(&*literal!(">")); ArcStr::from(__mm_s) })}
        },
        Deref @ DAE::Type::T_METABOXED { ty } => {
            let mut res: ArcStr;
            res = (unparseType(ty.clone())?).clone();
            return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("#")); __mm_s.push_str(&*res.clone()); ArcStr::from(__mm_s) })
        },
        Deref @ DAE::Type::T_METAOPTION { ty: Deref @ DAE::Type::T_UNKNOWN { .. } } => {
            return Ok(literal!("Option<Any>"))
        },
        Deref @ DAE::Type::T_METAOPTION { ty } => {
            let mut res: ArcStr;
            let mut tystr: ArcStr;
            tystr = (unparseType(ty.clone())?).clone();
            return Ok(stringAppendList(list![(literal!("Option<")).clone(), (tystr.clone()).clone(), (literal!(">")).clone()]))
        },
        Deref @ DAE::Type::T_METATYPE { ty } => {
            { inType = ty.clone(); continue '__tco; }
        },
        Deref @ DAE::Type::T_NORETCALL { .. } => {
            return Ok(literal!("#NORETCALL#"))
        },
        Deref @ DAE::Type::T_UNKNOWN { .. } => {
            return Ok(literal!("#T_UNKNOWN#"))
        },
        Deref @ DAE::Type::T_ANYTYPE { .. } => {
            return Ok(literal!("#ANYTYPE#"))
        },
        Deref @ DAE::Type::T_CODE { ty: codeType } => {
            return Ok(printCodeTypeStr(codeType.clone()))
        },
        Deref @ DAE::Type::T_FUNCTION_REFERENCE_VAR { functionType: ty } => {
            return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("#FUNCTION_REFERENCE_VAR#")); __mm_s.push_str(&*unparseType(ty.clone())?); ArcStr::from(__mm_s) })
        },
        Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { functionType: ty, .. } => {
            return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("#FUNCTION_REFERENCE_FUNC#")); __mm_s.push_str(&*unparseType(ty.clone())?); ArcStr::from(__mm_s) })
        },
        _ => {
            return Ok(literal!("Internal error TypesDump.unparseType: not implemented yet\n"))
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn unparseTypeNoAttr(mut inType: Arc<DAE::Type>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut ty: Arc<DAE::Type>;
    (ty, _) = stripTypeVars(inType);
    outString = (unparseType(ty)?).clone();
    Ok(outString)
}

pub fn unparsePropTypeNoAttr(mut inProps: DAE::Properties) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((match inProps {
        DAE::Properties::PROP { type_: ref ty, .. } => {
            unparseTypeNoAttr(ty.clone())?
        },
        DAE::Properties::PROP_TUPLE { type_: ref ty, .. } => {
            unparseTypeNoAttr(ty.clone())?
        },
    })).clone();
    Ok(outString)
}

pub fn unparseConst(mut inConst: DAE::Const) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((match inConst {
        DAE::Const::C_CONST { .. } => literal!("constant"),
        DAE::Const::C_PARAM { .. } => literal!("parameter"),
        DAE::Const::C_VAR { .. } => literal!("continuous"),
        DAE::Const::C_UNKNOWN { .. } => literal!("unknown"),
    })).clone();
    Ok(outString)
}

pub fn printConstStr(mut inConst: DAE::Const) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((match inConst {
        DAE::Const::C_CONST { .. } => literal!("C_CONST"),
        DAE::Const::C_PARAM { .. } => literal!("C_PARAM"),
        DAE::Const::C_VAR { .. } => literal!("C_VAR"),
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TypesDump.printConstStr")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("FrontEnd/TypesDump.mo"))?;
            bail!("fail")
        },
    })).clone();
    Ok(outString)
}

pub fn printTupleConstStr(mut inTupleConst: Arc<DAE::TupleConst>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((::match_deref::match_deref! { match &(inTupleConst) {
        Deref @ DAE::TupleConst::SINGLE_CONST { r#const: c } => {
            let mut cstr: ArcStr;
            cstr = (printConstStr(c.clone())?).clone();
            cstr.clone()
        },
        Deref @ DAE::TupleConst::TUPLE_CONST { tupleConstLst: constlist } => {
            let mut res: ArcStr;
            let mut res_1: ArcStr;
            let mut strlist: Arc<metamodelica::List<ArcStr>>;
            strlist = List::map(constlist.clone(), (std::sync::Arc::new(printTupleConstStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::TupleConst>) -> Result<ArcStr> + 'static>))?;
            res = stringDelimitList(strlist.clone(), (literal!(", ")).clone());
            res_1 = stringAppendList(list![(literal!("(")).clone(), (res.clone()).clone(), (literal!(")")).clone()]);
            res_1.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

pub fn printTypeStr(mut inType: Arc<DAE::Type>) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ('mc: {
        let __mc_input = inType.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_INTEGER { varLst: vars } => {
                    Ok(List::toString(vars.clone(), (std::sync::Arc::new(printVarStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>), (literal!("Integer")).clone(), (literal!("(")).clone(), (literal!(", ")).clone(), (literal!(")")).clone(), false, 0)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_REAL { varLst: vars } => {
                    Ok(List::toString(vars.clone(), (std::sync::Arc::new(printVarStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>), (literal!("Real")).clone(), (literal!("(")).clone(), (literal!(", ")).clone(), (literal!(")")).clone(), false, 0)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_STRING { varLst: vars } => {
                    Ok(List::toString(vars.clone(), (std::sync::Arc::new(printVarStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>), (literal!("String")).clone(), (literal!("(")).clone(), (literal!(", ")).clone(), (literal!(")")).clone(), false, 0)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_BOOL { varLst: vars } => {
                    Ok(List::toString(vars.clone(), (std::sync::Arc::new(printVarStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>), (literal!("Boolean")).clone(), (literal!("(")).clone(), (literal!(", ")).clone(), (literal!(")")).clone(), false, 0)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_CLOCK { varLst: vars } => {
                    Ok(List::toString(vars.clone(), (std::sync::Arc::new(printVarStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>), (literal!("Clock")).clone(), (literal!("(")).clone(), (literal!(", ")).clone(), (literal!(")")).clone(), false, 0)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ENUMERATION { literalVarLst: vars, .. } => {
                    Ok(List::toString(vars.clone(), (std::sync::Arc::new(printVarStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>), (literal!("Enumeration")).clone(), (literal!("(")).clone(), (literal!(", ")).clone(), (literal!(")")).clone(), false, 0)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_SUBTYPE_BASIC { complexClassType: st, complexType: t, varLst: vars, .. } => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut compType: ArcStr;
                    let mut r#str: ArcStr = r#str.clone();
                    compType = (printTypeStr(t.clone())).clone();
                    s1 = (ClassInfUtil::printStateStr(st.clone())).clone();
                    s2 = stringDelimitList(List::map(vars.clone(), (std::sync::Arc::new(printVarStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone());
                    r#str = stringAppendList(list![(literal!("composite(")).clone(), (s1.clone()).clone(), (literal!("{")).clone(), (s2.clone()).clone(), (literal!("}, derived from ")).clone(), (compType.clone()).clone(), (literal!(")")).clone()]);
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { r#str = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_COMPLEX { complexClassType: st, varLst: vars, .. } => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut r#str: ArcStr = r#str.clone();
                    s1 = (ClassInfUtil::printStateStr(st.clone())).clone();
                    s2 = stringDelimitList(List::map(vars.clone(), (std::sync::Arc::new(printVarStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone());
                    r#str = stringAppendList(list![(literal!("composite(")).clone(), (s1.clone()).clone(), (literal!("{")).clone(), (s2.clone()).clone(), (literal!("})")).clone()]);
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { r#str = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { dims, ty: t } => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut r#str: ArcStr = r#str.clone();
                    s1 = stringDelimitList(List::map(dims.clone(), (std::sync::Arc::new(ExpressionBasics::dimensionString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Dimension>) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone());
                    s2 = (printTypeStr(t.clone())).clone();
                    r#str = stringAppendList(list![(literal!("array(")).clone(), (s2.clone()).clone(), (literal!(")[")).clone(), (s1.clone()).clone(), (literal!("]")).clone()]);
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { r#str = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_FUNCTION { funcArg: params, funcResultType: restype, .. } => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut r#str: ArcStr = r#str.clone();
                    s1 = (printParamsStr(params.clone())?).clone();
                    s2 = (printTypeStr(restype.clone())).clone();
                    r#str = stringAppendList(list![(literal!("function(")).clone(), (s1.clone()).clone(), (literal!(") => ")).clone(), (s2.clone()).clone()]);
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*AbsynUtil::pathString(var_field!((*inType).path, DAE::Type::T_FUNCTION).clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }).clone();
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { r#str = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_TUPLE { types: tys, .. } => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr = r#str.clone();
                    s1 = stringDelimitList(List::map(tys.clone(), (std::sync::Arc::new(fnptr!(printTypeStr, Arc<DAE::Type>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone());
                    r#str = stringAppendList(list![(literal!("(")).clone(), (s1.clone()).clone(), (literal!(")")).clone()]);
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { r#str = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METATUPLE { types: tys } => {
                    let mut r#str: ArcStr = r#str.clone();
                    r#str = (printTypeStr(Arc::new(DAE::Type::T_TUPLE { types: tys.clone(), names: None }))).clone();
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { r#str = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METALIST { ty } => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr = r#str.clone();
                    s1 = (printTypeStr(ty.clone())).clone();
                    r#str = stringAppendList(list![(literal!("list<")).clone(), (s1.clone()).clone(), (literal!(">")).clone()]);
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { r#str = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METAOPTION { ty } => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr = r#str.clone();
                    s1 = (printTypeStr(ty.clone())).clone();
                    r#str = stringAppendList(list![(literal!("Option<")).clone(), (s1.clone()).clone(), (literal!(">")).clone()]);
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { r#str = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METAARRAY { ty } => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr = r#str.clone();
                    s1 = (printTypeStr(ty.clone())).clone();
                    r#str = stringAppendList(list![(literal!("array<")).clone(), (s1.clone()).clone(), (literal!(">")).clone()]);
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { r#str = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METABOXED { ty } => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr = r#str.clone();
                    s1 = (printTypeStr(ty.clone())).clone();
                    r#str = stringAppendList(list![(literal!("boxed<")).clone(), (s1.clone()).clone(), (literal!(">")).clone()]);
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { r#str = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METAPOLYMORPHIC { name: s1 } => {
                    let mut r#str: ArcStr = r#str.clone();
                    r#str = stringAppendList(list![(literal!("polymorphic<")).clone(), (s1.clone()).clone(), (literal!(">")).clone()]);
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { r#str = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_UNKNOWN { .. } => {
                    let mut r#str: ArcStr = r#str.clone();
                    r#str = (literal!("T_UNKNOWN")).clone();
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { r#str = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ANYTYPE { anyClassType: None } => {
                    let mut r#str: ArcStr = r#str.clone();
                    r#str = (literal!("ANYTYPE()")).clone();
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { r#str = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ANYTYPE { anyClassType: Some(st) } => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr = r#str.clone();
                    s1 = (ClassInfUtil::printStateStr(st.clone())).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ANYTYPE(")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { r#str = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_NORETCALL { .. } => {
                    Ok(literal!("()"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METATYPE { ty: t } => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr = r#str.clone();
                    s1 = (printTypeStr(t.clone())).clone();
                    r#str = stringAppendList(list![(literal!("METATYPE(")).clone(), (s1.clone()).clone(), (literal!(")")).clone()]);
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { r#str = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                t @ Deref @ DAE::Type::T_METARECORD { .. } => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr = r#str.clone();
                    s1 = AbsynUtil::pathStringNoQual(var_field!((**t).path, DAE::Type::T_METARECORD).clone(), (literal!(".")).clone(), false, false)?;
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("#")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!("#")); ArcStr::from(__mm_s) }).clone();
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { r#str = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                t @ Deref @ DAE::Type::T_METAUNIONTYPE { .. } => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr = r#str.clone();
                    s1 = AbsynUtil::pathStringNoQual(var_field!((**t).path, DAE::Type::T_METAUNIONTYPE).clone(), (literal!(".")).clone(), false, false)?;
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("#")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!("#")); ArcStr::from(__mm_s) }).clone();
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { r#str = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_CODE { ty: DAE::CodeType::C_EXPRESSION { .. } } => {
                    Ok(literal!("$Code(Expression)"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_CODE { ty: DAE::CodeType::C_EXPRESSION_OR_MODIFICATION { .. } } => {
                    Ok(literal!("$Code(ExpressionOrModification)"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_CODE { ty: DAE::CodeType::C_TYPENAME { .. } } => {
                    Ok(literal!("$Code(TypeName)"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_CODE { ty: DAE::CodeType::C_VARIABLENAME { .. } } => {
                    Ok(literal!("$Code(VariableName)"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_CODE { ty: DAE::CodeType::C_VARIABLENAMES { .. } } => {
                    Ok(literal!("$Code(VariableName[:])"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr = r#str.clone();
                    r#str = (literal!("TypesDump.printTypeStr failed")).clone();
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { r#str = __wb0; break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    }).clone();
    r#str
}

pub fn printConnectorTypeStr(mut it: Arc<DAE::Type>) -> Result<(ArcStr, ArcStr)> {
    let mut s: ArcStr = arcstr::literal!("");
    let mut s2: ArcStr = arcstr::literal!("");
    (s, s2) = 'mc: {
        let __mc_input = it.clone();
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::CONNECTOR { path: connectorName, isExpandable }, varLst: vars, .. } => {
                    let mut varNames: Arc<metamodelica::List<ArcStr>>;
                    let mut isExpandableStr: ArcStr;
                    let mut s: ArcStr = s.clone();
                    let mut s2: ArcStr = s2.clone();
                    varNames = List::map(vars.clone(), (std::sync::Arc::new(getVarName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>))?;
                    isExpandableStr = (if (isExpandable.clone()) {literal!("/* expandable */ ")} else {literal!("")}).clone();
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*isExpandableStr.clone()); __mm_s.push_str(&*AbsynUtil::pathString(connectorName.clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }).clone();
                    s2 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*stringDelimitList(varNames.clone(), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
                    Ok(((s.clone(), s2.clone()), s.clone(), s2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { s = __wb0; s2 = __wb1; break 'mc __v; }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_SUBTYPE_BASIC { complexClassType: ClassInf::State::CONNECTOR { path: connectorName, isExpandable }, varLst: vars, complexType: t, .. } => {
                    let mut varNames: Arc<metamodelica::List<ArcStr>>;
                    let mut isExpandableStr: ArcStr;
                    let mut s: ArcStr = s.clone();
                    let mut s2: ArcStr = s2.clone();
                    varNames = List::map(vars.clone(), (std::sync::Arc::new(getVarName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>))?;
                    isExpandableStr = (if (isExpandable.clone()) {literal!("/* expandable */ ")} else {literal!("")}).clone();
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*isExpandableStr.clone()); __mm_s.push_str(&*AbsynUtil::pathString(connectorName.clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }).clone();
                    s2 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*stringDelimitList(varNames.clone(), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("}")); __mm_s.push_str(&*literal!(" subtype of: ")); __mm_s.push_str(&*printTypeStr(t.clone())); ArcStr::from(__mm_s) }).clone();
                    Ok(((s.clone(), s2.clone()), s.clone(), s2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { s = __wb0; s2 = __wb1; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((literal!(""), unparseType(it.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((s, s2))
}

pub(crate) fn printParamsStr(mut inFuncArgLst: Arc<metamodelica::List<Arc<DAE::FuncArg>>>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ('mc: {
        let __mc_input = inFuncArgLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::FuncArg { name: n, ty: t, .. }, tail: Deref @ metamodelica::List::Nil } => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr = r#str.clone();
                    s1 = (printTypeStr(t.clone())).clone();
                    r#str = stringAppendList(list![(n.clone()).clone(), (literal!(" :: ")).clone(), (s1.clone()).clone()]);
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { r#str = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::FuncArg { name: n, ty: t, .. }, tail: params } => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut r#str: ArcStr = r#str.clone();
                    s1 = (printTypeStr(t.clone())).clone();
                    s2 = (printParamsStr(params.clone())?).clone();
                    r#str = stringAppendList(list![(n.clone()).clone(), (literal!(" :: ")).clone(), (s1.clone()).clone(), (literal!(" * ")).clone(), (s2.clone()).clone()]);
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { r#str = __wb0; break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(r#str)
}

pub fn unparseVarAttr(mut inVar: Arc<DAE::Var>) -> ArcStr {
    let mut outString: ArcStr;
    outString = ('mc: {
        let __mc_input = inVar;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Var { name: n, binding: Deref @ DAE::Binding::EQBOUND { exp: e, .. }, .. } => {
                    let mut res: ArcStr;
                    let mut bindStr: ArcStr;
                    bindStr = (ExpressionBasics::printExpStr(e.clone())?).clone();
                    res = stringAppendList(list![(n.clone()).clone(), (literal!(" = ")).clone(), (bindStr.clone()).clone()]);
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Var { name: n, binding: Deref @ DAE::Binding::VALBOUND { valBound: value, .. }, .. } => {
                    let mut res: ArcStr;
                    let mut valStr: ArcStr;
                    valStr = (ValuesDump::valString(value.clone())?).clone();
                    res = stringAppendList(list![(n.clone()).clone(), (literal!(" = ")).clone(), (valStr.clone()).clone()]);
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    }).clone();
    outString
}

pub fn unparseVar(mut inVar: Arc<DAE::Var>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((::match_deref::match_deref! { match &(inVar) {
        Deref @ DAE::Var { name: n, ty: typ, attributes: Deref @ DAE::Attributes { connectorType: ct, .. }, .. } => {
            let mut t: ArcStr;
            let mut res: ArcStr;
            let mut s: ArcStr;
            s = (connectorTypeStr(ct.clone())).clone();
            t = (unparseType(typ.clone())?).clone();
            res = stringAppendList(list![(literal!("  ")).clone(), (s.clone()).clone(), (t.clone()).clone(), (literal!(" ")).clone(), (n.clone()).clone(), (literal!(";\n")).clone()]);
            res.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

pub(crate) fn connectorTypeStr(mut ct: Arc<DAE::ConnectorType>) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = ('mc: {
        let __mc_input = ct;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ConnectorType::POTENTIAL { .. } => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ConnectorType::FLOW { .. } => {
                    Ok(literal!("flow "))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ConnectorType::STREAM { associatedFlow: _ } => {
                    Ok(literal!("stream "))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    }).clone();
    r#str
}

fn unparseParam(mut inFuncArg: Arc<DAE::FuncArg>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((::match_deref::match_deref! { match &(inFuncArg) {
        Deref @ DAE::FuncArg { name: id, ty, r#const: c, par: p, defaultBinding: None } => {
            let mut tstr: ArcStr;
            let mut res: ArcStr;
            let mut cstr: ArcStr;
            let mut pstr: ArcStr;
            tstr = (unparseType(ty.clone())?).clone();
            cstr = (constStrFriendly(c.clone())?).clone();
            pstr = (dumpVarParallelismStr(p.clone())?).clone();
            res = stringAppendList(list![(tstr.clone()).clone(), (literal!(" ")).clone(), (cstr.clone()).clone(), (pstr.clone()).clone(), (id.clone()).clone()]);
            res.clone()
        },
        Deref @ DAE::FuncArg { name: id, ty, r#const: c, par: p, defaultBinding: Some(exp) } => {
            let mut tstr: ArcStr;
            let mut res: ArcStr;
            let mut cstr: ArcStr;
            let mut estr: ArcStr;
            let mut pstr: ArcStr;
            tstr = (unparseType(ty.clone())?).clone();
            cstr = (constStrFriendly(c.clone())?).clone();
            estr = (ExpressionBasics::printExpStr(exp.clone())?).clone();
            pstr = (dumpVarParallelismStr(p.clone())?).clone();
            res = stringAppendList(list![(tstr.clone()).clone(), (literal!(" ")).clone(), (cstr.clone()).clone(), (pstr.clone()).clone(), (id.clone()).clone(), (literal!(" := ")).clone(), (estr.clone()).clone()]);
            res.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

pub fn printVarStr(mut inVar: Arc<DAE::Var>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ('mc: {
        let __mc_input = inVar;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Var { name: n, attributes: Deref @ DAE::Attributes { variability: var, .. }, ty: typ, binding: bind, .. } => {
                    let mut vs: ArcStr;
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut r#str: ArcStr = r#str.clone();
                    s1 = (printTypeStr(typ.clone())).clone();
                    vs = (SCodeDump::variabilityString(var.clone())?).clone();
                    s2 = (printBindingStr(bind.clone())?).clone();
                    r#str = stringAppendList(list![(s1.clone()).clone(), (literal!(" ")).clone(), (n.clone()).clone(), (literal!(" ")).clone(), (vs.clone()).clone(), (literal!(" ")).clone(), (s2.clone()).clone()]);
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { r#str = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Var { name: n, .. } => {
                    let mut r#str: ArcStr = r#str.clone();
                    r#str = stringAppendList(list![(n.clone()).clone()]);
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { r#str = __wb0; break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(r#str)
}

pub fn printBindingStr(mut inBinding: Arc<DAE::Binding>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((::match_deref::match_deref! { match &(inBinding.clone()) {
        Deref @ DAE::Binding::UNBOUND { .. } => {
            literal!("UNBOUND")
        },
        Deref @ DAE::Binding::EQBOUND { evaluatedExp: None, .. } => {
            let mut r#str: ArcStr;
            let mut str2: ArcStr;
            let mut res: ArcStr;
            let mut str3: ArcStr;
            r#str = (ExpressionBasics::printExpStr(var_field!((*inBinding).exp, DAE::Binding::EQBOUND).clone())?).clone();
            str2 = (printConstStr(var_field!((*inBinding).constant_, DAE::Binding::EQBOUND).clone())?).clone();
            str3 = (printBindingSourceStr(var_field!((*inBinding).source, DAE::Binding::EQBOUND).clone())?).clone();
            res = stringAppendList(list![(literal!("DAE.EQBOUND(")).clone(), (r#str.clone()).clone(), (literal!(", NONE(), ")).clone(), (str2.clone()).clone(), (literal!(", ")).clone(), (str3.clone()).clone(), (literal!(")")).clone()]);
            res.clone()
        },
        Deref @ DAE::Binding::EQBOUND { evaluatedExp: Some(v), .. } => {
            let mut r#str: ArcStr;
            let mut str2: ArcStr;
            let mut res: ArcStr;
            let mut v_str: ArcStr;
            let mut str3: ArcStr;
            r#str = (ExpressionBasics::printExpStr(var_field!((*inBinding).exp, DAE::Binding::EQBOUND).clone())?).clone();
            str2 = (printConstStr(var_field!((*inBinding).constant_, DAE::Binding::EQBOUND).clone())?).clone();
            v_str = (ValuesDump::valString(v.clone())?).clone();
            str3 = (printBindingSourceStr(var_field!((*inBinding).source, DAE::Binding::EQBOUND).clone())?).clone();
            res = stringAppendList(list![(literal!("DAE.EQBOUND(")).clone(), (r#str.clone()).clone(), (literal!(", SOME(")).clone(), (v_str.clone()).clone(), (literal!("), ")).clone(), (str2.clone()).clone(), (literal!(", ")).clone(), (str3.clone()).clone(), (literal!(")")).clone()]);
            res.clone()
        },
        Deref @ DAE::Binding::VALBOUND { valBound: v, .. } => {
            let mut res: ArcStr;
            let mut s: ArcStr;
            let mut str3: ArcStr;
            s = (ValuesDump::unparseValues(list![v.clone()])?).clone();
            str3 = (printBindingSourceStr(var_field!((*inBinding).source, DAE::Binding::VALBOUND).clone())?).clone();
            res = stringAppendList(list![(literal!("DAE.VALBOUND(")).clone(), (s.clone()).clone(), (literal!(", ")).clone(), (str3.clone()).clone(), (literal!(")")).clone()]);
            res.clone()
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TypesDump.printBindingStr")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("FrontEnd/TypesDump.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

pub(crate) fn printFarg(mut inFuncArg: Arc<DAE::FuncArg>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inFuncArg) {
        Deref @ DAE::FuncArg { name: n, ty, .. } => {
            Print::printErrorBuf((printTypeStr(ty.clone())).clone())?;
            Print::printErrorBuf((literal!(" ")).clone())?;
            Print::printErrorBuf((n.clone()).clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn printFargStr(mut inFuncArg: Arc<DAE::FuncArg>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((::match_deref::match_deref! { match &(inFuncArg) {
        Deref @ DAE::FuncArg { name: n, ty, r#const: c, par: _, defaultBinding: _ } => {
            let mut s: ArcStr;
            let mut res: ArcStr;
            let mut cs: ArcStr;
            s = (unparseType(ty.clone())?).clone();
            cs = (constStrFriendly(c.clone())?).clone();
            res = stringAppendList(list![(cs.clone()).clone(), (s.clone()).clone(), (literal!(" ")).clone(), (n.clone()).clone()]);
            res.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

pub fn getTypeName(mut inType: Arc<DAE::Type>) -> ArcStr {
    let mut outString: ArcStr;
    outString = ('mc: {
        let __mc_input = inType;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_INTEGER { .. } => {
                    Ok(literal!("Integer"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_REAL { .. } => {
                    Ok(literal!("Real"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_STRING { .. } => {
                    Ok(literal!("String"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_BOOL { .. } => {
                    Ok(literal!("Boolean"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_CLOCK { .. } => {
                    Ok(literal!("Clock"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_COMPLEX { complexClassType: st, .. } => {
                    let mut n: ArcStr;
                    n = (AbsynUtil::pathString(ClassInfUtil::getStateName(st.clone()), (literal!(".")).clone(), true, false)?).clone();
                    Ok(n.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_SUBTYPE_BASIC { complexClassType: st, .. } => {
                    let mut n: ArcStr;
                    n = (AbsynUtil::pathString(ClassInfUtil::getStateName(st.clone()), (literal!(".")).clone(), true, false)?).clone();
                    Ok(n.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                arrayty @ Deref @ DAE::Type::T_ARRAY { .. } => {
                    let mut dimstr: ArcStr;
                    let mut tystr: ArcStr;
                    let mut r#str: ArcStr;
                    let mut ty: Arc<DAE::Type>;
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
                    (ty, dims) = flattenArrayType(arrayty.clone());
                    dimstr = (ExpressionBasics::dimensionsString(dims.clone())?).clone();
                    tystr = (getTypeName(ty.clone())).clone();
                    r#str = stringAppendList(list![(tystr.clone()).clone(), (literal!("[")).clone(), (dimstr.clone()).clone(), (literal!("]")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METALIST { ty } => {
                    let mut n: ArcStr;
                    n = (getTypeName(ty.clone())).clone();
                    Ok(n.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!("Not nameable type or no type"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    }).clone();
    outString
}

pub(crate) fn constStrFriendly(mut r#const: DAE::Const) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = ((match r#const {
        DAE::Const::C_VAR { .. } => literal!(""),
        DAE::Const::C_PARAM { .. } => literal!("parameter "),
        DAE::Const::C_CONST { .. } => literal!("constant "),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

pub(crate) fn dumpVarParallelismStr(mut inVarParallelism: DAE::VarParallelism) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((match inVarParallelism {
        DAE::VarParallelism::NON_PARALLEL { .. } => literal!(""),
        DAE::VarParallelism::PARGLOBAL { .. } => literal!("parglobal "),
        DAE::VarParallelism::PARLOCAL { .. } => literal!("parlocal "),
    })).clone();
    Ok(outString)
}

pub(crate) fn printBindingSourceStr(mut bindingSource: DAE::BindingSource) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = ((match bindingSource {
        DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE { .. } => literal!("[DEFAULT VALUE]"),
        DAE::BindingSource::BINDING_FROM_START_VALUE { .. } => literal!("[START VALUE]"),
        DAE::BindingSource::BINDING_FROM_RECORD_SUBMODS { .. } => literal!("[RECORD SUBMODS]"),
        DAE::BindingSource::BINDING_FROM_DERIVED_RECORD_DECL { .. } => literal!("[DERIVED RECORD]"),
    })).clone();
    Ok(r#str)
}

pub fn flattenArrayType(mut inType: Arc<DAE::Type>) -> (Arc<DAE::Type>, Arc<metamodelica::List<Arc<DAE::Dimension>>>) {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_ARRAY { .. } => {
            let mut ty: Type;
            let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
            (ty, dims) = flattenArrayType(var_field!((*inType).ty, DAE::Type::T_ARRAY).clone());
            dims = listAppend(var_field!((*inType).dims, DAE::Type::T_ARRAY).clone(), dims.clone());
            return (ty.clone(), dims.clone())
        },
        Deref @ DAE::Type::T_SUBTYPE_BASIC { equalityConstraint: Some(_), .. } => {
            return (inType, metamodelica::nil())
        },
        Deref @ DAE::Type::T_SUBTYPE_BASIC { .. } => {
            { inType = var_field!((*inType).complexType, DAE::Type::T_SUBTYPE_BASIC).clone(); continue '__tco; }
        },
        _ => {
            return (inType, metamodelica::nil())
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

pub fn getVarName(mut v: Arc<DAE::Var>) -> Result<ArcStr> {
    let mut name: ArcStr = arcstr::literal!("");
    name = ((::match_deref::match_deref! { match &(v) {
        Deref @ DAE::Var { name: __esc_name, .. } => {
            name = (*__esc_name).clone();
            name.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(name)
}

pub fn stripTypeVars(mut inType: Arc<DAE::Type>) -> (Arc<DAE::Type>, Arc<metamodelica::List<Arc<DAE::Var>>>) {
    let mut outType: Arc<DAE::Type>;
    let mut outVars: Arc<metamodelica::List<Arc<DAE::Var>>>;
    (outType, outVars) = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_INTEGER { varLst: vars } => {
            (DAE::T_INTEGER_DEFAULT().clone(), vars.clone())
        },
        Deref @ DAE::Type::T_REAL { varLst: vars } => {
            (DAE::T_REAL_DEFAULT().clone(), vars.clone())
        },
        Deref @ DAE::Type::T_STRING { varLst: vars } => {
            (DAE::T_STRING_DEFAULT().clone(), vars.clone())
        },
        Deref @ DAE::Type::T_BOOL { varLst: vars } => {
            (DAE::T_BOOL_DEFAULT().clone(), vars.clone())
        },
        Deref @ DAE::Type::T_TUPLE { types: tys, names: _ } => {
            (Arc::new(DAE::Type::T_TUPLE { types: tys.clone(), names: None }), metamodelica::nil())
        },
        Deref @ DAE::Type::T_ARRAY { ty, dims } => {
            let mut vars: Arc<metamodelica::List<Arc<DAE::Var>>>;
            let mut ty = (*ty).clone();
            (ty, vars) = stripTypeVars(ty.clone());
            (Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: dims.clone() }), vars.clone())
        },
        Deref @ DAE::Type::T_SUBTYPE_BASIC { complexClassType: state, varLst: sub_vars, complexType: ty, equalityConstraint: ec } => {
            let mut vars: Arc<metamodelica::List<Arc<DAE::Var>>>;
            let mut ty = (*ty).clone();
            (ty, vars) = stripTypeVars(ty.clone());
            (Arc::new(DAE::Type::T_SUBTYPE_BASIC { complexClassType: state.clone(), varLst: sub_vars.clone(), complexType: ty.clone(), equalityConstraint: ec.clone() }), vars.clone())
        },
        _ => {
            (inType, metamodelica::nil())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outType, outVars)
}

pub fn printDimensionsStr(mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<ArcStr> {
    let mut res: ArcStr;
    res = stringDelimitList(List::map(dims, (std::sync::Arc::new(ExpressionBasics::dimensionString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Dimension>) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone());
    Ok(res)
}

pub fn printCodeTypeStr(mut ct: DAE::CodeType) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = ((match ct {
        DAE::CodeType::C_EXPRESSION { .. } => literal!("OpenModelica.Code.Expression"),
        DAE::CodeType::C_EXPRESSION_OR_MODIFICATION { .. } => literal!("OpenModelica.Code.ExpressionOrModification"),
        DAE::CodeType::C_MODIFICATION { .. } => literal!("OpenModelica.Code.Modification"),
        DAE::CodeType::C_TYPENAME { .. } => literal!("OpenModelica.Code.TypeName"),
        DAE::CodeType::C_VARIABLENAME { .. } => literal!("OpenModelica.Code.VariableName"),
        DAE::CodeType::C_VARIABLENAMES { .. } => literal!("OpenModelica.Code.VariableNames"),
        _ => literal!("TypesDump.printCodeTypeStr failed"),
    })).clone();
    r#str
}

pub fn getDimensions(mut inType: Arc<DAE::Type>) -> Arc<metamodelica::List<Arc<DAE::Dimension>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_ARRAY { .. } => return listAppend(var_field!((*inType).dims, DAE::Type::T_ARRAY).clone(), getDimensions(var_field!((*inType).ty, DAE::Type::T_ARRAY).clone())),
        Deref @ DAE::Type::T_METAARRAY { .. } => return metamodelica::cons(openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN(), getDimensions(var_field!((*inType).ty, DAE::Type::T_METAARRAY).clone())),
        Deref @ DAE::Type::T_SUBTYPE_BASIC { .. } => { inType = var_field!((*inType).complexType, DAE::Type::T_SUBTYPE_BASIC).clone(); continue '__tco; },
        Deref @ DAE::Type::T_METATYPE { .. } => { inType = var_field!((*inType).ty, DAE::Type::T_METATYPE).clone(); continue '__tco; },
        _ => return metamodelica::nil(),
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

