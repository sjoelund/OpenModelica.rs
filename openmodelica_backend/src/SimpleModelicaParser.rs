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

use crate::LexerModelicaDiff::Token;
use crate::LexerModelicaDiff::TokenId;
use crate::LexerModelicaDiff::modelicaDiffTokenEq;
use crate::LexerModelicaDiff::printToken;
use crate::LexerModelicaDiff::tokenContent;
use crate::LexerModelicaDiff;
use openmodelica_util::AvlSetString;
use openmodelica_util::DiffAlgorithm::Diff;
use openmodelica_util::DiffAlgorithm::diff;
use openmodelica_util::DiffAlgorithm;
use openmodelica_util::Error;
use openmodelica_util::Print;
use openmodelica_util::StackOverflow;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::List;

pub static newlineToken: std::sync::LazyLock<Token> = std::sync::LazyLock::new(|| { Token { fileName: (literal!("")).clone(), id: TokenId::NEWLINE.clone(), fileContents: (literal!("\n")).clone(), byteOffset: 1, length: 1, lineNumberStart: 1, columnNumberStart: 1, lineNumberEnd: 1, columnNumberEnd: 1 } });

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum ParseTree {
    EMPTY,
    NODE {
        label: Arc<ParseTree>,
        nodes: Arc<metamodelica::List<Arc<ParseTree>>>,
    },
    LEAF {
        token: Token,
    },
}
impl metamodelica::gc::MMTrace for ParseTree {
    fn mm_accept<__MMV: metamodelica::gc::dumpster::Visitor>(&self, __mmv: &mut __MMV) -> Result<(), ()> {
        match self {
            ParseTree::EMPTY => Ok(()),
            ParseTree::NODE { label, nodes } => {
                metamodelica::gc::MMTrace::mm_accept(label, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(nodes, __mmv)?;
                Ok(())
            }
            ParseTree::LEAF { token } => {
                metamodelica::gc::MMTrace::mm_accept(token, __mmv)?;
                Ok(())
            }
        }
    }
}
impl ParseTree {
    pub fn interned_EMPTY() -> Arc<ParseTree> {
        static INTERNED: std::sync::LazyLock<Arc<ParseTree>> = std::sync::LazyLock::new(|| Arc::new(ParseTree::EMPTY));
        (*INTERNED).clone()
    }
}
pub fn interned_EMPTY() -> Arc<ParseTree> { ParseTree::interned_EMPTY() }
impl Default for ParseTree {
    fn default() -> Self { Self::EMPTY }
}
pub use self::ParseTree::{EMPTY,NODE,LEAF};

pub fn parseTreeStr(mut trees: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut i: i32;
    i = Print::saveAndClearBuf()?;
    match '__try0: {
        for mut tree in &*trees.clone() {
            let mut tree = tree.clone();
            unwrap_break_err!(parseTreeStrWork(tree.clone()), '__try0);
        }
        r#str = (unwrap_break_err!(Print::getString(), '__try0)).clone();
        unwrap_break_err!(Print::restoreBuf(i.clone()), '__try0);
        Ok::<_, anyhow::Error>((r#str.clone(),))
    } {
        Ok((__try0_o0,)) => {
            r#str = __try0_o0;
        }
        Err(__try0_err) => {
            Print::restoreBuf(i.clone())?;
            return Err(__try0_err);
        }
    }
    Ok(r#str)
}

pub fn treeDiff(mut t1: Arc<metamodelica::List<Arc<ParseTree>>>, mut t2: Arc<metamodelica::List<Arc<ParseTree>>>, mut nTokens: i32) -> Result<Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>>> {
    let mut res: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>>;
    let mut within1: Arc<ParseTree>;
    let mut within2: Arc<ParseTree>;
    let mut t1_updated: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut t2_updated: Arc<metamodelica::List<Arc<ParseTree>>>;
    within1 = findWithin(t1.clone())?;
    within2 = findWithin(t2.clone())?;
    (t1_updated, t2_updated) = (::match_deref::match_deref! { match &((within1.clone(), within2.clone())) {
        (Deref @ ParseTree::EMPTY { .. }, Deref @ ParseTree::EMPTY { .. }) => (t1.clone(), t2.clone()),
        (_, Deref @ ParseTree::EMPTY { .. }) => (t1.clone(), metamodelica::cons(within1.clone(), metamodelica::cons(Arc::new(ParseTree::LEAF { token: newlineToken.clone() }), t2.clone()))),
        (Deref @ ParseTree::EMPTY { .. }, _) => (metamodelica::cons(within2.clone(), metamodelica::cons(Arc::new(ParseTree::LEAF { token: newlineToken.clone() }), metamodelica::cons(Arc::new(ParseTree::LEAF { token: newlineToken.clone() }), t1.clone()))), t2.clone()),
        _ => (t1.clone(), t2.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res = treeDiffWork1(t1_updated.clone(), t2_updated.clone(), nTokens.clone())?;
    Ok(res)
}

pub type CmpParseTreeFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<ParseTree>, Arc<ParseTree>) -> Result<bool> + 'static>;

pub fn parseTreeNodeStr(mut tree: Arc<ParseTree>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut i: i32;
    i = Print::saveAndClearBuf()?;
    match '__try0: {
        unwrap_break_err!(parseTreeStrWork(tree.clone()), '__try0);
        r#str = (unwrap_break_err!(Print::getString(), '__try0)).clone();
        unwrap_break_err!(Print::restoreBuf(i.clone()), '__try0);
        Ok::<_, anyhow::Error>((r#str.clone(),))
    } {
        Ok((__try0_o0,)) => {
            r#str = __try0_o0;
        }
        Err(__try0_err) => {
            Print::restoreBuf(i.clone())?;
            return Err(__try0_err);
        }
    }
    Ok(r#str)
}

pub type partialParser = std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> + 'static>;

pub fn stored_definition(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::WITHIN.clone())?;
    if b.clone() {
        (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), First::name.clone(), false)?;
        if b.clone() {
            (tokens, tree) = name(tokens.clone(), tree.clone())?;
        }
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::SEMICOLON.clone())?;
        outTree = metamodelica::cons(makeNode(tree.clone().reverse(), Arc::new(ParseTree::LEAF { token: makeToken(TokenId::IDENT.clone(), (literal!("$within")).clone()) })), metamodelica::nil());
        tree = metamodelica::nil();
    } else {
        outTree = metamodelica::nil();
    }
    (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), First::class_definition.clone(), false)?;
    while b.clone() {
        (tokens, tree, _) = scanOpt(tokens.clone(), tree.clone(), TokenId::FINAL.clone())?;
        (tokens, tree, _) = class_definition(tokens.clone(), tree.clone())?;
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::SEMICOLON.clone())?;
        (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), First::class_definition.clone(), false)?;
        outTree = metamodelica::cons(makeNode(tree.clone().reverse(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY()), outTree.clone());
        tree = metamodelica::nil();
    }
    (tokens, tree) = eatWhitespace(tokens.clone(), tree.clone())?;
    if !(tokens.clone().is_empty()) {
        error(tokens.clone(), tree.clone(), metamodelica::nil())?;
    }
    outTree = metamodelica::cons(makeNode(listAppend(tree.clone(), listAppend(outTree.clone(), inTree.clone())).reverse(), Arc::new(ParseTree::LEAF { token: makeToken(TokenId::IDENT.clone(), (literal!("$program")).clone()) })), metamodelica::nil());
    Ok((tokens, outTree))
}

fn class_definition(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>, Arc<ParseTree>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut nodeName: Arc<ParseTree>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    tree = metamodelica::nil();
    (tokens, tree, _) = scanOpt(tokens.clone(), tree.clone(), TokenId::ENCAPSULATED.clone())?;
    (tokens, tree) = class_prefixes(tokens.clone(), tree.clone())?;
    (tokens, tree, nodeName) = class_specifier(tokens.clone(), tree.clone())?;
    outTree = metamodelica::cons(makeNode(tree.clone().reverse(), nodeName.clone()), inTree.clone());
    Ok((tokens, outTree, nodeName))
}

fn class_prefixes(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut id: TokenId;
    let mut b: bool = false;
    (tokens, tree, _) = scanOpt(tokens.clone(), tree.clone(), TokenId::PARTIAL.clone())?;
    (tokens, tree, id) = peek(tokens.clone(), tree.clone())?;
    let () = (match id.clone() {
        TokenId::OPERATOR { .. } => {
            (tokens, tree) = consume(tokens.clone(), tree.clone())?;
            (tokens, tree, _) = LA1(tokens.clone(), tree.clone(), list![TokenId::RECORD.clone(), TokenId::FUNCTION.clone()], true)?;
            ()
        },
        TokenId::EXPANDABLE => {
            (tokens, tree) = consume(tokens.clone(), tree.clone())?;
            (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::CONNECTOR.clone())?;
            ()
        },
        mut id if (listMember(id.clone(), list![TokenId::PURE.clone(), TokenId::IMPURE.clone()])) => {
            (tokens, tree) = consume(tokens.clone(), tree.clone())?;
            (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::OPERATOR.clone())?;
            (tokens, tree) = scanOneOf(tokens.clone(), tree.clone(), if (b.clone()) {list![TokenId::FUNCTION.clone()]} else {list![TokenId::FUNCTION.clone(), TokenId::RECORD.clone()]})?;
            ()
        },
        _ => {
            (tokens, tree) = scanOneOf(tokens.clone(), tree.clone(), list![TokenId::CLASS.clone(), TokenId::MODEL.clone(), TokenId::RECORD.clone(), TokenId::BLOCK.clone(), TokenId::CONNECTOR.clone(), TokenId::TYPE.clone(), TokenId::PACKAGE.clone(), TokenId::FUNCTION.clone(), TokenId::OPERATOR.clone()])?;
            ()
        },
    });
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree))
}

fn class_specifier(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>, Arc<ParseTree>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut nodeName: Arc<ParseTree>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    tree = inTree.clone();
    (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::IDENT.clone())?;
    let __pa0 = ::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    nodeName = __pa0.clone();
    nodeName = parseTreeFilterWhitespace(nodeName.clone());
    if b.clone() {
        (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::EQUALS.clone())?;
        if b.clone() {
            (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::DER.clone())?;
            if b.clone() {
                (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::LPAR.clone())?;
                (tokens, tree) = name(tokens.clone(), tree.clone())?;
                (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::COMMA.clone())?;
                (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::IDENT.clone())?;
                loop {
                    (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::COMMA.clone())?;
                    if !(b.clone()) {
                        break;
                    }
                    (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::IDENT.clone())?;
                }
                (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::RPAR.clone())?;
                (tokens, tree) = comment(tokens.clone(), tree.clone())?;
            } else {
                (tokens, tree) = short_class_specifier1(tokens.clone(), tree.clone())?;
            }
        } else {
            (tokens, tree) = string_comment(tokens.clone(), tree.clone())?;
            (tokens, tree) = composition(tokens.clone(), tree.clone())?;
            (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::END.clone())?;
            (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::IDENT.clone())?;
        }
    } else {
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::EXTENDS.clone())?;
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::IDENT.clone())?;
        (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), First::class_modification.clone(), false)?;
        if b.clone() {
            (tokens, tree) = class_modification(tokens.clone(), tree.clone())?;
        }
        (tokens, tree) = string_comment(tokens.clone(), tree.clone())?;
        (tokens, tree) = composition(tokens.clone(), tree.clone())?;
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::END.clone())?;
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::IDENT.clone())?;
    }
    outTree = tree.clone();
    Ok((tokens, outTree, nodeName))
}

fn short_class_specifier1(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::ENUMERATION.clone())?;
    if b.clone() {
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::LPAR.clone())?;
        (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::COLON.clone())?;
        if !(b.clone()) {
            loop {
                (tokens, tree) = enumeration_literal(tokens.clone(), tree.clone())?;
                (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::COMMA.clone())?;
                if !(b.clone()) {
                    break;
                }
            }
        }
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::RPAR.clone())?;
    } else {
        (tokens, tree) = base_prefix(tokens.clone(), tree.clone())?;
        (tokens, tree) = name(tokens.clone(), tree.clone())?;
        (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), list![TokenId::LBRACK.clone()], false)?;
        if b.clone() {
            (tokens, tree) = array_subscripts(tokens.clone(), tree.clone())?;
        }
        (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), First::class_modification.clone(), false)?;
        if b.clone() {
            (tokens, tree) = class_modification(tokens.clone(), tree.clone())?;
        }
    }
    (tokens, tree) = comment(tokens.clone(), tree.clone())?;
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree))
}

fn enumeration_literal(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::IDENT.clone())?;
    (tokens, tree) = comment(tokens.clone(), tree.clone())?;
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree))
}

fn composition(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut id: TokenId;
    let mut b: bool;
    (tokens, tree) = element_list(tokens.clone(), tree.clone())?;
    loop {
        (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), list![TokenId::PROTECTED.clone(), TokenId::PUBLIC.clone(), TokenId::INITIAL.clone(), TokenId::EQUATION.clone(), TokenId::ALGORITHM.clone()], false)?;
        if !(b.clone()) {
            break;
        }
        (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), list![TokenId::PROTECTED.clone(), TokenId::PUBLIC.clone()], true)?;
        if b.clone() {
            (tokens, tree) = element_list(tokens.clone(), tree.clone())?;
        } else {
            (tokens, tree, _) = scanOpt(tokens.clone(), tree.clone(), TokenId::INITIAL.clone())?;
            (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), list![TokenId::ALGORITHM.clone()], false)?;
            if b.clone() {
                (tokens, tree) = algorithm_section(tokens.clone(), tree.clone())?;
            } else {
                (tokens, tree) = equation_section(tokens.clone(), tree.clone())?;
            }
        }
    }
    (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::EXTERNAL.clone())?;
    if b.clone() {
        (tokens, tree, _) = scanOpt(tokens.clone(), tree.clone(), TokenId::STRING.clone())?;
        (tokens, tree, id) = peek(tokens.clone(), tree.clone())?;
        if !(id.clone() == TokenId::ANNOTATION.clone() || id.clone() == TokenId::SEMICOLON.clone()) {
            (tokens, tree) = external_function_call(tokens.clone(), tree.clone())?;
        }
        (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), First::_annotation.clone(), false)?;
        if b.clone() {
            (tokens, tree) = _annotation(tokens.clone(), tree.clone())?;
        }
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::SEMICOLON.clone())?;
    }
    b = true;
    while b.clone() {
        (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), First::_annotation.clone(), false)?;
        if b.clone() {
            (tokens, tree) = _annotation(tokens.clone(), tree.clone())?;
            (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::SEMICOLON.clone())?;
        }
    }
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree))
}

fn external_function_call(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree, b) = LAk(tokens.clone(), tree.clone(), list![list![TokenId::IDENT.clone()], list![TokenId::LPAR.clone()]])?;
    if !(b.clone()) {
        (tokens, tree) = component_reference(tokens.clone(), tree.clone())?;
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::EQUALS.clone())?;
    }
    (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::IDENT.clone())?;
    (tokens, tree) = output_expression_list(tokens.clone(), tree.clone())?;
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree))
}

fn algorithm_section(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    (tokens, tree, _) = scanOpt(tokens.clone(), tree.clone(), TokenId::INITIAL.clone())?;
    (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::ALGORITHM.clone())?;
    (tokens, tree) = statement_list(tokens.clone(), tree.clone())?;
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), Arc::new(ParseTree::LEAF { token: makeToken(TokenId::IDENT.clone(), (literal!("$algorithm_section")).clone()) }));
    Ok((tokens, outTree))
}

fn statement(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>, ArcStr)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut label: ArcStr = literal!("$statement");
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut id: TokenId;
    let mut b: bool;
    (tokens, tree, id) = peek(tokens.clone(), tree.clone())?;
    if id.clone() == TokenId::BREAK.clone() || id.clone() == TokenId::RETURN.clone() {
        (tokens, tree) = consume(tokens.clone(), tree.clone())?;
        label = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$")); __mm_s.push_str(&*ArcStr::from(::std::format!("{:?}", id.clone()))); ArcStr::from(__mm_s) }).clone();
    } else if listMember(id.clone(), First::component_reference.clone()) {
        (tokens, tree) = component_reference(tokens.clone(), tree.clone())?;
        (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::ASSIGN.clone())?;
        if b.clone() {
            (tokens, tree) = expression(tokens.clone(), tree.clone())?;
            label = (literal!("$assign")).clone();
        } else {
            (tokens, tree) = function_call_args(tokens.clone(), tree.clone())?;
            label = (literal!("$statement_call")).clone();
        }
    } else if id.clone() == TokenId::IF.clone() {
        (tokens, tree) = consume(tokens.clone(), tree.clone())?;
        (tokens, tree) = expression(tokens.clone(), tree.clone())?;
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::THEN.clone())?;
        (tokens, tree) = statement_list(tokens.clone(), tree.clone())?;
        loop {
            (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::ELSEIF.clone())?;
            if !(b.clone()) {
                break;
            }
            (tokens, tree) = expression(tokens.clone(), tree.clone())?;
            (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::THEN.clone())?;
            (tokens, tree) = statement_list(tokens.clone(), tree.clone())?;
        }
        (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::ELSE.clone())?;
        if b.clone() {
            (tokens, tree) = statement_list(tokens.clone(), tree.clone())?;
        }
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::END.clone())?;
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::IF.clone())?;
        label = (literal!("$if")).clone();
    } else if id.clone() == TokenId::WHEN.clone() {
        (tokens, tree) = consume(tokens.clone(), tree.clone())?;
        (tokens, tree) = expression(tokens.clone(), tree.clone())?;
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::THEN.clone())?;
        (tokens, tree) = statement_list(tokens.clone(), tree.clone())?;
        loop {
            (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::ELSEWHEN.clone())?;
            if !(b.clone()) {
                break;
            }
            (tokens, tree) = expression(tokens.clone(), tree.clone())?;
            (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::THEN.clone())?;
            (tokens, tree) = statement_list(tokens.clone(), tree.clone())?;
        }
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::END.clone())?;
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::WHEN.clone())?;
        label = (literal!("$when")).clone();
    } else if id.clone() == TokenId::FOR.clone() {
        (tokens, tree) = consume(tokens.clone(), tree.clone())?;
        (tokens, tree) = for_indices(tokens.clone(), tree.clone())?;
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::LOOP.clone())?;
        (tokens, tree) = statement_list(tokens.clone(), tree.clone())?;
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::END.clone())?;
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::FOR.clone())?;
        label = (literal!("$for")).clone();
    } else if id.clone() == TokenId::WHILE.clone() {
        (tokens, tree) = consume(tokens.clone(), tree.clone())?;
        (tokens, tree) = expression(tokens.clone(), tree.clone())?;
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::LOOP.clone())?;
        (tokens, tree) = statement_list(tokens.clone(), tree.clone())?;
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::END.clone())?;
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::WHILE.clone())?;
        label = (literal!("$while")).clone();
    } else {
        (tokens, tree) = expression(tokens.clone(), tree.clone())?;
        (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::ASSIGN.clone())?;
        if b.clone() {
            (tokens, tree) = expression(tokens.clone(), tree.clone())?;
        }
        label = (literal!("$assign_expression")).clone();
    }
    (tokens, tree) = comment(tokens.clone(), tree.clone())?;
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree, label))
}

fn statement_list(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    let mut label: ArcStr;
    outTree = metamodelica::nil();
    loop {
        (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), Follow::statement_equation.clone(), false)?;
        if b.clone() {
            break;
        }
        (tokens, tree, label) = statement(tokens.clone(), tree.clone())?;
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::SEMICOLON.clone())?;
        outTree = metamodelica::cons(makeNode(tree.clone().reverse(), Arc::new(ParseTree::LEAF { token: makeToken(TokenId::IDENT.clone(), (label.clone()).clone()) })), outTree.clone());
        tree = metamodelica::nil();
    }
    outTree = listAppend(tree.clone(), listAppend(outTree.clone(), inTree.clone()));
    Ok((tokens, outTree))
}

fn equation_section(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    (tokens, tree, _) = scanOpt(tokens.clone(), tree.clone(), TokenId::INITIAL.clone())?;
    (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::EQUATION.clone())?;
    (tokens, tree) = equation_list(tokens.clone(), tree.clone())?;
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), Arc::new(ParseTree::LEAF { token: makeToken(TokenId::IDENT.clone(), (literal!("$equation_section")).clone()) }));
    Ok((tokens, outTree))
}

fn _equation(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>, ArcStr)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut label: ArcStr = literal!("$equation");
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut id: TokenId;
    let mut b: bool;
    (tokens, tree, id) = peek(tokens.clone(), tree.clone())?;
    if id.clone() == TokenId::IF.clone() {
        (tokens, tree) = consume(tokens.clone(), tree.clone())?;
        (tokens, tree) = expression(tokens.clone(), tree.clone())?;
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::THEN.clone())?;
        (tokens, tree) = equation_list(tokens.clone(), tree.clone())?;
        loop {
            (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::ELSEIF.clone())?;
            if !(b.clone()) {
                break;
            }
            (tokens, tree) = expression(tokens.clone(), tree.clone())?;
            (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::THEN.clone())?;
            (tokens, tree) = equation_list(tokens.clone(), tree.clone())?;
        }
        (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::ELSE.clone())?;
        if b.clone() {
            (tokens, tree) = equation_list(tokens.clone(), tree.clone())?;
        }
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::END.clone())?;
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::IF.clone())?;
        label = (literal!("$if_equation")).clone();
    } else if id.clone() == TokenId::WHEN.clone() {
        (tokens, tree) = consume(tokens.clone(), tree.clone())?;
        (tokens, tree) = expression(tokens.clone(), tree.clone())?;
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::THEN.clone())?;
        (tokens, tree) = equation_list(tokens.clone(), tree.clone())?;
        loop {
            (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::ELSEWHEN.clone())?;
            if !(b.clone()) {
                break;
            }
            (tokens, tree) = expression(tokens.clone(), tree.clone())?;
            (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::THEN.clone())?;
            (tokens, tree) = equation_list(tokens.clone(), tree.clone())?;
        }
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::END.clone())?;
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::WHEN.clone())?;
        label = (literal!("$when_equation")).clone();
    } else if id.clone() == TokenId::FOR.clone() {
        (tokens, tree) = consume(tokens.clone(), tree.clone())?;
        (tokens, tree) = for_indices(tokens.clone(), tree.clone())?;
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::LOOP.clone())?;
        (tokens, tree) = equation_list(tokens.clone(), tree.clone())?;
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::END.clone())?;
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::FOR.clone())?;
        label = (literal!("$for_equation")).clone();
    } else if id.clone() == TokenId::CONNECT.clone() {
        (tokens, tree) = consume(tokens.clone(), tree.clone())?;
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::LPAR.clone())?;
        (tokens, tree) = component_reference(tokens.clone(), tree.clone())?;
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::COMMA.clone())?;
        (tokens, tree) = component_reference(tokens.clone(), tree.clone())?;
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::RPAR.clone())?;
        label = (literal!("$connect_equation")).clone();
    } else {
        (tokens, tree) = expression(tokens.clone(), tree.clone())?;
        (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::EQUALS.clone())?;
        if b.clone() {
            (tokens, tree) = expression(tokens.clone(), tree.clone())?;
            label = (literal!("$equality_equation")).clone();
        } else {
            label = (literal!("$singleton_equation")).clone();
        }
    }
    (tokens, tree) = comment(tokens.clone(), tree.clone())?;
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree, label))
}

fn equation_list(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    let mut label: ArcStr;
    outTree = metamodelica::nil();
    loop {
        (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), Follow::statement_equation.clone(), false)?;
        if b.clone() {
            break;
        }
        (tokens, tree, label) = _equation(tokens.clone(), tree.clone())?;
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::SEMICOLON.clone())?;
        outTree = metamodelica::cons(makeNode(tree.clone().reverse(), Arc::new(ParseTree::LEAF { token: makeToken(TokenId::IDENT.clone(), (label.clone()).clone()) })), outTree.clone());
        tree = metamodelica::nil();
    }
    outTree = listAppend(tree.clone(), listAppend(outTree.clone(), inTree.clone()));
    Ok((tokens, outTree))
}

fn element_list(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    let mut isAnnotation: bool;
    let mut nodeName: Arc<ParseTree>;
    outTree = metamodelica::nil();
    loop {
        (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), First::element.clone(), false)?;
        if !(b.clone()) {
            break;
        }
        (tokens, tree, nodeName, isAnnotation) = element(tokens.clone(), tree.clone())?;
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::SEMICOLON.clone())?;
        if !(isAnnotation.clone()) {
            outTree = metamodelica::cons(makeNode(tree.clone().reverse(), nodeName.clone()), outTree.clone());
            tree = metamodelica::nil();
        }
    }
    outTree = listAppend(tree.clone(), listAppend(outTree.clone(), inTree.clone()));
    Ok((tokens, outTree))
}

fn element(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>, Arc<ParseTree>, bool)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut nodeName: Arc<ParseTree> = Arc::new(ParseTree::LEAF { token: makeToken(TokenId::IDENT.clone(), (literal!("$element")).clone()) });
    let mut isAnnotation: bool = false;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut id: TokenId;
    let mut b: bool = false;
    let mut b1: bool = false;
    (tokens, tree, id) = peek(tokens.clone(), tree.clone())?;
    nodeName = (match id.clone() {
        TokenId::IMPORT { .. } => {
            (tokens, tree) = import_clause(tokens.clone(), tree.clone())?;
            Arc::new(ParseTree::LEAF { token: makeToken(TokenId::IDENT.clone(), (literal!("$import")).clone()) })
        },
        TokenId::EXTENDS { .. } => {
            (tokens, tree) = extends_clause(tokens.clone(), tree.clone())?;
            Arc::new(ParseTree::LEAF { token: makeToken(TokenId::IDENT.clone(), (literal!("$extends")).clone()) })
        },
        TokenId::ANNOTATION { .. } => {
            (tokens, tree) = _annotation(tokens.clone(), tree.clone())?;
            isAnnotation = true;
            Arc::new(ParseTree::LEAF { token: makeToken(TokenId::IDENT.clone(), (literal!("$annotation")).clone()) })
        },
        _ => {
            (tokens, tree, _) = scanOpt(tokens.clone(), tree.clone(), TokenId::REDECLARE.clone())?;
            (tokens, tree, _) = scanOpt(tokens.clone(), tree.clone(), TokenId::FINAL.clone())?;
            (tokens, tree, _) = scanOpt(tokens.clone(), tree.clone(), TokenId::INNER.clone())?;
            (tokens, tree, _) = scanOpt(tokens.clone(), tree.clone(), TokenId::OUTER.clone())?;
            (tokens, tree, b1) = scanOpt(tokens.clone(), tree.clone(), TokenId::REPLACEABLE.clone())?;
            (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), First::class_definition.clone(), false)?;
            if b.clone() {
                (tokens, tree, nodeName) = class_definition(tokens.clone(), tree.clone())?;
            } else {
                (tokens, tree, nodeName) = component_clause(tokens.clone(), tree.clone())?;
            }
            if b1.clone() {
                (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), list![TokenId::CONSTRAINEDBY.clone()], false)?;
                if b.clone() {
                    (tokens, tree) = constraining_clause(tokens.clone(), tree.clone())?;
                    (tokens, tree) = comment(tokens.clone(), tree.clone())?;
                }
            }
            nodeName.clone()
        },
    });
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree, nodeName, isAnnotation))
}

fn constraining_clause(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::CONSTRAINEDBY.clone())?;
    (tokens, tree) = name(tokens.clone(), tree.clone())?;
    (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), First::class_modification.clone(), false)?;
    if b.clone() {
        (tokens, tree) = class_modification(tokens.clone(), tree.clone())?;
    }
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree))
}

fn component_clause(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>, Arc<ParseTree>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut nodeName: Arc<ParseTree>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    let mut nodeNames: Arc<metamodelica::List<Arc<ParseTree>>>;
    (tokens, tree) = type_prefix(tokens.clone(), tree.clone())?;
    (tokens, tree) = type_specifier(tokens.clone(), tree.clone())?;
    (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), list![TokenId::LBRACK.clone()], false)?;
    if b.clone() {
        (tokens, tree) = array_subscripts(tokens.clone(), tree.clone())?;
    }
    tree = metamodelica::cons(makeNode(tree.clone().reverse(), Arc::new(ParseTree::LEAF { token: makeToken(TokenId::IDENT.clone(), (literal!("$type_specifier")).clone()) })), metamodelica::nil());
    (tokens, tree, nodeNames) = component_list(tokens.clone(), tree.clone())?;
    nodeName = Arc::new(ParseTree::LEAF { token: makeToken(TokenId::IDENT.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$component:")); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut name in (nodeNames.clone()).into_iter().cloned() {
            let __x = parseTreeStr(metamodelica::cons(name.clone(), metamodelica::nil()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!(",")).clone())); ArcStr::from(__mm_s) }).clone()) });
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), nodeName.clone());
    Ok((tokens, outTree, nodeName))
}

fn import_clause(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::IMPORT.clone())?;
    (tokens, tree, b) = LAk(tokens.clone(), tree.clone(), list![list![TokenId::IDENT.clone()], list![TokenId::EQUALS.clone()]])?;
    if b.clone() {
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::IDENT.clone())?;
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::EQUALS.clone())?;
        (tokens, tree) = name(tokens.clone(), tree.clone())?;
    } else {
        (tokens, tree) = name(tokens.clone(), tree.clone())?;
        (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::STAR_EW.clone())?;
        if !(b.clone()) {
            (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::DOT.clone())?;
            if b.clone() {
                (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::LBRACE.clone())?;
                (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::IDENT.clone())?;
                loop {
                    (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::COMMA.clone())?;
                    if !(b.clone()) {
                        break;
                    }
                    (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::IDENT.clone())?;
                }
                (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::RBRACE.clone())?;
            }
        }
    }
    (tokens, tree) = comment(tokens.clone(), tree.clone())?;
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree))
}

use name as type_specifier;

use type_prefix as base_prefix;

fn type_prefix(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    (tokens, tree, _) = LA1(tokens.clone(), tree.clone(), list![TokenId::FLOW.clone(), TokenId::STREAM.clone()], true)?;
    (tokens, tree, _) = LA1(tokens.clone(), tree.clone(), list![TokenId::DISCRETE.clone(), TokenId::PARAMETER.clone(), TokenId::CONSTANT.clone()], true)?;
    (tokens, tree, _) = LA1(tokens.clone(), tree.clone(), list![TokenId::INPUT.clone(), TokenId::OUTPUT.clone()], true)?;
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree))
}

fn array_subscripts(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::LBRACK.clone())?;
    (tokens, tree) = subscript(tokens.clone(), tree.clone())?;
    loop {
        (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::COMMA.clone())?;
        if !(b.clone()) {
            break;
        }
        (tokens, tree) = subscript(tokens.clone(), tree.clone())?;
    }
    (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::RBRACK.clone())?;
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree))
}

fn subscript(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::COLON.clone())?;
    if !(b.clone()) {
        (tokens, tree) = expression(tokens.clone(), tree.clone())?;
    }
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree))
}

fn component_list(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut nodeNames: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    let mut nodeName: Arc<ParseTree>;
    (tokens, tree, nodeName) = component_declaration(tokens.clone(), tree.clone())?;
    nodeNames = metamodelica::cons(nodeName.clone(), nodeNames.clone());
    loop {
        (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::COMMA.clone())?;
        if !(b.clone()) {
            break;
        }
        (tokens, tree, nodeName) = component_declaration(tokens.clone(), tree.clone())?;
        nodeNames = metamodelica::cons(nodeName.clone(), nodeNames.clone());
    }
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree, nodeNames))
}

fn component_declaration(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>, Arc<ParseTree>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut nodeName: Arc<ParseTree>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree, nodeName) = declaration(tokens.clone(), tree.clone())?;
    (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::IF.clone())?;
    if b.clone() {
        (tokens, tree) = expression(tokens.clone(), tree.clone())?;
    }
    (tokens, tree) = comment(tokens.clone(), tree.clone())?;
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), nodeName.clone());
    Ok((tokens, outTree, nodeName))
}

fn declaration(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>, Arc<ParseTree>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut nodeName: Arc<ParseTree>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::IDENT.clone())?;
    let __pa0 = ::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    nodeName = __pa0.clone();
    nodeName = parseTreeFilterWhitespace(nodeName.clone());
    (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), list![TokenId::LBRACK.clone()], false)?;
    if b.clone() {
        (tokens, tree) = array_subscripts(tokens.clone(), tree.clone())?;
    }
    (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), First::modification.clone(), false)?;
    if b.clone() {
        (tokens, tree) = modification(tokens.clone(), tree.clone())?;
    }
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree, nodeName))
}

fn component_clause1(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>, Arc<ParseTree>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut nodeName: Arc<ParseTree>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    (tokens, tree) = type_prefix(tokens.clone(), tree.clone())?;
    (tokens, tree) = type_specifier(tokens.clone(), tree.clone())?;
    (tokens, tree, nodeName) = component_declaration1(tokens.clone(), tree.clone())?;
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree, nodeName))
}

fn component_declaration1(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>, Arc<ParseTree>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut nodeName: Arc<ParseTree>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    (tokens, tree, nodeName) = declaration(tokens.clone(), tree.clone())?;
    (tokens, tree) = comment(tokens.clone(), tree.clone())?;
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree, nodeName))
}

fn extends_clause(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::EXTENDS.clone())?;
    (tokens, tree) = name(tokens.clone(), tree.clone())?;
    (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), First::class_modification.clone(), false)?;
    if b.clone() {
        (tokens, tree) = class_modification(tokens.clone(), tree.clone())?;
    }
    (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), First::_annotation.clone(), false)?;
    if b.clone() {
        (tokens, tree) = _annotation(tokens.clone(), tree.clone())?;
    }
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree))
}

fn class_modification(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::LPAR.clone())?;
    (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), First::argument.clone(), false)?;
    if b.clone() {
        (tokens, tree) = argument_list(tokens.clone(), tree.clone())?;
    }
    (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::RPAR.clone())?;
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree))
}

fn argument_list(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    let mut nodeName: Arc<ParseTree>;
    (tokens, tree, nodeName) = argument(tokens.clone(), tree.clone())?;
    b = true;
    while b.clone() {
        (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::COMMA.clone())?;
        if b.clone() {
            (tokens, tree, nodeName) = argument(tokens.clone(), tree.clone())?;
        }
    }
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree))
}

fn argument(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>, Arc<ParseTree>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut nodeName: Arc<ParseTree>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    let mut node: Arc<ParseTree>;
    (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), list![TokenId::REDECLARE.clone()], false)?;
    if b.clone() {
        (tokens, tree, nodeName) = element_redeclaration(tokens.clone(), tree.clone())?;
    } else {
        (tokens, tree, nodeName) = element_modification_or_replaceable(tokens.clone(), tree.clone())?;
    }
    node = makeNode(tree.clone().reverse(), nodeName.clone());
    outTree = metamodelica::cons(node.clone(), inTree.clone());
    Ok((tokens, outTree, nodeName))
}

fn element_redeclaration(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>, Arc<ParseTree>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut nodeName: Arc<ParseTree>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::REDECLARE.clone())?;
    (tokens, tree, _) = scanOpt(tokens.clone(), tree.clone(), TokenId::EACH.clone())?;
    (tokens, tree, _) = scanOpt(tokens.clone(), tree.clone(), TokenId::FINAL.clone())?;
    (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), list![TokenId::REPLACEABLE.clone()], false)?;
    if b.clone() {
        (tokens, tree, nodeName) = element_replaceable(tokens.clone(), tree.clone())?;
    } else {
        (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), First::class_prefixes.clone(), false)?;
        if b.clone() {
            (tokens, tree, nodeName) = short_class_definition(tokens.clone(), tree.clone())?;
        } else {
            (tokens, tree, nodeName) = component_clause1(tokens.clone(), tree.clone())?;
        }
    }
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree, nodeName))
}

fn short_class_definition(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>, Arc<ParseTree>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut nodeName: Arc<ParseTree>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    (tokens, tree) = class_prefixes(tokens.clone(), tree.clone())?;
    (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::IDENT.clone())?;
    let __pa0 = ::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    nodeName = __pa0.clone();
    nodeName = parseTreeFilterWhitespace(nodeName.clone());
    (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::EQUALS.clone())?;
    (tokens, tree) = short_class_specifier1(tokens.clone(), tree.clone())?;
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree, nodeName))
}

fn element_modification_or_replaceable(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>, Arc<ParseTree>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut nodeName: Arc<ParseTree>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree, _) = scanOpt(tokens.clone(), tree.clone(), TokenId::EACH.clone())?;
    (tokens, tree, _) = scanOpt(tokens.clone(), tree.clone(), TokenId::FINAL.clone())?;
    (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), list![TokenId::REPLACEABLE.clone()], false)?;
    if b.clone() {
        (tokens, tree, nodeName) = element_replaceable(tokens.clone(), tree.clone())?;
    } else {
        (tokens, tree, nodeName) = element_modification(tokens.clone(), tree.clone())?;
    }
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree, nodeName))
}

fn element_replaceable(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>, Arc<ParseTree>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut nodeName: Arc<ParseTree>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::REPLACEABLE.clone())?;
    (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), First::component_clause.clone(), false)?;
    if b.clone() {
        (tokens, tree, nodeName) = component_clause1(tokens.clone(), tree.clone())?;
    } else {
        (tokens, tree, nodeName) = short_class_definition(tokens.clone(), tree.clone())?;
    }
    (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), list![TokenId::CONSTRAINEDBY.clone()], false)?;
    if b.clone() {
        (tokens, tree) = constraining_clause(tokens.clone(), tree.clone())?;
        (tokens, tree) = comment(tokens.clone(), tree.clone())?;
    }
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree, nodeName))
}

fn element_modification(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>, Arc<ParseTree>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut nodeName: Arc<ParseTree>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree) = name(tokens.clone(), tree.clone())?;
    let __pa0 = ::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    nodeName = __pa0.clone();
    nodeName = parseTreeFilterWhitespace(nodeName.clone());
    (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), First::modification.clone(), false)?;
    if b.clone() {
        (tokens, tree) = modification(tokens.clone(), tree.clone())?;
    }
    (tokens, tree) = string_comment(tokens.clone(), tree.clone())?;
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree, nodeName))
}

fn modification(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), First::class_modification.clone(), false)?;
    if b.clone() {
        (tokens, tree) = class_modification(tokens.clone(), tree.clone())?;
        (tokens, tree) = eatWhitespace(tokens.clone(), tree.clone())?;
        (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::EQUALS.clone())?;
        (tokens, tree) = eatWhitespace(tokens.clone(), tree.clone())?;
        if b.clone() {
            (tokens, tree) = expression(tokens.clone(), tree.clone())?;
        }
    } else {
        (tokens, tree) = eatWhitespace(tokens.clone(), tree.clone())?;
        (tokens, tree) = scanOneOf(tokens.clone(), tree.clone(), list![TokenId::EQUALS.clone(), TokenId::ASSIGN.clone()])?;
        (tokens, tree) = eatWhitespace(tokens.clone(), tree.clone())?;
        (tokens, tree) = expression(tokens.clone(), tree.clone())?;
    }
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree))
}

fn expression_list(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    loop {
        (tokens, tree) = expression(tokens.clone(), tree.clone())?;
        (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::COMMA.clone())?;
        if !(b.clone()) {
            break;
        }
    }
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree))
}

fn expression(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    let mut ifTrees: Arc<metamodelica::List<Arc<ParseTree>>> = inTree.clone();
    (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::IF.clone())?;
    if b.clone() {
        (tokens, tree) = expression(tokens.clone(), tree.clone())?;
        ifTrees = listAppend(makeNodePrependTree(tree.clone().reverse(), metamodelica::nil(), Arc::new(ParseTree::LEAF { token: makeToken(TokenId::IDENT.clone(), (literal!("$if_cond")).clone()) })), ifTrees.clone());
        tree = metamodelica::nil();
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::THEN.clone())?;
        (tokens, tree) = expression(tokens.clone(), tree.clone())?;
        ifTrees = listAppend(makeNodePrependTree(tree.clone().reverse(), metamodelica::nil(), Arc::new(ParseTree::LEAF { token: makeToken(TokenId::IDENT.clone(), (literal!("$then")).clone()) })), ifTrees.clone());
        tree = metamodelica::nil();
        loop {
            (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::ELSEIF.clone())?;
            if !(b.clone()) {
                break;
            }
            (tokens, tree) = expression(tokens.clone(), tree.clone())?;
            (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::THEN.clone())?;
            (tokens, tree) = expression(tokens.clone(), tree.clone())?;
            ifTrees = listAppend(makeNodePrependTree(tree.clone().reverse(), metamodelica::nil(), Arc::new(ParseTree::LEAF { token: makeToken(TokenId::IDENT.clone(), (literal!("$else_if")).clone()) })), ifTrees.clone());
            tree = metamodelica::nil();
        }
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::ELSE.clone())?;
        (tokens, tree) = expression(tokens.clone(), tree.clone())?;
        ifTrees = listAppend(makeNodePrependTree(tree.clone().reverse(), metamodelica::nil(), Arc::new(ParseTree::LEAF { token: makeToken(TokenId::IDENT.clone(), (literal!("$else")).clone()) })), ifTrees.clone());
        tree = metamodelica::nil();
        outTree = makeNodePrependTree(metamodelica::nil(), ifTrees.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
        return Ok((tokens.clone(), outTree.clone()));
    }
    (tokens, tree) = simple_expression(tokens.clone(), tree.clone())?;
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree))
}

fn simple_expression(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree) = logical_expression(tokens.clone(), tree.clone())?;
    (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::COLON.clone())?;
    if b.clone() {
        (tokens, tree) = logical_expression(tokens.clone(), tree.clone())?;
        (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::COLON.clone())?;
        if b.clone() {
            (tokens, tree) = logical_expression(tokens.clone(), tree.clone())?;
        }
    }
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree))
}

fn logical_expression(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree) = logical_term(tokens.clone(), tree.clone())?;
    loop {
        (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::OR.clone())?;
        if !(b.clone()) {
            break;
        }
        (tokens, tree) = logical_term(tokens.clone(), tree.clone())?;
    }
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree))
}

fn logical_term(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree) = logical_factor(tokens.clone(), tree.clone())?;
    loop {
        (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::AND.clone())?;
        if !(b.clone()) {
            break;
        }
        (tokens, tree) = logical_factor(tokens.clone(), tree.clone())?;
    }
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree))
}

fn logical_factor(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::NOT.clone())?;
    (tokens, tree) = relation(tokens.clone(), tree.clone())?;
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree))
}

fn relation(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    let rel_op: Arc<metamodelica::List<TokenId>> = list![TokenId::LESS.clone(), TokenId::LESSEQ.clone(), TokenId::GREATER.clone(), TokenId::GREATEREQ.clone(), TokenId::EQEQ.clone(), TokenId::LESSGT.clone()];
    (tokens, tree) = arithmetic_expression(tokens.clone(), tree.clone())?;
    loop {
        (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), rel_op.clone(), true)?;
        if !(b.clone()) {
            break;
        }
        (tokens, tree) = arithmetic_expression(tokens.clone(), tree.clone())?;
    }
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree))
}

fn arithmetic_expression(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    let add_op: Arc<metamodelica::List<TokenId>> = list![TokenId::PLUS.clone(), TokenId::MINUS.clone(), TokenId::PLUS_EW.clone(), TokenId::MINUS_EW.clone()];
    (tokens, tree, _) = LA1(tokens.clone(), tree.clone(), add_op.clone(), true)?;
    (tokens, tree) = term(tokens.clone(), tree.clone())?;
    loop {
        (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), add_op.clone(), true)?;
        if !(b.clone()) {
            break;
        }
        (tokens, tree) = term(tokens.clone(), tree.clone())?;
    }
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree))
}

fn term(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    let mul_op: Arc<metamodelica::List<TokenId>> = list![TokenId::STAR.clone(), TokenId::STAR_EW.clone(), TokenId::SLASH.clone(), TokenId::SLASH_EW.clone()];
    (tokens, tree) = factor(tokens.clone(), tree.clone())?;
    loop {
        (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), mul_op.clone(), true)?;
        if !(b.clone()) {
            break;
        }
        (tokens, tree) = factor(tokens.clone(), tree.clone())?;
    }
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree))
}

fn factor(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    let pow_op: Arc<metamodelica::List<TokenId>> = list![TokenId::POWER.clone(), TokenId::POWER_EW.clone()];
    (tokens, tree) = primary(tokens.clone(), tree.clone())?;
    loop {
        (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), pow_op.clone(), true)?;
        if !(b.clone()) {
            break;
        }
        (tokens, tree) = primary(tokens.clone(), tree.clone())?;
    }
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree))
}

fn primary(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut id: TokenId;
    let mut b: bool;
    let mut label: ArcStr = literal!("expression");
    (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), list![TokenId::UNSIGNED_INTEGER.clone(), TokenId::UNSIGNED_REAL.clone(), TokenId::FALSE.clone(), TokenId::TRUE.clone(), TokenId::END.clone(), TokenId::STRING.clone()], false)?;
    if b.clone() {
        (tokens, tree) = consume(tokens.clone(), tree.clone())?;
        outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
        return Ok((tokens.clone(), outTree.clone()));
    }
    (tokens, tree, id) = peek(tokens.clone(), tree.clone())?;
    if id.clone() == TokenId::LPAR.clone() {
        (tokens, tree) = output_expression_list(tokens.clone(), tree.clone())?;
        label = (literal!("$parenthesis")).clone();
    } else if id.clone() == TokenId::LBRACE.clone() {
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::LBRACE.clone())?;
        (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), list![TokenId::RBRACE.clone()], false)?;
        if !(b.clone()) {
            (tokens, tree) = function_arguments(tokens.clone(), tree.clone())?;
        }
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::RBRACE.clone())?;
        label = (literal!("$array")).clone();
    } else if id.clone() == TokenId::LBRACK.clone() {
        (tokens, tree) = consume(tokens.clone(), tree.clone())?;
        (tokens, tree) = expression_list(tokens.clone(), tree.clone())?;
        loop {
            (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::SEMICOLON.clone())?;
            if !(b.clone()) {
                break;
            }
            (tokens, tree) = expression_list(tokens.clone(), tree.clone())?;
        }
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::RBRACK.clone())?;
        label = (literal!("$matrix")).clone();
    } else if listMember(id.clone(), list![TokenId::DER.clone(), TokenId::INITIAL.clone()]) {
        (tokens, tree) = consume(tokens.clone(), tree.clone())?;
        (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), list![TokenId::LPAR.clone()], false)?;
        if b.clone() {
            (tokens, tree) = function_call_args(tokens.clone(), tree.clone())?;
        }
        label = (literal!("$initial")).clone();
    } else if listMember(id.clone(), list![TokenId::DOT.clone(), TokenId::IDENT.clone(), TokenId::FUNCTION.clone()]) {
        if id.clone() == TokenId::FUNCTION.clone() {
            (tokens, tree) = consume(tokens.clone(), tree.clone())?;
        }
        (tokens, tree) = component_reference(tokens.clone(), tree.clone())?;
        (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), list![TokenId::LPAR.clone()], false)?;
        if b.clone() {
            (tokens, tree) = function_call_args(tokens.clone(), tree.clone())?;
            label = (literal!("$call")).clone();
        }
    } else {
        error(tokens.clone(), tree.clone(), metamodelica::nil())?;
    }
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), Arc::new(ParseTree::LEAF { token: makeToken(TokenId::IDENT.clone(), (label.clone()).clone()) }));
    Ok((tokens, outTree))
}

fn function_call_args(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::LPAR.clone())?;
    (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::RPAR.clone())?;
    (tokens, tree) = eatWhitespace(tokens.clone(), tree.clone())?;
    if !(b.clone()) {
        (tokens, tree) = function_arguments(tokens.clone(), tree.clone())?;
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::RPAR.clone())?;
    }
    (tokens, tree) = eatWhitespace(tokens.clone(), tree.clone())?;
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree))
}

fn function_arguments(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    let mut tree2: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut trees: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ParseTree>>>>>;
    trees = metamodelica::nil();
    loop {
        (tokens, tree, b) = LAk(tokens.clone(), tree.clone(), list![list![TokenId::IDENT.clone()], list![TokenId::EQUALS.clone()]])?;
        if b.clone() {
            (tokens, tree) = named_arguments(tokens.clone(), tree.clone())?;
            trees = metamodelica::cons(tree.clone(), trees.clone());
            tree = metamodelica::nil();
            break;
        } else {
            (tokens, tree) = function_argument(tokens.clone(), tree.clone())?;
            (tokens, tree2, b) = scanOpt(tokens.clone(), metamodelica::nil(), TokenId::COMMA.clone())?;
            if b.clone() {
                (tokens, tree2) = eatWhitespace(tokens.clone(), tree2.clone())?;
            }
            if b.clone() {
                tree = metamodelica::cons(makeNode(tree2.clone().reverse(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY()), tree.clone());
            } else {
                (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::FOR.clone())?;
                if b.clone() {
                    (tokens, tree) = for_indices(tokens.clone(), tree.clone())?;
                }
                trees = metamodelica::cons(tree.clone(), trees.clone());
                tree = metamodelica::nil();
                break;
            }
        }
    }
    outTree = inTree.clone();
    for mut tree in &*trees.clone().reverse() {
        let mut tree = tree.clone();
        outTree = makeNodePrependTree(tree.clone().reverse(), outTree.clone(), Arc::new(ParseTree::LEAF { token: makeToken(TokenId::IDENT.clone(), (literal!("function_arguments")).clone()) }));
    }
    Ok((tokens, outTree))
}

fn function_argument(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::FUNCTION.clone())?;
    if b.clone() {
        (tokens, tree) = name(tokens.clone(), tree.clone())?;
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::LPAR.clone())?;
        (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), list![TokenId::IDENT.clone()], false)?;
        if b.clone() {
            (tokens, tree) = named_arguments(tokens.clone(), tree.clone())?;
        }
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::RPAR.clone())?;
    } else {
        (tokens, tree) = expression(tokens.clone(), tree.clone())?;
    }
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree))
}

fn named_arguments(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree) = named_argument(tokens.clone(), tree.clone())?;
    loop {
        (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::COMMA.clone())?;
        if !(b.clone()) {
            break;
        }
        (tokens, tree) = named_argument(tokens.clone(), tree.clone())?;
    }
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree))
}

fn named_argument(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut label: Arc<ParseTree>;
    (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::IDENT.clone())?;
    label = listHead(tree.clone())?;
    (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::EQUALS.clone())?;
    (tokens, tree) = expression(tokens.clone(), tree.clone())?;
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), label.clone());
    Ok((tokens, outTree))
}

fn for_indices(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree) = for_index(tokens.clone(), tree.clone())?;
    loop {
        (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::COMMA.clone())?;
        if !(b.clone()) {
            break;
        }
        (tokens, tree) = for_index(tokens.clone(), tree.clone())?;
    }
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree))
}

fn for_index(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::IDENT.clone())?;
    (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::IN.clone())?;
    if b.clone() {
        (tokens, tree) = expression(tokens.clone(), tree.clone())?;
    }
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree))
}

fn string_comment(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::STRING.clone())?;
    while b.clone() {
        (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::PLUS.clone())?;
        if b.clone() {
            (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::STRING.clone())?;
        }
    }
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree))
}

fn output_expression_list(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b1: bool;
    let mut b2: bool;
    (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::LPAR.clone())?;
    loop {
        (tokens, tree, b1) = scanOpt(tokens.clone(), tree.clone(), TokenId::COMMA.clone())?;
        (tokens, tree, b2) = scanOpt(tokens.clone(), tree.clone(), TokenId::RPAR.clone())?;
        if b2.clone() {
            break;
        }
        if !(b1.clone()) {
            (tokens, tree) = expression(tokens.clone(), tree.clone())?;
        }
    }
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree))
}

fn name(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree, _) = scanOpt(tokens.clone(), tree.clone(), TokenId::DOT.clone())?;
    (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::IDENT.clone())?;
    loop {
        (tokens, tree, b) = LAk(tokens.clone(), tree.clone(), list![list![TokenId::DOT.clone()], list![TokenId::IDENT.clone()]])?;
        if !(b.clone()) {
            break;
        }
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::DOT.clone())?;
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::IDENT.clone())?;
    }
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree))
}

fn component_reference(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree, _) = scanOpt(tokens.clone(), tree.clone(), TokenId::DOT.clone())?;
    loop {
        (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::IDENT.clone())?;
        (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), list![TokenId::LBRACK.clone()], false)?;
        if b.clone() {
            (tokens, tree) = array_subscripts(tokens.clone(), tree.clone())?;
        }
        (tokens, tree, b) = scanOpt(tokens.clone(), tree.clone(), TokenId::DOT.clone())?;
        if !(b.clone()) {
            break;
        }
    }
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree))
}

fn comment(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool;
    (tokens, tree) = string_comment(tokens.clone(), tree.clone())?;
    (tokens, tree, b) = LA1(tokens.clone(), tree.clone(), First::_annotation.clone(), false)?;
    if b.clone() {
        (tokens, tree) = _annotation(tokens.clone(), tree.clone())?;
    }
    outTree = makeNodePrependTree(tree.clone().reverse(), inTree.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY());
    Ok((tokens, outTree))
}

fn _annotation(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    tree = metamodelica::nil();
    (tokens, tree) = scan(tokens.clone(), tree.clone(), TokenId::ANNOTATION.clone())?;
    (tokens, tree) = class_modification(tokens.clone(), tree.clone())?;
    outTree = metamodelica::cons(makeNode(tree.clone().reverse(), Arc::new(ParseTree::LEAF { token: makeToken(TokenId::IDENT.clone(), (literal!("annotation")).clone()) })), inTree.clone());
    Ok((tokens, outTree))
}

fn findWithin(mut tree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<Arc<ParseTree>> {
    let mut w: Arc<ParseTree> = crate::SimpleModelicaParser::ParseTree::interned_EMPTY();
    let mut tok: Token = <Token as ::std::default::Default>::default();
    let mut tok2: Token = <Token as ::std::default::Default>::default();
    let mut rest: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut rest2: Arc<metamodelica::List<Arc<ParseTree>>>;
    w = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ ParseTree::NODE { label: Deref @ ParseTree::LEAF { token: tok }, nodes: Deref @ metamodelica::List::Cons { head: __esc_w @ Deref @ ParseTree::NODE { label: Deref @ ParseTree::LEAF { token: tok2 }, .. }, tail: __esc_rest } }, tail: __esc_rest2 } if (tokenContent(tok.clone())? == literal!("$program") && tokenContent(tok2.clone())? == literal!("$within")) => {
            w = (*__esc_w).clone();
            rest = (*__esc_rest).clone();
            rest2 = (*__esc_rest2).clone();
            w.clone()
        },
        _ => crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(w)
}

fn moveComments(mut t1: Arc<metamodelica::List<Arc<ParseTree>>>, mut t2: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<Arc<metamodelica::List<Arc<ParseTree>>>> {
    let mut t2: Arc<metamodelica::List<Arc<ParseTree>>> = t2;
    let mut c1: Arc<metamodelica::List<(Token, Arc<metamodelica::List<Arc<ParseTree>>>, ArcStr)>>;
    let mut c2: Arc<metamodelica::List<(Token, Arc<metamodelica::List<Arc<ParseTree>>>, ArcStr)>>;
    let mut tok: Token;
    let mut str1: ArcStr;
    let mut str2: ArcStr;
    let mut path1: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut path2: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tempTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    c1 = findCommentsWithLabels(t1.clone(), metamodelica::nil(), metamodelica::nil())?;
    c2 = findCommentsWithLabels(t2.clone(), metamodelica::nil(), metamodelica::nil())?;
    (_, c1, c2) = List::intersection1OnTrue(c1.clone(), c2.clone(), (std::sync::Arc::new(foundCommentEqual) as std::sync::Arc<dyn ::std::ops::Fn((Token, Arc<metamodelica::List<Arc<ParseTree>>>, ArcStr), (Token, Arc<metamodelica::List<Arc<ParseTree>>>, ArcStr)) -> Result<bool> + 'static>))?;
    for mut c in &*c2.clone() {
        let mut c = c.clone();
        if '__try0: {
            (tok, path1, str1) = c.clone();
            let ((_, __pa1, __pa2), __pa3) = unwrap_break_err!(List::findAndRemove1(c1.clone(), (std::sync::Arc::new(foundCommentTokenEqual) as std::sync::Arc<dyn ::std::ops::Fn((Token, Arc<metamodelica::List<Arc<ParseTree>>>, ArcStr), (Token, Arc<metamodelica::List<Arc<ParseTree>>>, ArcStr)) -> Result<bool> + 'static>), c.clone()), '__try0);
            path2 = __pa1.clone();
            str2 = __pa2.clone();
            c1 = __pa3.clone();
            let __pa4 = ::match_deref::match_deref! { match &(unwrap_break_err!(removeCommentAtLabelPath(t2.clone(), tok.clone(), path1.clone().reverse()), '__try0)) {
                (__pa4, true) => __pa4.clone(),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            tempTree = __pa4.clone();
            let __pa5 = ::match_deref::match_deref! { match &(unwrap_break_err!(addCommentAtLabelPath(tempTree.clone(), tok.clone(), path2.clone().reverse()), '__try0)) {
                (__pa5, true) => __pa5.clone(),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            tempTree = __pa5.clone();
            t2 = tempTree.clone();
            Ok::<(), anyhow::Error>(())
        }.is_err() {
        }
    }
    Ok(t2)
}

fn moveCommentsAfterDiff(mut res: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>>) -> Result<Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>>> {
    let mut res: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>> = res;
    let mut foundComment: ArcStr = arcstr::literal!("");
    let mut tree: Arc<ParseTree> = Arc::new(ParseTree::EMPTY);
    let mut foundTree: Arc<ParseTree> = Arc::new(ParseTree::EMPTY);
    let mut trees: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut before: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut after: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut acc2: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut comments: Arc<AvlSetString::Tree>;
    let mut acc: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>>;
    let mut lst: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>>;
    let mut diff: (Diff, Arc<metamodelica::List<Arc<ParseTree>>>);
    let mut found: bool = false;
    comments = findAddedComments(res.clone())?;
    acc = metamodelica::nil();
    lst = res.clone();
    if AvlSetString::isEmpty(comments.clone()) {
        return Ok(res.clone());
    }
    while !(lst.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lst.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        diff = __pa0.clone();
        lst = __pa1.clone();
        let () = (::match_deref::match_deref! { match &(diff.clone()) {
        (DiffAlgorithm::Diff::Delete, __esc_trees) => {
            trees = (*__esc_trees).clone();
            acc2 = metamodelica::nil();
            while !(trees.clone().is_empty()) {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(trees.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                tree = __pa0.clone();
                trees = __pa1.clone();
                (found, before, foundTree, after, foundComment) = fixDeletedComments(tree.clone(), comments.clone())?;
                if found.clone() {
                    acc = metamodelica::cons((Diff::Delete.clone(), listAppend(acc2.clone().reverse(), before.clone())), acc.clone());
                    res = listAppend(acc.clone().reverse(), metamodelica::cons((Diff::Equal.clone(), list![foundTree.clone()]), metamodelica::cons((Diff::Delete.clone(), listAppend(after.clone(), trees.clone())), lst.clone())));
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*DiffAlgorithm::printDiffTerminalColor(res.clone(), (std::sync::Arc::new(parseTreeNodeStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ParseTree>) -> Result<ArcStr> + 'static>))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    res = removeAddedCommentFromDiff(res.clone(), (foundComment.clone()).clone())?;
                    res = moveCommentsAfterDiff(res.clone())?;
                    return Ok(res.clone());
                }
                acc2 = metamodelica::cons(tree.clone(), acc2.clone());
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        acc = metamodelica::cons(diff.clone(), acc.clone());
    }
    Ok(res)
}

fn findAddedComments(mut tree: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>>) -> Result<Arc<AvlSetString::Tree>> {
    let mut comments: Arc<AvlSetString::Tree> = openmodelica_util::AvlSetString::Tree::interned_EMPTY();
    let mut addedTrees: Arc<metamodelica::List<Arc<ParseTree>>>;
    (addedTrees, _) = extractAdditionsDeletions(tree.clone())?;
    for mut t in &*addedTrees.clone() {
        let mut t = t.clone();
        comments = findAddedComments2(t.clone(), comments.clone())?;
    }
    Ok(comments)
}

fn findAddedComments2(mut tree: Arc<ParseTree>, mut comments: Arc<AvlSetString::Tree>) -> Result<Arc<AvlSetString::Tree>> {
    let mut comments: Arc<AvlSetString::Tree> = comments;
    let mut nodes: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    comments = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ ParseTree::LEAF { .. } if (parseTreeIsComment(tree.clone())) => AvlSetString::add(comments.clone(), (tokenContent(var_field!((*tree).token, ParseTree::LEAF).clone())?).clone())?,
        Deref @ ParseTree::NODE { nodes: __esc_nodes, .. } => {
            nodes = (*__esc_nodes).clone();
            for mut n in &*nodes.clone() {
                let mut n = n.clone();
                comments = findAddedComments2(n.clone(), comments.clone())?;
            }
            comments.clone()
        },
        _ => comments.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(comments)
}

fn removeAddedCommentFromDiff(mut tree: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>>, mut comment: ArcStr) -> Result<Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>>> {
    let mut tree: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>> = tree;
    let mut acc: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>>;
    let mut lst: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>>;
    let mut diff: (Diff, Arc<metamodelica::List<Arc<ParseTree>>>);
    let mut lst2: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool = false;
    lst = tree.clone();
    acc = metamodelica::nil();
    while !(lst.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lst.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        diff = __pa0.clone();
        lst = __pa1.clone();
        let () = (::match_deref::match_deref! { match &(diff.clone()) {
        (DiffAlgorithm::Diff::Add, __esc_lst2) => {
            lst2 = (*__esc_lst2).clone();
            (b, lst2) = removeAddedCommentFromDiff2(lst2.clone(), (comment.clone()).clone())?;
            if b.clone() {
                tree = listAppend(acc.clone().reverse(), metamodelica::cons((Diff::Add.clone(), lst2.clone()), lst.clone()));
                return Ok(tree.clone());
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        acc = metamodelica::cons(diff.clone(), acc.clone());
    }
    Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to remove comment `")); __mm_s.push_str(&*comment.clone()); __mm_s.push_str(&*literal!("` from diff; but we know it is in there somewhere")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("Parsers/SimpleModelicaParser.mo"))?;
    Ok(tree)
}

fn removeAddedCommentFromDiff2(mut trees: Arc<metamodelica::List<Arc<ParseTree>>>, mut comment: ArcStr) -> Result<(bool, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut removed: bool = false;
    let mut trees: Arc<metamodelica::List<Arc<ParseTree>>> = trees;
    let mut acc: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut lst: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut nodes: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut tree: Arc<ParseTree>;
    let mut content: ArcStr = arcstr::literal!("");
    acc = metamodelica::nil();
    lst = trees.clone();
    while !(lst.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lst.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        tree = __pa0.clone();
        lst = __pa1.clone();
        (removed, tree) = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ ParseTree::LEAF { .. } if (parseTreeIsComment(tree.clone())) => {
            content = (tokenContent(var_field!((*tree).token, ParseTree::LEAF).clone())?).clone();
            (content.clone() == comment.clone(), if (content.clone() == comment.clone()) {crate::SimpleModelicaParser::ParseTree::interned_EMPTY()} else {tree.clone()})
        },
        Deref @ ParseTree::NODE { nodes: __esc_nodes, .. } => {
            nodes = (*__esc_nodes).clone();
            (removed, nodes) = removeAddedCommentFromDiff2(nodes.clone(), (comment.clone()).clone())?;
            if removed.clone() {
                assign_variant_field!(tree => ParseTree::NODE; nodes = nodes.clone());
            }
            (removed.clone(), tree.clone())
        },
        _ => (false, tree.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        if removed.clone() {
            lst = if (isEmpty(tree.clone())) {lst.clone()} else {metamodelica::cons(tree.clone(), lst.clone())};
            lst = listAppend(acc.clone().reverse(), lst.clone());
            trees = lst.clone();
            return Ok((removed.clone(), trees.clone()));
        }
        acc = metamodelica::cons(tree.clone(), acc.clone());
    }
    Ok((removed, trees))
}

fn fixDeletedComments(mut tree: Arc<ParseTree>, mut addedComments: Arc<AvlSetString::Tree>) -> Result<(bool, Arc<metamodelica::List<Arc<ParseTree>>>, Arc<ParseTree>, Arc<metamodelica::List<Arc<ParseTree>>>, ArcStr)> {
    let mut found: bool = false;
    let mut before: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut foundTree: Arc<ParseTree> = tree.clone();
    let mut after: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut foundComment: ArcStr = literal!("");
    let mut nodes: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut before2: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut after2: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut b: bool = false;
    let mut t: Arc<ParseTree> = Arc::new(ParseTree::EMPTY);
    let mut content: ArcStr = arcstr::literal!("");
    found = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ ParseTree::LEAF { .. } if (parseTreeIsComment(tree.clone())) => {
            content = (tokenContent(var_field!((*tree).token, ParseTree::LEAF).clone())?).clone();
            b = AvlSetString::hasKey(addedComments.clone(), (content.clone()).clone())?;
            if b.clone() {
                foundComment = (content.clone()).clone();
            }
            b.clone()
        },
        Deref @ ParseTree::NODE { nodes: __esc_nodes, .. } => {
            nodes = (*__esc_nodes).clone();
            while !(nodes.clone().is_empty()) {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(nodes.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                t = __pa0.clone();
                nodes = __pa1.clone();
                (found, before2, foundTree, after2, foundComment) = fixDeletedComments(t.clone(), addedComments.clone())?;
                if found.clone() {
                    before = listAppend(before.clone().reverse(), before2.clone());
                    after = listAppend(after2.clone(), nodes.clone());
                    return Ok((found.clone(), before.clone(), foundTree.clone(), after.clone(), foundComment.clone()));
                }
                before = metamodelica::cons(t.clone(), before.clone());
            }
            before = metamodelica::nil();
            foundTree = tree.clone();
            after = metamodelica::nil();
            foundComment = (literal!("")).clone();
            false
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((found, before, foundTree, after, foundComment))
}

fn addCommentAtLabelPath(mut tree: Arc<metamodelica::List<Arc<ParseTree>>>, mut tok: Token, mut path: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Arc<ParseTree>>>, bool)> {
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = tree;
    let mut success: bool = false;
    let mut n: Arc<ParseTree>;
    let mut n2: Arc<ParseTree> = Arc::new(ParseTree::EMPTY);
    let mut label: Arc<ParseTree> = Arc::new(ParseTree::EMPTY);
    let mut pathFirst: Arc<ParseTree> = Arc::new(ParseTree::EMPTY);
    let mut rest: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut nodes: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut pathRest: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut delst: DoubleEnded::MutableList<Arc<ParseTree>>;
    let mut b: bool = false;
    if path.clone().is_empty() {
        success = true;
        tree = metamodelica::cons(Arc::new(ParseTree::LEAF { token: tok.clone() }), tree.clone());
        return Ok((tree.clone(), success.clone()));
    }
    delst = DoubleEnded::fromList(metamodelica::nil())?;
    rest = tree.clone();
    while !(rest.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        n = __pa0.clone();
        rest = __pa1.clone();
        (n2, b) = (::match_deref::match_deref! { match &((n.clone(), path.clone())) {
        (Deref @ ParseTree::NODE { label: Deref @ ParseTree::EMPTY { .. }, .. }, _) => {
            (nodes, b) = addCommentAtLabelPath(var_field!((*n).nodes, ParseTree::NODE).clone(), tok.clone(), path.clone())?;
            if b.clone() {
                n2 = Arc::new(ParseTree::NODE { label: crate::SimpleModelicaParser::ParseTree::interned_EMPTY(), nodes: nodes.clone() });
            } else {
                n2 = n.clone();
            }
            (n2.clone(), b.clone())
        },
        (Deref @ ParseTree::NODE { label, .. }, Deref @ metamodelica::List::Cons { head: pathFirst, tail: __esc_pathRest }) if (stringEq((labelPathStr(list![label.clone()])?).clone(), (labelPathStr(list![pathFirst.clone()])?).clone())) => {
            pathRest = (*__esc_pathRest).clone();
            (nodes, b) = addCommentAtLabelPath(var_field!((*n).nodes, ParseTree::NODE).clone(), tok.clone(), pathRest.clone())?;
            if b.clone() {
                n2 = Arc::new(ParseTree::NODE { label: label.clone(), nodes: nodes.clone() });
            } else {
                n2 = n.clone();
            }
            (n2.clone(), b.clone())
        },
        _ => (n.clone(), false),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        DoubleEnded::push_back(delst.clone(), n2.clone());
        if b.clone() {
            tree = DoubleEnded::toListAndClear(delst.clone(), rest.clone());
            success = true;
            return Ok((tree.clone(), success.clone()));
        }
    }
    Ok((tree, success))
}

fn removeCommentAtLabelPath(mut tree: Arc<metamodelica::List<Arc<ParseTree>>>, mut tok: Token, mut path: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Arc<ParseTree>>>, bool)> {
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = tree;
    let mut success: bool = false;
    let mut n: Arc<ParseTree>;
    let mut n2: Arc<ParseTree> = Arc::new(ParseTree::EMPTY);
    let mut label: Arc<ParseTree> = Arc::new(ParseTree::EMPTY);
    let mut pathFirst: Arc<ParseTree> = Arc::new(ParseTree::EMPTY);
    let mut rest: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut nodes: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut pathRest: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut delst: DoubleEnded::MutableList<Arc<ParseTree>>;
    let mut b: bool = false;
    if path.clone().is_empty() {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(removeCommentAtThisLabel(tree.clone(), tok.clone())?) {
            (__pa0, __pa1 @ true) => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        tree = __pa0.clone();
        success = __pa1.clone();
        return Ok((tree.clone(), success.clone()));
    }
    delst = DoubleEnded::fromList(metamodelica::nil())?;
    rest = tree.clone();
    while !(rest.clone().is_empty()) {
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        n = __pa2.clone();
        rest = __pa3.clone();
        (n2, b) = (::match_deref::match_deref! { match &((n.clone(), path.clone())) {
        (Deref @ ParseTree::NODE { label: Deref @ ParseTree::EMPTY { .. }, .. }, _) => {
            (nodes, b) = removeCommentAtLabelPath(var_field!((*n).nodes, ParseTree::NODE).clone(), tok.clone(), path.clone())?;
            if b.clone() {
                n2 = Arc::new(ParseTree::NODE { label: crate::SimpleModelicaParser::ParseTree::interned_EMPTY(), nodes: nodes.clone() });
            } else {
                n2 = n.clone();
            }
            (n2.clone(), b.clone())
        },
        (Deref @ ParseTree::NODE { label, .. }, Deref @ metamodelica::List::Cons { head: pathFirst, tail: __esc_pathRest }) if (stringEq((labelPathStr(list![label.clone()])?).clone(), (labelPathStr(list![pathFirst.clone()])?).clone())) => {
            pathRest = (*__esc_pathRest).clone();
            (nodes, b) = removeCommentAtLabelPath(var_field!((*n).nodes, ParseTree::NODE).clone(), tok.clone(), pathRest.clone())?;
            if b.clone() {
                n2 = Arc::new(ParseTree::NODE { label: label.clone(), nodes: nodes.clone() });
            } else {
                n2 = n.clone();
            }
            (n2.clone(), b.clone())
        },
        _ => (n.clone(), false),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        DoubleEnded::push_back(delst.clone(), n2.clone());
        if b.clone() {
            tree = DoubleEnded::toListAndClear(delst.clone(), rest.clone());
            success = true;
            return Ok((tree.clone(), success.clone()));
        }
    }
    Ok((tree, success))
}

fn removeCommentAtThisLabel(mut tree: Arc<metamodelica::List<Arc<ParseTree>>>, mut tok: Token) -> Result<(Arc<metamodelica::List<Arc<ParseTree>>>, bool)> {
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = tree;
    let mut success: bool = false;
    let mut delst: DoubleEnded::MutableList<Arc<ParseTree>>;
    let mut rest: Arc<metamodelica::List<Arc<ParseTree>>> = tree.clone();
    let mut nodes: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut n: Arc<ParseTree>;
    delst = DoubleEnded::fromList(metamodelica::nil())?;
    while !(rest.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        n = __pa0.clone();
        rest = __pa1.clone();
        let () = (::match_deref::match_deref! { match &(n.clone()) {
        Deref @ ParseTree::LEAF { .. } if (modelicaDiffTokenEq(var_field!((*n).token, ParseTree::LEAF).clone(), tok.clone())?) => {
            success = true;
            tree = DoubleEnded::toListAndClear(delst.clone(), rest.clone());
            return Ok((tree.clone(), success.clone()));
            bail!("fail")
        },
        Deref @ ParseTree::NODE { label: Deref @ ParseTree::EMPTY { .. }, .. } => {
            (nodes, success) = removeCommentAtThisLabel(var_field!((*n).nodes, ParseTree::NODE).clone(), tok.clone())?;
            if success.clone() {
                DoubleEnded::push_back(delst.clone(), Arc::new(ParseTree::NODE { label: crate::SimpleModelicaParser::ParseTree::interned_EMPTY(), nodes: nodes.clone() }));
                tree = DoubleEnded::toListAndClear(delst.clone(), rest.clone());
                return Ok((tree.clone(), success.clone()));
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        DoubleEnded::push_back(delst.clone(), n.clone());
    }
    Ok((tree, success))
}

fn findCommentsWithLabels(mut t1: Arc<metamodelica::List<Arc<ParseTree>>>, mut labelPath: Arc<metamodelica::List<Arc<ParseTree>>>, mut acc: Arc<metamodelica::List<(Token, Arc<metamodelica::List<Arc<ParseTree>>>, ArcStr)>>) -> Result<Arc<metamodelica::List<(Token, Arc<metamodelica::List<Arc<ParseTree>>>, ArcStr)>>> {
    let mut acc: Arc<metamodelica::List<(Token, Arc<metamodelica::List<Arc<ParseTree>>>, ArcStr)>> = acc;
    let mut nodes: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut tok: Token = <Token as ::std::default::Default>::default();
    let mut id: TokenId;
    let mut pathStr: ArcStr = arcstr::literal!("");
    for mut n in &*t1.clone() {
        let mut n = n.clone();
        let () = (::match_deref::match_deref! { match &(n.clone()) {
        Deref @ ParseTree::EMPTY { .. } => (),
        Deref @ ParseTree::LEAF { token: __esc_tok @ Token { id: __esc_id, .. } } if (parseTreeIsComment(n.clone())) => {
            tok = (*__esc_tok).clone();
            id = (*__esc_id).clone();
            pathStr = (labelPathStr(labelPath.clone())?).clone();
            acc = metamodelica::cons((tok.clone(), labelPath.clone(), pathStr.clone()), acc.clone());
            ()
        },
        Deref @ ParseTree::NODE { label: Deref @ ParseTree::EMPTY { .. }, nodes: __esc_nodes } => {
            nodes = (*__esc_nodes).clone();
            acc = findCommentsWithLabels(nodes.clone(), labelPath.clone(), acc.clone())?;
            ()
        },
        Deref @ ParseTree::NODE { nodes: __esc_nodes, .. } => {
            nodes = (*__esc_nodes).clone();
            acc = findCommentsWithLabels(nodes.clone(), metamodelica::cons(var_field!((*n).label, ParseTree::NODE).clone(), labelPath.clone()), acc.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(acc)
}

fn foundCommentEqual(mut c1: (Token, Arc<metamodelica::List<Arc<ParseTree>>>, ArcStr), mut c2: (Token, Arc<metamodelica::List<Arc<ParseTree>>>, ArcStr)) -> Result<bool> {
    let mut eq: bool;
    let mut tok1: Token;
    let mut tok2: Token;
    let mut s1: ArcStr;
    let mut s2: ArcStr;
    (tok1, _, s1) = c1.clone();
    (tok2, _, s2) = c2.clone();
    eq = modelicaDiffTokenEq(tok1.clone(), tok2.clone())?;
    if !(eq.clone()) {
        return Ok(eq.clone());
    }
    eq = stringEq((s1.clone()).clone(), (s2.clone()).clone());
    Ok(eq)
}

fn foundCommentTokenEqual(mut c1: (Token, Arc<metamodelica::List<Arc<ParseTree>>>, ArcStr), mut c2: (Token, Arc<metamodelica::List<Arc<ParseTree>>>, ArcStr)) -> Result<bool> {
    let mut eq: bool;
    let mut tok1: Token;
    let mut tok2: Token;
    (tok1, _, _) = c1.clone();
    (tok2, _, _) = c2.clone();
    eq = modelicaDiffTokenEq(tok1.clone(), tok2.clone())?;
    Ok(eq)
}

fn labelPathStr(mut labelPath: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut t in (labelPath.clone()).into_iter().cloned() {
            let __x = parseTreeStr(list![t.clone()])?;
            __acc = cons(__x, __acc);
        }
        __acc
    }), (literal!(".")).clone());
    Ok(r#str)
}

fn treeDiffWork1(mut t1: Arc<metamodelica::List<Arc<ParseTree>>>, mut t2: Arc<metamodelica::List<Arc<ParseTree>>>, mut nTokens: i32) -> Result<Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>>> {
    let mut res: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>>;
    let mut diffSubtreeWorkArray1: metamodelica::Array<Token>;
    let mut diffSubtreeWorkArray2: metamodelica::Array<Token>;
    if t1.clone().is_empty() {
        res = list![(Diff::Add.clone(), t2.clone())];
        return Ok(res.clone());
    } else if t2.clone().is_empty() {
        res = list![(Diff::Delete.clone(), t1.clone())];
        return Ok(res.clone());
    }
    diffSubtreeWorkArray1 = metamodelica::arrayCreate(nTokens.clone(), LexerModelicaDiff::noToken.clone());
    diffSubtreeWorkArray2 = metamodelica::arrayCreate(nTokens.clone(), LexerModelicaDiff::noToken.clone());
    if parseTreeEq(makeNode(t1.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY()), makeNode(t2.clone(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY()), diffSubtreeWorkArray1.clone(), diffSubtreeWorkArray2.clone())? {
        res = list![(Diff::Equal.clone(), t1.clone())];
        return Ok(res.clone());
    }
    (res, _) = treeDiffWork(t1.clone(), t2.clone(), 1, (std::sync::Arc::new({ let __pe_b2 = diffSubtreeWorkArray1.clone(); let __pe_b3 = diffSubtreeWorkArray2.clone(); move |__pe_a0, __pe_a1| parseTreeEq(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ParseTree>, Arc<ParseTree>) -> Result<bool> + 'static>))?;
    Ok(res)
}

fn treeDiffWork(mut t1: Arc<metamodelica::List<Arc<ParseTree>>>, mut t2: Arc<metamodelica::List<Arc<ParseTree>>>, mut depth: i32, mut compare: Arc<dyn ::std::ops::Fn(Arc<ParseTree>, Arc<ParseTree>) -> Result<bool> + 'static>) -> Result<(Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>>, Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>>)> {
    let mut res: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>> = metamodelica::nil();
    let mut resLocal: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>> = metamodelica::nil();
    let mut t2_strip: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut before: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut middle: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut after: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut addedTrees: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut deletedTrees: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut ts: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut addList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut delList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut nadd: i32;
    let mut ndel: i32;
    let mut addedTree: Arc<ParseTree>;
    let mut deletedTree: Arc<ParseTree>;
    let mut deleted: Arc<ParseTree> = Arc::new(ParseTree::EMPTY);
    let mut addedBeforeDeleted: bool;
    let mut joinTrees: bool;
    let mut tryFind: bool;
    let mut r#str: ArcStr;
    let mut debugString1: ArcStr = literal!("");
    let mut debugString2: ArcStr = literal!("");
    let mut d: Diff;
    let () = (::match_deref::match_deref! { match &((t1.clone(), t2.clone())) {
        (Deref @ metamodelica::List::Cons { head: Deref @ ParseTree::NODE { nodes: __esc_before, .. }, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: Deref @ ParseTree::NODE { nodes: __esc_after, .. }, tail: Deref @ metamodelica::List::Nil }) => {
            before = (*__esc_before).clone();
            after = (*__esc_after).clone();
            (res, _) = treeDiffWork(before.clone(), after.clone(), depth.clone(), compare.clone())?;
            return Ok((res.clone(), resLocal.clone()));
            ()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ ParseTree::NODE { nodes: __esc_before, .. }, tail: Deref @ metamodelica::List::Nil }, _) => {
            before = (*__esc_before).clone();
            (res, _) = treeDiffWork(before.clone(), t2.clone(), depth.clone(), compare.clone())?;
            return Ok((res.clone(), resLocal.clone()));
            ()
        },
        (_, Deref @ metamodelica::List::Cons { head: Deref @ ParseTree::NODE { nodes: __esc_after, .. }, tail: Deref @ metamodelica::List::Nil }) => {
            after = (*__esc_after).clone();
            (res, _) = treeDiffWork(t1.clone(), after.clone(), depth.clone(), compare.clone())?;
            return Ok((res.clone(), resLocal.clone()));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if parseTreeIsNewLine(listHead(t2.clone())?) {
        t2_strip = listRest(t2.clone())?;
    } else {
        t2_strip = t2.clone();
    }
    if debug.clone() {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Do diff at depth=")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", depth.clone()))); __mm_s.push_str(&*literal!(", len(t1)=")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", (t1.clone().len() as i32)))); __mm_s.push_str(&*literal!(", len(t2)=")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", (t2.clone().len() as i32)))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("top t1=")); __mm_s.push_str(&*firstTokenDebugStr(t1.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("top t2=")); __mm_s.push_str(&*firstTokenDebugStr(t2.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("all t1=")); __mm_s.push_str(&*parseTreeStr(t1.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("all t2=")); __mm_s.push_str(&*parseTreeStr(t2.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    res = diff(t1.clone(), t2.clone(), compare.clone(), (std::sync::Arc::new(fnptr!(parseTreeIsWhitespace, Arc<ParseTree>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ParseTree>) -> Result<bool> + 'static>), (std::sync::Arc::new(fnptr!(parseTreeIsWhitespaceNotComment, Arc<ParseTree>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ParseTree>) -> Result<bool> + 'static>), (std::sync::Arc::new(parseTreeNodeStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ParseTree>) -> Result<ArcStr> + 'static>))?;
    (nadd, ndel) = countDiffAddDelete(res.clone());
    if nadd.clone() > 1 {
        res = fixMoveOperations(res.clone(), compare.clone())?;
        (nadd, ndel) = countDiffAddDelete(res.clone());
    }
    res = filterDiffWhitespace(res.clone())?;
    if debug.clone() {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("nadd: ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", nadd.clone()))); __mm_s.push_str(&*literal!(" ndel: ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", ndel.clone()))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*DiffAlgorithm::printDiffTerminalColor(res.clone(), (std::sync::Arc::new(parseTreeNodeStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ParseTree>) -> Result<ArcStr> + 'static>))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        if nadd.clone() != ndel.clone() {
            for mut r in &*res.clone() {
                let mut r = r.clone();
                (d, ts) = r.clone();
                if d.clone() == Diff::Equal.clone() {
                    continue;
                }
                for mut t in &*ts.clone() {
                    let mut t = t.clone();
                    if isLabeledNode(t.clone()) {
                        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*ArcStr::from(::std::format!("{:?}", d.clone()))); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*parseTreeStr(metamodelica::cons(nodeLabel(t.clone()), metamodelica::nil()))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                }
            }
        }
    }
    if depth.clone() > 300 {
    } else if nadd.clone() == 1 && ndel.clone() == 1 {
        (addedTree, deletedTree, before, middle, after, addedBeforeDeleted) = extractSingleAddDiffBeforeAndAfter(res.clone())?;
        if if (!(middle.clone().is_empty())) {({
        let mut __acc: Option<bool> = None;
        for mut middleItem in (middle.clone()).into_iter().cloned() {
            let __x = parseTreeIsWhitespace(middleItem.clone());
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
        }
        __acc.unwrap_or(true)
    })} else {false} {
            if addedBeforeDeleted.clone() {
                before = listAppend(before.clone(), middle.clone());
            } else {
                after = listAppend(middle.clone(), after.clone());
            }
            middle = metamodelica::nil();
        }
        joinTrees = true;
        if compare(addedTree.clone(), deletedTree.clone())? {
            res = list![(Diff::Equal.clone(), list![deletedTree.clone()])];
        } else if isLeaf(deletedTree.clone()) && isLeaf(addedTree.clone()) {
            res = res.clone();
            joinTrees = false;
        } else if before.clone().is_empty() && after.clone().is_empty() {
            if debug.clone() {
                metamodelica::print((literal!("before and after empty\n")).clone());
            }
            res = res.clone();
        } else {
            (res, _) = treeDiffWork(getNodes(deletedTree.clone()), getNodes(addedTree.clone()), depth.clone() + 1, compare.clone())?;
        }
        if !(joinTrees.clone()) {
            res = res.clone();
            if debug.clone() {
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("not joining trees")); __mm_s.push_str(&*DiffAlgorithm::printDiffTerminalColor(res.clone(), (std::sync::Arc::new(parseTreeNodeStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ParseTree>) -> Result<ArcStr> + 'static>))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
        } else if middle.clone().is_empty() {
            if debug.clone() {
                metamodelica::print((literal!("middle empty\n")).clone());
            }
            res = metamodelica::cons((Diff::Equal.clone(), before.clone()), listAppend(res.clone(), list![(Diff::Equal.clone(), after.clone())]));
        } else {
            res = ({
        let mut __acc: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>> = metamodelica::nil();
        for mut i in (res.clone()).into_iter().cloned() {
            if !((::match_deref::match_deref! { match &(i.clone()) {
        (DiffAlgorithm::Diff::Delete, _) => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })) { continue; }
            let __x = i.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            if addedBeforeDeleted.clone() {
                res = metamodelica::cons((Diff::Equal.clone(), before.clone()), listAppend(res.clone(), metamodelica::cons((Diff::Equal.clone(), middle.clone()), metamodelica::cons((Diff::Delete.clone(), list![deletedTree.clone()]), list![(Diff::Equal.clone(), after.clone())]))));
            } else {
                res = metamodelica::cons((Diff::Equal.clone(), before.clone()), metamodelica::cons((Diff::Delete.clone(), list![deletedTree.clone()]), metamodelica::cons((Diff::Equal.clone(), middle.clone()), listAppend(res.clone(), list![(Diff::Equal.clone(), after.clone())]))));
            }
        }
        if debug.clone() {
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", depth.clone()))); __mm_s.push_str(&*literal!(" merged tree size: ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", ((DiffAlgorithm::printActual(res.clone(), (std::sync::Arc::new(parseTreeNodeStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ParseTree>) -> Result<ArcStr> + 'static>))).clone().len() as i32)))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", depth.clone()))); __mm_s.push_str(&*literal!(" before top=")); __mm_s.push_str(&*firstTokenDebugStr(before.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" before all=")); __mm_s.push_str(&*parseTreeStr(before.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" middle all=")); __mm_s.push_str(&*parseTreeStr(middle.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" after all=")); __mm_s.push_str(&*parseTreeStr(after.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("middle top=")); __mm_s.push_str(&*firstTokenDebugStr(middle.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("after top=")); __mm_s.push_str(&*firstTokenDebugStr(after.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("added top=")); __mm_s.push_str(&*firstTokenDebugStr(metamodelica::cons(addedTree.clone(), metamodelica::nil()))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("deleted top=")); __mm_s.push_str(&*firstTokenDebugStr(metamodelica::cons(deletedTree.clone(), metamodelica::nil()))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
    } else if nadd.clone() > 1 && ndel.clone() > 1 {
        (addedTrees, deletedTrees) = extractAdditionsDeletions(res.clone())?;
        addedTrees = ({
        let mut __acc: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
        for mut t in (addedTrees.clone()).into_iter().cloned() {
            if !(isLabeledNode(t.clone())) { continue; }
            let __x = t.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        deletedTrees = ({
        let mut __acc: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
        for mut t in (deletedTrees.clone()).into_iter().cloned() {
            if !(isLabeledNode(t.clone())) { continue; }
            let __x = t.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        if debug.clone() {
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("number of labeled nodes. add=")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", (addedTrees.clone().len() as i32)))); __mm_s.push_str(&*literal!(" del=")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", (deletedTrees.clone().len() as i32)))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*DiffAlgorithm::printDiffTerminalColor(res.clone(), (std::sync::Arc::new(parseTreeNodeStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ParseTree>) -> Result<ArcStr> + 'static>))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        for mut x in &*res.clone() {
            let mut x = x.clone();
            (d, ts) = x.clone();
            if d.clone() == Diff::Equal.clone() {
                continue;
            }
            addList = metamodelica::nil();
            delList = metamodelica::nil();
            for mut t in &*ts.clone() {
                let mut t = t.clone();
                if isEmpty(t.clone()) || parseTreeIsWhitespace(t.clone()) || isEmpty(nodeLabel(t.clone())) {
                    continue;
                }
                r#str = (parseTreeStr(metamodelica::cons(nodeLabel(t.clone()), metamodelica::nil()))?).clone();
                if d.clone() == Diff::Add.clone() {
                    addList = metamodelica::cons((r#str.clone()).clone(), addList.clone());
                } else {
                    delList = metamodelica::cons((r#str.clone()).clone(), delList.clone());
                }
            }
        }
        for mut added in &*addedTrees.clone() {
            let mut added = added.clone();
            tryFind = false;
            if '__try0: {
                (deleted, deletedTrees) = unwrap_break_err!(List::findAndRemove1(deletedTrees.clone(), (std::sync::Arc::new({ let __pe_b2 = compare.clone(); move |__pe_a0, __pe_a1| compareNodeLabels(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ParseTree>, Arc<ParseTree>) -> Result<bool> + 'static>), added.clone()), '__try0);
                Ok::<(), anyhow::Error>(())
            }.is_err() {
                if '__try1: {
                    (deleted, deletedTrees) = unwrap_break_err!(List::findAndRemove1(deletedTrees.clone(), (std::sync::Arc::new({ let __pe_b2 = compare.clone(); let __pe_b3 = delList.clone(); move |__pe_a0, __pe_a1| compareNodeLabelsSpecial(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ParseTree>, Arc<ParseTree>) -> Result<bool> + 'static>), added.clone()), '__try1);
                    Ok::<(), anyhow::Error>(())
                }.is_err() {
                    tryFind = true;
                }
            }
            if tryFind.clone() {
                continue;
            }
            (resLocal, _) = treeDiffWork(getNodes(deleted.clone()), getNodes(added.clone()), depth.clone() + 1, compare.clone())?;
            if debug.clone() {
                debugString1 = (DiffAlgorithm::printDiffTerminalColor(res.clone(), (std::sync::Arc::new(parseTreeNodeStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ParseTree>) -> Result<ArcStr> + 'static>))).clone();
            }
            res = replaceLabeledDiff(res.clone(), resLocal.clone(), nodeLabel(added.clone()), nodeLabel(deleted.clone()), compare.clone(), labelOrderDidNotChange(addList.clone(), delList.clone())?)?;
            if debug.clone() {
                debugString2 = (DiffAlgorithm::printDiffTerminalColor(res.clone(), (std::sync::Arc::new(parseTreeNodeStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ParseTree>) -> Result<ArcStr> + 'static>))).clone();
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("replaceLabeledDiff change for label:")); __mm_s.push_str(&*parseTreeNodeStr(nodeLabel(added.clone()))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("before replaceLabeledDiff: ")); __mm_s.push_str(&*debugString1.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("after replaceLabeledDiff: ")); __mm_s.push_str(&*debugString2.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
        }
    }
    if debug.clone() {
        metamodelica::print((literal!("Before filter WS\n")).clone());
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*DiffAlgorithm::printDiffXml(res.clone(), (std::sync::Arc::new(parseTreeNodeStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ParseTree>) -> Result<ArcStr> + 'static>))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    res = filterDiffWhitespace(res.clone())?;
    if debug.clone() {
        metamodelica::print((literal!("After filter WS\n")).clone());
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*DiffAlgorithm::printDiffXml(res.clone(), (std::sync::Arc::new(parseTreeNodeStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ParseTree>) -> Result<ArcStr> + 'static>))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    if depth.clone() == 1 {
    }
    Ok((res, resLocal))
}

fn compareNodeLabels(mut t1: Arc<ParseTree>, mut t2: Arc<ParseTree>, mut compare: Arc<dyn ::std::ops::Fn(Arc<ParseTree>, Arc<ParseTree>) -> Result<bool> + 'static>) -> Result<bool> {
    let mut b: bool;
    b = compare(nodeLabel(t1.clone()), nodeLabel(t2.clone()))?;
    Ok(b)
}

fn compareNodeLabelsSpecial(mut t1: Arc<ParseTree>, mut t2: Arc<ParseTree>, mut compare: Arc<dyn ::std::ops::Fn(Arc<ParseTree>, Arc<ParseTree>) -> Result<bool> + 'static>, mut delList: Arc<metamodelica::List<ArcStr>>) -> Result<bool> {
    let mut b: bool;
    b = nodeLabelIsComponent(t1.clone()) && nodeLabelIsComponent(t2.clone()) && !(listMember((parseTreeStr(metamodelica::cons(nodeLabel(t1.clone()), metamodelica::nil()))?).clone(), delList.clone()));
    Ok(b)
}

fn nodeLabelIsComponent(mut t1: Arc<ParseTree>) -> bool {
    let mut b: bool;
    let mut contents: ArcStr = arcstr::literal!("");
    b = (::match_deref::match_deref! { match &(nodeLabel(t1.clone())) {
        Deref @ ParseTree::LEAF { token: Token { id: TokenId::IDENT { .. }, fileContents: __esc_contents, .. } } => {
            contents = (*__esc_contents).clone();
            0 == System::strncmp((contents.clone()).clone(), (literal!("$component:")).clone(), 11)
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn filterDiffWhitespace(mut inDiff: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>>) -> Result<Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>>> {
    let mut diff: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>>;
    let mut diffLocal: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>> = inDiff.clone();
    let mut diff1: (Diff, Arc<metamodelica::List<Arc<ParseTree>>>) = (Diff::Add, metamodelica::nil());
    let mut diff2: (Diff, Arc<metamodelica::List<Arc<ParseTree>>>) = (Diff::Add, metamodelica::nil());
    let mut diff3: (Diff, Arc<metamodelica::List<Arc<ParseTree>>>) = (Diff::Add, metamodelica::nil());
    let mut diff4: (Diff, Arc<metamodelica::List<Arc<ParseTree>>>) = (Diff::Add, metamodelica::nil());
    let mut firstIter: bool;
    let mut lastTokenNewline: bool;
    let mut hasAddedWS: bool;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut treeLocal: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut tree1: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut tree2: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut tree3: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut tree4: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut treeLast: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree4First: Arc<ParseTree> = Arc::new(ParseTree::EMPTY);
    let mut t1: Arc<ParseTree> = Arc::new(ParseTree::EMPTY);
    let mut t2: Arc<ParseTree> = Arc::new(ParseTree::EMPTY);
    let mut t3: Arc<ParseTree> = Arc::new(ParseTree::EMPTY);
    let mut firstTreeSecondLast: Arc<ParseTree> = Arc::new(ParseTree::EMPTY);
    let mut firstTreeLast: Arc<ParseTree> = Arc::new(ParseTree::EMPTY);
    let mut length: i32 = 0;
    let mut level: i32;
    let mut indentation: Arc<metamodelica::List<i32>>;
    let mut diffEnum: Diff = Diff::Add;
    let mut diffEnum1: Diff = Diff::Add;
    let mut diffEnum2: Diff = Diff::Add;
    let mut indentationStr: ArcStr;
    let mut tok: Token = <Token as ::std::default::Default>::default();
    diff = metamodelica::nil();
    firstIter = true;
    while !(diffLocal.clone().is_empty()) {
        (diffEnum, treeLast) = listHead(diffLocal.clone())?;
        (firstTreeSecondLast, firstTreeLast) = (::match_deref::match_deref! { match &(treeLast.clone()) {
        Deref @ metamodelica::List::Nil => (crate::SimpleModelicaParser::ParseTree::interned_EMPTY(), crate::SimpleModelicaParser::ParseTree::interned_EMPTY()),
        Deref @ metamodelica::List::Cons { head: __esc_firstTreeLast, tail: Deref @ metamodelica::List::Nil } => {
            firstTreeLast = (*__esc_firstTreeLast).clone();
            (crate::SimpleModelicaParser::ParseTree::interned_EMPTY(), firstTreeLast.clone())
        },
        _ => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(List::lastN(treeLast.clone(), 2)?) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            firstTreeSecondLast = __pa0.clone();
            firstTreeLast = __pa1.clone();
            (firstTreeSecondLast.clone(), firstTreeLast.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        diffLocal = (::match_deref::match_deref! { match &(diffLocal.clone()) {
        Deref @ metamodelica::List::Cons { head: (_, Deref @ metamodelica::List::Nil), tail: __esc_diffLocal } => {
            diffLocal = (*__esc_diffLocal).clone();
            diffLocal.clone()
        },
        Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Delete, tree), tail: __esc_diffLocal @ Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Equal, _), tail: _ } } if (if (firstIter.clone()) {({
        let mut __acc: Option<bool> = None;
        for mut t in (tree.clone()).into_iter().cloned() {
            let __x = parseTreeIsWhitespaceNotComment(t.clone());
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
        }
        __acc.unwrap_or(true)
    })} else {false}) => {
            diffLocal = (*__esc_diffLocal).clone();
            diff = metamodelica::cons((Diff::Equal.clone(), tree.clone()), diff.clone());
            diffLocal.clone()
        },
        Deref @ metamodelica::List::Cons { head: __esc_diff1 @ (DiffAlgorithm::Diff::Equal, _), tail: Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Delete, tree), tail: __esc_diffLocal @ Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Equal, _), tail: _ } } } if (({
        let mut __acc: Option<bool> = None;
        for mut t in (tree.clone()).into_iter().cloned() {
            let __x = parseTreeIsWhitespaceNotComment(t.clone());
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
        }
        __acc.unwrap_or(true)
    })) => {
            diff1 = (*__esc_diff1).clone();
            diffLocal = (*__esc_diffLocal).clone();
            diff = metamodelica::cons((Diff::Equal.clone(), tree.clone()), metamodelica::cons(diff1.clone(), diff.clone()));
            diffLocal.clone()
        },
        Deref @ metamodelica::List::Cons { head: __esc_diff1 @ (DiffAlgorithm::Diff::Equal, _), tail: Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Delete, tree), tail: Deref @ metamodelica::List::Nil } } if (({
        let mut __acc: Option<bool> = None;
        for mut t in (tree.clone()).into_iter().cloned() {
            let __x = parseTreeIsWhitespaceNotComment(t.clone());
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
        }
        __acc.unwrap_or(true)
    })) => {
            diff1 = (*__esc_diff1).clone();
            diff = metamodelica::cons((Diff::Equal.clone(), tree.clone()), metamodelica::cons(diff1.clone(), diff.clone()));
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: (_, tree), tail: __esc_diffLocal } if (({
        let mut __acc: Option<bool> = None;
        for mut t in (tree.clone()).into_iter().cloned() {
            let __x = isEmpty(t.clone());
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
        }
        __acc.unwrap_or(true)
    })) => {
            diffLocal = (*__esc_diffLocal).clone();
            diffLocal.clone()
        },
        Deref @ metamodelica::List::Cons { head: __esc_diff1, tail: Deref @ metamodelica::List::Cons { head: (_, tree), tail: __esc_diffLocal } } if (({
        let mut __acc: Option<bool> = None;
        for mut t in (tree.clone()).into_iter().cloned() {
            let __x = isEmpty(t.clone());
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
        }
        __acc.unwrap_or(true)
    })) => {
            diff1 = (*__esc_diff1).clone();
            diffLocal = (*__esc_diffLocal).clone();
            metamodelica::cons(diff1.clone(), diffLocal.clone())
        },
        Deref @ metamodelica::List::Cons { head: (__esc_diffEnum, tree), tail: __esc_diffLocal } if (({
        let mut __acc: Option<bool> = None;
        for mut t in (tree.clone()).into_iter().cloned() {
            let __x = isEmpty(t.clone());
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.unwrap_or(false)
    })) => {
            diffEnum = (*__esc_diffEnum).clone();
            diffLocal = (*__esc_diffLocal).clone();
            metamodelica::cons((diffEnum.clone(), ({
        let mut __acc: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
        for mut t in (tree.clone()).into_iter().cloned() {
            if !(!(isEmpty(t.clone()))) { continue; }
            let __x = t.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), diffLocal.clone())
        },
        Deref @ metamodelica::List::Cons { head: __esc_diff1, tail: Deref @ metamodelica::List::Cons { head: (__esc_diffEnum, tree), tail: __esc_diffLocal } } if (({
        let mut __acc: Option<bool> = None;
        for mut t in (tree.clone()).into_iter().cloned() {
            let __x = isEmpty(t.clone());
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.unwrap_or(false)
    })) => {
            diff1 = (*__esc_diff1).clone();
            diffEnum = (*__esc_diffEnum).clone();
            diffLocal = (*__esc_diffLocal).clone();
            metamodelica::cons(diff1.clone(), metamodelica::cons((diffEnum.clone(), ({
        let mut __acc: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
        for mut t in (tree.clone()).into_iter().cloned() {
            if !(!(isEmpty(t.clone()))) { continue; }
            let __x = t.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), diffLocal.clone()))
        },
        Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Delete, __esc_tree1), tail: Deref @ metamodelica::List::Cons { head: __esc_diff2 @ (_, tree2), tail: Deref @ metamodelica::List::Cons { head: __esc_diff3 @ (_, tree3), tail: Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Add, Deref @ metamodelica::List::Cons { head: tree4First, tail: __esc_tree4 }), tail: __esc_diffLocal } } } } if (({
        let mut __acc: Option<bool> = None;
        for mut t in (tree2.clone()).into_iter().cloned() {
            let __x = parseTreeIsWhitespaceNotComment(t.clone());
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
        }
        __acc.unwrap_or(true)
    }) && ({
        let mut __acc: Option<bool> = None;
        for mut t in (tree3.clone()).into_iter().cloned() {
            let __x = parseTreeIsWhitespaceNotComment(t.clone());
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
        }
        __acc.unwrap_or(true)
    }) && modelicaDiffTokenEq(lastToken(firstTreeLast.clone())?, firstTokenInTree(tree4First.clone())?)?) => {
            tree1 = (*__esc_tree1).clone();
            diff2 = (*__esc_diff2).clone();
            diff3 = (*__esc_diff3).clone();
            tree4 = (*__esc_tree4).clone();
            diffLocal = (*__esc_diffLocal).clone();
            metamodelica::cons((Diff::Delete.clone(), removeLastTokenInTrees(tree1.clone())?), metamodelica::cons((Diff::Equal.clone(), list![Arc::new(ParseTree::LEAF { token: lastToken(firstTreeLast.clone())? })]), metamodelica::cons((Diff::Add.clone(), metamodelica::cons(removeFirstTokenInTree(tree4First.clone())?, tree4.clone())), metamodelica::cons(diff2.clone(), metamodelica::cons(diff3.clone(), diffLocal.clone())))))
        },
        Deref @ metamodelica::List::Cons { head: __esc_diff1 @ (DiffAlgorithm::Diff::Equal, __esc_tree1 @ Deref @ metamodelica::List::Cons { head: _, tail: _ }), tail: Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Delete, __esc_tree), tail: __esc_diffLocal @ Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Equal, tree2 @ Deref @ metamodelica::List::Cons { head: _, tail: _ }), tail: _ } } } if (needsWhitespaceBetweenTokens(lastToken(firstTreeLast.clone())?, firstTokenInTree((tree2.clone()).get(1)?)?)?) => {
            diff1 = (*__esc_diff1).clone();
            tree1 = (*__esc_tree1).clone();
            tree = (*__esc_tree).clone();
            diffLocal = (*__esc_diffLocal).clone();
            diff = metamodelica::cons((Diff::Equal.clone(), list![Arc::new(ParseTree::LEAF { token: makeToken(TokenId::WHITESPACE.clone(), (literal!(" ")).clone()) })]), metamodelica::cons(diff1.clone(), diff.clone()));
            diffLocal.clone()
        },
        Deref @ metamodelica::List::Cons { head: __esc_diff1 @ (DiffAlgorithm::Diff::Equal, __esc_tree1 @ Deref @ metamodelica::List::Cons { head: _, tail: _ }), tail: Deref @ metamodelica::List::Cons { head: __esc_diff2 @ (DiffAlgorithm::Diff::Add, tree2 @ Deref @ metamodelica::List::Cons { head: _, tail: _ }), tail: __esc_diffLocal } } if (needsWhitespaceBetweenTokens(lastToken(firstTreeLast.clone())?, firstTokenInTree((tree2.clone()).get(1)?)?)?) => {
            diff1 = (*__esc_diff1).clone();
            tree1 = (*__esc_tree1).clone();
            diff2 = (*__esc_diff2).clone();
            diffLocal = (*__esc_diffLocal).clone();
            diffLocal = metamodelica::cons(diff1.clone(), metamodelica::cons((Diff::Equal.clone(), list![Arc::new(ParseTree::LEAF { token: makeToken(TokenId::WHITESPACE.clone(), (literal!(" ")).clone()) })]), metamodelica::cons(diff2.clone(), diffLocal.clone())));
            diffLocal.clone()
        },
        Deref @ metamodelica::List::Cons { head: __esc_diff1 @ (DiffAlgorithm::Diff::Equal, __esc_tree1 @ Deref @ metamodelica::List::Cons { head: _, tail: _ }), tail: Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Delete, __esc_tree), tail: __esc_diffLocal @ Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Equal, __esc_tree2 @ Deref @ metamodelica::List::Cons { head: _, tail: _ }), tail: _ } } } => {
            diff1 = (*__esc_diff1).clone();
            tree1 = (*__esc_tree1).clone();
            tree = (*__esc_tree).clone();
            diffLocal = (*__esc_diffLocal).clone();
            tree2 = (*__esc_tree2).clone();
            diff = metamodelica::cons(diff1.clone(), diff.clone());
            metamodelica::cons((Diff::Delete.clone(), tree.clone()), diffLocal.clone())
        },
        Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Add, tree), tail: __esc_diffLocal @ Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Equal, _), tail: _ } } if (if (firstIter.clone()) {({
        let mut __acc: Option<bool> = None;
        for mut t in (tree.clone()).into_iter().cloned() {
            let __x = parseTreeIsWhitespaceNotComment(t.clone());
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
        }
        __acc.unwrap_or(true)
    })} else {false}) => {
            diffLocal = (*__esc_diffLocal).clone();
            diffLocal.clone()
        },
        Deref @ metamodelica::List::Cons { head: __esc_diff1 @ (DiffAlgorithm::Diff::Equal, _), tail: Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Add, tree), tail: __esc_diffLocal @ Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Equal, _), tail: _ } } } if (({
        let mut __acc: Option<bool> = None;
        for mut t in (tree.clone()).into_iter().cloned() {
            let __x = parseTreeIsWhitespaceNotComment(t.clone());
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
        }
        __acc.unwrap_or(true)
    })) => {
            diff1 = (*__esc_diff1).clone();
            diffLocal = (*__esc_diffLocal).clone();
            diff = metamodelica::cons(diff1.clone(), diff.clone());
            diffLocal.clone()
        },
        Deref @ metamodelica::List::Cons { head: __esc_diff1 @ (DiffAlgorithm::Diff::Delete, __esc_tree1), tail: Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Add, tree2), tail: __esc_diffLocal @ Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Equal, __esc_tree3), tail: _ } } } if (!(parseTreeIsNewLine(firstTreeLast.clone())) && parseTreeIsNewLine(List::last(tree2.clone())?)) => {
            diff1 = (*__esc_diff1).clone();
            tree1 = (*__esc_tree1).clone();
            diffLocal = (*__esc_diffLocal).clone();
            tree3 = (*__esc_tree3).clone();
            metamodelica::cons(diff1.clone(), metamodelica::cons((Diff::Add.clone(), List::stripLast(tree2.clone())?), diffLocal.clone()))
        },
        Deref @ metamodelica::List::Cons { head: __esc_diff1 @ (DiffAlgorithm::Diff::Delete, Deref @ metamodelica::List::Cons { head: t1, tail: Deref @ metamodelica::List::Nil }), tail: Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Add, Deref @ metamodelica::List::Cons { head: t2, tail: Deref @ metamodelica::List::Cons { head: t3, tail: Deref @ metamodelica::List::Nil } }), tail: __esc_diffLocal } } if (parseTreeIsOnlyIdent(t1.clone()) && parseTreeIsOnlyIdent(t3.clone()) && parseTreeIsWhitespaceNotComment(t2.clone())) => {
            diff1 = (*__esc_diff1).clone();
            diffLocal = (*__esc_diffLocal).clone();
            metamodelica::cons(diff1.clone(), metamodelica::cons((Diff::Add.clone(), list![t3.clone()]), diffLocal.clone()))
        },
        Deref @ metamodelica::List::Cons { head: __esc_diff1 @ (DiffAlgorithm::Diff::Delete, Deref @ metamodelica::List::Cons { head: t1, tail: Deref @ metamodelica::List::Nil }), tail: Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Add, Deref @ metamodelica::List::Cons { head: t2, tail: Deref @ metamodelica::List::Cons { head: t3, tail: Deref @ metamodelica::List::Nil } }), tail: __esc_diffLocal } } if (parseTreeIsOnlyIdent(t1.clone()) && parseTreeIsOnlyIdent(t2.clone()) && parseTreeIsWhitespaceNotComment(t3.clone())) => {
            diff1 = (*__esc_diff1).clone();
            diffLocal = (*__esc_diffLocal).clone();
            metamodelica::cons(diff1.clone(), metamodelica::cons((Diff::Add.clone(), list![t2.clone()]), diffLocal.clone()))
        },
        Deref @ metamodelica::List::Cons { head: __esc_diff1 @ (DiffAlgorithm::Diff::Equal, _), tail: Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Add, tree2), tail: __esc_diffLocal @ Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Equal, Deref @ metamodelica::List::Cons { head: t1, tail: _ }), tail: _ } } } if (!(parseTreeIsNewLine(List::last(tree2.clone())?)) && parseTreeIsOnlyEnd(t1.clone())) => {
            diff1 = (*__esc_diff1).clone();
            diffLocal = (*__esc_diffLocal).clone();
            metamodelica::cons(diff1.clone(), metamodelica::cons((Diff::Add.clone(), listAppend(tree2.clone(), list![Arc::new(ParseTree::LEAF { token: makeToken(TokenId::NEWLINE.clone(), (literal!("\n")).clone()) })])), diffLocal.clone()))
        },
        Deref @ metamodelica::List::Cons { head: __esc_diff1 @ (DiffAlgorithm::Diff::Equal, __esc_tree1), tail: Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Add, tree2 @ Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } }), tail: __esc_diffLocal } } if (!(needsWhitespaceBetweenTokens(lastToken(firstTreeLast.clone())?, firstTokenInTree(List::second(tree2.clone())?)?)?) && parseTreeIsWhitespaceNotComment(listHead(tree2.clone())?) && !(parseTreeIsNewLine(firstTreeLast.clone()))) => {
            diff1 = (*__esc_diff1).clone();
            tree1 = (*__esc_tree1).clone();
            diffLocal = (*__esc_diffLocal).clone();
            metamodelica::cons(diff1.clone(), metamodelica::cons((Diff::Add.clone(), listRest(tree2.clone())?), diffLocal.clone()))
        },
        Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Add, __esc_tree1 @ Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } }), tail: Deref @ metamodelica::List::Cons { head: __esc_diff2 @ (DiffAlgorithm::Diff::Equal, tree2), tail: __esc_diffLocal } } if (!(needsWhitespaceBetweenTokens(lastToken(firstTreeLast.clone())?, firstTokenInTree(listHead(tree2.clone())?)?)?) && !(parseTreeIsNewLine(firstTreeSecondLast.clone()) || parseTreeIsLineComment(firstTreeSecondLast.clone())) && parseTreeIsWhitespaceNotCommentOrNewline(firstTreeLast.clone())) => {
            tree1 = (*__esc_tree1).clone();
            diff2 = (*__esc_diff2).clone();
            diffLocal = (*__esc_diffLocal).clone();
            metamodelica::cons((Diff::Add.clone(), List::stripLast(tree1.clone())?), metamodelica::cons(diff2.clone(), diffLocal.clone()))
        },
        Deref @ metamodelica::List::Cons { head: __esc_diff1 @ (DiffAlgorithm::Diff::Equal, __esc_tree1), tail: Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Add, tree2), tail: __esc_diffLocal } } if (parseTreeIsNewLine(firstTreeLast.clone()) && parseTreeIsNewLine(listHead(tree2.clone())?)) => {
            diff1 = (*__esc_diff1).clone();
            tree1 = (*__esc_tree1).clone();
            diffLocal = (*__esc_diffLocal).clone();
            metamodelica::cons(diff1.clone(), metamodelica::cons((Diff::Add.clone(), listRest(tree2.clone())?), diffLocal.clone()))
        },
        Deref @ metamodelica::List::Cons { head: __esc_diff1 @ (DiffAlgorithm::Diff::Equal, __esc_tree1), tail: Deref @ metamodelica::List::Cons { head: __esc_diff2 @ (DiffAlgorithm::Diff::Add, __esc_tree2), tail: Deref @ metamodelica::List::Cons { head: __esc_diff3 @ (DiffAlgorithm::Diff::Equal, tree3), tail: Deref @ metamodelica::List::Cons { head: __esc_diff4 @ (DiffAlgorithm::Diff::Delete, Deref @ metamodelica::List::Cons { head: __esc_tree4First, tail: __esc_tree4 }), tail: __esc_diffLocal } } } } if (parseTreeIsNewLine(firstTreeLast.clone()) && ({
        let mut __acc: Option<bool> = None;
        for mut t in (tree3.clone()).into_iter().cloned() {
            let __x = parseTreeIsWhitespaceNotComment(t.clone());
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
        }
        __acc.unwrap_or(true)
    })) => {
            diff1 = (*__esc_diff1).clone();
            tree1 = (*__esc_tree1).clone();
            diff2 = (*__esc_diff2).clone();
            tree2 = (*__esc_tree2).clone();
            diff3 = (*__esc_diff3).clone();
            diff4 = (*__esc_diff4).clone();
            tree4First = (*__esc_tree4First).clone();
            tree4 = (*__esc_tree4).clone();
            diffLocal = (*__esc_diffLocal).clone();
            metamodelica::cons(diff1.clone(), metamodelica::cons(diff3.clone(), metamodelica::cons(diff2.clone(), metamodelica::cons(diff4.clone(), diffLocal.clone()))))
        },
        Deref @ metamodelica::List::Cons { head: __esc_diff1 @ (DiffAlgorithm::Diff::Equal, __esc_tree1), tail: Deref @ metamodelica::List::Cons { head: __esc_diff2 @ (DiffAlgorithm::Diff::Add, __esc_tree2), tail: Deref @ metamodelica::List::Cons { head: __esc_diff3 @ (DiffAlgorithm::Diff::Equal, tree3), tail: Deref @ metamodelica::List::Cons { head: __esc_diff4 @ (DiffAlgorithm::Diff::Delete, Deref @ metamodelica::List::Cons { head: __esc_tree4First, tail: __esc_tree4 }), tail: __esc_diffLocal } } } } if (parseTreeIsNewLine(firstTreeLast.clone()) && ({
        let mut __acc: Option<bool> = None;
        for mut t in (tree3.clone()).into_iter().cloned() {
            let __x = parseTreeIsWhitespaceNotComment(t.clone());
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
        }
        __acc.unwrap_or(true)
    })) => {
            diff1 = (*__esc_diff1).clone();
            tree1 = (*__esc_tree1).clone();
            diff2 = (*__esc_diff2).clone();
            tree2 = (*__esc_tree2).clone();
            diff3 = (*__esc_diff3).clone();
            diff4 = (*__esc_diff4).clone();
            tree4First = (*__esc_tree4First).clone();
            tree4 = (*__esc_tree4).clone();
            diffLocal = (*__esc_diffLocal).clone();
            metamodelica::cons(diff1.clone(), metamodelica::cons(diff3.clone(), metamodelica::cons(diff2.clone(), metamodelica::cons(diff4.clone(), diffLocal.clone()))))
        },
        Deref @ metamodelica::List::Cons { head: (diffEnum1, __esc_tree1), tail: Deref @ metamodelica::List::Cons { head: (diffEnum2, __esc_tree2), tail: __esc_diffLocal } } if (diffEnum1.clone() == diffEnum2.clone()) => {
            tree1 = (*__esc_tree1).clone();
            tree2 = (*__esc_tree2).clone();
            diffLocal = (*__esc_diffLocal).clone();
            metamodelica::cons((diffEnum1.clone(), listAppend(tree1.clone(), tree2.clone())), diffLocal.clone())
        },
        Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Add, __esc_tree1), tail: __esc_diffLocal @ Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Equal, tree2), tail: _ } } if (tokenId(lastToken(firstTreeLast.clone())?)? == TokenId::WHITESPACE.clone() && tokenId(firstToken(tree2.clone()))? == TokenId::NEWLINE.clone()) => {
            tree1 = (*__esc_tree1).clone();
            diffLocal = (*__esc_diffLocal).clone();
            diff = metamodelica::cons((Diff::Add.clone(), removeLastTokenInTrees(tree1.clone())?), diff.clone());
            diffLocal.clone()
        },
        Deref @ metamodelica::List::Cons { head: __esc_diff1, tail: __esc_diffLocal } => {
            diff1 = (*__esc_diff1).clone();
            diffLocal = (*__esc_diffLocal).clone();
            diff = metamodelica::cons(diff1.clone(), diff.clone());
            diffLocal.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
        firstIter = false;
    }
    diff = metamodelica::Dangerous::listReverseInPlace(diff.clone());
    lastTokenNewline = false;
    indentation = metamodelica::nil();
    hasAddedWS = false;
    for mut d in &*diff.clone() {
        let mut d = d.clone();
        let () = (::match_deref::match_deref! { match &(d.clone()) {
        (DiffAlgorithm::Diff::Add, __esc_tree) => {
            tree = (*__esc_tree).clone();
            for mut t in &*tree.clone() {
                let mut t = t.clone();
                let () = (::match_deref::match_deref! { match &(firstNTokensInTree_reverse(t.clone(), 2, metamodelica::nil())?) {
        Deref @ metamodelica::List::Cons { head: Token { id: TokenId::WHITESPACE, length: __esc_length, .. }, tail: Deref @ metamodelica::List::Cons { head: Token { id: TokenId::NEWLINE, .. }, tail: Deref @ metamodelica::List::Nil } } => {
            length = (*__esc_length).clone();
            hasAddedWS = true;
            ()
        },
        Deref @ metamodelica::List::Cons { head: Token { id: TokenId::WHITESPACE, length: __esc_length, .. }, tail: Deref @ metamodelica::List::Nil } if (lastTokenNewline.clone()) => {
            length = (*__esc_length).clone();
            hasAddedWS = true;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            ()
        },
        (_, __esc_tree) => {
            tree = (*__esc_tree).clone();
            for mut t in &*tree.clone() {
                let mut t = t.clone();
                let () = (::match_deref::match_deref! { match &(firstNTokensInTree_reverse(t.clone(), 2, metamodelica::nil())?) {
        Deref @ metamodelica::List::Cons { head: Token { id: TokenId::WHITESPACE, length: __esc_length, .. }, tail: Deref @ metamodelica::List::Cons { head: Token { id: TokenId::NEWLINE, .. }, tail: Deref @ metamodelica::List::Nil } } => {
            length = (*__esc_length).clone();
            indentation = metamodelica::cons(length.clone(), indentation.clone());
            lastTokenNewline = false;
            ()
        },
        Deref @ metamodelica::List::Cons { head: Token { id: TokenId::WHITESPACE, length: __esc_length, .. }, tail: Deref @ metamodelica::List::Nil } if (lastTokenNewline.clone()) => {
            length = (*__esc_length).clone();
            indentation = metamodelica::cons(length.clone(), indentation.clone());
            lastTokenNewline = false;
            ()
        },
        Deref @ metamodelica::List::Cons { head: Token { id: TokenId::NEWLINE, .. }, tail: Deref @ metamodelica::List::Nil } => {
            lastTokenNewline = true;
            ()
        },
        _ => {
            lastTokenNewline = false;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    if indentation.clone().is_empty() || !(hasAddedWS.clone()) {
        if debug.clone() {
            metamodelica::print((literal!("Skipping indentation as we could not auto-detect suitable indentation levels\n")).clone());
        }
        return Ok(diff.clone());
    }
    level = ({
        let mut __acc: Option<i32> = None;
        for mut l in (indentation.clone()).into_iter().cloned() {
            let __x = l.clone();
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
        }
        __acc.unwrap_or(i32::MAX)
    });
    indentationStr = (StringUtil::repeat((literal!(" ")).clone(), level.clone())).clone();
    diffLocal = metamodelica::nil();
    for mut d in &*diff.clone() {
        let mut d = d.clone();
        let () = (::match_deref::match_deref! { match &(d.clone()) {
        (DiffAlgorithm::Diff::Delete, __esc_tree) => {
            tree = (*__esc_tree).clone();
            diffLocal = metamodelica::cons(d.clone(), diffLocal.clone());
            ()
        },
        (__esc_diffEnum, __esc_tree) => {
            diffEnum = (*__esc_diffEnum).clone();
            tree = (*__esc_tree).clone();
            treeLocal = metamodelica::nil();
            hasAddedWS = false;
            for mut t in &*tree.clone() {
                let mut t = t.clone();
                let () = (::match_deref::match_deref! { match &((diffEnum.clone(), firstNTokensInTree_reverse(t.clone(), 2, metamodelica::nil())?)) {
        (DiffAlgorithm::Diff::Equal, _) => (),
        (_, Deref @ metamodelica::List::Cons { head: Token { id: TokenId::WHITESPACE, length: __esc_length, .. }, tail: Deref @ metamodelica::List::Cons { head: __esc_tok @ Token { id: TokenId::NEWLINE, .. }, tail: Deref @ metamodelica::List::Nil } }) => {
            length = (*__esc_length).clone();
            tok = (*__esc_tok).clone();
            treeLocal = metamodelica::cons(replaceFirstTokensInTree(t.clone(), list![tok.clone(), makeToken(TokenId::WHITESPACE.clone(), (indentationStr.clone()).clone())])?, treeLocal.clone());
            hasAddedWS = true;
            ()
        },
        (_, Deref @ metamodelica::List::Cons { head: Token { id: TokenId::WHITESPACE, length: __esc_length, .. }, tail: Deref @ metamodelica::List::Nil }) if (lastTokenNewline.clone()) => {
            length = (*__esc_length).clone();
            treeLocal = metamodelica::cons(replaceFirstTokensInTree(t.clone(), list![makeToken(TokenId::WHITESPACE.clone(), (indentationStr.clone()).clone())])?, treeLocal.clone());
            hasAddedWS = true;
            ()
        },
        _ => {
            treeLocal = metamodelica::cons(t.clone(), treeLocal.clone());
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                lastTokenNewline = (match lastToken(t.clone())? {
        Token { id: TokenId::NEWLINE, .. } => true,
        _ => false,
    });
            }
            diffLocal = if (hasAddedWS.clone()) {metamodelica::cons((diffEnum.clone(), treeLocal.clone().reverse()), diffLocal.clone())} else {metamodelica::cons(d.clone(), diffLocal.clone())};
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    diff = metamodelica::Dangerous::listReverseInPlace(diffLocal.clone());
    Ok(diff)
}

fn labelOrderDidNotChange(mut addList: Arc<metamodelica::List<ArcStr>>, mut delList: Arc<metamodelica::List<ArcStr>>) -> Result<bool> {
    let mut b: bool;
    let mut acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut del: Arc<metamodelica::List<ArcStr>> = delList.clone();
    let mut s: ArcStr;
    b = false;
    for mut item in &*addList.clone() {
        let mut item = item.clone();
        if listMember((item.clone()).clone(), acc.clone()) {
            return Ok(b.clone());
        }
        if listMember((item.clone()).clone(), del.clone()) {
            while item.clone() != listHead(del.clone())? {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(del.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                s = __pa0.clone();
                del = __pa1.clone();
                if listMember((s.clone()).clone(), acc.clone()) {
                    return Ok(b.clone());
                }
                acc = metamodelica::cons((s.clone()).clone(), acc.clone());
            }
            del = listRest(del.clone())?;
        }
        acc = metamodelica::cons((item.clone()).clone(), acc.clone());
    }
    for mut item in &*delList.clone() {
        let mut item = item.clone();
        if listMember((item.clone()).clone(), acc.clone()) {
            return Ok(b.clone());
        }
        acc = metamodelica::cons((item.clone()).clone(), acc.clone());
    }
    b = true;
    Ok(b)
}

fn makeToken(mut id: TokenId, mut r#str: ArcStr) -> Token {
    let mut token: Token;
    token = Token { fileName: (literal!("<dummy>")).clone(), id: id.clone(), fileContents: (r#str.clone()).clone(), byteOffset: 1, length: ((r#str.clone()).clone().len() as i32), lineNumberStart: 0, columnNumberStart: 0, lineNumberEnd: 0, columnNumberEnd: 0 };
    token
}

fn replaceLabeledDiff(mut inDiff: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>>, mut diffedNodes: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>>, mut labelOfDiffedAddedNodes: Arc<ParseTree>, mut labelOfDiffedDeletedNodes: Arc<ParseTree>, mut compare: Arc<dyn ::std::ops::Fn(Arc<ParseTree>, Arc<ParseTree>) -> Result<bool> + 'static>, mut inAllLabelsAreInOrder: bool) -> Result<Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>>> {
    let mut res: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>> = metamodelica::nil();
    let mut filtered: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>> = metamodelica::nil();
    let mut lst: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut acc: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut found: bool = false;
    let mut allLabelsAreInOrder: bool = inAllLabelsAreInOrder.clone();
    let mut d: Diff = Diff::Add;
    if parseTreeStr(metamodelica::cons(labelOfDiffedDeletedNodes.clone(), metamodelica::nil()))? == literal!("$equation_section") {
        allLabelsAreInOrder = false;
    }
    for mut diff in &*inDiff.clone() {
        let mut diff = diff.clone();
        res = (::match_deref::match_deref! { match &(diff.clone()) {
        (DiffAlgorithm::Diff::Equal, _) => metamodelica::cons(diff.clone(), res.clone()),
        (DiffAlgorithm::Diff::Add, lst) if (!(({
        let mut __acc: Option<bool> = None;
        for mut t in (lst.clone()).into_iter().cloned() {
            let __x = compare(nodeLabel(t.clone()), labelOfDiffedAddedNodes.clone())?;
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.unwrap_or(false)
    }))) => metamodelica::cons(diff.clone(), res.clone()),
        (DiffAlgorithm::Diff::Delete, lst) if (!(({
        let mut __acc: Option<bool> = None;
        for mut t in (lst.clone()).into_iter().cloned() {
            let __x = compare(nodeLabel(t.clone()), labelOfDiffedDeletedNodes.clone())?;
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.unwrap_or(false)
    }))) => metamodelica::cons(diff.clone(), res.clone()),
        (DiffAlgorithm::Diff::Add, __esc_lst) if (allLabelsAreInOrder.clone()) => {
            lst = (*__esc_lst).clone();
            metamodelica::cons((Diff::Add.clone(), ({
        let mut __acc: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
        for mut t in (lst.clone()).into_iter().cloned() {
            if !(!(compare(nodeLabel(t.clone()), labelOfDiffedAddedNodes.clone())?)) { continue; }
            let __x = t.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), res.clone())
        },
        (DiffAlgorithm::Diff::Delete, __esc_lst) if (!(allLabelsAreInOrder.clone())) => {
            lst = (*__esc_lst).clone();
            metamodelica::cons((Diff::Delete.clone(), ({
        let mut __acc: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
        for mut t in (lst.clone()).into_iter().cloned() {
            if !(!(compare(nodeLabel(t.clone()), labelOfDiffedDeletedNodes.clone())?)) { continue; }
            let __x = t.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), res.clone())
        },
        (__esc_d, __esc_lst) => {
            d = (*__esc_d).clone();
            lst = (*__esc_lst).clone();
            acc = metamodelica::nil();
            for mut t in &*lst.clone() {
                let mut t = t.clone();
                if !(found.clone()) && compare(nodeLabel(t.clone()), if (allLabelsAreInOrder.clone()) {labelOfDiffedDeletedNodes.clone()} else {labelOfDiffedAddedNodes.clone()})? {
                    if !(acc.clone().is_empty()) {
                        res = metamodelica::cons((Diff::Add.clone(), acc.clone().reverse()), res.clone());
                        acc = metamodelica::nil();
                    }
                    filtered = ({
        let mut __acc: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>> = metamodelica::nil();
        for mut i in (diffedNodes.clone()).into_iter().cloned() {
            if !((::match_deref::match_deref! { match &(i.clone()) {
        (DiffAlgorithm::Diff::Delete, _) => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })) { continue; }
            let __x = i.clone();
            __acc = cons(__x, __acc);
        }
        __acc
    });
                    res = listAppend(filtered.clone(), res.clone());
                    found = true;
                } else {
                    res = metamodelica::cons((d.clone(), list![t.clone()]), res.clone());
                }
            }
            if !(acc.clone().is_empty()) {
                res = metamodelica::cons((Diff::Add.clone(), acc.clone().reverse()), res.clone());
            }
            res.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    res = res.clone().reverse();
    Ok(res)
}

fn isEmpty(mut tree: Arc<ParseTree>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ ParseTree::EMPTY { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn isLabeledNode(mut tree: Arc<ParseTree>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ ParseTree::NODE { label: Deref @ ParseTree::EMPTY { .. }, .. } => false,
        Deref @ ParseTree::NODE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn nodeLabel(mut tree: Arc<ParseTree>) -> Arc<ParseTree> {
    let mut label: Arc<ParseTree>;
    label = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ ParseTree::NODE { .. } => var_field!((*tree).label, ParseTree::NODE).clone(),
        _ => crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    label
}

fn parseTreeEq(mut t1: Arc<ParseTree>, mut t2: Arc<ParseTree>, mut diffSubtreeWorkArray1: metamodelica::Array<Token>, mut diffSubtreeWorkArray2: metamodelica::Array<Token>) -> Result<bool> {
    let mut b: bool;
    let mut len1: i32;
    let mut len2: i32;
    let mut commentLen1: i32;
    let mut commentLen2: i32;
    (len1, commentLen1) = findTokens(t1.clone(), diffSubtreeWorkArray1.clone(), 0, 0)?;
    (len2, commentLen2) = findTokens(t2.clone(), diffSubtreeWorkArray2.clone(), 0, 0)?;
    b = false;
    if len1.clone() != len2.clone() || commentLen1.clone() != commentLen2.clone() {
        return Ok(b.clone());
    }
    for mut i in 1..=len1.clone() {
        if !(modelicaDiffTokenEq(({let __elt = diffSubtreeWorkArray1.borrow()[(i.clone()-1) as usize].clone(); __elt}), ({let __elt = diffSubtreeWorkArray2.borrow()[(i.clone()-1) as usize].clone(); __elt}))?) {
            return Ok(b.clone());
        }
    }
    for mut i in 1..=commentLen1.clone() {
        if !(modelicaDiffTokenEq(({let __elt = diffSubtreeWorkArray1.borrow()[(metamodelica::arrayLength(diffSubtreeWorkArray1.clone()) - (i.clone() - 1)-1) as usize].clone(); __elt}), ({let __elt = diffSubtreeWorkArray2.borrow()[(metamodelica::arrayLength(diffSubtreeWorkArray2.clone()) - (i.clone() - 1)-1) as usize].clone(); __elt}))?) {
            return Ok(b.clone());
        }
    }
    b = true;
    Ok(b)
}

fn findTokens(mut t: Arc<ParseTree>, mut work: metamodelica::Array<Token>, mut inCount: i32, mut inCommentCount: i32) -> Result<(i32, i32)> {
    let mut count: i32 = inCount.clone();
    let mut commentCount: i32 = inCommentCount.clone();
    if parseTreeIsComment(t.clone()) {
        metamodelica::arrayUpdate(work.clone(), metamodelica::arrayLength(work.clone()) - commentCount.clone(), firstTokenInTree(t.clone())?)?;
        commentCount = commentCount.clone() + 1;
        return Ok((count.clone(), commentCount.clone()));
    } else if parseTreeIsWhitespace(t.clone()) {
        return Ok((count.clone(), commentCount.clone()));
    }
    let () = (::match_deref::match_deref! { match &(t.clone()) {
        Deref @ ParseTree::EMPTY { .. } => (),
        Deref @ ParseTree::LEAF { .. } => {
            count = count.clone() + 1;
            metamodelica::arrayUpdate(work.clone(), count.clone(), var_field!((*t).token, ParseTree::LEAF).clone())?;
            ()
        },
        Deref @ ParseTree::NODE { .. } => {
            for mut n in &*var_field!((*t).nodes, ParseTree::NODE).clone() {
                let mut n = n.clone();
                (count, commentCount) = findTokens(n.clone(), work.clone(), count.clone(), commentCount.clone())?;
            }
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((count, commentCount))
}

fn replaceFirstTokensInTree(mut t: Arc<ParseTree>, mut tokens: Arc<metamodelica::List<Token>>) -> Result<Arc<ParseTree>> {
    let mut tree: Arc<ParseTree>;
    let __pa0 = ::match_deref::match_deref! { match &(replaceFirstTokensInTreeWork(t.clone(), tokens.clone())?) {
        (__pa0, Deref @ metamodelica::List::Nil) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    tree = __pa0.clone();
    Ok(tree)
}

fn replaceFirstTokensInTreeWork(mut t: Arc<ParseTree>, mut inTokens: Arc<metamodelica::List<Token>>) -> Result<(Arc<ParseTree>, Arc<metamodelica::List<Token>>)> {
    let mut tree: Arc<ParseTree> = t.clone();
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut work: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut acc: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut n: Arc<ParseTree> = Arc::new(ParseTree::EMPTY);
    let mut tok: Token = <Token as ::std::default::Default>::default();
    (tree, tokens) = (::match_deref::match_deref! { match &((tree.clone(), tokens.clone())) {
        (__esc_tree, Deref @ metamodelica::List::Nil) => {
            tree = (*__esc_tree).clone();
            (tree.clone(), tokens.clone())
        },
        (Deref @ ParseTree::EMPTY { .. }, _) => (tree.clone(), tokens.clone()),
        (Deref @ ParseTree::LEAF { .. }, Deref @ metamodelica::List::Cons { head: __esc_tok, tail: __esc_tokens }) => {
            tokens = (*__esc_tokens).clone();
            tok = (*__esc_tok).clone();
            (Arc::new(ParseTree::LEAF { token: tok.clone() }), tokens.clone())
        },
        (Deref @ ParseTree::NODE { .. }, __esc_tokens) => {
            tokens = (*__esc_tokens).clone();
            work = var_field!((*tree).nodes, ParseTree::NODE).clone();
            acc = metamodelica::nil();
            while !(work.clone().is_empty()) {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(work.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                n = __pa0.clone();
                work = __pa1.clone();
                (n, tokens) = replaceFirstTokensInTreeWork(n.clone(), tokens.clone())?;
                if tokens.clone().is_empty() {
                    assign_variant_field!(tree => ParseTree::NODE; nodes = List::append_reverse(acc.clone(), metamodelica::cons(n.clone(), work.clone())));
                    return Ok((tree.clone(), tokens.clone()));
                } else {
                    acc = metamodelica::cons(n.clone(), acc.clone());
                }
            }
            assign_variant_field!(tree => ParseTree::NODE; nodes = acc.clone().reverse());
            (tree.clone(), tokens.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((tree, tokens))
}

fn firstNTokensInTree_reverse(mut t: Arc<ParseTree>, mut n: i32, mut acc: Arc<metamodelica::List<Token>>) -> Result<Arc<metamodelica::List<Token>>> {
    let mut tokens: Arc<metamodelica::List<Token>> = acc.clone();
    if (tokens.clone().len() as i32) > 1 {
        return Ok(tokens.clone());
    }
    tokens = (::match_deref::match_deref! { match &(t.clone()) {
        Deref @ ParseTree::EMPTY { .. } => tokens.clone(),
        Deref @ ParseTree::LEAF { .. } => metamodelica::cons(var_field!((*t).token, ParseTree::LEAF).clone(), tokens.clone()),
        Deref @ ParseTree::NODE { .. } => {
            for mut node in &*var_field!((*t).nodes, ParseTree::NODE).clone() {
                let mut node = node.clone();
                tokens = firstNTokensInTree_reverse(node.clone(), n.clone(), tokens.clone())?;
                if (tokens.clone().len() as i32) > 1 {
                    return Ok(tokens.clone());
                }
            }
            acc.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(tokens)
}

fn removeFirstTokenInTree(mut t: Arc<ParseTree>) -> Result<Arc<ParseTree>> {
    let mut t: Arc<ParseTree> = t;
    t = (::match_deref::match_deref! { match &(t.clone()) {
        Deref @ ParseTree::EMPTY { .. } => {
            bail!("fail")
        },
        Deref @ ParseTree::LEAF { .. } => {
            crate::SimpleModelicaParser::ParseTree::interned_EMPTY()
        },
        Deref @ ParseTree::NODE { label, nodes: Deref @ metamodelica::List::Cons { head: node, tail: nodes } } => {
            makeNode(metamodelica::cons(removeFirstTokenInTree(node.clone())?, nodes.clone()), label.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(t)
}

fn removeLastTokenInTree(mut t: Arc<ParseTree>) -> Result<Arc<ParseTree>> {
    let mut t: Arc<ParseTree> = t;
    t = (::match_deref::match_deref! { match &(t.clone()) {
        Deref @ ParseTree::EMPTY { .. } => {
            bail!("fail")
        },
        Deref @ ParseTree::LEAF { .. } => {
            crate::SimpleModelicaParser::ParseTree::interned_EMPTY()
        },
        Deref @ ParseTree::NODE { label, nodes } => {
            let mut node: Arc<ParseTree> = Arc::new(ParseTree::EMPTY);
            let mut nodes = (*nodes).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(nodes.clone().reverse()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            node = __pa0.clone();
            nodes = __pa1.clone();
            makeNode(metamodelica::cons(removeLastTokenInTree(node.clone())?, nodes.clone()).reverse(), label.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(t)
}

fn removeLastTokenInTrees(mut ts: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<Arc<metamodelica::List<Arc<ParseTree>>>> {
    let mut ts: Arc<metamodelica::List<Arc<ParseTree>>> = ts;
    let mut t: Arc<ParseTree>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ts.clone().reverse()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    t = __pa0.clone();
    ts = __pa1.clone();
    ts = metamodelica::cons(removeLastTokenInTree(t.clone())?, ts.clone()).reverse();
    Ok(ts)
}

fn firstTokenInTree(mut t: Arc<ParseTree>) -> Result<Token> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(t.clone()) {
        Deref @ ParseTree::EMPTY { .. } => {
            metamodelica::print((literal!("No first token in tree\n")).clone());
            return Ok(bail!("fail"))
        },
        Deref @ ParseTree::LEAF { .. } => return Ok(var_field!((*t).token, ParseTree::LEAF).clone()),
        Deref @ ParseTree::NODE { .. } => { t = (var_field!((*t).nodes, ParseTree::NODE).clone()).get(1)?; continue '__tco; },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn lastToken(mut t: Arc<ParseTree>) -> Result<Token> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(t.clone()) {
        Deref @ ParseTree::EMPTY { .. } => {
            if debug.clone() {
                metamodelica::print((literal!("lastToken fail\n")).clone());
            }
            return Ok(bail!("fail"))
        },
        Deref @ ParseTree::LEAF { .. } => return Ok(var_field!((*t).token, ParseTree::LEAF).clone()),
        Deref @ ParseTree::NODE { .. } => { t = List::last(var_field!((*t).nodes, ParseTree::NODE).clone())?; continue '__tco; },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn fixMoveOperations(mut inDiff: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>>, mut compare: Arc<dyn ::std::ops::Fn(Arc<ParseTree>, Arc<ParseTree>) -> Result<bool> + 'static>) -> Result<Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>>> {
    let mut diff: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>> = metamodelica::nil();
    let mut lst: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut deleted: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut lst2: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut changeFound: bool = false;
    let mut d1: (Diff, Arc<metamodelica::List<Arc<ParseTree>>>) = (Diff::Add, metamodelica::nil());
    for mut d in &*inDiff.clone() {
        let mut d = d.clone();
        let () = (::match_deref::match_deref! { match &(d.clone()) {
        (DiffAlgorithm::Diff::Delete, __esc_lst) => {
            lst = (*__esc_lst).clone();
            deleted = listAppend(lst.clone(), deleted.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    if deleted.clone().is_empty() {
        diff = inDiff.clone();
        return Ok(diff.clone());
    }
    for mut d in &*inDiff.clone() {
        let mut d = d.clone();
        d1 = (::match_deref::match_deref! { match &(d.clone()) {
        (DiffAlgorithm::Diff::Add, __esc_lst) => {
            lst = (*__esc_lst).clone();
            d1 = d.clone();
            for mut l1 in &*lst.clone() {
                let mut l1 = l1.clone();
                if List::isMemberOnTrue(l1.clone(), deleted.clone(), compare.clone())? {
                    changeFound = true;
                    lst2 = metamodelica::nil();
                    for mut l2 in &*lst.clone() {
                        let mut l2 = l2.clone();
                        match '__try0: {
                            lst2 = metamodelica::cons(unwrap_break_err!(List::getMemberOnTrue(l2.clone(), deleted.clone(), compare.clone()), '__try0), lst2.clone());
                            Ok::<_, anyhow::Error>((lst2.clone(),))
                        } {
                            Ok((__try0_o0,)) => {
                                lst2 = __try0_o0;
                            }
                            Err(_) => {
                                lst2 = metamodelica::cons(l2.clone(), lst2.clone());
                            }
                        }
                    }
                    d1 = (Diff::Add.clone(), metamodelica::Dangerous::listReverseInPlace(lst2.clone()));
                    break;
                }
            }
            d1.clone()
        },
        _ => d.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        diff = metamodelica::cons(d1.clone(), diff.clone());
    }
    diff = if (changeFound.clone()) {metamodelica::Dangerous::listReverseInPlace(diff.clone())} else {inDiff.clone()};
    Ok(diff)
}

fn makeNode(mut nodes: Arc<metamodelica::List<Arc<ParseTree>>>, mut label: Arc<ParseTree>) -> Arc<ParseTree> {
    let mut node: Arc<ParseTree> = Arc::new(ParseTree::EMPTY);
    node = (::match_deref::match_deref! { match &((({
        let mut __acc: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
        for mut n in (nodes.clone()).into_iter().cloned() {
            if !(!(isEmpty(n.clone()))) { continue; }
            let __x = n.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), label.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ ParseTree::EMPTY { .. }) => crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
        (Deref @ metamodelica::List::Cons { head: __esc_node, tail: Deref @ metamodelica::List::Nil }, Deref @ ParseTree::EMPTY { .. }) => {
            node = (*__esc_node).clone();
            node.clone()
        },
        _ => Arc::new(ParseTree::NODE { label: label.clone(), nodes: nodes.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    node
}

fn makeNodePrependTree(mut nodes: Arc<metamodelica::List<Arc<ParseTree>>>, mut tree: Arc<metamodelica::List<Arc<ParseTree>>>, mut label: Arc<ParseTree>) -> Arc<metamodelica::List<Arc<ParseTree>>> {
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    outTree = if (!(nodes.clone().is_empty())) {metamodelica::cons(makeNode(nodes.clone(), label.clone()), tree.clone())} else {tree.clone()};
    outTree
}

fn isLeaf(mut t: Arc<ParseTree>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(t.clone()) {
        Deref @ ParseTree::LEAF { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn firstToken(mut t: Arc<metamodelica::List<Arc<ParseTree>>>) -> Token {
    let mut token: Token = <Token as ::std::default::Default>::default();
    token = (::match_deref::match_deref! { match &(t.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ ParseTree::NODE { nodes, .. }, tail: _ } => {
            firstToken(nodes.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ ParseTree::LEAF { token: __esc_token }, tail: _ } => {
            token = (*__esc_token).clone();
            token.clone()
        },
        _ => {
            LexerModelicaDiff::noToken.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    token
}

fn firstTokenDebugStr(mut t: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut l: Arc<metamodelica::List<Token>>;
    l = metamodelica::cons(firstToken(t.clone()), metamodelica::nil());
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Error::infoStr(topTokenSourceInfo(l.clone())?)?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*topTokenStr(l.clone())?); ArcStr::from(__mm_s) }).clone();
    Ok(r#str)
}

fn getNodes(mut t: Arc<ParseTree>) -> Arc<metamodelica::List<Arc<ParseTree>>> {
    let mut nodes: Arc<metamodelica::List<Arc<ParseTree>>>;
    nodes = (::match_deref::match_deref! { match &(t.clone()) {
        Deref @ ParseTree::NODE { .. } => var_field!((*t).nodes, ParseTree::NODE).clone(),
        _ => list![t.clone()],
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    nodes
}

fn extractSingleAddDiffBeforeAndAfter(mut diffs: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>>) -> Result<(Arc<ParseTree>, Arc<ParseTree>, Arc<metamodelica::List<Arc<ParseTree>>>, Arc<metamodelica::List<Arc<ParseTree>>>, Arc<metamodelica::List<Arc<ParseTree>>>, bool)> {
    let mut addedTree: Arc<ParseTree> = Arc::new(ParseTree::EMPTY);
    let mut deletedTree: Arc<ParseTree> = Arc::new(ParseTree::EMPTY);
    let mut before: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut middle: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut after: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut addedBeforeDeleted: bool = false;
    let mut foundAdded: bool = false;
    let mut foundDeleted: bool = false;
    let mut acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ParseTree>>>>> = metamodelica::nil();
    let mut trees: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut lst: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut d: Diff = Diff::Add;
    let mut addCount: i32 = 0;
    for mut diff in &*diffs.clone() {
        let mut diff = diff.clone();
        let () = (::match_deref::match_deref! { match &(diff.clone()) {
        (DiffAlgorithm::Diff::Add, __esc_lst) => {
            lst = (*__esc_lst).clone();
            addCount = 0;
            for mut tree in &*lst.clone() {
                let mut tree = tree.clone();
                addCount = addCount.clone() + 1;
                if parseTreeIsNewLine(tree.clone()) && addCount.clone() > 1 && addCount.clone() == (lst.clone().len() as i32) {
                    acc = metamodelica::cons(list![tree.clone()], acc.clone());
                } else if parseTreeIsWhitespace(tree.clone()) {
                    acc = acc.clone();
                } else {
                    if foundAdded.clone() {
                        Error::addInternalError((literal!("Found multiple Add subtrees")).clone(), metamodelica::sourceInfo!("Parsers/SimpleModelicaParser.mo"))?;
                        bail!("fail");
                    }
                    addedTree = tree.clone();
                    foundAdded = true;
                    if foundDeleted.clone() {
                        middle = List::flattenReverse(acc.clone())?;
                    } else {
                        addedBeforeDeleted = true;
                        before = List::flattenReverse(acc.clone())?;
                    }
                    acc = metamodelica::nil();
                }
            }
            ()
        },
        (DiffAlgorithm::Diff::Delete, __esc_lst) => {
            lst = (*__esc_lst).clone();
            for mut tree in &*lst.clone() {
                let mut tree = tree.clone();
                if parseTreeIsWhitespace(tree.clone()) {
                    acc = metamodelica::cons(list![tree.clone()], acc.clone());
                } else {
                    if foundDeleted.clone() {
                        Error::addInternalError((literal!("Found multiple Delete subtrees")).clone(), metamodelica::sourceInfo!("Parsers/SimpleModelicaParser.mo"))?;
                        bail!("fail");
                    }
                    deletedTree = tree.clone();
                    foundDeleted = true;
                    if foundAdded.clone() {
                        middle = List::flattenReverse(acc.clone())?;
                    } else {
                        addedBeforeDeleted = false;
                        before = List::flattenReverse(acc.clone())?;
                    }
                    acc = metamodelica::nil();
                }
            }
            ()
        },
        (DiffAlgorithm::Diff::Equal, __esc_trees) => {
            trees = (*__esc_trees).clone();
            acc = metamodelica::cons(trees.clone(), acc.clone());
            ()
        },
        (__esc_d, _) => {
            d = (*__esc_d).clone();
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Found ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{:?}", d.clone()))); __mm_s.push_str(&*literal!(" subtrees with multiple or zero entries")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("Parsers/SimpleModelicaParser.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    let true = (foundAdded.clone()) else { bail!("pattern mismatch") };
    let true = (foundDeleted.clone()) else { bail!("pattern mismatch") };
    after = List::flattenReverse(acc.clone())?;
    Ok((addedTree, deletedTree, before, middle, after, addedBeforeDeleted))
}

fn extractAdditionsDeletions(mut diffs: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>>) -> Result<(Arc<metamodelica::List<Arc<ParseTree>>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut addedTrees: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut deletedTrees: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut addedTreesAcc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ParseTree>>>>> = metamodelica::nil();
    let mut deletedTreesAcc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ParseTree>>>>> = metamodelica::nil();
    let mut lst: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    for mut diff in &*diffs.clone() {
        let mut diff = diff.clone();
        let () = (::match_deref::match_deref! { match &(diff.clone()) {
        (DiffAlgorithm::Diff::Add, __esc_lst) => {
            lst = (*__esc_lst).clone();
            addedTreesAcc = metamodelica::cons(lst.clone(), addedTreesAcc.clone());
            ()
        },
        (DiffAlgorithm::Diff::Delete, __esc_lst) => {
            lst = (*__esc_lst).clone();
            deletedTreesAcc = metamodelica::cons(lst.clone(), deletedTreesAcc.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    addedTrees = List::flattenReverse(addedTreesAcc.clone())?;
    deletedTrees = List::flattenReverse(deletedTreesAcc.clone())?;
    Ok((addedTrees, deletedTrees))
}

fn countDiffAddDelete(mut diffs: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<ParseTree>>>)>>) -> (i32, i32) {
    let mut nadd: i32 = 0;
    let mut ndel: i32 = 0;
    let mut d: Diff;
    let mut l: Arc<metamodelica::List<Arc<ParseTree>>>;
    for mut diff in &*diffs.clone() {
        let mut diff = diff.clone();
        (d, l) = diff.clone();
        if d.clone() == Diff::Add.clone() {
            nadd = nadd.clone() + ({
        let mut __acc: i32 = 0;
        for mut t in (l.clone()).into_iter().cloned() {
            let __x = if (parseTreeIsWhitespace(t.clone())) {0} else {1};
            __acc += __x;
        }
        __acc
    });
        } else if d.clone() == Diff::Delete.clone() {
            ndel = ndel.clone() + ({
        let mut __acc: i32 = 0;
        for mut t in (l.clone()).into_iter().cloned() {
            let __x = if (parseTreeIsWhitespace(t.clone())) {0} else {1};
            __acc += __x;
        }
        __acc
    });
        }
    }
    (nadd, ndel)
}

pub static whiteSpaceTokenIds: std::sync::LazyLock<Arc<metamodelica::List<TokenId>>> = std::sync::LazyLock::new(|| { list![TokenId::LINE_COMMENT.clone(), TokenId::BLOCK_COMMENT.clone(), TokenId::NEWLINE.clone(), TokenId::WHITESPACE.clone()] });

pub static whiteSpaceTokenIdsNotComment: std::sync::LazyLock<Arc<metamodelica::List<TokenId>>> = std::sync::LazyLock::new(|| { list![TokenId::NEWLINE.clone(), TokenId::WHITESPACE.clone()] });

pub static tokenIdsComment: std::sync::LazyLock<Arc<metamodelica::List<TokenId>>> = std::sync::LazyLock::new(|| { list![TokenId::LINE_COMMENT.clone(), TokenId::BLOCK_COMMENT.clone()] });

fn parseTreeIsWhitespace(mut t1: Arc<ParseTree>) -> bool {
    let mut b: bool;
    let mut id: TokenId;
    b = (::match_deref::match_deref! { match &(t1.clone()) {
        Deref @ ParseTree::LEAF { .. } => listMember(var_field!((*t1).token, ParseTree::LEAF).id.clone(), whiteSpaceTokenIds.clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn parseTreeIsNewLine(mut t1: Arc<ParseTree>) -> bool {
    let mut b: bool;
    let mut id: TokenId;
    b = (::match_deref::match_deref! { match &(t1.clone()) {
        Deref @ ParseTree::LEAF { .. } => var_field!((*t1).token, ParseTree::LEAF).id.clone() == TokenId::NEWLINE.clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn parseTreeIsWhitespaceNotComment(mut t1: Arc<ParseTree>) -> bool {
    let mut b: bool;
    let mut id: TokenId;
    b = (::match_deref::match_deref! { match &(t1.clone()) {
        Deref @ ParseTree::LEAF { .. } => listMember(var_field!((*t1).token, ParseTree::LEAF).id.clone(), whiteSpaceTokenIdsNotComment.clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn parseTreeIsWhitespaceNotCommentOrNewline(mut t1: Arc<ParseTree>) -> bool {
    let mut b: bool;
    let mut id: TokenId;
    b = (::match_deref::match_deref! { match &(t1.clone()) {
        Deref @ ParseTree::LEAF { .. } => var_field!((*t1).token, ParseTree::LEAF).id.clone() == TokenId::WHITESPACE.clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn parseTreeIsComment(mut t1: Arc<ParseTree>) -> bool {
    let mut b: bool;
    let mut id: TokenId;
    b = (::match_deref::match_deref! { match &(t1.clone()) {
        Deref @ ParseTree::LEAF { .. } => listMember(var_field!((*t1).token, ParseTree::LEAF).id.clone(), tokenIdsComment.clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn parseTreeIsLineComment(mut t1: Arc<ParseTree>) -> bool {
    let mut b: bool;
    let mut id: TokenId;
    b = (::match_deref::match_deref! { match &(t1.clone()) {
        Deref @ ParseTree::LEAF { .. } => var_field!((*t1).token, ParseTree::LEAF).id.clone() == TokenId::LINE_COMMENT.clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn parseTreeIsOnlyIdent(mut t1: Arc<ParseTree>) -> bool {
    let mut b: bool;
    let mut id: TokenId;
    b = (::match_deref::match_deref! { match &(t1.clone()) {
        Deref @ ParseTree::LEAF { .. } => var_field!((*t1).token, ParseTree::LEAF).id.clone() == TokenId::IDENT.clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn parseTreeIsOnlyEnd(mut t1: Arc<ParseTree>) -> bool {
    let mut b: bool;
    let mut id: TokenId;
    b = (::match_deref::match_deref! { match &(t1.clone()) {
        Deref @ ParseTree::LEAF { .. } => var_field!((*t1).token, ParseTree::LEAF).id.clone() == TokenId::END.clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn parseTreeFilterWhitespace(mut t: Arc<ParseTree>) -> Arc<ParseTree> {
    let mut t: Arc<ParseTree> = t;
    let mut id: TokenId;
    let mut changed: bool = false;
    let mut n2: Arc<ParseTree> = Arc::new(ParseTree::EMPTY);
    let mut nodes: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    t = (::match_deref::match_deref! { match &(t.clone()) {
        Deref @ ParseTree::LEAF { .. } if (listMember(var_field!((*t).token, ParseTree::LEAF).id.clone(), whiteSpaceTokenIds.clone())) => crate::SimpleModelicaParser::ParseTree::interned_EMPTY(),
        Deref @ ParseTree::NODE { .. } => {
            changed = false;
            nodes = metamodelica::nil();
            for mut n in &*var_field!((*t).nodes, ParseTree::NODE).clone() {
                let mut n = n.clone();
                n2 = parseTreeFilterWhitespace(n.clone());
                if !(referenceEq(&*(n.clone()),&*(n2.clone()))) {
                    changed = true;
                }
                if !(isEmpty(n2.clone())) {
                    nodes = metamodelica::cons(n2.clone(), nodes.clone());
                }
            }
            if (changed.clone()) {Arc::new(ParseTree::NODE { label: var_field!((*t).label, ParseTree::NODE).clone(), nodes: nodes.clone().reverse() })} else {t.clone()}
        },
        _ => t.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    t
}

fn eatWhitespace(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut id: TokenId = TokenId::_NO_TOKEN;
    let mut t: Token;
    tree = inTree.clone();
    while (::match_deref::match_deref! { match &(tokens.clone()) {
        Deref @ metamodelica::List::Cons { head: Token { id: __esc_id, .. }, tail: _ } => {
            id = (*__esc_id).clone();
            listMember(id.clone(), list![TokenId::LINE_COMMENT.clone(), TokenId::BLOCK_COMMENT.clone(), TokenId::NEWLINE.clone(), TokenId::WHITESPACE.clone()])
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(tokens.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        t = __pa0.clone();
        tokens = __pa1.clone();
        tree = metamodelica::cons(Arc::new(ParseTree::LEAF { token: t.clone() }), tree.clone());
    }
    outTree = tree.clone();
    Ok((tokens, outTree))
}

fn scanOpt(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>, mut id: TokenId) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>, bool)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut found: bool;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut id2: TokenId = TokenId::_NO_TOKEN;
    let mut t: Token = <Token as ::std::default::Default>::default();
    let mut tokens2: Arc<metamodelica::List<Token>> = metamodelica::nil();
    (tokens, tree) = eatWhitespace(tokens.clone(), inTree.clone())?;
    (tokens, tree, found) = (::match_deref::match_deref! { match &(tokens.clone()) {
        Deref @ metamodelica::List::Cons { head: __esc_t @ Token { id: id2, .. }, tail: __esc_tokens2 } if (id.clone() == id2.clone()) => {
            t = (*__esc_t).clone();
            tokens2 = (*__esc_tokens2).clone();
            (tokens2.clone(), metamodelica::cons(Arc::new(ParseTree::LEAF { token: t.clone() }), tree.clone()), true)
        },
        _ => (tokens.clone(), tree.clone(), false),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if !(found.clone()) {
        outTree = inTree.clone();
        tokens = inTokens.clone();
    } else {
        outTree = tree.clone();
    }
    Ok((tokens, outTree, found))
}

fn scan(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>, mut id: TokenId) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut found: bool;
    tree = inTree.clone();
    (tokens, tree, found) = scanOpt(tokens.clone(), tree.clone(), id.clone())?;
    if !(found.clone()) {
        error(tokens.clone(), tree.clone(), list![id.clone()])?;
    }
    outTree = tree.clone();
    Ok((tokens, outTree))
}

fn scanOneOf(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>, mut ids: Arc<metamodelica::List<TokenId>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut found: bool;
    tree = inTree.clone();
    (tokens, tree, found) = LA1(tokens.clone(), tree.clone(), ids.clone(), true)?;
    if !(found.clone()) {
        error(tokens.clone(), tree.clone(), ids.clone())?;
    }
    outTree = tree.clone();
    Ok((tokens, outTree))
}

fn error(mut tokens: Arc<metamodelica::List<Token>>, mut tree: Arc<metamodelica::List<Arc<ParseTree>>>, mut expected: Arc<metamodelica::List<TokenId>>) -> Result<()> {
    let mut i: i32;
    let mut s: ArcStr = arcstr::literal!("");
    let mut strs: Arc<metamodelica::List<ArcStr>>;
    let mut res: Arc<metamodelica::List<ArcStr>>;
    let mut info: SourceInfo;
    info = topTokenSourceInfo(tokens.clone())?;
    res = metamodelica::cons(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to scan top of input: ")); __mm_s.push_str(&*if (debug.clone()) {debugTokenStr(tokens.clone())?} else {topTokenStr(tokens.clone())?}); __mm_s.push_str(&*literal!("\n  Expected one of: ")); __mm_s.push_str(&*if (expected.clone().is_empty()) {literal!("<EOF>")} else {stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut id in (expected.clone()).into_iter().cloned() {
            let __x = tokenIdStr(id.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!(", ")).clone())}); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::nil());
    res = metamodelica::cons(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  Current parse tree is:\n")); __mm_s.push_str(&*parseTreeStr(tree.clone().reverse())?); __mm_s.push_str(&*literal!("\n  The parser stack is:\n")); ArcStr::from(__mm_s) }).clone(), res.clone());
    StackOverflow::setStacktraceMessages(0, 100);
    for mut s in &*StackOverflow::readableStacktraceMessages()? {
        let mut s = s.clone();
        (i, strs) = System::regex((s.clone()).clone(), (literal!("SimpleModelicaParser[^A-Za-z]([A-Za-z_0-9_]*)")).clone(), 2, true, false);
        let () = (::match_deref::match_deref! { match &((i.clone(), strs.clone())) {
        (2, Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: s, tail: Deref @ metamodelica::List::Nil } }) if (s.clone() != literal!("error")) => {
            res = metamodelica::cons((literal!("\n")).clone(), metamodelica::cons((s.clone()).clone(), res.clone()));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Error::addInternalError(stringAppendList(res.clone().reverse()), info.clone())?;
    bail!("fail");
    Ok(())
}

fn tokenIdStr(mut id: TokenId) -> ArcStr {
    let mut r#str: ArcStr = ArcStr::from(::std::format!("{:?}", id.clone()));
    r#str
}

fn peek(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>, TokenId)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut id: TokenId = TokenId::_NO_TOKEN;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    tree = inTree.clone();
    (tokens, tree) = eatWhitespace(tokens.clone(), tree.clone())?;
    id = (::match_deref::match_deref! { match &(tokens.clone()) {
        Deref @ metamodelica::List::Cons { head: Token { id: __esc_id, .. }, tail: _ } => {
            id = (*__esc_id).clone();
            id.clone()
        },
        _ => TokenId::_NO_TOKEN.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outTree = tree.clone();
    Ok((tokens, outTree, id))
}

fn consume(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut t: Token;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(tokens.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    t = __pa0.clone();
    tokens = __pa1.clone();
    outTree = metamodelica::cons(Arc::new(ParseTree::LEAF { token: t.clone() }), inTree.clone());
    Ok((tokens, outTree))
}

fn LA1(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>, mut ids: Arc<metamodelica::List<TokenId>>, mut consume: bool) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>, bool)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut found: bool;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut id: TokenId = TokenId::_NO_TOKEN;
    tree = inTree.clone();
    (tokens, tree) = eatWhitespace(tokens.clone(), tree.clone())?;
    found = (::match_deref::match_deref! { match &(tokens.clone()) {
        Deref @ metamodelica::List::Cons { head: Token { id: __esc_id, .. }, tail: _ } => {
            id = (*__esc_id).clone();
            listMember(id.clone(), ids.clone())
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if found.clone() && consume.clone() {
        (tokens, tree) = self::consume(tokens.clone(), tree.clone())?;
    }
    if !(found.clone()) {
        outTree = inTree.clone();
        tokens = inTokens.clone();
    } else {
        outTree = tree.clone();
    }
    Ok((tokens, outTree, found))
}

fn LAk(mut inTokens: Arc<metamodelica::List<Token>>, mut inTree: Arc<metamodelica::List<Arc<ParseTree>>>, mut idsLst: Arc<metamodelica::List<Arc<metamodelica::List<TokenId>>>>) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Arc<ParseTree>>>, bool)> {
    let mut tokens: Arc<metamodelica::List<Token>> = inTokens.clone();
    let mut outTree: Arc<metamodelica::List<Arc<ParseTree>>>;
    let mut found: bool = true;
    let mut tree: Arc<metamodelica::List<Arc<ParseTree>>> = metamodelica::nil();
    let mut id: TokenId = TokenId::_NO_TOKEN;
    let mut tmp: Arc<metamodelica::List<Token>>;
    tree = inTree.clone();
    (tokens, tree) = eatWhitespace(tokens.clone(), tree.clone())?;
    outTree = tree.clone();
    tmp = tokens.clone();
    for mut ids in &*idsLst.clone() {
        let mut ids = ids.clone();
        found = (::match_deref::match_deref! { match &(tmp.clone()) {
        Deref @ metamodelica::List::Cons { head: Token { id: __esc_id, .. }, tail: __esc_tmp } => {
            id = (*__esc_id).clone();
            tmp = (*__esc_tmp).clone();
            listMember(id.clone(), ids.clone())
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        if !(found.clone()) {
            return Ok((tokens.clone(), outTree.clone(), found.clone()));
        }
        (tmp, _) = eatWhitespace(tmp.clone(), metamodelica::nil())?;
    }
    Ok((tokens, outTree, found))
}

fn parseTreeStrWork(mut tree: Arc<ParseTree>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ ParseTree::LEAF { .. } => {
            Print::printBuf((tokenContent(var_field!((*tree).token, ParseTree::LEAF).clone())?).clone())?;
            ()
        },
        Deref @ ParseTree::EMPTY { .. } => (),
        Deref @ ParseTree::NODE { .. } => {
            for mut n in &*var_field!((*tree).nodes, ParseTree::NODE).clone() {
                let mut n = n.clone();
                parseTreeStrWork(n.clone())?;
            }
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn topTokenStr(mut tokens: Arc<metamodelica::List<Token>>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut id: TokenId = TokenId::_NO_TOKEN;
    let mut t: Token = <Token as ::std::default::Default>::default();
    r#str = ((::match_deref::match_deref! { match &(tokens.clone()) {
        Deref @ metamodelica::List::Cons { head: __esc_t @ Token { id: __esc_id, .. }, tail: _ } => {
            t = (*__esc_t).clone();
            id = (*__esc_id).clone();
            { let mut __mm_s = String::new(); __mm_s.push_str(&*ArcStr::from(::std::format!("{:?}", id.clone()))); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*tokenContent(t.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        _ => literal!("EOF"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(r#str)
}

fn debugTokenStr(mut tokens: Arc<metamodelica::List<Token>>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut t in (tokens.clone()).into_iter().cloned() {
            let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*ArcStr::from(::std::format!("{:?}", t.id.clone()))); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*tokenContent(t.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!("\n")).clone());
    Ok(r#str)
}

fn topTokenSourceInfo(mut tokens: Arc<metamodelica::List<Token>>) -> Result<SourceInfo> {
    let mut info: SourceInfo;
    let mut t: Token = <Token as ::std::default::Default>::default();
    info = (::match_deref::match_deref! { match &(tokens.clone()) {
        Deref @ metamodelica::List::Cons { head: __esc_t, tail: _ } => {
            t = (*__esc_t).clone();
            LexerModelicaDiff::tokenSourceInfo(t.clone())?
        },
        _ => SourceInfo { fileName: (literal!("<SimpleModelicaParser>")).clone(), isReadOnly: false, lineNumberStart: 0, columnNumberStart: 0, lineNumberEnd: 0, columnNumberEnd: 0, lastModification: metamodelica::OrderedFloat(0.0_f64) },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(info)
}

fn needsWhitespaceBetweenTokens(mut first: Token, mut last: Token) -> Result<bool> {
    let mut b: bool;
    let notident: Arc<metamodelica::List<TokenId>> = list![TokenId::ASSIGN.clone(), TokenId::BLOCK_COMMENT.clone(), TokenId::COLON.clone(), TokenId::COLONCOLON.clone(), TokenId::COMMA.clone(), TokenId::DOT.clone(), TokenId::EQEQ.clone(), TokenId::EQUALS.clone(), TokenId::GREATER.clone(), TokenId::GREATEREQ.clone(), TokenId::LBRACE.clone(), TokenId::LBRACK.clone(), TokenId::LESS.clone(), TokenId::LESSEQ.clone(), TokenId::LESSGT.clone(), TokenId::LINE_COMMENT.clone(), TokenId::LPAR.clone(), TokenId::MINUS.clone(), TokenId::MINUS_EW.clone(), TokenId::NEWLINE.clone(), TokenId::OPERATOR.clone(), TokenId::PLUS.clone(), TokenId::PLUS_EW.clone(), TokenId::POWER.clone(), TokenId::POWER_EW.clone(), TokenId::RBRACE.clone(), TokenId::RBRACK.clone(), TokenId::RPAR.clone(), TokenId::SEMICOLON.clone(), TokenId::SLASH.clone(), TokenId::SLASH_EW.clone(), TokenId::STAR.clone(), TokenId::STAR_EW.clone(), TokenId::UNSIGNED_INTEGER.clone(), TokenId::UNSIGNED_REAL.clone(), TokenId::WHITESPACE.clone()];
    if listMember(tokenId(first.clone())?, notident.clone()) || listMember(tokenId(last.clone())?, notident.clone()) {
        b = false;
        return Ok(b.clone());
    }
    b = true;
    Ok(b)
}

fn tokenId(mut t: Token) -> Result<TokenId> {
    let mut id: TokenId;
    let LexerModelicaDiff::TOKEN { id: __pa0, .. } = (t.clone()) else { bail!("pattern mismatch") };
    id = __pa0.clone();
    Ok(id)
}

pub mod First {
    use super::*;
    pub static class_prefixes: std::sync::LazyLock<Arc<metamodelica::List<TokenId>>> = std::sync::LazyLock::new(|| { list![TokenId::PARTIAL.clone(), TokenId::CLASS.clone(), TokenId::MODEL.clone(), TokenId::OPERATOR.clone(), TokenId::RECORD.clone(), TokenId::BLOCK.clone(), TokenId::EXPANDABLE.clone(), TokenId::CONNECTOR.clone(), TokenId::TYPE.clone(), TokenId::PACKAGE.clone(), TokenId::PURE.clone(), TokenId::IMPURE.clone(), TokenId::FUNCTION.clone()] });

    pub static class_definition: std::sync::LazyLock<Arc<metamodelica::List<TokenId>>> = std::sync::LazyLock::new(|| { metamodelica::cons(TokenId::FINAL.clone(), metamodelica::cons(TokenId::ENCAPSULATED.clone(), class_prefixes.clone())) });

    pub static type_prefix: std::sync::LazyLock<Arc<metamodelica::List<TokenId>>> = std::sync::LazyLock::new(|| { list![TokenId::FLOW.clone(), TokenId::STREAM.clone(), TokenId::DISCRETE.clone(), TokenId::PARAMETER.clone(), TokenId::CONSTANT.clone(), TokenId::INPUT.clone(), TokenId::OUTPUT.clone()] });

    pub static class_modification: std::sync::LazyLock<Arc<metamodelica::List<TokenId>>> = std::sync::LazyLock::new(|| { list![TokenId::LPAR.clone()] });

    pub static _annotation: std::sync::LazyLock<Arc<metamodelica::List<TokenId>>> = std::sync::LazyLock::new(|| { list![TokenId::ANNOTATION.clone()] });

    pub static element_redeclaration: std::sync::LazyLock<Arc<metamodelica::List<TokenId>>> = std::sync::LazyLock::new(|| { list![TokenId::REDECLARE.clone()] });

    pub static name: std::sync::LazyLock<Arc<metamodelica::List<TokenId>>> = std::sync::LazyLock::new(|| { list![TokenId::DOT.clone(), TokenId::IDENT.clone()] });

    pub static element_modification_or_replaceable: std::sync::LazyLock<Arc<metamodelica::List<TokenId>>> = std::sync::LazyLock::new(|| { metamodelica::cons(TokenId::EACH.clone(), metamodelica::cons(TokenId::FINAL.clone(), metamodelica::cons(TokenId::REPLACEABLE.clone(), name.clone()))) });

    pub static argument: std::sync::LazyLock<Arc<metamodelica::List<TokenId>>> = std::sync::LazyLock::new(|| { listAppend(element_modification_or_replaceable.clone(), element_redeclaration.clone()) });

    pub static modification: std::sync::LazyLock<Arc<metamodelica::List<TokenId>>> = std::sync::LazyLock::new(|| { list![TokenId::LPAR.clone(), TokenId::EQUALS.clone(), TokenId::ASSIGN.clone()] });

    pub static component_clause: std::sync::LazyLock<Arc<metamodelica::List<TokenId>>> = std::sync::LazyLock::new(|| { listAppend(type_prefix.clone(), name.clone()) });

    pub static element: std::sync::LazyLock<Arc<metamodelica::List<TokenId>>> = std::sync::LazyLock::new(|| { listAppend(component_clause.clone(), listAppend(class_definition.clone(), list![TokenId::ANNOTATION.clone(), TokenId::IMPORT.clone(), TokenId::EXTENDS.clone(), TokenId::REDECLARE.clone(), TokenId::FINAL.clone(), TokenId::INNER.clone(), TokenId::OUTER.clone(), TokenId::REPLACEABLE.clone()])) });

    pub static statement: std::sync::LazyLock<Arc<metamodelica::List<TokenId>>> = std::sync::LazyLock::new(|| { list![TokenId::DOT.clone(), TokenId::IDENT.clone(), TokenId::LPAR.clone(), TokenId::BREAK.clone(), TokenId::RETURN.clone(), TokenId::IF.clone(), TokenId::FOR.clone(), TokenId::WHILE.clone(), TokenId::WHEN.clone()] });

    pub static component_reference: std::sync::LazyLock<Arc<metamodelica::List<TokenId>>> = std::sync::LazyLock::new(|| { list![TokenId::DOT.clone(), TokenId::IDENT.clone()] });

    /*  constant list<TokenId> function_arguments =
        TokenId.FUNCTION ::
        TokenId.IDENT ::
        expression
      ; */
}

pub mod Follow {
    use super::*;
    pub static statement_equation: std::sync::LazyLock<Arc<metamodelica::List<TokenId>>> = std::sync::LazyLock::new(|| { list![TokenId::INITIAL.clone(), TokenId::EQUATION.clone(), TokenId::ALGORITHM.clone(), TokenId::PUBLIC.clone(), TokenId::PROTECTED.clone(), TokenId::EXTERNAL.clone(), TokenId::ANNOTATION.clone(), TokenId::ELSE.clone(), TokenId::ELSEIF.clone(), TokenId::END.clone(), TokenId::ELSEWHEN.clone()] });

}

pub const debug: bool = false;

