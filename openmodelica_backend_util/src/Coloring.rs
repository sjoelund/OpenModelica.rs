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

use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::Flags;
use openmodelica_util::Graph;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::GCExt;
use openmodelica_util_datatypes_basic::List;

pub fn createColoring(mut sparseArray: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut sparseArrayT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut sizeVars: i32, mut sizeVarswithDep: i32) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut coloredArray: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let debug: bool = false;
    let mut nodesList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut colored: metamodelica::Array<i32> = Default::default();
    let mut forbiddenColor: metamodelica::Array<i32> = Default::default();
    let mut sparseGraph: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut sparseGraphT: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut arraysparseGraph: metamodelica::Array<(i32, Arc<metamodelica::List<i32>>)> = Default::default();
    let mut maxColor: i32 = 0;
    match '__try0: {
        if unwrap_break_err!(Flags::isSet(Flags::DUMP_SPARSE_VERBOSE.clone()), '__try0) {
            println!("{}", (literal!("analytical Jacobians[SPARSE] -> build sparse graph.\n")).clone());
        }
        nodesList = List::intRange2(1, sizeVarswithDep.clone());
        sparseGraph = unwrap_break_err!(Graph::buildGraph(nodesList.clone(), (std::sync::Arc::new(createBipartiteGraph) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), sparseArray.clone()), '__try0);
        sparseGraphT = unwrap_break_err!(Graph::buildGraph(List::intRange2(1, sizeVars.clone()), (std::sync::Arc::new(createBipartiteGraph) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), sparseArrayT.clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::DUMP_SPARSE_VERBOSE.clone()), '__try0) {
            println!("{}", (literal!("sparse graph: \n")).clone());
            unwrap_break_err!(Graph::printGraphInt(sparseGraph.clone()), '__try0);
            println!("{}", (literal!("transposed sparse graph: \n")).clone());
            unwrap_break_err!(Graph::printGraphInt(sparseGraphT.clone()), '__try0);
            println!("{}", (literal!("analytical Jacobians[SPARSE] -> builded graph for coloring.\n")).clone());
        }
        forbiddenColor = arrayCreate(sizeVars.clone(), 0);
        colored = arrayCreate(sizeVars.clone(), 0);
        arraysparseGraph = metamodelica::arrayFromVec(sparseGraph.clone().into_iter().cloned().collect());
        if debug.clone() {
            unwrap_break_err!(execStat((literal!("generateSparsePattern -> coloring start ")).clone()), '__try0);
        }
        if sizeVars.clone() > 0 {
            unwrap_break_err!(Graph::partialDistance2colorInt(sparseGraphT.clone(), forbiddenColor.clone(), nodesList.clone(), arraysparseGraph.clone(), colored.clone()), '__try0);
        }
        if debug.clone() {
            unwrap_break_err!(execStat((literal!("generateSparsePattern -> coloring end ")).clone()), '__try0);
        }
        GCExt::free(forbiddenColor.clone());
        GCExt::free(arraysparseGraph.clone());
        maxColor = unwrap_break_err!(Array::fold(colored.clone(), (std::sync::Arc::new(fnptr!(intMax, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), 0), '__try0);
        coloredArray = arrayCreate(maxColor.clone(), metamodelica::nil());
        unwrap_break_err!(mapIndexColors(colored.clone(), sizeVars.clone(), coloredArray.clone()), '__try0);
        GCExt::free(colored.clone());
        if unwrap_break_err!(Flags::isSet(Flags::DUMP_SPARSE_VERBOSE.clone()), '__try0) {
            println!("{}", (literal!("Print Coloring Cols: \n")).clone());
            unwrap_break_err!(dumpColoring(Arc::new(coloredArray.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())), '__try0);
        }
        Ok::<_, anyhow::Error>((arraysparseGraph.clone(), colored.clone(), coloredArray.clone(), forbiddenColor.clone(), maxColor.clone(), nodesList.clone(), sparseGraph.clone(), sparseGraphT.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5, __try0_o6, __try0_o7)) => {
            arraysparseGraph = __try0_o0;
            colored = __try0_o1;
            coloredArray = __try0_o2;
            forbiddenColor = __try0_o3;
            maxColor = __try0_o4;
            nodesList = __try0_o5;
            sparseGraph = __try0_o6;
            sparseGraphT = __try0_o7;
        }
        Err(__try0_err) => {
            Error::addInternalError((literal!("function createColoring failed")).clone(), metamodelica::sourceInfo!())?;
            return Err(__try0_err);
        }
    }
    Ok(coloredArray)
}

fn createBipartiteGraph(mut inNode: i32, mut inSparsePattern: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outEdges: Arc<metamodelica::List<i32>> = metamodelica::nil();
    if inNode.clone() >= 1 && inNode.clone() <= (inSparsePattern.clone().borrow().len() as i32) {
        outEdges = inSparsePattern.clone().borrow()[(inNode.clone()-1) as usize].clone();
    } else {
        outEdges = metamodelica::nil();
    }
    Ok(outEdges)
}

fn mapIndexColors(mut inColors: metamodelica::Array<i32>, mut inMaxIndex: i32, mut inArray: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    let mut index: i32 = 0;
    match '__try0: {
        for mut i in 1..=inMaxIndex.clone() {
            index = inColors.clone().borrow()[(i.clone()-1) as usize].clone();
            {let _arr = inArray.clone(); let _val = metamodelica::cons(i.clone(), inArray.clone().borrow()[(index.clone()-1) as usize].clone()); _arr.borrow_mut()[(index.clone()-1) as usize] = _val; _arr};
        }
        Ok::<(), anyhow::Error>(())
    } {
        Ok(()) => {}
        Err(__try0_err) => {
            Error::addInternalError((literal!("function mapIndexColors failed")).clone(), metamodelica::sourceInfo!())?;
            return Err(__try0_err);
        }
    }
    Ok(())
}

fn dumpColoring(mut pattern: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<()> {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Print sparse pattern: ")); __mm_s.push_str(&*intString((pattern.clone().len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    for mut row in &*pattern.clone() {
        let mut row = row.clone();
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*stringDelimitList(List::map(row.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone())); __mm_s.push_str(&*literal!("}\n")); ArcStr::from(__mm_s) }).clone());
    }
    println!("{}", (literal!("\n")).clone());
    Ok(())
}

