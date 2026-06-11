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
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_types::DAE;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Program {
    pub name: ArcStr,
    pub functions: Arc<metamodelica::List<Function>>,
}

impl metamodelica::gc::MMTrace for Program {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.functions, __mmv)?;
        Ok(())
    }
}
pub type PROGRAM = Program;


#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Var {
    pub name: ArcStr,
    pub ty: Arc<DAE::Type>,
    /// Used for setjmp semantics in C.
    pub volatile: bool,
}

impl metamodelica::gc::MMTrace for Var {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.ty, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.volatile, __mmv)?;
        Ok(())
    }
}
impl Default for Var {
    fn default() -> Self {
        Self {
            name: Default::default(),
            ty: Default::default(),
            volatile: Default::default(),
        }
    }
}

pub type VAR = Var;


#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct VarBuf {
    pub name: ArcStr,
}

impl metamodelica::gc::MMTrace for VarBuf {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        Ok(())
    }
}
impl Default for VarBuf {
    fn default() -> Self {
        Self {
            name: Default::default(),
        }
    }
}

pub type VARBUF = VarBuf;


#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct VarBufPtr {
    pub name: ArcStr,
}

impl metamodelica::gc::MMTrace for VarBufPtr {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        Ok(())
    }
}
impl Default for VarBufPtr {
    fn default() -> Self {
        Self {
            name: Default::default(),
        }
    }
}

pub type VARBUFPTR = VarBufPtr;


#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum OutVar {
    OUT_VAR {
        var: Var,
    },
    OUT_WILD,
}
impl metamodelica::gc::MMTrace for OutVar {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            OutVar::OUT_VAR { var } => {
                metamodelica::gc::MMTrace::mm_accept(var, __mmv)?;
                Ok(())
            }
            OutVar::OUT_WILD => Ok(()),
        }
    }
}
impl Default for OutVar {
    fn default() -> Self { Self::OUT_WILD }
}
pub use self::OutVar::{OUT_VAR,OUT_WILD};

pub fn varString(mut var: Var) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*DAEDump::daeTypeStr(var.ty.clone())?); __mm_s.push_str(&*literal!(") ")); __mm_s.push_str(&*var.name.clone()); ArcStr::from(__mm_s) }).clone();
    Ok(r#str)
}

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Function {
    pub name: Arc<Absyn::Path>,
    pub locals: Arc<metamodelica::List<Var>>,
    pub localBufs: Arc<metamodelica::List<VarBuf>>,
    pub localBufPtrs: Arc<metamodelica::List<VarBufPtr>>,
    pub inputs: Arc<metamodelica::List<Var>>,
    pub outputs: Arc<metamodelica::List<Var>>,
    pub body: Arc<metamodelica::List<Block>>,
    pub entryId: i32,
    pub exitId: i32,
}

impl metamodelica::gc::MMTrace for Function {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.locals, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.localBufs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.localBufPtrs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.inputs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.outputs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.body, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.entryId, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.exitId, __mmv)?;
        Ok(())
    }
}
impl Default for Function {
    fn default() -> Self {
        Self {
            name: Default::default(),
            locals: Default::default(),
            localBufs: Default::default(),
            localBufPtrs: Default::default(),
            inputs: Default::default(),
            outputs: Default::default(),
            body: Default::default(),
            entryId: Default::default(),
            exitId: Default::default(),
        }
    }
}

pub type FUNCTION = Function;


/// Basic block.
///  No control flow within block.
///  Can branch or jump on exit, called the block's terminator.
#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Block {
    pub id: i32,
    pub stmts: Arc<metamodelica::List<Stmt>>,
    pub terminator: Terminator,
}

impl metamodelica::gc::MMTrace for Block {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.id, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.stmts, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.terminator, __mmv)?;
        Ok(())
    }
}
impl Default for Block {
    fn default() -> Self {
        Self {
            id: Default::default(),
            stmts: Default::default(),
            terminator: Default::default(),
        }
    }
}

pub type BLOCK = Block;


#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Terminator {
    GOTO {
        next: i32,
    },
    BRANCH {
        condition: Var,
        onTrue: i32,
        onFalse: i32,
    },
    CALL {
        func: Arc<Absyn::Path>,
        builtin: bool,
        inputs: Arc<metamodelica::List<Var>>,
        outputs: Arc<metamodelica::List<OutVar>>,
        next: i32,
    },
    RETURN,
    SWITCH {
        condition: Var,
        cases: Arc<metamodelica::List<(i32, i32)>>,
    },
    /// used for fail() stmts
    LONGJMP,
    /// used for match-continue fail() handling
    PUSHJMP {
        /// where to save old jmp_buf
        old_buf: VarBufPtr,
        /// what to use as new jmp_buf
        new_buf: VarBuf,
        /// where to goto next and the setjmp target
        next: i32,
    },
    /// used for match-continue fail() handling
    POPJMP {
        /// what to reset to
        old_buf: VarBufPtr,
        next: i32,
    },
    ASSERT {
        condition: Var,
        message: Var,
        level: Var,
        next: i32,
    },
    TERMINATE {
        message: Var,
    },
}
impl metamodelica::gc::MMTrace for Terminator {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Terminator::GOTO { next } => {
                metamodelica::gc::MMTrace::mm_accept(next, __mmv)?;
                Ok(())
            }
            Terminator::BRANCH { condition, onTrue, onFalse } => {
                metamodelica::gc::MMTrace::mm_accept(condition, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(onTrue, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(onFalse, __mmv)?;
                Ok(())
            }
            Terminator::CALL { func, builtin, inputs, outputs, next } => {
                metamodelica::gc::MMTrace::mm_accept(func, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(builtin, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(inputs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(outputs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(next, __mmv)?;
                Ok(())
            }
            Terminator::RETURN => Ok(()),
            Terminator::SWITCH { condition, cases } => {
                metamodelica::gc::MMTrace::mm_accept(condition, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(cases, __mmv)?;
                Ok(())
            }
            Terminator::LONGJMP => Ok(()),
            Terminator::PUSHJMP { old_buf, new_buf, next } => {
                metamodelica::gc::MMTrace::mm_accept(old_buf, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(new_buf, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(next, __mmv)?;
                Ok(())
            }
            Terminator::POPJMP { old_buf, next } => {
                metamodelica::gc::MMTrace::mm_accept(old_buf, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(next, __mmv)?;
                Ok(())
            }
            Terminator::ASSERT { condition, message, level, next } => {
                metamodelica::gc::MMTrace::mm_accept(condition, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(message, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(level, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(next, __mmv)?;
                Ok(())
            }
            Terminator::TERMINATE { message } => {
                metamodelica::gc::MMTrace::mm_accept(message, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for Terminator {
    fn default() -> Self { Self::RETURN }
}
pub use self::Terminator::{GOTO,BRANCH,CALL,RETURN,SWITCH,LONGJMP,PUSHJMP,POPJMP,ASSERT,TERMINATE};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Stmt {
    NOP,
    ASSIGN {
        dest: Var,
        src: RValue,
    },
}
impl metamodelica::gc::MMTrace for Stmt {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Stmt::NOP => Ok(()),
            Stmt::ASSIGN { dest, src } => {
                metamodelica::gc::MMTrace::mm_accept(dest, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(src, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for Stmt {
    fn default() -> Self { Self::NOP }
}
pub use self::Stmt::{NOP,ASSIGN};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum RValue {
    VARIABLE {
        src: Var,
    },
    UNARYOP {
        op: UnaryOp,
        src: Var,
    },
    BINARYOP {
        op: BinaryOp,
        lsrc: Var,
        rsrc: Var,
    },
    LITERALINTEGER {
        value: i32,
    },
    LITERALREAL {
        value: metamodelica::Real,
    },
    LITERALBOOLEAN {
        value: bool,
    },
    LITERALSTRING {
        value: ArcStr,
    },
    LITERALMETATYPE {
        elements: Arc<metamodelica::List<Var>>,
        ty: Arc<DAE::Type>,
    },
    UNIONTYPEVARIANT {
        src: Var,
    },
    ISSOME {
        src: Var,
    },
    ISCONS {
        src: Var,
    },
    /// get value from metamodelica object
    METAFIELD {
        src: Var,
        index: i32,
        /// type of value
        ty: Arc<DAE::Type>,
    },
}
impl metamodelica::gc::MMTrace for RValue {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            RValue::VARIABLE { src } => {
                metamodelica::gc::MMTrace::mm_accept(src, __mmv)?;
                Ok(())
            }
            RValue::UNARYOP { op, src } => {
                metamodelica::gc::MMTrace::mm_accept(op, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(src, __mmv)?;
                Ok(())
            }
            RValue::BINARYOP { op, lsrc, rsrc } => {
                metamodelica::gc::MMTrace::mm_accept(op, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(lsrc, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(rsrc, __mmv)?;
                Ok(())
            }
            RValue::LITERALINTEGER { value } => {
                metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                Ok(())
            }
            RValue::LITERALREAL { value } => {
                metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                Ok(())
            }
            RValue::LITERALBOOLEAN { value } => {
                metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                Ok(())
            }
            RValue::LITERALSTRING { value } => {
                metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                Ok(())
            }
            RValue::LITERALMETATYPE { elements, ty } => {
                metamodelica::gc::MMTrace::mm_accept(elements, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            RValue::UNIONTYPEVARIANT { src } => {
                metamodelica::gc::MMTrace::mm_accept(src, __mmv)?;
                Ok(())
            }
            RValue::ISSOME { src } => {
                metamodelica::gc::MMTrace::mm_accept(src, __mmv)?;
                Ok(())
            }
            RValue::ISCONS { src } => {
                metamodelica::gc::MMTrace::mm_accept(src, __mmv)?;
                Ok(())
            }
            RValue::METAFIELD { src, index, ty } => {
                metamodelica::gc::MMTrace::mm_accept(src, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for RValue {
    fn default() -> Self {
        Self::VARIABLE {
            src: Default::default(),
        }
    }
}
pub use self::RValue::{VARIABLE,UNARYOP,BINARYOP,LITERALINTEGER,LITERALREAL,LITERALBOOLEAN,LITERALSTRING,LITERALMETATYPE,UNIONTYPEVARIANT,ISSOME,ISCONS,METAFIELD};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum UnaryOp {
    MOVE,
    UMINUS,
    NOT,
    UNBOX,
    BOX,
}
impl metamodelica::gc::MMTrace for UnaryOp {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            UnaryOp::MOVE => Ok(()),
            UnaryOp::UMINUS => Ok(()),
            UnaryOp::NOT => Ok(()),
            UnaryOp::UNBOX => Ok(()),
            UnaryOp::BOX => Ok(()),
        }
    }
}
pub use self::UnaryOp::{MOVE,UMINUS,NOT,UNBOX,BOX};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum BinaryOp {
    ADD,
    SUB,
    MUL,
    DIV,
    POW,
    LESS,
    LESSEQ,
    GREATER,
    GREATEREQ,
    EQUAL,
    NEQUAL,
}
impl metamodelica::gc::MMTrace for BinaryOp {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            BinaryOp::ADD => Ok(()),
            BinaryOp::SUB => Ok(()),
            BinaryOp::MUL => Ok(()),
            BinaryOp::DIV => Ok(()),
            BinaryOp::POW => Ok(()),
            BinaryOp::LESS => Ok(()),
            BinaryOp::LESSEQ => Ok(()),
            BinaryOp::GREATER => Ok(()),
            BinaryOp::GREATEREQ => Ok(()),
            BinaryOp::EQUAL => Ok(()),
            BinaryOp::NEQUAL => Ok(()),
        }
    }
}
pub use self::BinaryOp::{ADD,SUB,MUL,DIV,POW,LESS,LESSEQ,GREATER,GREATEREQ,EQUAL,NEQUAL};

