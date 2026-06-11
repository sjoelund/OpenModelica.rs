// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_frontend_base::Algorithm;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::ClassInfUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionDumpTpl;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::Values;
use openmodelica_simcode_types::SimCode;
use openmodelica_simcode_types::SimCodeFunction;
use openmodelica_simcode_types::SimCodeVar;
use openmodelica_simcode_util::SimCodeFunctionUtil;
use openmodelica_tpl::Tpl;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::Settings;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub fn translateModel(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, ref i_simCode @ SimCode::SimCode { modelInfo: SimCode::ModelInfo { name: ref i_modelInfo_name, .. }, .. }) => {
            let mut txt_1: Tpl::Text;
            let mut txt_0: Tpl::Text;
            txt_0 = generateXml(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            txt_1 = dotPathXml(Tpl::emptyTxt.clone(), i_modelInfo_name.clone())?;
            txt_1 = Tpl::writeTok(txt_1.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".xml")).clone() }))?;
            Tpl::textFile(txt_0.clone(), (Tpl::textString(txt_1.clone())?).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn generateXml(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, ref i_simCode @ SimCode::SimCode { modelInfo: ref i_modelInfo @ SimCode::ModelInfo { functions: ref i_modelInfo_functions, .. }, simulationSettingsOpt: ref i_simulationSettingsOpt, allEquations: ref i_allEquations, initialEquations: ref i_initialEquations, recordDecls: ref i_recordDecls, classAttributes: ref i_classAttributes, .. }) => {
            let mut l_prefix: Tpl::Text;
            let mut ret_1: ArcStr;
            let mut l_guid: Tpl::Text;
            ret_1 = (System::getUUIDStr()).clone();
            l_guid = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1.clone()).clone())?;
            l_prefix = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("https://github.com/JModelica/JModelica/tree/master/XML")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n")).clone(), (literal!("<OpenModelicaModelDescription\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("xmlns:exp=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_prefix.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("/daeExpressions.xsd\"\n")).clone(), (literal!("xmlns:equ=\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_prefix.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("/daeEquations.xsd\"\n")).clone(), (literal!("xmlns:fun=\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_prefix.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("/daeFunctions.xsd\"\n")).clone(), (literal!("xmlns:opt=\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_prefix.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("/daeOptimization.xsd\"\n")).clone(), (literal!("xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\"\n")).clone()], lastHasNewLine: true }))?;
            txt = modelDescriptionXml(txt.clone(), i_simCode.clone(), (Tpl::textString(l_guid.clone())?).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(">\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = vendorAnnotationsXml(txt.clone(), i_simCode.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = defaultExperiment(txt.clone(), i_simulationSettingsOpt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = modelVariablesXml(txt.clone(), i_modelInfo.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = bindingEquationsXml(txt.clone(), i_modelInfo.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = equationsXml(txt.clone(), i_allEquations.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = initialEquationsXml(txt.clone(), i_modelInfo.clone(), i_initialEquations.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = algorithmicEquationsXml(txt.clone(), i_allEquations.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = recordsXml(txt.clone(), i_recordDecls.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = functionsXml(txt.clone(), i_modelInfo_functions.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = objectiveFunctionXml(txt.clone(), i_classAttributes.clone(), i_simCode.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</OpenModelicaModelDescription>")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn vendorAnnotationsXml(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::SimCode { modelInfo: SimCode::ModelInfo { varInfo: SimCode::VarInfo { numZeroCrossings: _, .. }, .. }, .. }) => {
            let mut ret_1: ArcStr;
            let mut l_generationTool: Tpl::Text;
            l_generationTool = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OpenModelica Compiler ")).clone() }))?;
            ret_1 = (Settings::getVersionNr()).clone();
            l_generationTool = Tpl::writeStr(l_generationTool.clone(), (ret_1.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<VendorAnnotations>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Tool name=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_generationTool.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\"> </Tool>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</VendorAnnotations>")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn modelDescriptionXml(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_guid: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone(), in_a_guid.clone()) {
        (mut txt, SimCode::SimCode { modelInfo: SimCode::ModelInfo { varInfo: SimCode::VarInfo { numStateVars: mut i_modelInfo_varInfo_numStateVars, numZeroCrossings: mut i_modelInfo_varInfo_numZeroCrossings, .. }, name: ref i_modelInfo_name, .. }, fileNamePrefix: mut i_fileNamePrefix, .. }, mut a_guid) => {
            let mut l_numberOfEventIndicators: Tpl::Text;
            let mut l_numberOfContinuousStates: Tpl::Text;
            let mut l_variableNamingConvention: Tpl::Text;
            let mut ret_8: Util::DateTime;
            let mut l_generationDateAndTime: Tpl::Text;
            let mut l_version: Tpl::Text;
            let mut l_author: Tpl::Text;
            let mut l_description: Tpl::Text;
            let mut ret_3: ArcStr;
            let mut l_modelIdentifier: Tpl::Text;
            let mut l_modelName: Tpl::Text;
            let mut l_fmiVersion: Tpl::Text;
            l_fmiVersion = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("1.0")).clone() }))?;
            l_modelName = dotPathXml(Tpl::emptyTxt.clone(), i_modelInfo_name.clone())?;
            ret_3 = (System::stringReplace((i_fileNamePrefix.clone()).clone(), (literal!(".")).clone(), (literal!("_")).clone())?).clone();
            l_modelIdentifier = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_3.clone()).clone())?;
            l_description = Tpl::emptyTxt.clone();
            l_author = Tpl::emptyTxt.clone();
            l_version = Tpl::emptyTxt.clone();
            ret_8 = Util::getCurrentDateTime();
            l_generationDateAndTime = xsdateTimeXml(Tpl::emptyTxt.clone(), ret_8.clone())?;
            l_variableNamingConvention = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("structured")).clone() }))?;
            l_numberOfContinuousStates = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(i_modelInfo_varInfo_numStateVars.clone())).clone())?;
            l_numberOfEventIndicators = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(i_modelInfo_varInfo_numZeroCrossings.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fmiVersion=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fmiVersion.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("modelName=\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_modelName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("modelIdentifier=\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_modelIdentifier.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("guid=\"{")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_guid.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\"\n")).clone(), (literal!("generationDateAndTime=\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_generationDateAndTime.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("variableNamingConvention=\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_variableNamingConvention.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("numberOfContinuousStates=\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_numberOfContinuousStates.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("numberOfEventIndicators=\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_numberOfEventIndicators.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn xsdateTimeXml(mut in_txt: Tpl::Text, mut in_a_dt: Util::DateTime) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_dt.clone()) {
        (mut txt, Util::DateTime { year: mut i_year, mon: mut i_mon, mday: mut i_mday, hour: mut i_hour, min: mut i_min, sec: mut i_sec }) => {
            let mut ret_4: ArcStr;
            let mut ret_3: ArcStr;
            let mut ret_2: ArcStr;
            let mut ret_1: ArcStr;
            let mut ret_0: ArcStr;
            txt = Tpl::writeStr(txt.clone(), (intString(i_year.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-")).clone() }))?;
            ret_0 = (SimCodeFunctionUtil::twodigit(i_mon.clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-")).clone() }))?;
            ret_1 = (SimCodeFunctionUtil::twodigit(i_mday.clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_1.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("T")).clone() }))?;
            ret_2 = (SimCodeFunctionUtil::twodigit(i_hour.clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_2.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(":")).clone() }))?;
            ret_3 = (SimCodeFunctionUtil::twodigit(i_min.clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_3.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(":")).clone() }))?;
            ret_4 = (SimCodeFunctionUtil::twodigit(i_sec.clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_4.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn defaultExperiment(mut in_txt: Tpl::Text, mut in_a_simulationSettingsOpt: Option<SimCode::SimulationSettings>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simulationSettingsOpt.clone()) {
        (mut txt, Some(SimCode::SimulationSettings { startTime: mut i_de_startTime, stopTime: mut i_de_stopTime, tolerance: mut i_de_tolerance, .. })) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<DefaultExperiment startTime=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(i_de_startTime.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" stopTime=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(i_de_stopTime.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" tolerance=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(i_de_tolerance.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" />")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_49(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_49 in &*items.clone() {
        let mut lstElt_49 = lstElt_49.clone();
        txt = (match lstElt_49.clone() {
        mut i_var => {
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_50(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_50 in &*items.clone() {
        let mut lstElt_50 = lstElt_50.clone();
        txt = (match lstElt_50.clone() {
        mut i_var => {
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_51(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_51 in &*items.clone() {
        let mut lstElt_51 = lstElt_51.clone();
        txt = (match lstElt_51.clone() {
        mut i_var => {
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_52(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_52 in &*items.clone() {
        let mut lstElt_52 = lstElt_52.clone();
        txt = (match lstElt_52.clone() {
        mut i_var => {
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_53(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_53 in &*items.clone() {
        let mut lstElt_53 = lstElt_53.clone();
        txt = (match lstElt_53.clone() {
        mut i_var => {
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_54(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_54 in &*items.clone() {
        let mut lstElt_54 = lstElt_54.clone();
        txt = (match lstElt_54.clone() {
        mut i_var => {
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_55(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_55 in &*items.clone() {
        let mut lstElt_55 = lstElt_55.clone();
        txt = (match lstElt_55.clone() {
        mut i_var => {
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_56(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_56 in &*items.clone() {
        let mut lstElt_56 = lstElt_56.clone();
        txt = (match lstElt_56.clone() {
        mut i_var => {
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_57(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_57 in &*items.clone() {
        let mut lstElt_57 = lstElt_57.clone();
        txt = (match lstElt_57.clone() {
        mut i_var => {
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_58(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_58 in &*items.clone() {
        let mut lstElt_58 = lstElt_58.clone();
        txt = (match lstElt_58.clone() {
        mut i_var => {
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_59(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_59 in &*items.clone() {
        let mut lstElt_59 = lstElt_59.clone();
        txt = (match lstElt_59.clone() {
        mut i_var => {
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_60(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_60 in &*items.clone() {
        let mut lstElt_60 = lstElt_60.clone();
        txt = (match lstElt_60.clone() {
        mut i_var => {
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_61(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_61 in &*items.clone() {
        let mut lstElt_61 = lstElt_61.clone();
        txt = (match lstElt_61.clone() {
        mut i_var => {
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_62(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_62 in &*items.clone() {
        let mut lstElt_62 = lstElt_62.clone();
        txt = (match lstElt_62.clone() {
        mut i_var => {
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_63(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_63 in &*items.clone() {
        let mut lstElt_63 = lstElt_63.clone();
        txt = (match lstElt_63.clone() {
        mut i_var => {
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_64(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_64 in &*items.clone() {
        let mut lstElt_64 = lstElt_64.clone();
        txt = (match lstElt_64.clone() {
        mut i_var => {
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_65(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_65 in &*items.clone() {
        let mut lstElt_65 = lstElt_65.clone();
        txt = (match lstElt_65.clone() {
        mut i_var => {
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_66(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_66 in &*items.clone() {
        let mut lstElt_66 = lstElt_66.clone();
        txt = (match lstElt_66.clone() {
        mut i_var => {
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_67(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_67 in &*items.clone() {
        let mut lstElt_67 = lstElt_67.clone();
        txt = (match lstElt_67.clone() {
        mut i_var => {
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_68(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_68 in &*items.clone() {
        let mut lstElt_68 = lstElt_68.clone();
        txt = (match lstElt_68.clone() {
        mut i_var => {
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_69(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_69 in &*items.clone() {
        let mut lstElt_69 = lstElt_69.clone();
        txt = (match lstElt_69.clone() {
        mut i_var => {
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

pub(crate) fn modelVariablesXml(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_modelInfo.clone()) {
        (mut txt, SimCode::ModelInfo { vars: SimCodeVar::SimVars { stateVars: ref i_vars_stateVars, derivativeVars: ref i_vars_derivativeVars, algVars: ref i_vars_algVars, discreteAlgVars: ref i_vars_discreteAlgVars, intAlgVars: ref i_vars_intAlgVars, boolAlgVars: ref i_vars_boolAlgVars, outputVars: ref i_vars_outputVars, aliasVars: ref i_vars_aliasVars, intAliasVars: ref i_vars_intAliasVars, boolAliasVars: ref i_vars_boolAliasVars, paramVars: ref i_vars_paramVars, intParamVars: ref i_vars_intParamVars, boolParamVars: ref i_vars_boolParamVars, stringAlgVars: ref i_vars_stringAlgVars, stringParamVars: ref i_vars_stringParamVars, stringAliasVars: ref i_vars_stringAliasVars, extObjVars: ref i_vars_extObjVars, constVars: ref i_vars_constVars, intConstVars: ref i_vars_intConstVars, boolConstVars: ref i_vars_boolConstVars, stringConstVars: ref i_vars_stringConstVars, .. }, .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<ModelVariables>\n")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_49(txt.clone(), i_vars_stateVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_50(txt.clone(), i_vars_derivativeVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_51(txt.clone(), i_vars_algVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_52(txt.clone(), i_vars_discreteAlgVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_53(txt.clone(), i_vars_intAlgVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_54(txt.clone(), i_vars_boolAlgVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_55(txt.clone(), i_vars_outputVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_56(txt.clone(), i_vars_aliasVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_57(txt.clone(), i_vars_intAliasVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_58(txt.clone(), i_vars_boolAliasVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_59(txt.clone(), i_vars_paramVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_60(txt.clone(), i_vars_intParamVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_61(txt.clone(), i_vars_boolParamVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_62(txt.clone(), i_vars_stringAlgVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_63(txt.clone(), i_vars_stringParamVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_64(txt.clone(), i_vars_stringAliasVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_65(txt.clone(), i_vars_extObjVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_66(txt.clone(), i_vars_constVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_67(txt.clone(), i_vars_intConstVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_68(txt.clone(), i_vars_boolConstVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_69(txt.clone(), i_vars_stringConstVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</ModelVariables>")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn ScalarVariableXml(mut in_txt: Tpl::Text, mut in_a_simVar: SimCodeVar::SimVar) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simVar.clone()) {
        (mut txt, mut i_simVar @ SimCodeVar::SimVar { name: _, .. }) => {
            txt = ScalarVariableAttributesXml(txt.clone(), i_simVar.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_72(mut in_txt: Tpl::Text, mut in_a_comment: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_comment.clone())) {
        (txt, Deref @ "") => {
            txt.clone()
        },
        (txt, i_comment) => {
            let mut ret_0: ArcStr;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("description=\"")).clone() }))?;
            ret_0 = (Util::escapeModelicaStringToXmlString((i_comment.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn ScalarVariableAttributesXml(mut in_txt: Tpl::Text, mut in_a_simVar: SimCodeVar::SimVar) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simVar.clone()) {
        (mut txt, SimCodeVar::SimVar { varKind: mut i_varKind, comment: mut i_comment, aliasvar: mut i_aliasvar, causality: mut i_causality, name: ref i_name, type_: ref i_type__, unit: mut i_unit, displayUnit: mut i_displayUnit, minValue: mut i_minValue, maxValue: mut i_maxValue, initialValue: mut i_initialValue, isFixed: mut i_isFixed, .. }) => {
            let mut l_variableCategory: Tpl::Text;
            let mut l_caus: Tpl::Text;
            let mut l_alias: Tpl::Text;
            let mut l_description: Tpl::Text;
            let mut l_variability: Tpl::Text;
            let mut ret_1: i32;
            let mut l_valueReference: Tpl::Text;
            ret_1 = System::tmpTick();
            l_valueReference = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_1.clone())).clone())?;
            l_variability = getVariablityXml(Tpl::emptyTxt.clone(), i_varKind.clone())?;
            l_description = fun_72(Tpl::emptyTxt.clone(), (i_comment.clone()).clone())?;
            l_alias = getAliasVarXml(Tpl::emptyTxt.clone(), i_aliasvar.clone())?;
            l_caus = getCausalityXml(Tpl::emptyTxt.clone(), i_causality.clone())?;
            l_variableCategory = variableCategoryXml(Tpl::emptyTxt.clone(), i_varKind.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<ScalarVariable name=\"")).clone() }))?;
            txt = crefStrXml(txt.clone(), i_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_description.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" valueReference=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_valueReference.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" variability=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_variability.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" causality=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_caus.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" alias=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_alias.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\">\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = ScalarVariableTypeXml(txt.clone(), i_type__.clone(), (i_unit.clone()).clone(), (i_displayUnit.clone()).clone(), i_minValue.clone(), i_maxValue.clone(), i_initialValue.clone(), i_isFixed.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<QualifiedName>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = qualifiedNamePartXml(txt.clone(), i_name.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</QualifiedName>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<isLinearTimedVariables>\n")).clone(), (literal!("  <TimePoint index=\"0\" isLinear=\"true\"/>\n")).clone(), (literal!("</isLinearTimedVariables>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<VariableCategory>")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_variableCategory.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</VariableCategory>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</ScalarVariable>")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn getCausalityXml(mut in_txt: Tpl::Text, mut in_a_c: Option<SimCodeVar::Causality>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_c.clone()) {
        (mut txt, Some(SimCodeVar::Causality::NONECAUS { .. })) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("none")).clone() }))?;
            txt.clone()
        },
        (mut txt, Some(SimCodeVar::Causality::OUTPUT { .. })) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("output")).clone() }))?;
            txt.clone()
        },
        (mut txt, Some(SimCodeVar::Causality::INPUT { .. })) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("input")).clone() }))?;
            txt.clone()
        },
        (mut txt, Some(SimCodeVar::Causality::LOCAL { .. })) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("local")).clone() }))?;
            txt.clone()
        },
        (mut txt, Some(SimCodeVar::Causality::PARAMETER { .. })) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("parameter")).clone() }))?;
            txt.clone()
        },
        (mut txt, Some(SimCodeVar::Causality::CALCULATED_PARAMETER { .. })) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("calculatedParameter")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("internal")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn getVariablityXml(mut in_txt: Tpl::Text, mut in_a_varKind: BackendDAE::VarKind) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_varKind.clone()) {
        (mut txt, BackendDAE::VarKind::DISCRETE { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("discrete")).clone() }))?;
            txt.clone()
        },
        (mut txt, BackendDAE::VarKind::PARAM { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("parameter")).clone() }))?;
            txt.clone()
        },
        (mut txt, BackendDAE::VarKind::CONST { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("constant")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("continuous")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn getAliasVarXml(mut in_txt: Tpl::Text, mut in_a_aliasvar: SimCodeVar::AliasVariable) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_aliasvar.clone()) {
        (mut txt, SimCodeVar::AliasVariable::NOALIAS { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("noAlias")).clone() }))?;
            txt.clone()
        },
        (mut txt, SimCodeVar::AliasVariable::ALIAS { varName: ref i_varName }) => {
            txt = crefStrXml(txt.clone(), i_varName.clone())?;
            txt.clone()
        },
        (mut txt, SimCodeVar::AliasVariable::NEGATEDALIAS { varName: ref i_varName }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-")).clone() }))?;
            txt = crefStrXml(txt.clone(), i_varName.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn variableCategoryXml(mut in_txt: Tpl::Text, mut in_a_varKind: BackendDAE::VarKind) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_varKind.clone()) {
        (mut txt, BackendDAE::VarKind::VARIABLE { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("algebraic")).clone() }))?;
            txt.clone()
        },
        (mut txt, BackendDAE::VarKind::STATE { index: _, .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("state")).clone() }))?;
            txt.clone()
        },
        (mut txt, BackendDAE::VarKind::STATE_DER { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("derivative")).clone() }))?;
            txt.clone()
        },
        (mut txt, BackendDAE::VarKind::DUMMY_DER { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("algebraic")).clone() }))?;
            txt.clone()
        },
        (mut txt, BackendDAE::VarKind::DUMMY_STATE { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("algebraic")).clone() }))?;
            txt.clone()
        },
        (mut txt, BackendDAE::VarKind::DISCRETE { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("algebraic")).clone() }))?;
            txt.clone()
        },
        (mut txt, BackendDAE::VarKind::PARAM { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("independentParameter")).clone() }))?;
            txt.clone()
        },
        (mut txt, BackendDAE::VarKind::CONST { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("independentConstant")).clone() }))?;
            txt.clone()
        },
        (mut txt, BackendDAE::VarKind::EXTOBJ { fullClassName: ref i_fullClassName }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("externalObject_")).clone() }))?;
            txt = dotPathXml(txt.clone(), i_fullClassName.clone())?;
            txt.clone()
        },
        (mut txt, BackendDAE::VarKind::JAC_VAR { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("jacobianVar")).clone() }))?;
            txt.clone()
        },
        (mut txt, BackendDAE::VarKind::JAC_TMP_VAR { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("jacobianTmpVar")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 283, 14), (literal!("Unexpected simVarTypeName varKind")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn ScalarVariableTypeXml(mut in_txt: Tpl::Text, mut in_a_type__: Arc<DAE::Type>, mut in_a_unit: ArcStr, mut in_a_displayUnit: ArcStr, mut in_a_minValue: Option<Arc<DAE::Exp>>, mut in_a_maxValue: Option<Arc<DAE::Exp>>, mut in_a_initialValue: Option<Arc<DAE::Exp>>, mut in_a_isFixed: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_type__.clone(), in_a_unit.clone(), in_a_displayUnit.clone(), in_a_minValue.clone(), in_a_maxValue.clone(), in_a_initialValue.clone(), in_a_isFixed.clone())) {
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }, _, _, a_minValue, a_maxValue, a_initialValue, a_isFixed) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Integer ")).clone() }))?;
            txt = ScalarVariableTypeCommonAttributeXml(txt.clone(), a_initialValue.clone(), a_isFixed.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = ScalarVariableTypeMinAttribute(txt.clone(), a_minValue.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = ScalarVariableTypeMaxAttribute(txt.clone(), a_maxValue.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }, a_unit, a_displayUnit, a_minValue, a_maxValue, a_initialValue, a_isFixed) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Real ")).clone() }))?;
            txt = ScalarVariableTypeCommonAttributeXml(txt.clone(), a_initialValue.clone(), a_isFixed.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = ScalarVariableTypeMinAttribute(txt.clone(), a_minValue.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = ScalarVariableTypeMaxAttribute(txt.clone(), a_maxValue.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = ScalarVariableTypeRealAttributeXml(txt.clone(), (a_unit.clone()).clone(), (a_displayUnit.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }, _, _, _, _, a_initialValue, a_isFixed) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Boolean ")).clone() }))?;
            txt = ScalarVariableTypeCommonAttributeXml(txt.clone(), a_initialValue.clone(), a_isFixed.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }, _, _, _, _, a_initialValue, a_isFixed) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<String ")).clone() }))?;
            txt = ScalarVariableTypeCommonAttributeXml(txt.clone(), a_initialValue.clone(), a_isFixed.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ENUMERATION { index: _, .. }, _, _, _, _, a_initialValue, a_isFixed) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Real ")).clone() }))?;
            txt = ScalarVariableTypeCommonAttributeXml(txt.clone(), a_initialValue.clone(), a_isFixed.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/>")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("UNKOWN_TYPE")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_79(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_isFixed: bool, mut in_a_exp: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_isFixed.clone(), in_a_exp.clone())) {
        (txt, false, _, _) => {
            txt.clone()
        },
        (txt, _, a_isFixed, a_exp) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("start=\"")).clone() }))?;
            txt = initValXml(txt.clone(), a_exp.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" fixed=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (Tpl::booleanString(a_isFixed.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn ScalarVariableTypeCommonAttributeXml(mut in_txt: Tpl::Text, mut in_a_initialValue: Option<Arc<DAE::Exp>>, mut in_a_isFixed: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_initialValue.clone(), in_a_isFixed.clone())) {
        (txt, Some(i_exp), a_isFixed) => {
            let mut ret_2: bool;
            let mut ret_1: bool;
            let mut ret_0: bool;
            let mut txt = (*txt).clone();
            ret_0 = Expression::isEvaluatedConst(i_exp.clone());
            ret_1 = Expression::isCref(i_exp.clone());
            ret_2 = boolOr(ret_0.clone(), ret_1.clone());
            txt = fun_79(txt.clone(), ret_2.clone(), a_isFixed.clone(), i_exp.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn ScalarVariableTypeMinAttribute(mut in_txt: Tpl::Text, mut in_a_minValue: Option<Arc<DAE::Exp>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_minValue.clone())) {
        (txt, Some(i_exp)) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("min=\"")).clone() }))?;
            txt = initValXml(txt.clone(), i_exp.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn ScalarVariableTypeMaxAttribute(mut in_txt: Tpl::Text, mut in_a_maxValue: Option<Arc<DAE::Exp>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_maxValue.clone())) {
        (txt, Some(i_exp)) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("max=\"")).clone() }))?;
            txt = initValXml(txt.clone(), i_exp.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_83(mut in_txt: Tpl::Text, mut in_a_bool: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_bool.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("0")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("1")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn initValXml(mut in_txt: Tpl::Text, mut in_a_initialValue: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_initialValue.clone())) {
        (txt, Deref @ DAE::Exp::ICONST { integer: i_integer }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_integer.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::RCONST { real: i_real }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (realString(i_real.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::SCONST { string: i_string }) => {
            let mut ret_0: ArcStr;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("&quot;")).clone() }))?;
            ret_0 = (Util::escapeModelicaStringToXmlString((i_string.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("&quot;")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::BCONST { bool: i_bool }) => {
            let mut txt = (*txt).clone();
            txt = fun_83(txt.clone(), i_bool.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::ENUM_LITERAL { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CREF { componentRef: i_componentRef, .. }) => {
            let mut txt = (*txt).clone();
            txt = crefStrXml(txt.clone(), i_componentRef.clone())?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*ERROR* initial value of unknown type")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_85(mut in_txt: Tpl::Text, mut in_a_unit: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_unit.clone())) {
        (txt, Deref @ "") => {
            txt.clone()
        },
        (txt, i_unit) => {
            let mut ret_0: ArcStr;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("unit=\"")).clone() }))?;
            ret_0 = (Util::escapeModelicaStringToXmlString((i_unit.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_86(mut in_txt: Tpl::Text, mut in_a_displayUnit: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_displayUnit.clone())) {
        (txt, Deref @ "") => {
            txt.clone()
        },
        (txt, i_displayUnit) => {
            let mut ret_0: ArcStr;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("displayUnit=\"")).clone() }))?;
            ret_0 = (Util::escapeModelicaStringToXmlString((i_displayUnit.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn ScalarVariableTypeRealAttributeXml(mut txt: Tpl::Text, mut a_unit: ArcStr, mut a_displayUnit: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_displayUnit__: Tpl::Text;
    let mut l_unit__: Tpl::Text;
    l_unit__ = fun_85(Tpl::emptyTxt.clone(), (a_unit.clone()).clone())?;
    l_displayUnit__ = fun_86(Tpl::emptyTxt.clone(), (a_displayUnit.clone()).clone())?;
    out_txt = Tpl::writeText(txt.clone(), l_unit__.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_displayUnit__.clone())?;
    Ok(out_txt)
}

fn fun_88(mut in_txt: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_context.clone(), in_a_cr.clone())) {
        (txt, SimCodeFunction::Context::FUNCTION_CONTEXT { cref_prefix: _, .. }, a_cr) => {
            let mut ret_1: ArcStr;
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_0 = crefStrXml(Tpl::emptyTxt.clone(), a_cr.clone())?;
            ret_1 = (System::unquoteIdentifier((Tpl::textString(txt_0.clone())?).clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_1.clone()).clone())?;
            txt.clone()
        },
        (txt, _, a_cr) => {
            let mut txt = (*txt).clone();
            txt = crefXml(txt.clone(), a_cr.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn contextCrefXml(mut txt: Tpl::Text, mut a_cr: Arc<DAE::ComponentRef>, mut a_context: SimCodeFunction::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_88(txt.clone(), a_context.clone(), a_cr.clone())?;
    Ok(out_txt)
}

fn fun_90(mut in_txt: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_context.clone(), in_a_name.clone()) {
        (mut txt, SimCodeFunction::Context::FUNCTION_CONTEXT { cref_prefix: _, .. }, mut a_name) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_name) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn contextIteratorNameXml(mut txt: Tpl::Text, mut a_name: ArcStr, mut a_context: SimCodeFunction::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_90(txt.clone(), a_context.clone(), (a_name.clone()).clone())?;
    Ok(out_txt)
}

pub(crate) fn crefXml(mut in_txt: Tpl::Text, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cr.clone())) {
        (txt, i_cr @ Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "xloc", .. }) => {
            let mut txt = (*txt).clone();
            txt = crefStrXml(txt.clone(), i_cr.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:Time>time</exp:Time>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::ComponentRef::WILD { .. }) => {
            txt.clone()
        },
        (txt, i_cr) => {
            let mut txt = (*txt).clone();
            txt = crefToXmlStr(txt.clone(), i_cr.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_93(mut in_txt: Tpl::Text, mut in_a_arrayTest: Tpl::Text, mut in_a_subscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut in_a_ident: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_arrayTest.clone(), in_a_subscriptLst.clone(), in_a_ident.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, _, a_ident) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_ident.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"/>")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_subscriptLst, a_ident) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_ident.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\">\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = arraysubscriptsStrXml(txt.clone(), a_subscriptLst.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:QualifiedNamePart>")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_94(mut in_txt: Tpl::Text, mut in_a_arrayTest: Tpl::Text, mut in_a_subscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut in_a_componentRef: Arc<DAE::ComponentRef>, mut in_a_ident: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_arrayTest.clone(), in_a_subscriptLst.clone(), in_a_componentRef.clone(), in_a_ident.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, _, a_componentRef, a_ident) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_ident.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\"/>\n")).clone() }))?;
            txt = qualifiedNamePartXml(txt.clone(), a_componentRef.clone())?;
            txt.clone()
        },
        (txt, _, a_subscriptLst, a_componentRef, a_ident) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_ident.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\">\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = arraysubscriptsStrXml(txt.clone(), a_subscriptLst.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = qualifiedNamePartXml(txt.clone(), a_componentRef.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:QualifiedNamePart>")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn qualifiedNamePartXml(mut in_txt: Tpl::Text, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((in_txt.clone(), in_a_cr.clone())) {
        (txt, Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: i_subscriptLst, ident: i_ident, .. }) => {
            let mut l_arrayTest: Tpl::Text;
            let mut txt = (*txt).clone();
            l_arrayTest = arraysubscriptsStrXml(Tpl::emptyTxt.clone(), i_subscriptLst.clone())?;
            return Ok(fun_93(txt.clone(), l_arrayTest.clone(), i_subscriptLst.clone(), (i_ident.clone()).clone())?)
        },
        (txt, Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$DER", componentRef: i_componentRef, .. }) => {
            let mut txt = (*txt).clone();
            { (in_txt, in_a_cr) = (txt.clone(), i_componentRef.clone()); continue '__tco; }
        },
        (txt, Deref @ DAE::ComponentRef::CREF_QUAL { subscriptLst: i_subscriptLst, ident: i_ident, componentRef: i_componentRef, .. }) => {
            let mut l_arrayTest: Tpl::Text;
            let mut txt = (*txt).clone();
            l_arrayTest = arraysubscriptsStrXml(Tpl::emptyTxt.clone(), i_subscriptLst.clone())?;
            return Ok(fun_94(txt.clone(), l_arrayTest.clone(), i_subscriptLst.clone(), i_componentRef.clone(), (i_ident.clone()).clone())?)
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CREF_NOT_IDENT_OR_QUAL")).clone() }))?)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn lm_96(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_96 in &*items.clone() {
        let mut lstElt_96 = lstElt_96.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_96.clone()) {
        i_s => {
            txt = arraysubscriptStrXml(txt.clone(), i_s.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub(crate) fn arraysubscriptsStrXml(mut in_txt: Tpl::Text, mut in_a_subscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_subscripts.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, i_subscripts) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:ArraySubscripts>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_96(txt.clone(), i_subscripts.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:ArraySubscripts>")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn arraysubscriptStrXml(mut in_txt: Tpl::Text, mut in_a_subscript: Arc<DAE::Subscript>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_subscript.clone())) {
        (txt, Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: i_i } }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:IndexExpression>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:IntegerLiteral>")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_i.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</exp:IntegerLiteral>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:IndexExpression>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Subscript::SLICE { exp: Deref @ DAE::Exp::ICONST { integer: i_i } }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:IndexExpression>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:IntegerLiteral>")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_i.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</exp:IntegerLiteral>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:IndexExpression>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Subscript::WHOLEDIM { .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("WHOLEDIM")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("UNKNOWN_SUBSCRIPT")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn crefToXmlStr(mut in_txt: Tpl::Text, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cr.clone())) {
        (txt, i_cr @ Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Identifier>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = qualifiedNamePartXml(txt.clone(), i_cr.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Identifier>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$DER", componentRef: i_componentRef, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Der>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = crefToXmlStr(txt.clone(), i_componentRef.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Der>")).clone() }))?;
            txt.clone()
        },
        (txt, i_cr @ Deref @ DAE::ComponentRef::CREF_QUAL { ident: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Identifier>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = qualifiedNamePartXml(txt.clone(), i_cr.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Identifier>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::ComponentRef::OPTIMICA_ATTR_INST_CREF { componentRef: i_componentRef, instant: i_instant }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:TimedVariable timePointIndex = \"0\">\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = crefToXmlStr(txt.clone(), i_componentRef.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:Instant>")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_instant.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</exp:Instant>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:TimedVariable>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::ComponentRef::WILD { .. }) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CREF_NOT_IDENT_OR_QUAL")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn crefStrXml(mut in_txt: Tpl::Text, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((in_txt.clone(), in_a_cr.clone())) {
        (txt, Deref @ DAE::ComponentRef::CREF_IDENT { ident: i_ident, subscriptLst: i_subscriptLst, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_ident.clone()).clone())?;
            return Ok(subscriptsStrXml(txt.clone(), i_subscriptLst.clone())?)
        },
        (txt, Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$DER", componentRef: i_componentRef, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("der(")).clone() }))?;
            txt = crefStrXml(txt.clone(), i_componentRef.clone())?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?)
        },
        (txt, Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$PRE", componentRef: i_componentRef, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("pre(")).clone() }))?;
            txt = crefStrXml(txt.clone(), i_componentRef.clone())?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?)
        },
        (txt, Deref @ DAE::ComponentRef::CREF_QUAL { ident: i_ident, subscriptLst: i_subscriptLst, componentRef: i_componentRef, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_ident.clone()).clone())?;
            txt = subscriptsStrXml(txt.clone(), i_subscriptLst.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }))?;
            { (in_txt, in_a_cr) = (txt.clone(), i_componentRef.clone()); continue '__tco; }
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CREF_NOT_IDENT_OR_QUAL")).clone() }))?)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn fun_101(mut in_txt: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_context.clone(), in_a_cr.clone())) {
        (txt, SimCodeFunction::Context::FUNCTION_CONTEXT { cref_prefix: _, .. }, a_cr) => {
            let mut txt = (*txt).clone();
            txt = arrayCrefStrXml(txt.clone(), a_cr.clone())?;
            txt.clone()
        },
        (txt, _, a_cr) => {
            let mut txt = (*txt).clone();
            txt = arrayCrefXmlStr(txt.clone(), a_cr.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn contextArrayCrefXml(mut txt: Tpl::Text, mut a_cr: Arc<DAE::ComponentRef>, mut a_context: SimCodeFunction::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_101(txt.clone(), a_context.clone(), a_cr.clone())?;
    Ok(out_txt)
}

pub(crate) fn arrayCrefXmlStr(mut txt: Tpl::Text, mut a_cr: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = arrayCrefXmlStr2(txt.clone(), a_cr.clone())?;
    Ok(out_txt)
}

pub(crate) fn arrayCrefXmlStr2(mut in_txt: Tpl::Text, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cr.clone())) {
        (txt, Deref @ DAE::ComponentRef::CREF_IDENT { ident: i_ident, .. }) => {
            let mut ret_0: ArcStr;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:QualifiedName>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            ret_0 = (System::unquoteIdentifier((i_ident.clone()).clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\">")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::ComponentRef::CREF_QUAL { ident: i_ident, subscriptLst: i_subscriptLst, componentRef: i_componentRef, .. }) => {
            let mut ret_1: ArcStr;
            let mut txt = (*txt).clone();
            ret_1 = (System::unquoteIdentifier((i_ident.clone()).clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_1.clone()).clone())?;
            txt = subscriptsStrXml(txt.clone(), i_subscriptLst.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("$P")).clone() }))?;
            txt = arrayCrefXmlStr2(txt.clone(), i_componentRef.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("testing array")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CREF_NOT_IDENT_OR_QUAL")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn arrayCrefStrXml(mut in_txt: Tpl::Text, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((in_txt.clone(), in_a_cr.clone())) {
        (txt, Deref @ DAE::ComponentRef::CREF_IDENT { ident: i_ident, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Identifier>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamepart name =\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_ident.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\"/>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Identifier>")).clone() }))?)
        },
        (txt, Deref @ DAE::ComponentRef::CREF_QUAL { ident: i_ident, componentRef: i_componentRef, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_ident.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }))?;
            { (in_txt, in_a_cr) = (txt.clone(), i_componentRef.clone()); continue '__tco; }
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CREF_NOT_IDENT_OR_QUAL")).clone() }))?)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn lm_106(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_106 in &*items.clone() {
        let mut lstElt_106 = lstElt_106.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_106.clone()) {
        i_s => {
            txt = subscriptStrXml(txt.clone(), i_s.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub(crate) fn subscriptsStrXml(mut in_txt: Tpl::Text, mut in_a_subscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_subscripts.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, i_subscripts) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_106(txt.clone(), i_subscripts.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn subscriptStrXml(mut in_txt: Tpl::Text, mut in_a_subscript: Arc<DAE::Subscript>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_subscript.clone())) {
        (txt, Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: i_i } }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_i.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Subscript::SLICE { exp: Deref @ DAE::Exp::ICONST { integer: i_i } }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_i.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Subscript::WHOLEDIM { .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("WHOLEDIM")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("UNKNOWN_SUBSCRIPT")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn expCrefXml(mut in_txt: Tpl::Text, mut in_a_ecr: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ecr.clone())) {
        (txt, Deref @ DAE::Exp::CREF { componentRef: i_componentRef, .. }) => {
            let mut txt = (*txt).clone();
            txt = crefXml(txt.clone(), i_componentRef.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: i_arg_componentRef, .. }, tail: Deref @ metamodelica::List::Nil }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Der>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = crefXml(txt.clone(), i_arg_componentRef.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Der>")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ERROR_NOT_A_CREF")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn crefFunctionNameXml(mut in_txt: Tpl::Text, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((in_txt.clone(), in_a_cr.clone())) {
        (txt, Deref @ DAE::ComponentRef::CREF_IDENT { ident: i_ident, .. }) => {
            let mut ret_1: ArcStr;
            let mut ret_0: ArcStr;
            let mut txt = (*txt).clone();
            ret_0 = (System::unquoteIdentifier((i_ident.clone()).clone())).clone();
            ret_1 = (System::stringReplace((ret_0.clone()).clone(), (literal!("_")).clone(), (literal!("__")).clone())?).clone();
            return Ok(Tpl::writeStr(txt.clone(), (ret_1.clone()).clone())?)
        },
        (txt, Deref @ DAE::ComponentRef::CREF_QUAL { ident: i_ident, componentRef: i_componentRef, .. }) => {
            let mut ret_3: ArcStr;
            let mut ret_2: ArcStr;
            let mut txt = (*txt).clone();
            ret_2 = (System::unquoteIdentifier((i_ident.clone()).clone())).clone();
            ret_3 = (System::stringReplace((ret_2.clone()).clone(), (literal!("_")).clone(), (literal!("__")).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_3.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            { (in_txt, in_a_cr) = (txt.clone(), i_componentRef.clone()); continue '__tco; }
        },
        (txt, _) => {
            return Ok(txt.clone())
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn dotPathXml(mut in_txt: Tpl::Text, mut in_a_path: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((in_txt.clone(), in_a_path.clone())) {
        (txt, Deref @ Absyn::Path::QUALIFIED { name: i_name, path: i_path }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }))?;
            { (in_txt, in_a_path) = (txt.clone(), i_path.clone()); continue '__tco; }
        },
        (txt, Deref @ Absyn::Path::IDENT { name: i_name_1 }) => {
            let mut txt = (*txt).clone();
            return Ok(Tpl::writeStr(txt.clone(), (i_name_1.clone()).clone())?)
        },
        (txt, Deref @ Absyn::Path::FULLYQUALIFIED { path: i_path }) => {
            let mut txt = (*txt).clone();
            { (in_txt, in_a_path) = (txt.clone(), i_path.clone()); continue '__tco; }
        },
        (txt, _) => {
            return Ok(txt.clone())
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn replaceDotAndUnderscoreXml(mut in_txt: Tpl::Text, mut in_a_str: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_str.clone()) {
        (mut txt, mut i_name) => {
            let mut ret_4: ArcStr;
            let mut ret_3: ArcStr;
            let mut l_str__underscores: Tpl::Text;
            let mut ret_1: ArcStr;
            let mut l_str__dots: Tpl::Text;
            ret_1 = (System::stringReplace((i_name.clone()).clone(), (literal!(".")).clone(), (literal!("_")).clone())?).clone();
            l_str__dots = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1.clone()).clone())?;
            ret_3 = (System::stringReplace((Tpl::textString(l_str__dots.clone())?).clone(), (literal!("_")).clone(), (literal!("__")).clone())?).clone();
            l_str__underscores = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_3.clone()).clone())?;
            ret_4 = (System::unquoteIdentifier((Tpl::textString(l_str__underscores.clone())?).clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_4.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn underscorePathXml(mut in_txt: Tpl::Text, mut in_a_path: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((in_txt.clone(), in_a_path.clone())) {
        (txt, Deref @ Absyn::Path::QUALIFIED { name: i_name, path: i_path }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\"/>\n")).clone() }))?;
            { (in_txt, in_a_path) = (txt.clone(), i_path.clone()); continue '__tco; }
        },
        (txt, Deref @ Absyn::Path::IDENT { name: i_name_1 }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name_1.clone()).clone())?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"/>")).clone() }))?)
        },
        (txt, Deref @ Absyn::Path::FULLYQUALIFIED { path: i_path }) => {
            let mut txt = (*txt).clone();
            { (in_txt, in_a_path) = (txt.clone(), i_path.clone()); continue '__tco; }
        },
        (txt, _) => {
            return Ok(txt.clone())
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn lm_114(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_114 in &*items.clone() {
        let mut lstElt_114 = lstElt_114.clone();
        txt = (match lstElt_114.clone() {
        mut i_var => {
            txt = bindingEquationXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_115(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_115 in &*items.clone() {
        let mut lstElt_115 = lstElt_115.clone();
        txt = (match lstElt_115.clone() {
        mut i_var => {
            txt = bindingEquationXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_116(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_116 in &*items.clone() {
        let mut lstElt_116 = lstElt_116.clone();
        txt = (match lstElt_116.clone() {
        mut i_var => {
            txt = bindingEquationXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_117(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_117 in &*items.clone() {
        let mut lstElt_117 = lstElt_117.clone();
        txt = (match lstElt_117.clone() {
        mut i_var => {
            txt = bindingEquationXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

pub(crate) fn bindingEquationsXml(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_modelInfo.clone()) {
        (mut txt, SimCode::ModelInfo { varInfo: SimCode::VarInfo { numStateVars: _, .. }, vars: SimCodeVar::SimVars { paramVars: ref i_vars_paramVars, intParamVars: ref i_vars_intParamVars, boolParamVars: ref i_vars_boolParamVars, stringParamVars: ref i_vars_stringParamVars, .. }, .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<equ:BindingEquations>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_114(txt.clone(), i_vars_paramVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_115(txt.clone(), i_vars_intParamVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_116(txt.clone(), i_vars_boolParamVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_117(txt.clone(), i_vars_stringParamVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</equ:BindingEquations>")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_119(mut in_txt: Tpl::Text, mut in_a_initialValue: Option<Arc<DAE::Exp>>, mut in_a_varName: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_initialValue.clone(), in_a_varName.clone())) {
        (txt, Some(i_exp), a_varName) => {
            let mut l_preExp: Tpl::Text;
            let mut l_varDecls: Tpl::Text;
            let mut txt = (*txt).clone();
            l_varDecls = Tpl::emptyTxt.clone();
            l_preExp = Tpl::emptyTxt.clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<equ:BindingEquation>\n")).clone(), (literal!("  <equ:Parameter>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), a_varName.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </equ:Parameter>\n")).clone(), (literal!("  <equ:BindingExp>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            (txt, l_preExp, l_varDecls) = daeExpXml(txt.clone(), i_exp.clone(), SimCodeFunction::contextOther().clone(), l_preExp.clone(), l_varDecls.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </equ:BindingExp>\n")).clone(), (literal!("</equ:BindingEquation>")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn bindingEquationXml(mut in_txt: Tpl::Text, mut in_a_var: SimCodeVar::SimVar) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_var.clone()) {
        (mut txt, SimCodeVar::SimVar { name: ref i_name, initialValue: mut i_initialValue, .. }) => {
            let mut l_varName: Tpl::Text;
            l_varName = qualifiedNamePartXml(Tpl::emptyTxt.clone(), i_name.clone())?;
            txt = fun_119(txt.clone(), i_initialValue.clone(), l_varName.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_121(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_tmp: Tpl::Text, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_tmp: Tpl::Text = a_tmp;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_121 in &*items.clone() {
        let mut lstElt_121 = lstElt_121.clone();
        (txt, a_tmp, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_121.clone()) {
        i_eq => {
            (txt, a_varDecls, a_tmp) = equation_Xml(txt.clone(), i_eq.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), a_varDecls.clone(), a_tmp.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_tmp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_tmp, a_varDecls))
}

pub(crate) fn equationsXml(mut txt: Tpl::Text, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_eqs: Tpl::Text;
    let mut l_tmp: Tpl::Text;
    let mut l_jens: Tpl::Text;
    let mut l_varDecls: Tpl::Text;
    l_varDecls = Tpl::emptyTxt.clone();
    System::tmpTickReset(0);
    l_jens = Tpl::emptyTxt.clone();
    l_tmp = Tpl::emptyTxt.clone();
    l_eqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    (l_eqs, l_tmp, l_varDecls) = lm_121(l_eqs.clone(), a_allEquationsPlusWhen.clone(), l_tmp.clone(), l_varDecls.clone())?;
    l_eqs = Tpl::popIter(l_eqs.clone())?;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<equ:DynamicEquations>\n")).clone() }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_tmp.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeText(out_txt.clone(), l_eqs.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</equ:DynamicEquations>")).clone() }))?;
    Ok(out_txt)
}

fn lm_123(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_123 in &*items.clone() {
        let mut lstElt_123 = lstElt_123.clone();
        (txt, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_123.clone()) {
        i_eq => {
            (txt, a_varDecls) = equationAlgorithmXml(txt.clone(), i_eq.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_varDecls))
}

pub(crate) fn algorithmicEquationsXml(mut txt: Tpl::Text, mut a_allEquations: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_algs: Tpl::Text;
    let mut l_varDecls: Tpl::Text;
    l_varDecls = Tpl::emptyTxt.clone();
    l_algs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    (l_algs, l_varDecls) = lm_123(l_algs.clone(), a_allEquations.clone(), l_varDecls.clone())?;
    l_algs = Tpl::popIter(l_algs.clone())?;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Algorithm>\n")).clone() }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_algs.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:Algorithm>")).clone() }))?;
    Ok(out_txt)
}

fn lm_125(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_125 in &*items.clone() {
        let mut lstElt_125 = lstElt_125.clone();
        (txt, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_125.clone()) {
        i_stmt => {
            (txt, a_varDecls) = algStatementXml(txt.clone(), i_stmt.clone(), SimCodeFunction::contextFunction().clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_varDecls))
}

fn fun_126(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_ALGORITHM { statements: i_statements, .. }, a_varDecls) => {
            let mut l_alg: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_alg = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_alg, a_varDecls) = lm_125(l_alg.clone(), i_statements.clone(), a_varDecls.clone())?;
            l_alg = Tpl::popIter(l_alg.clone())?;
            txt = Tpl::writeText(txt.clone(), l_alg.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub(crate) fn equationAlgorithmXml(mut txt: Tpl::Text, mut a_eq: Arc<SimCode::SimEqSystem>, mut a_context: SimCodeFunction::Context, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = fun_126(txt.clone(), a_eq.clone(), a_varDecls.clone())?;
    Ok((out_txt, out_a_varDecls))
}

fn lm_128(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_tmp: Tpl::Text, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_tmp: Tpl::Text = a_tmp;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_128 in &*items.clone() {
        let mut lstElt_128 = lstElt_128.clone();
        (txt, a_tmp, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_128.clone()) {
        i_eq => {
            (txt, a_varDecls, a_tmp) = equation_Xml(txt.clone(), i_eq.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), a_varDecls.clone(), a_tmp.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_tmp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_tmp, a_varDecls))
}

fn lm_129(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_129 in &*items.clone() {
        let mut lstElt_129 = lstElt_129.clone();
        txt = (match lstElt_129.clone() {
        mut i_var => {
            txt = initialEquationXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_130(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_130 in &*items.clone() {
        let mut lstElt_130 = lstElt_130.clone();
        txt = (match lstElt_130.clone() {
        mut i_var => {
            txt = initialEquationXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_131(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_131 in &*items.clone() {
        let mut lstElt_131 = lstElt_131.clone();
        txt = (match lstElt_131.clone() {
        mut i_var => {
            txt = initialEquationXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_132(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_132 in &*items.clone() {
        let mut lstElt_132 = lstElt_132.clone();
        txt = (match lstElt_132.clone() {
        mut i_var => {
            txt = initialEquationXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_133(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_133 in &*items.clone() {
        let mut lstElt_133 = lstElt_133.clone();
        txt = (match lstElt_133.clone() {
        mut i_var => {
            txt = initialEquationXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_134(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_134 in &*items.clone() {
        let mut lstElt_134 = lstElt_134.clone();
        txt = (match lstElt_134.clone() {
        mut i_var => {
            txt = initialEquationXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_135(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_135 in &*items.clone() {
        let mut lstElt_135 = lstElt_135.clone();
        txt = (match lstElt_135.clone() {
        mut i_var => {
            txt = initialEquationXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

pub(crate) fn initialEquationsXml(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo, mut in_a_initialEqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_modelInfo.clone(), in_a_initialEqs.clone())) {
        (txt, SimCode::ModelInfo { varInfo: SimCode::VarInfo { numStateVars: _, .. }, vars: SimCodeVar::SimVars { stateVars: i_vars_stateVars, derivativeVars: i_vars_derivativeVars, algVars: i_vars_algVars, discreteAlgVars: i_vars_discreteAlgVars, intAlgVars: i_vars_intAlgVars, boolAlgVars: i_vars_boolAlgVars, stringAlgVars: i_vars_stringAlgVars, .. }, .. }, a_initialEqs) => {
            let mut l_eqs: Tpl::Text;
            let mut l_tmp: Tpl::Text;
            let mut l_jens: Tpl::Text;
            let mut l_varDecls: Tpl::Text;
            let mut txt = (*txt).clone();
            l_varDecls = Tpl::emptyTxt.clone();
            System::tmpTickReset(0);
            l_jens = Tpl::emptyTxt.clone();
            l_tmp = Tpl::emptyTxt.clone();
            l_eqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_eqs, l_tmp, l_varDecls) = lm_128(l_eqs.clone(), a_initialEqs.clone(), l_tmp.clone(), l_varDecls.clone())?;
            l_eqs = Tpl::popIter(l_eqs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<equ:InitialEquations>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_129(txt.clone(), i_vars_stateVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_130(txt.clone(), i_vars_derivativeVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_131(txt.clone(), i_vars_algVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_132(txt.clone(), i_vars_discreteAlgVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_133(txt.clone(), i_vars_intAlgVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_134(txt.clone(), i_vars_boolAlgVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_135(txt.clone(), i_vars_stringAlgVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_tmp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_eqs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</equ:InitialEquations>")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_137(mut in_txt: Tpl::Text, mut in_a_initialValue: Option<Arc<DAE::Exp>>, mut in_a_identName: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_initialValue.clone(), in_a_identName.clone())) {
        (txt, Some(i_exp), a_identName) => {
            let mut l_preExp: Tpl::Text;
            let mut l_varDecls: Tpl::Text;
            let mut txt = (*txt).clone();
            l_varDecls = Tpl::emptyTxt.clone();
            l_preExp = Tpl::emptyTxt.clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<equ:Equation>\n")).clone(), (literal!("  <exp:Sub>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), a_identName.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            (txt, l_preExp, l_varDecls) = daeExpXml(txt.clone(), i_exp.clone(), SimCodeFunction::contextOther().clone(), l_preExp.clone(), l_varDecls.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </exp:Sub>\n")).clone(), (literal!("</equ:Equation>")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn initialEquationXml(mut in_txt: Tpl::Text, mut in_a_var: SimCodeVar::SimVar) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_var.clone()) {
        (mut txt, SimCodeVar::SimVar { name: ref i_name, initialValue: mut i_initialValue, .. }) => {
            let mut l_identName: Tpl::Text;
            l_identName = crefXml(Tpl::emptyTxt.clone(), i_name.clone())?;
            txt = fun_137(txt.clone(), i_initialValue.clone(), l_identName.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_139(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>, mut in_a_varD: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varD: Tpl::Text;
    (out_txt, out_a_varD) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone(), in_a_varD.clone(), in_a_context.clone())) {
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { index: _, .. }, a_varD, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varD = (*a_varD).clone();
            (txt, a_varD) = equationSimpleAssignXml(txt.clone(), i_e.clone(), a_context.clone(), a_varD.clone())?;
            (txt.clone(), a_varD.clone())
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS { index: _, .. }, a_varD, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varD = (*a_varD).clone();
            (txt, a_varD) = equationSimpleAssignXml(txt.clone(), i_e.clone(), a_context.clone(), a_varD.clone())?;
            (txt.clone(), a_varD.clone())
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN { index: _, .. }, a_varD, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varD = (*a_varD).clone();
            (txt, a_varD) = equationArrayCallAssignXml(txt.clone(), i_e.clone(), a_context.clone(), a_varD.clone())?;
            (txt.clone(), a_varD.clone())
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_IFEQUATION { index: _, .. }, a_varD, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("IfEquation Assign Not implemente yet")).clone() }))?;
            (txt.clone(), a_varD.clone())
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: _, .. }, a_varD, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varD = (*a_varD).clone();
            (txt, a_varD) = equationLinearXml(txt.clone(), i_e.clone(), a_context.clone(), a_varD.clone())?;
            (txt.clone(), a_varD.clone())
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem: _, .. }, a_varD, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varD = (*a_varD).clone();
            (txt, a_varD) = equationNonlinearXml(txt.clone(), i_e.clone(), a_context.clone(), a_varD.clone())?;
            (txt.clone(), a_varD.clone())
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_WHEN { index: _, .. }, a_varD, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            (txt.clone(), a_varD.clone())
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_ALIAS { aliasOf: _, .. }, a_varD, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            (txt.clone(), a_varD.clone())
        },
        (txt, _, a_varD, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NOT IMPLEMENTED EQUATION")).clone() }))?;
            (txt.clone(), a_varD.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varD))
}

pub(crate) fn equation_Xml(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text, mut in_a_eqs: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_eqs: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_eqs) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone(), in_a_context.clone(), in_a_varDecls.clone(), in_a_eqs.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_MIXED { index: _, .. }, _, a_varDecls, a_eqs) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" MIXED EQUATION NOT IMPLEMENTED ")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_eqs.clone())
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_ALGORITHM { statements: Deref @ metamodelica::List::Nil, .. }, _, a_varDecls, a_eqs) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_eqs.clone())
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_ALGORITHM { index: _, .. }, _, a_varDecls, a_eqs) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_eqs.clone())
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_WHEN { index: _, .. }, a_context, a_varDecls, a_eqs) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = equationWhenXml(txt.clone(), i_e.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone(), a_eqs.clone())
        },
        (txt, i_eq, a_context, a_varDecls, a_eqs) => {
            let mut l_x: Tpl::Text;
            let mut l_varD: Tpl::Text;
            let mut l_tmp: Tpl::Text;
            let mut ret_1: i32;
            let mut l_ix: Tpl::Text;
            let mut a_eqs = (*a_eqs).clone();
            ret_1 = System::tmpTickIndex(10);
            l_ix = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_1.clone())).clone())?;
            l_tmp = Tpl::emptyTxt.clone();
            l_varD = Tpl::emptyTxt.clone();
            (l_x, l_varD) = fun_139(Tpl::emptyTxt.clone(), i_eq.clone(), l_varD.clone(), a_context.clone())?;
            a_eqs = Tpl::writeTok(a_eqs.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<equ:Equation>\n")).clone(), (literal!("  <exp:Sub>\n")).clone()], lastHasNewLine: true }))?;
            a_eqs = Tpl::pushBlock(a_eqs.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            a_eqs = Tpl::writeText(a_eqs.clone(), l_x.clone())?;
            a_eqs = Tpl::softNewLine(a_eqs.clone())?;
            a_eqs = Tpl::popBlock(a_eqs.clone())?;
            a_eqs = Tpl::writeTok(a_eqs.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </exp:Sub>\n")).clone(), (literal!("</equ:Equation>")).clone()], lastHasNewLine: false }))?;
            a_eqs = Tpl::writeTok(a_eqs.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            (txt.clone(), a_varDecls.clone(), a_eqs.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_eqs))
}

pub(crate) fn old_equation_Xml(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_MIXED { index: _, .. }, a_context, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = equationSimpleAssignXml(txt.clone(), i_e.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { index: _, .. }, a_context, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = equationSimpleAssignXml(txt.clone(), i_e.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS { index: _, .. }, a_context, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = equationSimpleAssignXml(txt.clone(), i_e.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN { index: _, .. }, a_context, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = equationArrayCallAssignXml(txt.clone(), i_e.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_ALGORITHM { index: _, .. }, _, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: _, .. }, _, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" equations are not implemented yet")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem: _, .. }, _, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equations are not implemented yet ")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_WHEN { index: _, .. }, a_context, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = equationWhenXml(txt.clone(), i_e.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NOT IMPLEMENTED EQUATION")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

fn fun_142(mut in_txt: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_expPart: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_preExp.clone(), in_a_expPart.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_expPart) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_expPart.clone())?;
            txt.clone()
        },
        (txt, i_preExp, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), i_preExp.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_143(mut in_txt: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_expPart: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_preExp.clone(), in_a_expPart.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_expPart) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_expPart.clone())?;
            txt.clone()
        },
        (txt, i_preExp, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), i_preExp.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn equationSimpleAssignXml(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { exp: i_exp, cref: i_cref, .. }, a_context, a_varDecls) => {
            let mut l_result: Tpl::Text;
            let mut l_expPart: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_expPart, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            l_result = fun_142(Tpl::emptyTxt.clone(), l_preExp.clone(), l_expPart.clone())?;
            txt = crefXml(txt.clone(), i_cref.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_result.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS { exp: i_exp, cref: i_cref, .. }, a_context, a_varDecls) => {
            let mut l_result: Tpl::Text;
            let mut l_expPart: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_expPart, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            l_result = fun_143(Tpl::emptyTxt.clone(), l_preExp.clone(), l_expPart.clone())?;
            txt = crefXml(txt.clone(), i_cref.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_result.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

fn fun_145(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_eqn_exp: Arc<DAE::Exp>, mut in_a_lhs_componentRef: Arc<DAE::ComponentRef>, mut in_a_expPart: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_eqn_exp.clone(), in_a_lhs_componentRef.clone(), in_a_expPart.clone())) {
        (txt, Deref @ "boolean", _, a_lhs_componentRef, a_expPart) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_expPart.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = crefXml(txt.clone(), a_lhs_componentRef.clone())?;
            txt.clone()
        },
        (txt, Deref @ "integer", _, a_lhs_componentRef, a_expPart) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_expPart.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = crefXml(txt.clone(), a_lhs_componentRef.clone())?;
            txt.clone()
        },
        (txt, Deref @ "real", _, a_lhs_componentRef, a_expPart) => {
            let mut txt = (*txt).clone();
            txt = crefXml(txt.clone(), a_lhs_componentRef.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_expPart.clone())?;
            txt.clone()
        },
        (txt, _, a_eqn_exp, _, _) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("No runtime support for this sort of array call: ")).clone() }))?;
            txt_0 = ExpressionDumpTpl::dumpExp(txt_0.clone(), a_eqn_exp.clone(), (literal!("\"")).clone())?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 850, 14), (Tpl::textString(txt_0.clone())?).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn equationArrayCallAssignXml(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN { lhs: Deref @ DAE::Exp::CREF { componentRef: i_lhs_componentRef, .. }, exp: i_eqn_exp @ i_exp, .. }, a_context, a_varDecls) => {
            let mut str_3: ArcStr;
            let mut txt_2: Tpl::Text;
            let mut l_expPart: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_expPart, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt_2 = expTypeFromExpShortXml(Tpl::emptyTxt.clone(), i_eqn_exp.clone())?;
            str_3 = (Tpl::textString(txt_2.clone())?).clone();
            txt = fun_145(txt.clone(), (str_3.clone()).clone(), i_eqn_exp.clone(), i_lhs_componentRef.clone(), l_expPart.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

fn lm_147(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<(i32, i32, Arc<SimCode::SimEqSystem>)>>, mut a_varDecls: Tpl::Text, mut a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_147 in &*items.clone() {
        let mut lstElt_147 = lstElt_147.clone();
        (txt, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_147.clone()) {
        (_, _, Deref @ SimCode::SimEqSystem::SES_RESIDUAL { exp: i_eq_exp, .. }) => {
            let mut l_expPart: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            l_preExp = Tpl::emptyTxt.clone();
            (l_expPart, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_eq_exp.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeText(txt.clone(), l_preExp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_expPart.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_varDecls))
}

fn lm_148(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut a_varDecls: Tpl::Text, mut a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_148 in &*items.clone() {
        let mut lstElt_148 = lstElt_148.clone();
        (txt, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_148.clone()) {
        i_exp => {
            let mut l_expPart: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            l_preExp = Tpl::emptyTxt.clone();
            (l_expPart, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeText(txt.clone(), l_preExp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 3 }))?;
            txt = Tpl::writeText(txt.clone(), l_expPart.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_varDecls))
}

pub(crate) fn equationLinearXml(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: Deref @ SimCode::LinearSystem { simJac: i_ls_simJac, beqs: i_ls_beqs, .. }, .. }, a_context, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (txt, a_varDecls) = lm_147(txt.clone(), i_ls_simJac.clone(), a_varDecls.clone(), a_context.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (txt, a_varDecls) = lm_148(txt.clone(), i_ls_beqs.clone(), a_varDecls.clone(), a_context.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

fn lm_150(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_tmp: Tpl::Text, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_tmp: Tpl::Text = a_tmp;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_150 in &*items.clone() {
        let mut lstElt_150 = lstElt_150.clone();
        (txt, a_tmp, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_150.clone()) {
        i_eq2 => {
            (txt, a_varDecls, a_tmp) = functionExtraResidualsPreBody(txt.clone(), i_eq2.clone(), a_varDecls.clone(), a_tmp.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_tmp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_tmp, a_varDecls))
}

fn lm_151(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_151 in &*items.clone() {
        let mut lstElt_151 = lstElt_151.clone();
        (txt, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_151.clone()) {
        Deref @ SimCode::SimEqSystem::SES_RESIDUAL { exp: i_eq2_exp, .. } => {
            let mut l_expPart: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            l_preExp = Tpl::emptyTxt.clone();
            (l_expPart, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_eq2_exp.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeText(txt.clone(), l_preExp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_expPart.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_varDecls))
}

fn fun_152(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem: Deref @ SimCode::NonlinearSystem { eqs: i_nls_eqs, .. }, .. }) => {
            let mut l_body: Tpl::Text;
            let mut l_prebody: Tpl::Text;
            let mut l_tmp: Tpl::Text;
            let mut l_varDecls: Tpl::Text;
            let mut txt = (*txt).clone();
            l_varDecls = Tpl::emptyTxt.clone();
            l_tmp = Tpl::emptyTxt.clone();
            l_prebody = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_prebody, l_tmp, l_varDecls) = lm_150(l_prebody.clone(), i_nls_eqs.clone(), l_tmp.clone(), l_varDecls.clone())?;
            l_prebody = Tpl::popIter(l_prebody.clone())?;
            l_body = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_body, l_varDecls) = lm_151(l_body.clone(), i_nls_eqs.clone(), l_varDecls.clone())?;
            l_body = Tpl::popIter(l_body.clone())?;
            txt = Tpl::writeText(txt.clone(), l_tmp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_prebody.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_body.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn equationNonlinearXml(mut txt: Tpl::Text, mut a_eq: Arc<SimCode::SimEqSystem>, mut a_context: SimCodeFunction::Context, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    out_txt = fun_152(txt.clone(), a_eq.clone())?;
    out_a_varDecls = a_varDecls.clone();
    Ok((out_txt, out_a_varDecls))
}

pub(crate) fn functionExtraResidualsPreBody(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>, mut in_a_varDecls: Tpl::Text, mut in_a_eqs: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_eqs: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_eqs) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone(), in_a_varDecls.clone(), in_a_eqs.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_RESIDUAL { index: _, .. }, a_varDecls, a_eqs) => {
            (txt.clone(), a_varDecls.clone(), a_eqs.clone())
        },
        (txt, i_eq, a_varDecls, a_eqs) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_eqs = (*a_eqs).clone();
            (txt, a_varDecls, a_eqs) = equation_Xml(txt.clone(), i_eq.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), a_varDecls.clone(), a_eqs.clone())?;
            (txt.clone(), a_varDecls.clone(), a_eqs.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_eqs))
}

fn lm_155(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut a_helpInits: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_helpInits: Tpl::Text = a_helpInits;
    for mut lstElt_155 in &*items.clone() {
        let mut lstElt_155 = lstElt_155.clone();
        (txt, a_helpInits) = (::match_deref::match_deref! { match &(lstElt_155.clone()) {
        i_e => {
            let mut l_helpInit: Tpl::Text;
            l_helpInit = crefToXmlStr(Tpl::emptyTxt.clone(), i_e.clone())?;
            a_helpInits = Tpl::writeText(a_helpInits.clone(), l_helpInit.clone())?;
            a_helpInits = Tpl::writeTok(a_helpInits.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_helpInits.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_helpInits))
}

fn fun_156(mut in_txt: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_helpInits: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_preExp.clone(), in_a_helpInits.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_helpInits) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_helpInits.clone())?;
            txt.clone()
        },
        (txt, i_preExp, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), i_preExp.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_157(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut a_helpInits: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_helpInits: Tpl::Text = a_helpInits;
    for mut lstElt_157 in &*items.clone() {
        let mut lstElt_157 = lstElt_157.clone();
        (txt, a_helpInits) = (::match_deref::match_deref! { match &(lstElt_157.clone()) {
        i_e => {
            let mut l_helpInit: Tpl::Text;
            l_helpInit = crefToXmlStr(Tpl::emptyTxt.clone(), i_e.clone())?;
            a_helpInits = Tpl::writeText(a_helpInits.clone(), l_helpInit.clone())?;
            a_helpInits = Tpl::writeTok(a_helpInits.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_helpInits.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_helpInits))
}

fn fun_158(mut in_txt: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_helpInits: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_preExp.clone(), in_a_helpInits.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_helpInits) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_helpInits.clone())?;
            txt.clone()
        },
        (txt, i_preExp, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), i_preExp.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn equationWhenXml(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_WHEN { whenStmtLst: i_whenStmtLst, conditions: i_conditions, elseWhen: None, .. }, a_context, a_varDecls) => {
            let mut l_cond: Tpl::Text;
            let mut l_body: Tpl::Text;
            let mut l_helpIf: Tpl::Text;
            let mut l_helpInits: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            l_helpInits = Tpl::emptyTxt.clone();
            l_helpIf = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_helpIf, l_helpInits) = lm_155(l_helpIf.clone(), i_conditions.clone(), l_helpInits.clone())?;
            l_helpIf = Tpl::popIter(l_helpIf.clone())?;
            (l_body, a_varDecls) = whenOps(Tpl::emptyTxt.clone(), i_whenStmtLst.clone(), a_context.clone(), a_varDecls.clone())?;
            l_cond = fun_156(Tpl::emptyTxt.clone(), l_preExp.clone(), l_helpInits.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<equ:When>\n")).clone(), (literal!("  <equ:Condition>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_cond.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </equ:Condition>\n")).clone(), (literal!("  <equ:Equation>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_body.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </equ:Equation>\n")).clone(), (literal!("</equ:When>")).clone()], lastHasNewLine: false }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_WHEN { whenStmtLst: i_whenStmtLst, conditions: i_conditions, elseWhen: Some(i_elseWhenEq), .. }, a_context, a_varDecls) => {
            let mut l_elseWhen: Tpl::Text;
            let mut l_cond: Tpl::Text;
            let mut l_body: Tpl::Text;
            let mut l_helpIf: Tpl::Text;
            let mut l_helpInits: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            l_helpInits = Tpl::emptyTxt.clone();
            l_helpIf = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" || ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_helpIf, l_helpInits) = lm_157(l_helpIf.clone(), i_conditions.clone(), l_helpInits.clone())?;
            l_helpIf = Tpl::popIter(l_helpIf.clone())?;
            (l_body, a_varDecls) = whenOps(Tpl::emptyTxt.clone(), i_whenStmtLst.clone(), a_context.clone(), a_varDecls.clone())?;
            (l_elseWhen, l_preExp, l_helpInits, a_varDecls) = equationElseWhenXml(Tpl::emptyTxt.clone(), i_elseWhenEq.clone(), a_context.clone(), l_preExp.clone(), l_helpInits.clone(), a_varDecls.clone())?;
            l_cond = fun_158(Tpl::emptyTxt.clone(), l_preExp.clone(), l_helpInits.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<equ:When>\n")).clone(), (literal!("  <equ:Condition>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_cond.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </equ:Condition>\n")).clone(), (literal!("  <equ:Equation>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_body.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </equ:Equation>\n")).clone(), (literal!("</equ:When>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), l_elseWhen.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

fn lm_160(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut a_helpInits: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_helpInits: Tpl::Text = a_helpInits;
    for mut lstElt_160 in &*items.clone() {
        let mut lstElt_160 = lstElt_160.clone();
        (txt, a_helpInits) = (::match_deref::match_deref! { match &(lstElt_160.clone()) {
        i_e => {
            let mut l_helpInit: Tpl::Text;
            l_helpInit = crefToXmlStr(Tpl::emptyTxt.clone(), i_e.clone())?;
            a_helpInits = Tpl::writeText(a_helpInits.clone(), l_helpInit.clone())?;
            a_helpInits = Tpl::writeTok(a_helpInits.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_helpInits.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_helpInits))
}

fn fun_161(mut in_txt: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_helpInits: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_preExp.clone(), in_a_helpInits.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_helpInits) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_helpInits.clone())?;
            txt.clone()
        },
        (txt, i_preExp, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), i_preExp.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_162(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut a_helpInits: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_helpInits: Tpl::Text = a_helpInits;
    for mut lstElt_162 in &*items.clone() {
        let mut lstElt_162 = lstElt_162.clone();
        (txt, a_helpInits) = (::match_deref::match_deref! { match &(lstElt_162.clone()) {
        i_e => {
            let mut l_helpInit: Tpl::Text;
            l_helpInit = crefToXmlStr(Tpl::emptyTxt.clone(), i_e.clone())?;
            a_helpInits = Tpl::writeText(a_helpInits.clone(), l_helpInit.clone())?;
            a_helpInits = Tpl::writeTok(a_helpInits.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_helpInits.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_helpInits))
}

fn fun_163(mut in_txt: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_helpInits: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_preExp.clone(), in_a_helpInits.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_helpInits) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_helpInits.clone())?;
            txt.clone()
        },
        (txt, i_preExp, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), i_preExp.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn equationElseWhenXml(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_helpInits: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_helpInits: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_helpInits, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_helpInits.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_WHEN { whenStmtLst: i_whenStmtLst, conditions: i_conditions, elseWhen: None, .. }, a_context, a_preExp, a_helpInits, a_varDecls) => {
            let mut l_cond: Tpl::Text;
            let mut l_body: Tpl::Text;
            let mut l_helpIf: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_helpInits = (*a_helpInits).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_helpIf = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" || ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_helpIf, a_helpInits) = lm_160(l_helpIf.clone(), i_conditions.clone(), a_helpInits.clone())?;
            l_helpIf = Tpl::popIter(l_helpIf.clone())?;
            (l_body, a_varDecls) = whenOps(Tpl::emptyTxt.clone(), i_whenStmtLst.clone(), a_context.clone(), a_varDecls.clone())?;
            l_cond = fun_161(Tpl::emptyTxt.clone(), a_preExp.clone(), a_helpInits.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<equ:ElseWhen>\n")).clone(), (literal!("  <equ:Condition>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_cond.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </equ:Condition>\n")).clone(), (literal!("  <equ:Equation>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_body.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </equ:Equation>\n")).clone(), (literal!("</equ:ElseWhen>")).clone()], lastHasNewLine: false }))?;
            (txt.clone(), a_preExp.clone(), a_helpInits.clone(), a_varDecls.clone())
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_WHEN { whenStmtLst: i_whenStmtLst, conditions: i_conditions, elseWhen: Some(i_elseWhenEq), .. }, a_context, a_preExp, a_helpInits, a_varDecls) => {
            let mut l_elseWhen: Tpl::Text;
            let mut l_cond: Tpl::Text;
            let mut l_body: Tpl::Text;
            let mut l_helpIf: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_helpInits = (*a_helpInits).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_helpIf = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" || ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_helpIf, a_helpInits) = lm_162(l_helpIf.clone(), i_conditions.clone(), a_helpInits.clone())?;
            l_helpIf = Tpl::popIter(l_helpIf.clone())?;
            (l_body, a_varDecls) = whenOps(Tpl::emptyTxt.clone(), i_whenStmtLst.clone(), a_context.clone(), a_varDecls.clone())?;
            (l_elseWhen, a_preExp, a_helpInits, a_varDecls) = equationElseWhenXml(Tpl::emptyTxt.clone(), i_elseWhenEq.clone(), a_context.clone(), a_preExp.clone(), a_helpInits.clone(), a_varDecls.clone())?;
            l_cond = fun_163(Tpl::emptyTxt.clone(), a_preExp.clone(), a_helpInits.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<equ:ElseWhen>\n")).clone(), (literal!("  <equ:Condition>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_cond.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </equ:Condition>\n")).clone(), (literal!("  <equ:Equation>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_body.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </equ:Equation>\n")).clone(), (literal!("</equ:ElseWhen>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), l_elseWhen.clone())?;
            (txt.clone(), a_preExp.clone(), a_helpInits.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_helpInits, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_helpInits.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_helpInits, out_a_varDecls))
}

fn fun_165(mut in_txt: Tpl::Text, mut in_a_whenOp: BackendDAE::WhenOperator, mut in_a_varDecls: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_whenOp.clone(), in_a_varDecls.clone(), in_a_context.clone())) {
        (txt, BackendDAE::WhenOperator::ASSIGN { left: Deref @ DAE::Exp::CREF { componentRef: i_cr, .. }, right: i_right, .. }, a_varDecls, a_context) => {
            let mut l_exp: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_exp, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_right.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  <exp:Sub>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = crefXml(txt.clone(), i_cr.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_exp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("  </exp:Sub>")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, BackendDAE::WhenOperator::REINIT { value: i_value, stateVar: i_stateVar, .. }, a_varDecls, _) => {
            let mut l_val: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_val, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_value.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Reinit>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = crefXml(txt.clone(), i_stateVar.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_val.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Reinit>")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, BackendDAE::WhenOperator::TERMINATE { message: i_message, .. }, a_varDecls, _) => {
            let mut l_msgVar: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_msgVar, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_message.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeText(txt.clone(), l_preExp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_msgVar.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, BackendDAE::WhenOperator::ASSERT { source: Deref @ DAE::ElementSource { info: i_info, .. }, condition: i_condition, message: i_message, .. }, a_varDecls, _) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = assertCommonXml(txt.clone(), i_condition.clone(), i_message.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), a_varDecls.clone(), i_info.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, a_varDecls, _) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

fn lm_166(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<BackendDAE::WhenOperator>>, mut a_varDecls: Tpl::Text, mut a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_166 in &*items.clone() {
        let mut lstElt_166 = lstElt_166.clone();
        (txt, a_varDecls) = (match lstElt_166.clone() {
        mut i_whenOp => {
            (txt, a_varDecls) = fun_165(txt.clone(), i_whenOp.clone(), a_varDecls.clone(), a_context.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
    });
    }
    Ok((txt, a_varDecls))
}

pub(crate) fn whenOps(mut txt: Tpl::Text, mut a_whenOps: Arc<metamodelica::List<BackendDAE::WhenOperator>>, mut a_context: SimCodeFunction::Context, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut l_body: Tpl::Text;
    l_body = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    (l_body, out_a_varDecls) = lm_166(l_body.clone(), a_whenOps.clone(), a_varDecls.clone(), a_context.clone())?;
    l_body = Tpl::popIter(l_body.clone())?;
    out_txt = Tpl::writeText(txt.clone(), l_body.clone())?;
    Ok((out_txt, out_a_varDecls))
}

fn lm_168(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_168 in &*items.clone() {
        let mut lstElt_168 = lstElt_168.clone();
        txt = (match lstElt_168.clone() {
        mut i_rd => {
            txt = recordDeclarationXml(txt.clone(), i_rd.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

pub(crate) fn recordsXml(mut txt: Tpl::Text, mut a_recordDecls: Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:RecordsList>\n")).clone() }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
    out_txt = Tpl::pushIter(out_txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    out_txt = lm_168(out_txt.clone(), a_recordDecls.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:RecordsList>")).clone() }))?;
    Ok(out_txt)
}

fn lm_170(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_170 in &*items.clone() {
        let mut lstElt_170 = lstElt_170.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_170.clone()) {
        i_var => {
            txt = recordBodyXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub(crate) fn recordDeclarationXml(mut in_txt: Tpl::Text, mut in_a_recDecl: SimCodeFunction::RecordDeclaration) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_recDecl.clone()) {
        (mut txt, SimCodeFunction::RecordDeclaration::RECORD_DECL_FULL { name: mut i_name, variables: ref i_variables, .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:Record>\n")).clone(), (literal!("  <fun:Name>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart  name ='")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("'/>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  </fun:Name>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_170(txt.clone(), i_variables.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:Record>")).clone() }))?;
            txt.clone()
        },
        (mut txt, SimCodeFunction::RecordDeclaration::RECORD_DECL_DEF { path: _, .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("  Record Declaration definition is not yet implemented")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn recordBodyXml(mut in_txt: Tpl::Text, mut in_a_var: Arc<SimCodeFunction::Variable::Variable>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_var.clone())) {
        (txt, i_var @ Deref @ SimCodeFunction::Variable::VARIABLE { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, name: i_name, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:Field  type=\"Record\">\n")).clone(), (literal!("  <fun:Name>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = contextCrefXml(txt.clone(), i_name.clone(), SimCodeFunction::contextFunction().clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\"/>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Name>\n")).clone(), (literal!("  <fun:Record>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = varTypeXml(txt.clone(), i_var.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Record>\n")).clone(), (literal!("</fun:Field>")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, i_var @ Deref @ SimCodeFunction::Variable::VARIABLE { name: i_var_name, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<fun:Field  type=\"")).clone() }))?;
            txt = varTypeXml(txt.clone(), i_var.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\">\n")).clone(), (literal!("  <fun:Name>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = crefStrXml(txt.clone(), i_var_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\"/>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Name>\n")).clone(), (literal!("</fun:Field>")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, Deref @ SimCodeFunction::Variable::FUNCTION_PTR { name: i_name_1, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_fnptr ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name_1.clone()).clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_173(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_173 in &*items.clone() {
        let mut lstElt_173 = lstElt_173.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_173.clone()) {
        i_fn => {
            txt = functionXml(txt.clone(), i_fn.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub(crate) fn functionsXml(mut txt: Tpl::Text, mut a_functions: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:FunctionsList>\n")).clone() }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
    out_txt = Tpl::pushIter(out_txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    out_txt = lm_173(out_txt.clone(), a_functions.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:FunctionsList>")).clone() }))?;
    Ok(out_txt)
}

pub(crate) fn functionXml(mut in_txt: Tpl::Text, mut in_a_fn: Arc<SimCodeFunction::Function::Function>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fn.clone())) {
        (txt, i_fn @ Deref @ SimCodeFunction::Function::FUNCTION { name: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = regularFunctionXml(txt.clone(), i_fn.clone())?;
            txt.clone()
        },
        (txt, i_fn @ Deref @ SimCodeFunction::Function::EXTERNAL_FUNCTION { name: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = externalFunctionXml(txt.clone(), i_fn.clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCodeFunction::Function::RECORD_CONSTRUCTOR { name: _, .. }) => {
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_176(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_176 in &*items.clone() {
        let mut lstElt_176 = lstElt_176.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_176.clone()) {
        i_var => {
            txt = funOutputVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_177(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_177 in &*items.clone() {
        let mut lstElt_177 = lstElt_177.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_177.clone()) {
        i_var => {
            txt = funArgDefinitionXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub(crate) fn regularFunctionXml(mut in_txt: Tpl::Text, mut in_a_fn: Arc<SimCodeFunction::Function::Function>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fn.clone())) {
        (txt, Deref @ SimCodeFunction::Function::FUNCTION { name: i_name, body: i_body, outVars: i_outVars, functionArguments: i_functionArguments, .. }) => {
            let mut l_bodyPart: Tpl::Text;
            let mut l_varInits: Tpl::Text;
            let mut l_varDecls: Tpl::Text;
            let mut l_fname: Tpl::Text;
            let mut txt = (*txt).clone();
            System::tmpTickReset(1);
            l_fname = underscorePathXml(Tpl::emptyTxt.clone(), i_name.clone())?;
            l_varDecls = Tpl::emptyTxt.clone();
            l_varInits = Tpl::emptyTxt.clone();
            (l_bodyPart, l_varDecls) = funStatementXml(Tpl::emptyTxt.clone(), i_body.clone(), l_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:Function>\n")).clone(), (literal!("  <fun:Name>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_fname.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  </fun:Name>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_176(txt.clone(), i_outVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_177(txt.clone(), i_functionArguments.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Algorithm>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_bodyPart.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</fun:Algorithm>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:Function>")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_179(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_179 in &*items.clone() {
        let mut lstElt_179 = lstElt_179.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_179.clone()) {
        i_var => {
            txt = funOutputVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_180(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_180 in &*items.clone() {
        let mut lstElt_180 = lstElt_180.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_180.clone()) {
        i_var => {
            txt = funArgDefinitionXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub(crate) fn externalFunctionXml(mut in_txt: Tpl::Text, mut in_a_fn: Arc<SimCodeFunction::Function::Function>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fn.clone())) {
        (txt, i_efn @ Deref @ SimCodeFunction::Function::EXTERNAL_FUNCTION { name: i_name, outVars: i_outVars, funArgs: i_funArgs, .. }) => {
            let mut l_callPart: Tpl::Text;
            let mut l_fname: Tpl::Text;
            let mut l_varDecls: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            System::tmpTickReset(1);
            l_preExp = Tpl::emptyTxt.clone();
            l_varDecls = Tpl::emptyTxt.clone();
            l_fname = underscorePathXml(Tpl::emptyTxt.clone(), i_name.clone())?;
            (l_callPart, l_preExp, l_varDecls) = extFunCallXml(Tpl::emptyTxt.clone(), i_efn.clone(), l_preExp.clone(), l_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:Function>\n")).clone(), (literal!("  <fun:Name>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_fname.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  </fun:Name>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_179(txt.clone(), i_outVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_180(txt.clone(), i_funArgs.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Algorithm>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_callPart.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</fun:Algorithm>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:Function>")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn funArgNameXml(mut in_txt: Tpl::Text, mut in_a_var: Arc<SimCodeFunction::Variable::Variable>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_var.clone())) {
        (txt, Deref @ SimCodeFunction::Variable::VARIABLE { name: i_name, .. }) => {
            let mut txt = (*txt).clone();
            txt = contextCrefXml(txt.clone(), i_name.clone(), SimCodeFunction::contextFunction().clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCodeFunction::Variable::FUNCTION_PTR { name: i_name_1, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name_1.clone()).clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn funOutputVariableXml(mut in_txt: Tpl::Text, mut in_a_var: Arc<SimCodeFunction::Variable::Variable>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_var.clone())) {
        (txt, i_var @ Deref @ SimCodeFunction::Variable::VARIABLE { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, name: i_name, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:OutputVariable type=\"Record\">\n")).clone(), (literal!("  <fun:Name>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = contextCrefXml(txt.clone(), i_name.clone(), SimCodeFunction::contextFunction().clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\"/>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Name>\n")).clone(), (literal!("  <fun:Record>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = varTypeXml(txt.clone(), i_var.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Record>\n")).clone(), (literal!("</fun:OutputVariable>")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, i_var @ Deref @ SimCodeFunction::Variable::VARIABLE { name: i_name, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<fun:OutputVariable type=\"")).clone() }))?;
            txt = varTypeXml(txt.clone(), i_var.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\">\n")).clone(), (literal!("  <fun:Name>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = contextCrefXml(txt.clone(), i_name.clone(), SimCodeFunction::contextFunction().clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\"/>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Name>\n")).clone(), (literal!("</fun:OutputVariable>")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, Deref @ SimCodeFunction::Variable::FUNCTION_PTR { name: i_name_1, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name_1.clone()).clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn funArgDefinitionXml(mut in_txt: Tpl::Text, mut in_a_var: Arc<SimCodeFunction::Variable::Variable>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_var.clone())) {
        (txt, i_var @ Deref @ SimCodeFunction::Variable::VARIABLE { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, name: i_name, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:InputVariable type=\"Record\">\n")).clone(), (literal!("  <fun:Name>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = contextCrefXml(txt.clone(), i_name.clone(), SimCodeFunction::contextFunction().clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\"/>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Name>\n")).clone(), (literal!("  <fun:Record>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = varTypeXml(txt.clone(), i_var.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Record>\n")).clone(), (literal!("</fun:InputVariable>")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, i_var @ Deref @ SimCodeFunction::Variable::VARIABLE { name: i_name, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<fun:InputVariable type=\"")).clone() }))?;
            txt = varTypeXml(txt.clone(), i_var.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\">\n")).clone(), (literal!("  <fun:Name>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = contextCrefXml(txt.clone(), i_name.clone(), SimCodeFunction::contextFunction().clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\"/>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Name>\n")).clone(), (literal!("</fun:InputVariable>")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, Deref @ SimCodeFunction::Variable::FUNCTION_PTR { name: i_name_1, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_fnptr ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name_1.clone()).clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn funVarDeclarationsXml(mut in_txt: Tpl::Text, mut in_a_var: Arc<SimCodeFunction::Variable::Variable>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_var.clone())) {
        (txt, i_var @ Deref @ SimCodeFunction::Variable::VARIABLE { name: i_name, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<fun:protectedVariable type=\"")).clone() }))?;
            txt = varTypeXml(txt.clone(), i_var.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\">\n")).clone(), (literal!("  <fun:Name>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = contextCrefXml(txt.clone(), i_name.clone(), SimCodeFunction::contextFunction().clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\"/>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Name>\n")).clone(), (literal!("</fun:ProtectedVariable>")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, Deref @ SimCodeFunction::Variable::FUNCTION_PTR { name: i_name_1, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_fnptr ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name_1.clone()).clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_186(mut in_txt: Tpl::Text, mut in_a_language: ArcStr, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_language.clone(), in_a_name.clone())) {
        (txt, Deref @ "C", a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedName name=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"/>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "FORTRAN 77", a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedName name=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"/>")).clone() }))?;
            txt.clone()
        },
        (txt, i_language, _) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Unsupport external language: ")).clone() }))?;
            txt_0 = Tpl::writeStr(txt_0.clone(), (i_language.clone()).clone())?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 1250, 14), (Tpl::textString(txt_0.clone())?).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn extFunctionNameXml(mut txt: Tpl::Text, mut a_name: ArcStr, mut a_language: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_186(txt.clone(), (a_language.clone()).clone(), (a_name.clone()).clone())?;
    Ok(out_txt)
}

fn fun_188(mut in_txt: Tpl::Text, mut in_a_type: Arc<DAE::Type>, mut in_a_isInput: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_type.clone(), in_a_isInput.clone())) {
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("int")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("double")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("const char*")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("int")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ENUMERATION { index: _, .. }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("int")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ARRAY { ty: i_ty, .. }, a_isInput) => {
            let mut txt = (*txt).clone();
            txt = extTypeXml(txt.clone(), i_ty.clone(), a_isInput.clone(), true)?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { path: _ }, .. }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void *")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: i_rname }, .. }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("struct ")).clone() }))?;
            txt = underscorePathXml(txt.clone(), i_rname.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METATYPE { ty: _ }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METABOXED { ty: _ }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
        (txt, i_type, _) => {
            let mut txt_0: Tpl::Text;
            let mut ret_0: ArcStr;
            let mut txt = (*txt).clone();
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Unknown external C type ")).clone() }))?;
            ret_0 = (TypesDump::unparseType(i_type.clone())?).clone();
            txt_0 = Tpl::writeStr(txt_0.clone(), (ret_0.clone()).clone())?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 1268, 14), (Tpl::textString(txt_0.clone())?).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_189(mut in_txt: Tpl::Text, mut in_mArg: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone())) {
        (txt, Deref @ "const char*") => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("const ")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_190(mut in_txt: Tpl::Text, mut in_a_isArray: bool, mut in_a_s: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_isArray.clone(), in_a_s.clone()) {
        (mut txt, false, mut a_s) => {
            txt = Tpl::writeText(txt.clone(), a_s.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_s) => {
            let mut str_0: ArcStr;
            str_0 = (Tpl::textString(a_s.clone())?).clone();
            txt = fun_189(txt.clone(), (str_0.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), a_s.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_191(mut in_txt: Tpl::Text, mut in_a_isInput: bool, mut in_a_isArray: bool, mut in_a_s: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_isInput.clone(), in_a_isArray.clone(), in_a_s.clone()) {
        (mut txt, false, _, mut a_s) => {
            txt = Tpl::writeText(txt.clone(), a_s.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, mut a_isArray, mut a_s) => {
            txt = fun_190(txt.clone(), a_isArray.clone(), a_s.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_192(mut in_txt: Tpl::Text, mut in_a_type: Arc<DAE::Type>, mut in_a_isArray: bool, mut in_a_isInput: bool, mut in_a_s: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_type.clone(), in_a_isArray.clone(), in_a_isInput.clone(), in_a_s.clone())) {
        (txt, Deref @ DAE::Type::T_ARRAY { ty: _, .. }, _, _, a_s) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_s.clone())?;
            txt.clone()
        },
        (txt, _, a_isArray, a_isInput, a_s) => {
            let mut txt = (*txt).clone();
            txt = fun_191(txt.clone(), a_isInput.clone(), a_isArray.clone(), a_s.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn extTypeXml(mut txt: Tpl::Text, mut a_type: Arc<DAE::Type>, mut a_isInput: bool, mut a_isArray: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_s: Tpl::Text;
    l_s = fun_188(Tpl::emptyTxt.clone(), a_type.clone(), a_isInput.clone())?;
    out_txt = fun_192(txt.clone(), a_type.clone(), a_isArray.clone(), a_isInput.clone(), l_s.clone())?;
    Ok(out_txt)
}

fn fun_194(mut in_txt: Tpl::Text, mut in_a_type: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_type.clone())) {
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("int")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("double")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("char")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("int")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ENUMERATION { index: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("int")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ARRAY { ty: i_ty, .. }) => {
            let mut txt = (*txt).clone();
            txt = extTypeF77Xml(txt.clone(), i_ty.clone(), true)?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { path: _ }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void*")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: i_rname }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("struct ")).clone() }))?;
            txt = underscorePathXml(txt.clone(), i_rname.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METATYPE { ty: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void*")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METABOXED { ty: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void*")).clone() }))?;
            txt.clone()
        },
        (txt, i_type) => {
            let mut txt_0: Tpl::Text;
            let mut ret_0: ArcStr;
            let mut txt = (*txt).clone();
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Unknown external F77 type ")).clone() }))?;
            ret_0 = (TypesDump::unparseType(i_type.clone())?).clone();
            txt_0 = Tpl::writeStr(txt_0.clone(), (ret_0.clone()).clone())?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 1287, 14), (Tpl::textString(txt_0.clone())?).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_195(mut in_txt: Tpl::Text, mut in_a_isReference: bool, mut in_a_s: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_isReference.clone(), in_a_s.clone()) {
        (mut txt, false, mut a_s) => {
            txt = Tpl::writeText(txt.clone(), a_s.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_s) => {
            txt = Tpl::writeText(txt.clone(), a_s.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_196(mut in_txt: Tpl::Text, mut in_a_type: Arc<DAE::Type>, mut in_a_isReference: bool, mut in_a_s: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_type.clone(), in_a_isReference.clone(), in_a_s.clone())) {
        (txt, Deref @ DAE::Type::T_ARRAY { ty: _, .. }, _, a_s) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_s.clone())?;
            txt.clone()
        },
        (txt, _, a_isReference, a_s) => {
            let mut txt = (*txt).clone();
            txt = fun_195(txt.clone(), a_isReference.clone(), a_s.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn extTypeF77Xml(mut txt: Tpl::Text, mut a_type: Arc<DAE::Type>, mut a_isReference: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_s: Tpl::Text;
    l_s = fun_194(Tpl::emptyTxt.clone(), a_type.clone())?;
    out_txt = fun_196(txt.clone(), a_type.clone(), a_isReference.clone(), l_s.clone())?;
    Ok(out_txt)
}

fn fun_198(mut in_txt: Tpl::Text, mut in_a_dotPath: bool, mut in_a_name: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_dotPath.clone(), in_a_name.clone())) {
        (txt, false, a_name) => {
            let mut txt = (*txt).clone();
            txt = underscorePathXml(txt.clone(), a_name.clone())?;
            txt.clone()
        },
        (txt, _, a_name) => {
            let mut txt = (*txt).clone();
            txt = dotPathXml(txt.clone(), a_name.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_199(mut in_txt: Tpl::Text, mut in_a_dotPath: bool, mut in_a_name: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_dotPath.clone(), in_a_name.clone())) {
        (txt, false, a_name) => {
            let mut txt = (*txt).clone();
            txt = underscorePathXml(txt.clone(), a_name.clone())?;
            txt.clone()
        },
        (txt, _, a_name) => {
            let mut txt = (*txt).clone();
            txt = dotPathXml(txt.clone(), a_name.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_200(mut in_txt: Tpl::Text, mut in_a_dotPath: bool, mut in_a_name: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_dotPath.clone(), in_a_name.clone())) {
        (txt, false, a_name) => {
            let mut txt = (*txt).clone();
            txt = underscorePathXml(txt.clone(), a_name.clone())?;
            txt.clone()
        },
        (txt, _, a_name) => {
            let mut txt = (*txt).clone();
            txt = dotPathXml(txt.clone(), a_name.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn functionNameXml(mut in_txt: Tpl::Text, mut in_a_fn: Arc<SimCodeFunction::Function::Function>, mut in_a_dotPath: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fn.clone(), in_a_dotPath.clone())) {
        (txt, Deref @ SimCodeFunction::Function::FUNCTION { name: i_name, .. }, a_dotPath) => {
            let mut txt = (*txt).clone();
            txt = fun_198(txt.clone(), a_dotPath.clone(), i_name.clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCodeFunction::Function::EXTERNAL_FUNCTION { name: i_name, .. }, a_dotPath) => {
            let mut txt = (*txt).clone();
            txt = fun_199(txt.clone(), a_dotPath.clone(), i_name.clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCodeFunction::Function::RECORD_CONSTRUCTOR { name: i_name, .. }, a_dotPath) => {
            let mut txt = (*txt).clone();
            txt = fun_200(txt.clone(), a_dotPath.clone(), i_name.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn extVarNameXml(mut txt: Tpl::Text, mut a_cr: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = crefXml(txt.clone(), a_cr.clone())?;
    Ok(out_txt)
}

fn fun_203(mut in_txt: Tpl::Text, mut in_a_language: ArcStr, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_fun: Arc<SimCodeFunction::Function::Function>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_language.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_fun.clone())) {
        (txt, Deref @ "C", a_varDecls, a_preExp, a_fun) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_preExp, a_varDecls) = extFunCallCXml(txt.clone(), a_fun.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, Deref @ "FORTRAN 77", a_varDecls, a_preExp, a_fun) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_preExp, a_varDecls) = extFunCallF77Xml(txt.clone(), a_fun.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_varDecls, a_preExp, _) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

pub(crate) fn extFunCallXml(mut in_txt: Tpl::Text, mut in_a_fun: Arc<SimCodeFunction::Function::Function>, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fun.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, i_fun @ Deref @ SimCodeFunction::Function::EXTERNAL_FUNCTION { language: i_language, .. }, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls, a_preExp) = fun_203(txt.clone(), (i_language.clone()).clone(), a_varDecls.clone(), a_preExp.clone(), i_fun.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn lm_205(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCodeFunction::SimExtArg::SimExtArg>>>, mut a_varDecls: Tpl::Text, mut a_preExp: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    let mut a_preExp: Tpl::Text = a_preExp;
    for mut lstElt_205 in &*items.clone() {
        let mut lstElt_205 = lstElt_205.clone();
        (txt, a_varDecls, a_preExp) = (::match_deref::match_deref! { match &(lstElt_205.clone()) {
        i_arg => {
            (txt, a_preExp, a_varDecls) = extArgCXml(txt.clone(), i_arg.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_varDecls, a_preExp))
}

fn fun_206(mut in_txt: Tpl::Text, mut in_a_extReturn: Arc<SimCodeFunction::SimExtArg::SimExtArg>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_extReturn.clone())) {
        (txt, Deref @ SimCodeFunction::SimExtArg::SIMEXTARG { cref: i_c, .. }) => {
            let mut txt = (*txt).clone();
            txt = extVarNameXml(txt.clone(), i_c.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn extFunCallCXml(mut in_txt: Tpl::Text, mut in_a_fun: Arc<SimCodeFunction::Function::Function>, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fun.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ SimCodeFunction::Function::EXTERNAL_FUNCTION { extArgs: i_extArgs, extReturn: i_extReturn, extName: i_extName, .. }, a_preExp, a_varDecls) => {
            let mut l_returnAssign: Tpl::Text;
            let mut l_args: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_args = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!(" ")).clone()], lastHasNewLine: false })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_args, a_varDecls, a_preExp) = lm_205(l_args.clone(), i_extArgs.clone(), a_varDecls.clone(), a_preExp.clone())?;
            l_args = Tpl::popIter(l_args.clone())?;
            l_returnAssign = fun_206(Tpl::emptyTxt.clone(), i_extReturn.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Assign>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_returnAssign.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:Expression>\n")).clone(), (literal!("  <exp:FunctionCall>\n")).clone(), (literal!("    <exp:Name>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_extName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\" />\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("    </exp:Name>\n")).clone(), (literal!("    <exp:Arguments>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            txt = Tpl::writeText(txt.clone(), l_args.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("    </exp:Arguments>\n")).clone(), (literal!("  </exp:FunctionCall>\n")).clone(), (literal!("</fun:Expression>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:Assign>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn lm_208(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCodeFunction::SimExtArg::SimExtArg>>>, mut a_varDecls: Tpl::Text, mut a_preExp: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    let mut a_preExp: Tpl::Text = a_preExp;
    for mut lstElt_208 in &*items.clone() {
        let mut lstElt_208 = lstElt_208.clone();
        (txt, a_varDecls, a_preExp) = (::match_deref::match_deref! { match &(lstElt_208.clone()) {
        i_arg => {
            (txt, a_preExp, a_varDecls) = extArgF77Xml(txt.clone(), i_arg.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_varDecls, a_preExp))
}

fn fun_209(mut in_txt: Tpl::Text, mut in_a_extReturn: Arc<SimCodeFunction::SimExtArg::SimExtArg>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_extReturn.clone())) {
        (txt, Deref @ SimCodeFunction::SimExtArg::SIMEXTARG { cref: i_c, .. }) => {
            let mut txt = (*txt).clone();
            txt = extVarNameXml(txt.clone(), i_c.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn extFunCallF77Xml(mut in_txt: Tpl::Text, mut in_a_fun: Arc<SimCodeFunction::Function::Function>, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fun.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ SimCodeFunction::Function::EXTERNAL_FUNCTION { extArgs: i_extArgs, extReturn: i_extReturn, extName: i_extName, .. }, a_preExp, a_varDecls) => {
            let mut l_returnAssign: Tpl::Text;
            let mut l_args: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_args = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_args, a_varDecls, a_preExp) = lm_208(l_args.clone(), i_extArgs.clone(), a_varDecls.clone(), a_preExp.clone())?;
            l_args = Tpl::popIter(l_args.clone())?;
            l_returnAssign = fun_209(Tpl::emptyTxt.clone(), i_extReturn.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Assign>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_returnAssign.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:Expression>\n")).clone(), (literal!("  <exp:FunctionCall>\n")).clone(), (literal!("    <exp:Name>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_extName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\" />\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("    </exp:Name>\n")).clone(), (literal!("    <exp:Arguments>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            txt = Tpl::writeText(txt.clone(), l_args.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("    </exp:Arguments>\n")).clone(), (literal!("  </exp:FunctionCall>\n")).clone(), (literal!("</fun:Expression>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:Assign>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub(crate) fn extArgCXml(mut in_txt: Tpl::Text, mut in_a_extArg: Arc<SimCodeFunction::SimExtArg::SimExtArg>, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_extArg.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ SimCodeFunction::SimExtArg::SIMEXTARG { cref: i_c, outputIndex: _, isArray: true, type_: _, .. }, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = extVarNameXml(txt.clone(), i_c.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ SimCodeFunction::SimExtArg::SIMEXTARG { cref: i_c, isInput: _, outputIndex: 0, type_: _, .. }, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = extVarNameXml(txt.clone(), i_c.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ SimCodeFunction::SimExtArg::SIMEXTARG { cref: i_c, isInput: _, outputIndex: _, type_: _, .. }, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = extVarNameXml(txt.clone(), i_c.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ SimCodeFunction::SimExtArg::SIMEXTARGEXP { exp: i_exp, .. }, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExternalXmlExp(txt.clone(), i_exp.clone(), SimCodeFunction::contextFunction().clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("test daeexternal xml")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ SimCodeFunction::SimExtArg::SIMEXTARGSIZE { cref: i_c, exp: i_exp, .. }, a_preExp, a_varDecls) => {
            let mut l_dim: Tpl::Text;
            let mut l_name: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_name = extVarNameXml(Tpl::emptyTxt.clone(), i_c.clone())?;
            (l_dim, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), SimCodeFunction::contextFunction().clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Size>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_name.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_dim.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Size>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub(crate) fn extArgF77Xml(mut in_txt: Tpl::Text, mut in_a_extArg: Arc<SimCodeFunction::SimExtArg::SimExtArg>, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_extArg.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ SimCodeFunction::SimExtArg::SIMEXTARG { cref: i_c, isArray: true, type_: _, .. }, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = extVarNameXml(txt.clone(), i_c.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ SimCodeFunction::SimExtArg::SIMEXTARG { cref: i_c, outputIndex: _, type_: Deref @ DAE::Type::T_INTEGER { varLst: _ }, .. }, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = extVarNameXml(txt.clone(), i_c.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ SimCodeFunction::SimExtArg::SIMEXTARG { cref: i_c, outputIndex: _, type_: Deref @ DAE::Type::T_STRING { varLst: _ }, .. }, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = extVarNameXml(txt.clone(), i_c.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ SimCodeFunction::SimExtArg::SIMEXTARG { cref: i_c, outputIndex: _, type_: _, .. }, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = extVarNameXml(txt.clone(), i_c.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ SimCodeFunction::SimExtArg::SIMEXTARGEXP { exp: i_exp, type_: Deref @ DAE::Type::T_STRING { varLst: _ } }, a_preExp, a_varDecls) => {
            let mut l_texp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_texp, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), SimCodeFunction::contextFunction().clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeText(txt.clone(), l_texp.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ SimCodeFunction::SimExtArg::SIMEXTARGSIZE { cref: i_c, exp: i_exp, .. }, a_preExp, a_varDecls) => {
            let mut l_name: Tpl::Text;
            let mut l_dim: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_dim, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), SimCodeFunction::contextFunction().clone(), a_preExp.clone(), a_varDecls.clone())?;
            l_name = extVarNameXml(Tpl::emptyTxt.clone(), i_c.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Size>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_name.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_dim.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Size>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn lm_213(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::ClassAttributes>>>, mut a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_213 in &*items.clone() {
        let mut lstElt_213 = lstElt_213.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_213.clone()) {
        i_classAttribute => {
            txt = classAttributesXml(txt.clone(), i_classAttribute.clone(), a_simCode.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub(crate) fn objectiveFunctionXml(mut txt: Tpl::Text, mut a_classAttributes: Arc<metamodelica::List<Arc<DAE::ClassAttributes>>>, mut a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    out_txt = lm_213(out_txt.clone(), a_classAttributes.clone(), a_simCode.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

fn fun_215(mut in_txt: Tpl::Text, mut in_a_objetiveE: Option<Arc<DAE::Exp>>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_objetiveE.clone(), in_a_varDecls.clone(), in_a_preExp.clone())) {
        (txt, Some(i_exp), a_varDecls, a_preExp) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<opt:ObjectiveFunction>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            (txt, a_preExp, a_varDecls) = daeExpXml(txt.clone(), i_exp.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</opt:ObjectiveFunction>")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_varDecls, a_preExp) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_216(mut in_txt: Tpl::Text, mut in_a_objectiveIntegrandE: Option<Arc<DAE::Exp>>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_objectiveIntegrandE.clone(), in_a_varDecls.clone(), in_a_preExp.clone())) {
        (txt, Some(i_exp), a_varDecls, a_preExp) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<opt:IntegrandObjectiveFunction>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            (txt, a_preExp, a_varDecls) = daeExpXml(txt.clone(), i_exp.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</opt:IntegrandObjectiveFunction>")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_varDecls, a_preExp) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_217(mut in_txt: Tpl::Text, mut in_a_startTimeE: Option<Arc<DAE::Exp>>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_startTimeE.clone(), in_a_varDecls.clone(), in_a_preExp.clone())) {
        (txt, Some(i_exp), a_varDecls, a_preExp) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<opt:IntervalStartTime>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<opt:Value>")).clone() }))?;
            (txt, a_preExp, a_varDecls) = daeExpValueXml(txt.clone(), i_exp.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</opt:Value>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</opt:IntervalStartTime>")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_varDecls, a_preExp) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_218(mut in_txt: Tpl::Text, mut in_a_finalTimeE: Option<Arc<DAE::Exp>>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_finalTimeE.clone(), in_a_varDecls.clone(), in_a_preExp.clone())) {
        (txt, Some(i_exp), a_varDecls, a_preExp) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<opt:IntervalFinalTime>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<opt:Value>")).clone() }))?;
            (txt, a_preExp, a_varDecls) = daeExpValueXml(txt.clone(), i_exp.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</opt:Value>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</opt:IntervalFinalTime>")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_varDecls, a_preExp) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_219(mut in_txt: Tpl::Text, mut in_a_startTimeE: Option<Arc<DAE::Exp>>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_startTimeE.clone(), in_a_varDecls.clone(), in_a_preExp.clone())) {
        (txt, Some(i_exp), a_varDecls, a_preExp) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("index = \"")).clone() }))?;
            (txt, a_preExp, a_varDecls) = daeExpValueXml(txt.clone(), i_exp.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_varDecls, a_preExp) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_220(mut in_txt: Tpl::Text, mut in_a_finalTimeE: Option<Arc<DAE::Exp>>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_finalTimeE.clone(), in_a_varDecls.clone(), in_a_preExp.clone())) {
        (txt, Some(i_exp), a_varDecls, a_preExp) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("value = \"")).clone() }))?;
            (txt, a_preExp, a_varDecls) = daeExpValueXml(txt.clone(), i_exp.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_varDecls, a_preExp) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_221(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::SimCode { modelInfo: SimCode::ModelInfo { name: _, .. }, constraints: ref i_constraints, .. }) => {
            txt = constraintsXml(txt.clone(), i_constraints.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn classAttributesXml(mut in_txt: Tpl::Text, mut in_a_classAttribute: Arc<DAE::ClassAttributes>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_classAttribute.clone(), in_a_simCode.clone())) {
        (txt, Deref @ DAE::ClassAttributes { objetiveE: i_objetiveE, objectiveIntegrandE: i_objectiveIntegrandE, startTimeE: i_startTimeE, finalTimeE: i_finalTimeE }, a_simCode) => {
            let mut l_constraints: Tpl::Text;
            let mut l_timePointValue: Tpl::Text;
            let mut l_timePointIndex: Tpl::Text;
            let mut l_finalTime: Tpl::Text;
            let mut l_startTime: Tpl::Text;
            let mut l_objectiveIntegrand: Tpl::Text;
            let mut l_objectiveFunction: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut l_varDecls: Tpl::Text;
            let mut txt = (*txt).clone();
            l_varDecls = Tpl::emptyTxt.clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_objectiveFunction, l_varDecls, l_preExp) = fun_215(Tpl::emptyTxt.clone(), i_objetiveE.clone(), l_varDecls.clone(), l_preExp.clone())?;
            (l_objectiveIntegrand, l_varDecls, l_preExp) = fun_216(Tpl::emptyTxt.clone(), i_objectiveIntegrandE.clone(), l_varDecls.clone(), l_preExp.clone())?;
            (l_startTime, l_varDecls, l_preExp) = fun_217(Tpl::emptyTxt.clone(), i_startTimeE.clone(), l_varDecls.clone(), l_preExp.clone())?;
            (l_finalTime, l_varDecls, l_preExp) = fun_218(Tpl::emptyTxt.clone(), i_finalTimeE.clone(), l_varDecls.clone(), l_preExp.clone())?;
            (l_timePointIndex, l_varDecls, l_preExp) = fun_219(Tpl::emptyTxt.clone(), i_startTimeE.clone(), l_varDecls.clone(), l_preExp.clone())?;
            (l_timePointValue, l_varDecls, l_preExp) = fun_220(Tpl::emptyTxt.clone(), i_finalTimeE.clone(), l_varDecls.clone(), l_preExp.clone())?;
            l_constraints = fun_221(Tpl::emptyTxt.clone(), a_simCode.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<opt:Optimization>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_objectiveFunction.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_objectiveIntegrand.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_startTime.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_finalTime.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<opt:TimePoints>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<opt:TimePoint ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_timePointIndex.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_timePointValue.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(">\n")).clone(), (literal!("</opt:TimePoint>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</opt:TimePoints>\n")).clone(), (literal!("<opt:PathConstraints>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_constraints.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</opt:PathConstraints>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</opt:Optimization>")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => {
            let mut txt = (*txt).clone();
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 1509, 16), (literal!("Unknown Optimization attribute")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_223(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Constraint>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_223 in &*items.clone() {
        let mut lstElt_223 = lstElt_223.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_223.clone()) {
        i_constraint => {
            txt = constraintXml(txt.clone(), i_constraint.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub(crate) fn constraintsXml(mut txt: Tpl::Text, mut a_constraints: Arc<metamodelica::List<Arc<DAE::Constraint>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    out_txt = lm_223(out_txt.clone(), a_constraints.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

fn lm_225(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut a_varDecls: Tpl::Text, mut a_preExp: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    let mut a_preExp: Tpl::Text = a_preExp;
    for mut lstElt_225 in &*items.clone() {
        let mut lstElt_225 = lstElt_225.clone();
        (txt, a_varDecls, a_preExp) = (::match_deref::match_deref! { match &(lstElt_225.clone()) {
        i_constraint => {
            (txt, a_preExp, a_varDecls) = daeExpConstraintXml(txt.clone(), i_constraint.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_varDecls, a_preExp))
}

pub(crate) fn constraintXml(mut in_txt: Tpl::Text, mut in_a_cons: Arc<DAE::Constraint>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cons.clone())) {
        (txt, Deref @ DAE::Constraint::CONSTRAINT_EXPS { constraintLst: i_constraintLst }) => {
            let mut l_constrain: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut l_varDecls: Tpl::Text;
            let mut txt = (*txt).clone();
            l_varDecls = Tpl::emptyTxt.clone();
            l_preExp = Tpl::emptyTxt.clone();
            l_constrain = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_constrain, l_varDecls, l_preExp) = lm_225(l_constrain.clone(), i_constraintLst.clone(), l_varDecls.clone(), l_preExp.clone())?;
            l_constrain = Tpl::popIter(l_constrain.clone())?;
            txt = Tpl::writeText(txt.clone(), l_constrain.clone())?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 1532, 16), (literal!("Unknown Constraint List")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_227(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_227 in &*items.clone() {
        let mut lstElt_227 = lstElt_227.clone();
        (txt, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_227.clone()) {
        i_stmt => {
            (txt, a_varDecls) = algStatementXml(txt.clone(), i_stmt.clone(), SimCodeFunction::contextFunction().clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_varDecls))
}

pub(crate) fn funStatementXml(mut txt: Tpl::Text, mut a_statementLst: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    (out_txt, out_a_varDecls) = lm_227(out_txt.clone(), a_statementLst.clone(), a_varDecls.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok((out_txt, out_a_varDecls))
}

fn fun_229(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>, mut in_a_varDecls: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone(), in_a_varDecls.clone(), in_a_context.clone())) {
        (txt, i_s @ Deref @ DAE::Statement::STMT_ASSIGN { type_: _, .. }, a_varDecls, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStmtAssignXml(txt.clone(), i_s.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, i_s @ Deref @ DAE::Statement::STMT_ASSIGN_ARR { type_: _, .. }, a_varDecls, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStmtAssignArrXml(txt.clone(), i_s.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, i_s @ Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { type_: _, .. }, a_varDecls, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStmtTupleAssignXml(txt.clone(), i_s.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, i_s @ Deref @ DAE::Statement::STMT_IF { exp: _, .. }, a_varDecls, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStmtIfXml(txt.clone(), i_s.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, i_s @ Deref @ DAE::Statement::STMT_FOR { type_: _, .. }, a_varDecls, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStmtForXml(txt.clone(), i_s.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, i_s @ Deref @ DAE::Statement::STMT_WHILE { exp: _, .. }, a_varDecls, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStmtWhileXml(txt.clone(), i_s.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, i_s @ Deref @ DAE::Statement::STMT_ASSERT { cond: _, .. }, a_varDecls, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStmtAssertXml(txt.clone(), i_s.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, i_s @ Deref @ DAE::Statement::STMT_TERMINATE { msg: _, .. }, a_varDecls, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStmtTerminateXml(txt.clone(), i_s.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, i_s @ Deref @ DAE::Statement::STMT_WHEN { exp: _, .. }, a_varDecls, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStmtWhenXml(txt.clone(), i_s.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Statement::STMT_BREAK { source: _ }, a_varDecls, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<fun:Break/>")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Statement::STMT_RETURN { source: _ }, a_varDecls, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<fun:Return/>")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, i_s @ Deref @ DAE::Statement::STMT_NORETCALL { exp: _, .. }, a_varDecls, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStmtNoretcallXml(txt.clone(), i_s.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, i_s @ Deref @ DAE::Statement::STMT_REINIT { var: _, .. }, a_varDecls, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStmtReinitXml(txt.clone(), i_s.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, a_varDecls, _) => {
            let mut txt = (*txt).clone();
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 1562, 14), (literal!("ALG_STATEMENT NYI")).clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub(crate) fn algStatementXml(mut txt: Tpl::Text, mut a_stmt: Arc<DAE::Statement>, mut a_context: SimCodeFunction::Context, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut l_res: Tpl::Text;
    (l_res, out_a_varDecls) = fun_229(Tpl::emptyTxt.clone(), a_stmt.clone(), a_varDecls.clone(), a_context.clone())?;
    out_txt = Tpl::writeText(txt.clone(), l_res.clone())?;
    Ok((out_txt, out_a_varDecls))
}

fn fun_231(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_val: Arc<DAE::Exp>, mut in_a_varDecls: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_val.clone(), in_a_varDecls.clone(), in_a_context.clone())) {
        (txt, Deref @ DAE::Exp::ASUB { exp: i_arr, sub: Deref @ metamodelica::List::Cons { head: i_idx, tail: Deref @ metamodelica::List::Nil } }, a_val, a_varDecls, a_context) => {
            let mut l_val1: Tpl::Text;
            let mut l_idx1: Tpl::Text;
            let mut l_arr1: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_arr1, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_arr.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            (l_idx1, l_preExp, a_varDecls) = daeSubscriptXML(Tpl::emptyTxt.clone(), i_idx.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            (l_val1, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_val.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeText(txt.clone(), l_preExp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_arr1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_idx1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_val1.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls, _) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

fn fun_232(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_exp1: Arc<DAE::Exp>, mut in_a_val: Arc<DAE::Exp>, mut in_a_varDecls: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_exp: Arc<DAE::Exp>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_exp1.clone(), in_a_val.clone(), in_a_varDecls.clone(), in_a_context.clone(), in_a_exp.clone())) {
        (txt, Deref @ "metatype", _, a_val, a_varDecls, a_context, a_exp) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = fun_231(txt.clone(), a_exp.clone(), a_val.clone(), a_varDecls.clone(), a_context.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, a_exp1, a_val, a_varDecls, a_context, _) => {
            let mut l_expPart: Tpl::Text;
            let mut l_varPart: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_varPart, l_preExp, a_varDecls) = daeExpAsubXml(Tpl::emptyTxt.clone(), a_exp1.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            (l_expPart, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_val.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Assign>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_varPart.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Expression>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_expPart.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</fun:Expression>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:Assign>")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub(crate) fn algStmtAssignXml(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Statement::STMT_ASSIGN { exp1: Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD { .. }, .. }, exp: i_e, .. }, a_context, a_varDecls) => {
            let mut l_expPart: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_expPart, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:Assign>\n")).clone(), (literal!("  <fun:Expression>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_expPart.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Expression>\n")).clone(), (literal!("</fun:Assign>")).clone()], lastHasNewLine: false }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Statement::STMT_ASSIGN { exp1: i_exp1 @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_VAR { functionType: _ }, .. }, exp: i_exp, .. }, a_context, a_varDecls) => {
            let mut l_varPart: Tpl::Text;
            let mut l_expPart: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_varPart, l_preExp, a_varDecls) = scalarLhsCrefXml(Tpl::emptyTxt.clone(), i_exp1.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            (l_expPart, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Assign>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_varPart.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Expression>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_expPart.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</fun:Expression>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:Assign>")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Statement::STMT_ASSIGN { exp1: i_exp1 @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { builtin: _, .. }, .. }, exp: i_exp, .. }, a_context, a_varDecls) => {
            let mut l_varPart: Tpl::Text;
            let mut l_expPart: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_varPart, l_preExp, a_varDecls) = scalarLhsCrefXml(Tpl::emptyTxt.clone(), i_exp1.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            (l_expPart, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Assign>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_varPart.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Expression>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_expPart.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</fun:Expression>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:Assign>")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Statement::STMT_ASSIGN { exp1: i_exp1 @ Deref @ DAE::Exp::CREF { componentRef: _, .. }, exp: i_exp, .. }, a_context, a_varDecls) => {
            let mut l_varPart: Tpl::Text;
            let mut l_expPart: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_varPart, l_preExp, a_varDecls) = scalarLhsCrefXml(Tpl::emptyTxt.clone(), i_exp1.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            (l_expPart, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Assign>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_varPart.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Expression>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_expPart.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</fun:Expression>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:Assign>")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Statement::STMT_ASSIGN { exp1: i_exp1 @ Deref @ DAE::Exp::ASUB { exp: _, .. }, exp: i_exp @ i_val, .. }, a_context, a_varDecls) => {
            let mut str_4: ArcStr;
            let mut txt_3: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            txt_3 = expTypeFromExpShortXml(Tpl::emptyTxt.clone(), i_exp.clone())?;
            str_4 = (Tpl::textString(txt_3.clone())?).clone();
            (txt, a_varDecls) = fun_232(txt.clone(), (str_4.clone()).clone(), i_exp1.clone(), i_val.clone(), a_varDecls.clone(), a_context.clone(), i_exp.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Statement::STMT_ASSIGN { exp1: i_exp1, exp: i_exp, .. }, a_context, a_varDecls) => {
            let mut l_expPart2: Tpl::Text;
            let mut l_expPart1: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_expPart1, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp1.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            (l_expPart2, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Assign>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_expPart1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Expression>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_expPart2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</fun:Expression>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:Assign>")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

fn fun_234(mut in_txt: Tpl::Text, mut in_a_ispec: Tpl::Text, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_cr: Arc<DAE::ComponentRef>, mut in_a_expPart: Tpl::Text, mut in_a_t: Arc<DAE::Type>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ispec.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone(), in_a_cr.clone(), in_a_expPart.clone(), in_a_t.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_varDecls, a_preExp, a_context, a_cr, a_expPart, a_t) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Assign>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = copyArrayDataXml(txt.clone(), a_t.clone(), (Tpl::textString(a_expPart.clone())?).clone(), a_cr.clone(), a_context.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Expression>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_preExp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</fun:Expression>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:Assign>")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, i_ispec, a_varDecls, a_preExp, a_context, a_cr, a_expPart, a_t) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            txt = Tpl::writeText(txt.clone(), a_preExp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            (txt, a_varDecls) = indexedAssignXml(txt.clone(), a_t.clone(), (Tpl::textString(a_expPart.clone())?).clone(), a_cr.clone(), (Tpl::textString(i_ispec.clone())?).clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub(crate) fn algStmtAssignArrXml(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Statement::STMT_ASSIGN_ARR { exp: i_e, lhs: Deref @ DAE::Exp::CREF { componentRef: i_cr, .. }, type_: i_t, .. }, a_context, a_varDecls) => {
            let mut l_ispec: Tpl::Text;
            let mut l_expPart: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_expPart, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            (l_ispec, l_preExp, a_varDecls) = indexSpecFromCrefXml(Tpl::emptyTxt.clone(), i_cr.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            (txt, a_varDecls) = fun_234(txt.clone(), l_ispec.clone(), a_varDecls.clone(), l_preExp.clone(), a_context.clone(), i_cr.clone(), l_expPart.clone(), i_t.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

fn fun_236(mut in_txt: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_ispec: ArcStr, mut in_a_exp: ArcStr, mut in_a_cref: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_context.clone(), in_a_ispec.clone(), in_a_exp.clone(), in_a_cref.clone()) {
        (mut txt, SimCodeFunction::Context::FUNCTION_CONTEXT { cref_prefix: _, .. }, _, _, mut a_cref) => {
            txt = Tpl::writeText(txt.clone(), a_cref.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_ispec, mut a_exp, mut a_cref) => {
            txt = Tpl::writeStr(txt.clone(), (a_exp.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeStr(txt.clone(), (a_ispec.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_cref.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn indexedAssignXml(mut txt: Tpl::Text, mut a_ty: Arc<DAE::Type>, mut a_exp: ArcStr, mut a_cr: Arc<DAE::ComponentRef>, mut a_ispec: ArcStr, mut a_context: SimCodeFunction::Context, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut l_cref: Tpl::Text;
    let mut l_type: Tpl::Text;
    l_type = expTypeArrayXml(Tpl::emptyTxt.clone(), a_ty.clone())?;
    l_cref = contextArrayCrefXml(Tpl::emptyTxt.clone(), a_cr.clone(), a_context.clone())?;
    out_txt = fun_236(txt.clone(), a_context.clone(), (a_ispec.clone()).clone(), (a_exp.clone()).clone(), l_cref.clone())?;
    out_a_varDecls = a_varDecls.clone();
    Ok((out_txt, out_a_varDecls))
}

fn fun_238(mut in_txt: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_cref: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_context.clone(), in_a_cref.clone()) {
        (mut txt, SimCodeFunction::Context::FUNCTION_CONTEXT { cref_prefix: _, .. }, mut a_cref) => {
            txt = Tpl::writeText(txt.clone(), a_cref.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_cref) => {
            txt = Tpl::writeText(txt.clone(), a_cref.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn copyArrayDataXml(mut txt: Tpl::Text, mut a_ty: Arc<DAE::Type>, mut a_exp: ArcStr, mut a_cr: Arc<DAE::ComponentRef>, mut a_context: SimCodeFunction::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_cref: Tpl::Text;
    let mut l_type: Tpl::Text;
    l_type = expTypeArrayXml(Tpl::emptyTxt.clone(), a_ty.clone())?;
    l_cref = contextArrayCrefXml(Tpl::emptyTxt.clone(), a_cr.clone(), a_context.clone())?;
    out_txt = fun_238(txt.clone(), a_context.clone(), l_cref.clone())?;
    Ok(out_txt)
}

fn lm_240(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_240 in &*items.clone() {
        let mut lstElt_240 = lstElt_240.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_240.clone()) {
        i_e => {
            txt = ExpressionDumpTpl::dumpExp(txt.clone(), i_e.clone(), (literal!("\"")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_241(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut a_varDecls: Tpl::Text, mut a_afterExp: Tpl::Text, mut a_context: SimCodeFunction::Context, mut a_retStruct: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    let mut a_afterExp: Tpl::Text = a_afterExp;
    for mut lstElt_241 in &*items.clone() {
        let mut lstElt_241 = lstElt_241.clone();
        (txt, a_varDecls, a_afterExp) = (::match_deref::match_deref! { match &(lstElt_241.clone()) {
        i_cr => {
            let mut x_i1: i32;
            let mut l_rhsStr: Tpl::Text;
            x_i1 = Tpl::getIteri_i0(txt.clone())?;
            l_rhsStr = Tpl::writeText(Tpl::emptyTxt.clone(), a_retStruct.clone())?;
            l_rhsStr = Tpl::writeTok(l_rhsStr.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".targ")).clone() }))?;
            l_rhsStr = Tpl::writeStr(l_rhsStr.clone(), (intString(x_i1.clone())).clone())?;
            (txt, a_afterExp, a_varDecls) = writeLhsCrefXml(txt.clone(), i_cr.clone(), (Tpl::textString(l_rhsStr.clone())?).clone(), a_context.clone(), a_afterExp.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_varDecls, a_afterExp))
}

fn lm_242(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut a_varDecls: Tpl::Text, mut a_afterExp: Tpl::Text, mut a_context: SimCodeFunction::Context, mut a_prefix: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    let mut a_afterExp: Tpl::Text = a_afterExp;
    for mut lstElt_242 in &*items.clone() {
        let mut lstElt_242 = lstElt_242.clone();
        (txt, a_varDecls, a_afterExp) = (::match_deref::match_deref! { match &(lstElt_242.clone()) {
        i_cr => {
            let mut x_i1: i32;
            let mut l_rhsStr: Tpl::Text;
            x_i1 = Tpl::getIteri_i0(txt.clone())?;
            l_rhsStr = Tpl::writeText(Tpl::emptyTxt.clone(), a_prefix.clone())?;
            l_rhsStr = Tpl::writeTok(l_rhsStr.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_targ")).clone() }))?;
            l_rhsStr = Tpl::writeStr(l_rhsStr.clone(), (intString(x_i1.clone())).clone())?;
            (txt, a_afterExp, a_varDecls) = writeLhsCrefXml(txt.clone(), i_cr.clone(), (Tpl::textString(l_rhsStr.clone())?).clone(), a_context.clone(), a_afterExp.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_varDecls, a_afterExp))
}

fn fun_243(mut in_txt: Tpl::Text, mut in_mArg: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone())) {
        (txt, Deref @ "modelica_metatype") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = NULL")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_244(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_rhsStr: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_rhsStr.clone())) {
        (txt, Deref @ "modelica_metatype", a_rhsStr) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_GC_add_root(&")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_rhsStr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", mmc_GC_local_state, \"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_rhsStr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\");")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_245(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut a_varDecls: Tpl::Text, mut a_prefix: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_245 in &*items.clone() {
        let mut lstElt_245 = lstElt_245.clone();
        (txt, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_245.clone()) {
        i_cr => {
            let mut x_i1: i32;
            let mut str_5: ArcStr;
            let mut l_addRoot: Tpl::Text;
            let mut str_3: ArcStr;
            let mut l_initVar: Tpl::Text;
            let mut l_typ: Tpl::Text;
            let mut l_rhsStr: Tpl::Text;
            x_i1 = Tpl::getIteri_i0(txt.clone())?;
            l_rhsStr = Tpl::writeText(Tpl::emptyTxt.clone(), a_prefix.clone())?;
            l_rhsStr = Tpl::writeTok(l_rhsStr.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_targ")).clone() }))?;
            l_rhsStr = Tpl::writeStr(l_rhsStr.clone(), (intString(x_i1.clone())).clone())?;
            l_typ = expTypeFromExpModelicaXml(Tpl::emptyTxt.clone(), i_cr.clone())?;
            str_3 = (Tpl::textString(l_typ.clone())?).clone();
            l_initVar = fun_243(Tpl::emptyTxt.clone(), (str_3.clone()).clone())?;
            str_5 = (Tpl::textString(l_typ.clone())?).clone();
            l_addRoot = fun_244(Tpl::emptyTxt.clone(), (str_5.clone()).clone(), l_rhsStr.clone())?;
            a_varDecls = Tpl::writeText(a_varDecls.clone(), l_typ.clone())?;
            a_varDecls = Tpl::writeTok(a_varDecls.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            a_varDecls = Tpl::writeText(a_varDecls.clone(), l_rhsStr.clone())?;
            a_varDecls = Tpl::writeText(a_varDecls.clone(), l_initVar.clone())?;
            a_varDecls = Tpl::writeTok(a_varDecls.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            a_varDecls = Tpl::writeText(a_varDecls.clone(), l_addRoot.clone())?;
            a_varDecls = Tpl::writeTok(a_varDecls.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_varDecls))
}

pub(crate) fn algStmtTupleAssignXml(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { exp: i_exp @ Deref @ DAE::Exp::CALL { path: _, .. }, expExpLst: i_expExpLst, .. }, a_context, a_varDecls) => {
            let mut l_lhsCrefs: Tpl::Text;
            let mut l_retStruct: Tpl::Text;
            let mut l_marker: Tpl::Text;
            let mut l_crefs: Tpl::Text;
            let mut l_afterExp: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            l_afterExp = Tpl::emptyTxt.clone();
            l_crefs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_crefs = lm_240(l_crefs.clone(), i_expExpLst.clone())?;
            l_crefs = Tpl::popIter(l_crefs.clone())?;
            l_marker = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            l_marker = Tpl::writeText(l_marker.clone(), l_crefs.clone())?;
            l_marker = Tpl::writeTok(l_marker.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") = ")).clone() }))?;
            l_marker = ExpressionDumpTpl::dumpExp(l_marker.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            l_preExp = Tpl::writeTok(l_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* algStmtTupleAssign: preExp buffer created for ")).clone() }))?;
            l_preExp = Tpl::writeText(l_preExp.clone(), l_marker.clone())?;
            l_preExp = Tpl::writeTok(l_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" */")).clone() }))?;
            l_preExp = Tpl::writeTok(l_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            l_afterExp = Tpl::writeTok(l_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* algStmtTupleAssign: afterExp buffer created for ")).clone() }))?;
            l_afterExp = Tpl::writeText(l_afterExp.clone(), l_marker.clone())?;
            l_afterExp = Tpl::writeTok(l_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" */")).clone() }))?;
            l_afterExp = Tpl::writeTok(l_afterExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            (l_retStruct, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            l_lhsCrefs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_lhsCrefs, a_varDecls, l_afterExp) = lm_241(l_lhsCrefs.clone(), i_expExpLst.clone(), a_varDecls.clone(), l_afterExp.clone(), a_context.clone(), l_retStruct.clone())?;
            l_lhsCrefs = Tpl::popIter(l_lhsCrefs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:FunctionCallStatement>\n")).clone(), (literal!("  <fun:OutputArgument>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_lhsCrefs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  </fun:OutputArgument>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_retStruct.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:FunctionCallStatement>")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { exp: Deref @ DAE::Exp::MATCHEXPRESSION { matchType: _, .. }, expExpLst: i_expExpLst, .. }, a_context, a_varDecls) => {
            let mut ret_7: i32;
            let mut l_prefix: Tpl::Text;
            let mut l_lhsCrefs: Tpl::Text;
            let mut l_afterExp: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            l_afterExp = Tpl::emptyTxt.clone();
            l_prefix = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("tmp")).clone() }))?;
            ret_7 = System::tmpTick();
            l_prefix = Tpl::writeStr(l_prefix.clone(), (intString(ret_7.clone())).clone())?;
            l_lhsCrefs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_lhsCrefs, a_varDecls, l_afterExp) = lm_242(l_lhsCrefs.clone(), i_expExpLst.clone(), a_varDecls.clone(), l_afterExp.clone(), a_context.clone(), l_prefix.clone())?;
            l_lhsCrefs = Tpl::popIter(l_lhsCrefs.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (txt, a_varDecls) = lm_245(txt.clone(), i_expExpLst.clone(), a_varDecls.clone(), l_prefix.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_preExp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lhsCrefs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_afterExp.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 1755, 12), (literal!("algStmtTupleAssign failed")).clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

fn fun_247(mut in_txt: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_lhsStr: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_context.clone(), in_a_lhsStr.clone()) {
        (mut txt, SimCodeFunction::Context::SIMULATION_CONTEXT { genDiscrete: _ }, mut a_lhsStr) => {
            txt = Tpl::writeText(txt.clone(), a_lhsStr.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_lhsStr) => {
            txt = Tpl::writeText(txt.clone(), a_lhsStr.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_248(mut in_txt: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_lhsStr: Tpl::Text, mut in_a_rhsStr: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_context.clone(), in_a_lhsStr.clone(), in_a_rhsStr.clone()) {
        (mut txt, SimCodeFunction::Context::SIMULATION_CONTEXT { genDiscrete: _ }, mut a_lhsStr, mut a_rhsStr) => {
            txt = Tpl::writeStr(txt.clone(), (a_rhsStr.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_lhsStr.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_lhsStr, _) => {
            txt = Tpl::writeText(txt.clone(), a_lhsStr.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn writeLhsCrefXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_rhsStr: ArcStr, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_rhsStr.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD { .. }, .. }, _, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<fun:EmptyOutputArgument></fun:EmptyOutputArgument>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_exp @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { ty: _, .. }, .. }, _, a_context, a_preExp, a_varDecls) => {
            let mut l_lhsStr: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_lhsStr, a_preExp, a_varDecls) = scalarLhsCrefXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = fun_247(txt.clone(), a_context.clone(), l_lhsStr.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::UNARY { exp: i_e @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { ty: _, .. }, .. }, .. }, a_rhsStr, a_context, a_preExp, a_varDecls) => {
            let mut l_lhsStr: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_lhsStr, a_preExp, a_varDecls) = scalarLhsCrefXml(Tpl::emptyTxt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = fun_248(txt.clone(), a_context.clone(), l_lhsStr.clone(), (a_rhsStr.clone()).clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_exp @ Deref @ DAE::Exp::CREF { componentRef: _, .. }, _, a_context, a_preExp, a_varDecls) => {
            let mut l_lhsStr: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_lhsStr, a_preExp, a_varDecls) = scalarLhsCrefXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lhsStr.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::UNARY { exp: i_e @ Deref @ DAE::Exp::CREF { componentRef: _, .. }, .. }, _, a_context, a_preExp, a_varDecls) => {
            let mut l_lhsStr: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_lhsStr, a_preExp, a_varDecls) = scalarLhsCrefXml(Tpl::emptyTxt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lhsStr.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_exp, a_rhsStr, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("/* SimCodeC.tpl template: writeLhsCref: UNHANDLED LHS\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("* ")).clone() }))?;
            txt = ExpressionDumpTpl::dumpExp(txt.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_rhsStr.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*/")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn lm_250(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut a_varDecls: Tpl::Text, mut a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_250 in &*items.clone() {
        let mut lstElt_250 = lstElt_250.clone();
        (txt, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_250.clone()) {
        i_stmt => {
            (txt, a_varDecls) = algStatementXml(txt.clone(), i_stmt.clone(), a_context.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_varDecls))
}

pub(crate) fn algStmtIfXml(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Statement::STMT_IF { exp: i_exp, statementLst: i_statementLst, else_: i_else__, .. }, a_context, a_varDecls) => {
            let mut l_condExp: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_condExp, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:If>\n")).clone(), (literal!("  <fun:Condition>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_condExp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Condition>\n")).clone(), (literal!("  <fun:Statements>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (txt, a_varDecls) = lm_250(txt.clone(), i_statementLst.clone(), a_varDecls.clone(), a_context.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  </fun:Statements>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            (txt, a_varDecls) = elseExprXml(txt.clone(), i_else__.clone(), a_context.clone(), a_varDecls.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:If>")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub(crate) fn algStmtForXml(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, i_s @ Deref @ DAE::Statement::STMT_FOR { range: Deref @ DAE::Exp::RANGE { ty: _, .. }, .. }, a_context, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStmtForRangeXml(txt.clone(), i_s.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, i_s @ Deref @ DAE::Statement::STMT_FOR { type_: _, .. }, a_context, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStmtForGenericXml(txt.clone(), i_s.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

fn lm_253(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut a_varDecls: Tpl::Text, mut a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_253 in &*items.clone() {
        let mut lstElt_253 = lstElt_253.clone();
        (txt, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_253.clone()) {
        i_stmt => {
            (txt, a_varDecls) = algStatementXml(txt.clone(), i_stmt.clone(), a_context.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_varDecls))
}

pub(crate) fn algStmtForRangeXml(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Statement::STMT_FOR { range: i_rng @ Deref @ DAE::Exp::RANGE { ty: _, .. }, type_: i_type__, iterIsArray: i_iterIsArray, statementLst: i_statementLst, iter: i_iter, .. }, a_context, a_varDecls) => {
            let mut l_stmtStr: Tpl::Text;
            let mut l_identTypeShort: Tpl::Text;
            let mut l_identType: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_identType = expTypeXml(Tpl::emptyTxt.clone(), i_type__.clone(), i_iterIsArray.clone())?;
            l_identTypeShort = expTypeShortXml(Tpl::emptyTxt.clone(), i_type__.clone())?;
            l_stmtStr = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_stmtStr, a_varDecls) = lm_253(l_stmtStr.clone(), i_statementLst.clone(), a_varDecls.clone(), a_context.clone())?;
            l_stmtStr = Tpl::popIter(l_stmtStr.clone())?;
            (txt, l_stmtStr, a_varDecls) = algStmtForRange_implXml(txt.clone(), i_rng.clone(), (i_iter.clone()).clone(), (Tpl::textString(l_identType.clone())?).clone(), (Tpl::textString(l_identTypeShort.clone())?).clone(), l_stmtStr.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

fn fun_255(mut in_txt: Tpl::Text, mut in_a_step: Option<Arc<DAE::Exp>>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_step.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone())) {
        (txt, Some(i_eo), a_varDecls, a_preExp, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_preExp, a_varDecls) = daeExpXml(txt.clone(), i_eo.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_varDecls, a_preExp, _) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_256(mut in_txt: Tpl::Text, mut in_a_range: Arc<DAE::Exp>, mut in_a_iterator: ArcStr, mut in_a_body: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_range.clone(), in_a_iterator.clone(), in_a_body.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::RANGE { start: i_start, step: i_step, stop: i_stop, .. }, a_iterator, a_body, a_context, a_varDecls) => {
            let mut l_stopValue: Tpl::Text;
            let mut l_stepValue: Tpl::Text;
            let mut l_startValue: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut l_iterName: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_iterName = contextIteratorNameXml(Tpl::emptyTxt.clone(), (a_iterator.clone()).clone(), a_context.clone())?;
            l_preExp = Tpl::emptyTxt.clone();
            (l_startValue, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_start.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            (l_stepValue, a_varDecls, l_preExp) = fun_255(Tpl::emptyTxt.clone(), i_step.clone(), a_varDecls.clone(), l_preExp.clone(), a_context.clone())?;
            (l_stopValue, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_stop.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:For>\n")).clone(), (literal!("  <fun:Index>\n")).clone(), (literal!("    <fun:IterationVariable>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_iterName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\"/>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("    </fun:IterationVariable>\n")).clone(), (literal!("    <fun:IterationSet>\n")).clone(), (literal!("      <exp:Range>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 8 }))?;
            txt = Tpl::writeText(txt.clone(), l_startValue.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_stepValue.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_stopValue.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("      </exp:Range>\n")).clone(), (literal!("    </fun:IterationSet>\n")).clone(), (literal!("  </fun:Index>\n")).clone(), (literal!("  <fun:Statements>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), a_body.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Statements>\n")).clone(), (literal!("</fun:For>")).clone()], lastHasNewLine: false }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, _, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub(crate) fn algStmtForRange_implXml(mut txt: Tpl::Text, mut a_range: Arc<DAE::Exp>, mut a_iterator: ArcStr, mut a_type: ArcStr, mut a_shortType: ArcStr, mut a_body: Tpl::Text, mut a_context: SimCodeFunction::Context, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_body: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = fun_256(txt.clone(), a_range.clone(), (a_iterator.clone()).clone(), a_body.clone(), a_context.clone(), a_varDecls.clone())?;
    out_a_body = a_body.clone();
    Ok((out_txt, out_a_body, out_a_varDecls))
}

fn lm_258(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut a_varDecls: Tpl::Text, mut a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_258 in &*items.clone() {
        let mut lstElt_258 = lstElt_258.clone();
        (txt, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_258.clone()) {
        i_stmt => {
            (txt, a_varDecls) = algStatementXml(txt.clone(), i_stmt.clone(), a_context.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_varDecls))
}

pub(crate) fn algStmtForGenericXml(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Statement::STMT_FOR { type_: i_type__, iterIsArray: i_iterIsArray, statementLst: i_statementLst, range: i_range, iter: i_iter, .. }, a_context, a_varDecls) => {
            let mut l_stmtStr: Tpl::Text;
            let mut l_arrayType: Tpl::Text;
            let mut l_iterType: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_iterType = expTypeXml(Tpl::emptyTxt.clone(), i_type__.clone(), i_iterIsArray.clone())?;
            l_arrayType = expTypeArrayXml(Tpl::emptyTxt.clone(), i_type__.clone())?;
            l_stmtStr = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_stmtStr, a_varDecls) = lm_258(l_stmtStr.clone(), i_statementLst.clone(), a_varDecls.clone(), a_context.clone())?;
            l_stmtStr = Tpl::popIter(l_stmtStr.clone())?;
            (txt, l_stmtStr, a_varDecls) = algStmtForGeneric_implXml(txt.clone(), i_range.clone(), (i_iter.clone()).clone(), (Tpl::textString(l_iterType.clone())?).clone(), (Tpl::textString(l_arrayType.clone())?).clone(), i_iterIsArray.clone(), l_stmtStr.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub(crate) fn algStmtForGeneric_implXml(mut txt: Tpl::Text, mut a_exp: Arc<DAE::Exp>, mut a_iterator: ArcStr, mut a_type: ArcStr, mut a_arrayType: ArcStr, mut a_iterIsArray: bool, mut a_body: Tpl::Text, mut a_context: SimCodeFunction::Context, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_body: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut l_evar: Tpl::Text;
    let mut l_preExp: Tpl::Text;
    let mut l_iterName: Tpl::Text;
    l_iterName = contextIteratorNameXml(Tpl::emptyTxt.clone(), (a_iterator.clone()).clone(), a_context.clone())?;
    l_preExp = Tpl::emptyTxt.clone();
    (l_evar, l_preExp, out_a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_exp.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:For>\n")).clone(), (literal!("  <fun:Index>\n")).clone(), (literal!("    <fun:IterationVariable>\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_iterName.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\"/>\n")).clone() }))?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("    </fun:IterationVariable>\n")).clone(), (literal!("    <fun:IterationSet>\n")).clone(), (literal!("      <exp:Array>\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 8 }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_preExp.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("      </exp:Array>\n")).clone(), (literal!("    </fun:IterationSet>\n")).clone(), (literal!("  </fun:Index>\n")).clone(), (literal!("  <fun:Statements>\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
    out_txt = Tpl::writeText(out_txt.clone(), a_body.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Statements>\n")).clone(), (literal!("</fun:For>")).clone()], lastHasNewLine: false }))?;
    out_a_body = a_body.clone();
    Ok((out_txt, out_a_body, out_a_varDecls))
}

fn lm_261(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut a_varDecls: Tpl::Text, mut a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_261 in &*items.clone() {
        let mut lstElt_261 = lstElt_261.clone();
        (txt, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_261.clone()) {
        i_stmt => {
            (txt, a_varDecls) = algStatementXml(txt.clone(), i_stmt.clone(), a_context.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_varDecls))
}

pub(crate) fn algStmtWhileXml(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Statement::STMT_WHILE { exp: i_exp, statementLst: i_statementLst, .. }, a_context, a_varDecls) => {
            let mut l_var: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_var, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:While>\n")).clone(), (literal!("  <fun:Condition>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_var.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Condition>\n")).clone(), (literal!("  <fun:Statements>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (txt, a_varDecls) = lm_261(txt.clone(), i_statementLst.clone(), a_varDecls.clone(), a_context.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Statements>\n")).clone(), (literal!("</fun:While>")).clone()], lastHasNewLine: false }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub(crate) fn algStmtAssertXml(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Statement::STMT_ASSERT { source: Deref @ DAE::ElementSource { info: i_info, .. }, cond: i_cond, msg: i_msg, .. }, a_context, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = assertCommonXml(txt.clone(), i_cond.clone(), i_msg.clone(), a_context.clone(), a_varDecls.clone(), i_info.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub(crate) fn algStmtTerminateXml(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Statement::STMT_TERMINATE { msg: i_msg, .. }, a_context, a_varDecls) => {
            let mut l_msgVar: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_msgVar, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_msg.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeText(txt.clone(), l_preExp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_msgVar.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub(crate) fn algStmtNoretcallXml(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Statement::STMT_NORETCALL { exp: i_exp, .. }, a_context, a_varDecls) => {
            let mut l_expPart: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_expPart, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeText(txt.clone(), l_preExp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_expPart.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

fn lm_266(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_266 in &*items.clone() {
        let mut lstElt_266 = lstElt_266.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_266.clone()) {
        i_e => {
            txt = crefToXmlStr(txt.clone(), i_e.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_267(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut a_varDecls: Tpl::Text, mut a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_267 in &*items.clone() {
        let mut lstElt_267 = lstElt_267.clone();
        (txt, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_267.clone()) {
        i_stmt => {
            (txt, a_varDecls) = algStatementXml(txt.clone(), i_stmt.clone(), a_context.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_varDecls))
}

pub(crate) fn algStmtWhenXml(mut in_txt: Tpl::Text, mut in_a_when: Arc<DAE::Statement>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_when.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Statement::STMT_WHEN { conditions: i_conditions, statementLst: i_statementLst, elseWhen: i_elseWhen, .. }, a_context, a_varDecls) => {
            let mut l_else: Tpl::Text;
            let mut l_statements: Tpl::Text;
            let mut l_cond: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_cond = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_cond = lm_266(l_cond.clone(), i_conditions.clone())?;
            l_cond = Tpl::popIter(l_cond.clone())?;
            l_statements = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_statements, a_varDecls) = lm_267(l_statements.clone(), i_statementLst.clone(), a_varDecls.clone(), a_context.clone())?;
            l_statements = Tpl::popIter(l_statements.clone())?;
            (l_else, a_varDecls) = algStatementWhenElseXml(Tpl::emptyTxt.clone(), i_elseWhen.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:When>\n")).clone(), (literal!("  <fun:Condition>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_cond.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Condition>\n")).clone(), (literal!("  <fun:Statements>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_statements.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  </fun:Statements>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_else.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

fn lm_269(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_269 in &*items.clone() {
        let mut lstElt_269 = lstElt_269.clone();
        (txt, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_269.clone()) {
        i_stmt => {
            (txt, a_varDecls) = algStatementXml(txt.clone(), i_stmt.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_varDecls))
}

fn lm_270(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_270 in &*items.clone() {
        let mut lstElt_270 = lstElt_270.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_270.clone()) {
        i_e => {
            txt = crefToXmlStr(txt.clone(), i_e.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub(crate) fn algStatementWhenElseXml(mut in_txt: Tpl::Text, mut in_a_stmt: Option<Arc<DAE::Statement>>, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone(), in_a_varDecls.clone())) {
        (txt, Some(Deref @ DAE::Statement::STMT_WHEN { statementLst: i_when_statementLst, elseWhen: i_when_elseWhen, conditions: i_when_conditions, .. }), a_varDecls) => {
            let mut l_elseCondStr: Tpl::Text;
            let mut l_else: Tpl::Text;
            let mut l_statements: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_statements = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_statements, a_varDecls) = lm_269(l_statements.clone(), i_when_statementLst.clone(), a_varDecls.clone())?;
            l_statements = Tpl::popIter(l_statements.clone())?;
            (l_else, a_varDecls) = algStatementWhenElseXml(Tpl::emptyTxt.clone(), i_when_elseWhen.clone(), a_varDecls.clone())?;
            l_elseCondStr = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!(" ")).clone()], lastHasNewLine: false })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_elseCondStr = lm_270(l_elseCondStr.clone(), i_when_conditions.clone())?;
            l_elseCondStr = Tpl::popIter(l_elseCondStr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Condition>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_elseCondStr.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</fun:Condition>\n")).clone(), (literal!("<fun:Statements>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_statements.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</fun:Statements>\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_else.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:When>")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub(crate) fn algStmtReinitXml(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Statement::STMT_REINIT { var: i_var, value: i_value, .. }, a_context, a_varDecls) => {
            let mut l_expPart2: Tpl::Text;
            let mut l_expPart1: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_expPart1, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_var.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            (l_expPart2, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_value.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Reinit>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_expPart1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_expPart2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Reinit>")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub(crate) fn indexSpecFromCrefXml(mut in_txt: Tpl::Text, mut in_a_cr: Arc<DAE::ComponentRef>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cr.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: i_subs @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpCrefRhsIndexSpecXml(txt.clone(), i_subs.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn lm_274(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut a_varDecls: Tpl::Text, mut a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_274 in &*items.clone() {
        let mut lstElt_274 = lstElt_274.clone();
        (txt, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_274.clone()) {
        i_stmt => {
            (txt, a_varDecls) = algStatementXml(txt.clone(), i_stmt.clone(), a_context.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_varDecls))
}

fn lm_275(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut a_varDecls: Tpl::Text, mut a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_275 in &*items.clone() {
        let mut lstElt_275 = lstElt_275.clone();
        (txt, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_275.clone()) {
        i_stmt => {
            (txt, a_varDecls) = algStatementXml(txt.clone(), i_stmt.clone(), a_context.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_varDecls))
}

pub(crate) fn elseExprXml(mut in_txt: Tpl::Text, mut in_a_else__: Arc<DAE::Else>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((in_txt.clone(), in_a_else__.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Else::NOELSE { .. }, _, a_varDecls) => {
            return Ok((txt.clone(), a_varDecls.clone()))
        },
        (txt, Deref @ DAE::Else::ELSEIF { exp: i_exp, statementLst: i_statementLst, else_: i_else__ }, a_context, a_varDecls) => {
            let mut l_condExp: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_condExp, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:ElseIf>\n")).clone(), (literal!("  <fun:Condition>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_condExp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  </fun:Condition>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (txt, a_varDecls) = lm_274(txt.clone(), i_statementLst.clone(), a_varDecls.clone(), a_context.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</fun:ElseIf>\n")).clone() }))?;
            { (in_txt, in_a_else__, in_a_context, in_a_varDecls) = (txt.clone(), i_else__.clone(), a_context.clone(), a_varDecls.clone()); continue '__tco; }
        },
        (txt, Deref @ DAE::Else::ELSE { statementLst: i_statementLst }, a_context, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Else>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (txt, a_varDecls) = lm_275(txt.clone(), i_statementLst.clone(), a_varDecls.clone(), a_context.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:Else>")).clone() }))?;
            return Ok((txt.clone(), a_varDecls.clone()))
        },
        (txt, _, _, a_varDecls) => {
            return Ok((txt.clone(), a_varDecls.clone()))
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn fun_277(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_ecr_componentRef: Arc<DAE::ComponentRef>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_ecr: Arc<DAE::Exp>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_ecr_componentRef.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone(), in_a_ecr.clone())) {
        (txt, false, _, a_varDecls, a_preExp, a_context, a_ecr) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_preExp, a_varDecls) = daeExpCrefLhsXml(txt.clone(), a_ecr.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_ecr_componentRef, a_varDecls, a_preExp, _, _) => {
            let mut txt = (*txt).clone();
            txt = crefXml(txt.clone(), a_ecr_componentRef.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

pub(crate) fn scalarLhsCrefXml(mut in_txt: Tpl::Text, mut in_a_ecr: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ecr.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::CREF { componentRef: i_cr, ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_VAR { functionType: _ } }, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = crefStrXml(txt.clone(), i_cr.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_ecr @ Deref @ DAE::Exp::CREF { componentRef: i_ecr_componentRef @ Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, .. }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut ret_0: bool;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            ret_0 = SimCodeFunctionUtil::crefNoSub(i_ecr_componentRef.clone());
            (txt, a_varDecls, a_preExp) = fun_277(txt.clone(), ret_0.clone(), i_ecr_componentRef.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone(), i_ecr.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CREF { componentRef: i_ecr_componentRef @ Deref @ DAE::ComponentRef::CREF_QUAL { ident: _, .. }, .. }, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = crefXml(txt.clone(), i_ecr_componentRef.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD { .. }, .. }, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ONLY_IDENT_OR_QUAL_CREF_SUPPORTED_SLHS")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_279(mut in_txt: Tpl::Text, mut in_a_e: Tpl::Text, mut in_a_preExp: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_e.clone(), in_a_preExp.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_preExp) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_preExp.clone())?;
            txt.clone()
        },
        (txt, i_e, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), i_e.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_280(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_preExp: Tpl::Text, mut in_a_eStr1: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_preExp.clone(), in_a_eStr1.clone()) {
        (mut txt, false, _, mut a_eStr1) => {
            txt = Tpl::writeText(txt.clone(), a_eStr1.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_preExp, _) => {
            txt = Tpl::writeText(txt.clone(), a_preExp.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn daeExpXml(mut txt: Tpl::Text, mut a_exp: Arc<DAE::Exp>, mut a_context: SimCodeFunction::Context, mut a_preExp: Tpl::Text, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut ret_4: bool;
    let mut ret_3: i32;
    let mut l_eStr2: Tpl::Text;
    let mut l_eStr1: Tpl::Text;
    let mut l_e: Tpl::Text;
    (l_e, out_a_preExp, out_a_varDecls) = daeExpXml_dispatch(Tpl::emptyTxt.clone(), a_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
    l_eStr1 = fun_279(Tpl::emptyTxt.clone(), l_e.clone(), out_a_preExp.clone())?;
    ret_3 = System::stringFind((Tpl::textString(l_eStr1.clone())?).clone(), (literal!("tmp")).clone())?;
    ret_4 = intEq(0, ret_3.clone());
    l_eStr2 = fun_280(Tpl::emptyTxt.clone(), ret_4.clone(), out_a_preExp.clone(), l_eStr1.clone())?;
    out_txt = Tpl::writeText(txt.clone(), l_eStr2.clone())?;
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_282(mut in_txt: Tpl::Text, mut in_a_bool: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_bool.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("0")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("1")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn daeExpXml_dispatch(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::ICONST { integer: i_integer }, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:IntegerLiteral>")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_integer.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:IntegerLiteral>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::RCONST { real: i_real }, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:RealLiteral>")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(i_real.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:RealLiteral>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::SCONST { string: i_string }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:StringLiteral>")).clone() }))?;
            (txt, a_preExp, a_varDecls) = daeExpSconstXml(txt.clone(), (i_string.clone()).clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:StringLiteral>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::BCONST { bool: i_bool }, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:BooleanLiteral>")).clone() }))?;
            txt = fun_282(txt.clone(), i_bool.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:BooleanLiteral>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::ENUM_LITERAL { index: i_index, .. }, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ DAE::Exp::CREF { componentRef: _, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpCrefRhsXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ DAE::Exp::BINARY { exp1: _, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpBinaryXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ DAE::Exp::UNARY { operator: _, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpUnaryXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ DAE::Exp::LBINARY { exp1: _, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpLbinaryXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ DAE::Exp::LUNARY { operator: _, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpLunaryXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ DAE::Exp::RELATION { exp1: _, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpRelationXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ DAE::Exp::IFEXP { expCond: _, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpIfXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ DAE::Exp::CALL { path: _, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpCallXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ DAE::Exp::ARRAY { ty: _, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpArrayXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ DAE::Exp::MATRIX { ty: _, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpMatrixXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ DAE::Exp::RANGE { ty: _, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpRangeXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ DAE::Exp::CAST { ty: _, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpCastXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ DAE::Exp::ASUB { exp: _, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpAsubXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::TSUB { exp: i_exp, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpXml(txt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ DAE::Exp::SIZE { exp: _, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpSizeXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::TUPLE { PR: _ }, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Tuple Not yet Implemented")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ DAE::Exp::BOX { exp: _ }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpBoxXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ DAE::Exp::UNBOX { exp: _, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpUnboxXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ DAE::Exp::SHARED_LITERAL { index: _, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpSharedLiteralXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone());
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_exp, _, a_preExp, a_varDecls) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Unknown expression: ")).clone() }))?;
            txt_0 = ExpressionDumpTpl::dumpExp(txt_0.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 2141, 14), (Tpl::textString(txt_0.clone())?).clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_284(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone())) {
        (txt, Deref @ DAE::Exp::ICONST { integer: i_integer }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_integer.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::RCONST { real: i_real }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (realString(i_real.clone())).clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn daeExpValueXml(mut txt: Tpl::Text, mut a_exp: Arc<DAE::Exp>, mut a_context: SimCodeFunction::Context, mut a_preExp: Tpl::Text, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    out_txt = fun_284(txt.clone(), a_exp.clone())?;
    out_a_preExp = a_preExp.clone();
    out_a_varDecls = a_varDecls.clone();
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_286(mut in_txt: Tpl::Text, mut in_mArg: Arc<DAE::Type>, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Type::T_ARRAY { ty: _, .. }, a_exp, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpXml(txt.clone(), a_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub(crate) fn daeExternalXmlExp(mut txt: Tpl::Text, mut a_exp: Arc<DAE::Exp>, mut a_context: SimCodeFunction::Context, mut a_preExp: Tpl::Text, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut ret_0: Arc<DAE::Type>;
    ret_0 = Expression::r#typeof(a_exp.clone())?;
    (out_txt, out_a_preExp, out_a_varDecls) = fun_286(txt.clone(), ret_0.clone(), a_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub(crate) fn daeExpSconstXml(mut txt: Tpl::Text, mut a_string: ArcStr, mut a_context: SimCodeFunction::Context, mut a_preExp: Tpl::Text, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut ret_0: ArcStr;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
    ret_0 = (Util::escapeModelicaStringToXmlString((a_string.clone()).clone())?).clone();
    out_txt = Tpl::writeStr(out_txt.clone(), (ret_0.clone()).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
    out_a_preExp = a_preExp.clone();
    out_a_varDecls = a_varDecls.clone();
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_289(mut in_txt: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_cr: Arc<DAE::ComponentRef>, mut in_a_t: Arc<DAE::Type>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_context.clone(), in_a_cr.clone(), in_a_t.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_exp.clone())) {
        (txt, i_context @ SimCodeFunction::Context::FUNCTION_CONTEXT { cref_prefix: _, .. }, _, _, a_varDecls, a_preExp, a_exp) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_preExp, a_varDecls) = daeExpCrefRhs2Xml(txt.clone(), a_exp.clone(), i_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, i_context, a_cr, a_t, a_varDecls, a_preExp, _) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_preExp, a_varDecls) = daeExpRecordCrefRhsXml(txt.clone(), a_t.clone(), a_cr.clone(), i_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

pub(crate) fn daeExpCrefRhsXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, i_exp @ Deref @ DAE::Exp::CREF { componentRef: i_cr, ty: i_t @ Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. } }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls, a_preExp) = fun_289(txt.clone(), a_context.clone(), i_cr.clone(), i_t.clone(), a_varDecls.clone(), a_preExp.clone(), i_exp.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CREF { componentRef: i_cr, ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { builtin: _, .. } }, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = crefFunctionNameXml(txt.clone(), i_cr.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CREF { componentRef: i_cr, ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_VAR { functionType: _ } }, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = crefStrXml(txt.clone(), i_cr.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_exp, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpCrefRhs2Xml(txt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn lm_291(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut a_varDecls: Tpl::Text, mut a_preExp: Tpl::Text, mut a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    let mut a_preExp: Tpl::Text = a_preExp;
    for mut lstElt_291 in &*items.clone() {
        let mut lstElt_291 = lstElt_291.clone();
        (txt, a_varDecls, a_preExp) = (::match_deref::match_deref! { match &(lstElt_291.clone()) {
        Deref @ DAE::Subscript::INDEX { exp: i_exp } => {
            (txt, a_preExp, a_varDecls) = daeExpXml(txt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_varDecls, a_preExp))
}

fn lm_292(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut a_varDecls: Tpl::Text, mut a_preExp: Tpl::Text, mut a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    let mut a_preExp: Tpl::Text = a_preExp;
    for mut lstElt_292 in &*items.clone() {
        let mut lstElt_292 = lstElt_292.clone();
        (txt, a_varDecls, a_preExp) = (::match_deref::match_deref! { match &(lstElt_292.clone()) {
        Deref @ DAE::Subscript::INDEX { exp: i_exp } => {
            (txt, a_preExp, a_varDecls) = daeExpXml(txt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_varDecls, a_preExp))
}

fn fun_293(mut in_txt: Tpl::Text, mut in_mArg: Arc<DAE::Type>, mut in_a_ecr: Arc<DAE::Exp>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_cr: Arc<DAE::ComponentRef>, mut in_a_arrName: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_ecr.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone(), in_a_cr.clone(), in_a_arrName.clone())) {
        (txt, Deref @ DAE::Type::T_ARRAY { dims: i_et_dims, .. }, _, a_varDecls, a_preExp, a_context, a_cr, a_arrName) => {
            let mut ret_0: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(&")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_arrName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")[")).clone() }))?;
            ret_0 = ComponentReferenceBasics::crefSubs(a_cr.clone())?;
            (txt, a_preExp, a_varDecls) = threadDimSubListXml(txt.clone(), i_et_dims.clone(), ret_0.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_ecr, a_varDecls, a_preExp, _, _, _) => {
            let mut txt_1: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_1 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Indexing non-array ")).clone() }))?;
            txt_1 = ExpressionDumpTpl::dumpExp(txt_1.clone(), a_ecr.clone(), (literal!("\"")).clone())?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 2256, 28), (Tpl::textString(txt_1.clone())?).clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_294(mut in_txt: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_ecr: Arc<DAE::Exp>, mut in_a_arrName: Tpl::Text, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_context.clone(), in_a_ecr.clone(), in_a_arrName.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_cr.clone())) {
        (txt, i_context @ SimCodeFunction::Context::FUNCTION_CONTEXT { cref_prefix: _, .. }, _, a_arrName, a_varDecls, a_preExp, a_cr) => {
            let mut ret_1: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
            let mut l_dimsValuesStr: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            ret_1 = ComponentReferenceBasics::crefSubs(a_cr.clone())?;
            l_dimsValuesStr = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!(" ")).clone()], lastHasNewLine: false })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_dimsValuesStr, a_varDecls, a_preExp) = lm_292(l_dimsValuesStr.clone(), ret_1.clone(), a_varDecls.clone(), a_preExp.clone(), i_context.clone())?;
            l_dimsValuesStr = Tpl::popIter(l_dimsValuesStr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Identifier>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_arrName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\">\n")).clone(), (literal!("  <exp:ArraySubscripts>\n")).clone(), (literal!("    <exp:IndexExpression>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            txt = Tpl::writeText(txt.clone(), l_dimsValuesStr.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("    </exp:IndexExpression>\n")).clone(), (literal!("  </exp:ArraySubscripts>\n")).clone(), (literal!("</exp:QualifiedNamePart>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Identifier>")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, i_context, a_ecr, a_arrName, a_varDecls, a_preExp, a_cr) => {
            let mut ret_2: Arc<DAE::Type>;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            ret_2 = ComponentReference::crefLastType(a_cr.clone())?;
            (txt, a_varDecls, a_preExp) = fun_293(txt.clone(), ret_2.clone(), a_ecr.clone(), a_varDecls.clone(), a_preExp.clone(), i_context.clone(), a_cr.clone(), a_arrName.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_295(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_ecr: Arc<DAE::Exp>, mut in_a_arrName: Tpl::Text, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_ecr.clone(), in_a_arrName.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone(), in_a_cr.clone())) {
        (txt, Deref @ "metatype_array", _, a_arrName, a_varDecls, a_preExp, a_context, a_cr) => {
            let mut ret_1: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
            let mut l_dimsValuesStr: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            ret_1 = ComponentReferenceBasics::crefSubs(a_cr.clone())?;
            l_dimsValuesStr = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_dimsValuesStr, a_varDecls, a_preExp) = lm_291(l_dimsValuesStr.clone(), ret_1.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone())?;
            l_dimsValuesStr = Tpl::popIter(l_dimsValuesStr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("arrayGet(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_arrName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_dimsValuesStr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") /* DAE.CREF */")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_ecr, a_arrName, a_varDecls, a_preExp, a_context, a_cr) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_varDecls, a_preExp) = fun_294(txt.clone(), a_context.clone(), a_ecr.clone(), a_arrName.clone(), a_varDecls.clone(), a_preExp.clone(), a_cr.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_296(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_ecr: Arc<DAE::Exp>, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_context: SimCodeFunction::Context, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_ecr.clone(), in_a_preExp.clone(), in_a_varDecls.clone(), in_a_ty.clone(), in_a_context.clone(), in_a_cr.clone())) {
        (txt, false, _, a_preExp, a_varDecls, a_ty, a_context, a_cr) => {
            let mut ret_4: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
            let mut l_spec1: Tpl::Text;
            let mut l_tmp: Tpl::Text;
            let mut l_arrayType: Tpl::Text;
            let mut l_arrName: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_arrName = contextArrayCrefXml(Tpl::emptyTxt.clone(), a_cr.clone(), a_context.clone())?;
            l_arrayType = expTypeArrayXml(Tpl::emptyTxt.clone(), a_ty.clone())?;
            (l_tmp, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_arrayType.clone())?).clone(), a_varDecls.clone())?;
            ret_4 = ComponentReferenceBasics::crefSubs(a_cr.clone())?;
            (l_spec1, a_preExp, a_varDecls) = daeExpCrefRhsIndexSpecXml(Tpl::emptyTxt.clone(), ret_4.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_arrName.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_spec1.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), l_tmp.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, a_ecr, a_preExp, a_varDecls, a_ty, a_context, a_cr) => {
            let mut str_9: ArcStr;
            let mut ret_8: i32;
            let mut ret_7: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
            let mut l_dimsLenStr: Tpl::Text;
            let mut ret_5: Arc<DAE::ComponentRef>;
            let mut l_arrayType: Tpl::Text;
            let mut l_arrName: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            ret_5 = ComponentReferenceBasics::crefStripLastSubs(a_cr.clone())?;
            l_arrName = contextCrefXml(Tpl::emptyTxt.clone(), ret_5.clone(), a_context.clone())?;
            l_arrayType = expTypeArrayXml(Tpl::emptyTxt.clone(), a_ty.clone())?;
            ret_7 = ComponentReferenceBasics::crefSubs(a_cr.clone())?;
            ret_8 = (ret_7.clone().len() as i32);
            l_dimsLenStr = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_8.clone())).clone())?;
            str_9 = (Tpl::textString(l_arrayType.clone())?).clone();
            (txt, a_varDecls, a_preExp) = fun_295(txt.clone(), (str_9.clone()).clone(), a_ecr.clone(), l_arrName.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone(), a_cr.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_297(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_ecr_componentRef: Arc<DAE::ComponentRef>, mut in_a_ecr: Arc<DAE::Exp>, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_context: SimCodeFunction::Context, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_ecr_componentRef.clone(), in_a_ecr.clone(), in_a_preExp.clone(), in_a_varDecls.clone(), in_a_ty.clone(), in_a_context.clone(), in_a_cr.clone())) {
        (txt, false, _, a_ecr, a_preExp, a_varDecls, a_ty, a_context, a_cr) => {
            let mut ret_0: bool;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            ret_0 = SimCodeFunctionUtil::crefSubIsScalar(a_cr.clone())?;
            (txt, a_preExp, a_varDecls) = fun_296(txt.clone(), ret_0.clone(), a_ecr.clone(), a_preExp.clone(), a_varDecls.clone(), a_ty.clone(), a_context.clone(), a_cr.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, a_ecr_componentRef, _, a_preExp, a_varDecls, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = crefXml(txt.clone(), a_ecr_componentRef.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_298(mut in_txt: Tpl::Text, mut in_a_box: Tpl::Text, mut in_a_ecr_componentRef: Arc<DAE::ComponentRef>, mut in_a_ecr: Arc<DAE::Exp>, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_context: SimCodeFunction::Context, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_box.clone(), in_a_ecr_componentRef.clone(), in_a_ecr.clone(), in_a_preExp.clone(), in_a_varDecls.clone(), in_a_ty.clone(), in_a_context.clone(), in_a_cr.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_ecr_componentRef, a_ecr, a_preExp, a_varDecls, a_ty, a_context, a_cr) => {
            let mut ret_0: bool;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            ret_0 = SimCodeFunctionUtil::crefIsScalar(a_cr.clone(), a_context.clone())?;
            (txt, a_preExp, a_varDecls) = fun_297(txt.clone(), ret_0.clone(), a_ecr_componentRef.clone(), a_ecr.clone(), a_preExp.clone(), a_varDecls.clone(), a_ty.clone(), a_context.clone(), a_cr.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_box, _, _, a_preExp, a_varDecls, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), i_box.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub(crate) fn daeExpCrefRhs2Xml(mut in_txt: Tpl::Text, mut in_a_ecr: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ecr.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, i_ecr @ Deref @ DAE::Exp::CREF { componentRef: i_ecr_componentRef @ i_cr, ty: i_ty }, a_context, a_preExp, a_varDecls) => {
            let mut l_box: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_box, a_preExp, a_varDecls) = daeExpCrefRhsArrayBoxXml(Tpl::emptyTxt.clone(), i_ecr.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt, a_preExp, a_varDecls) = fun_298(txt.clone(), l_box.clone(), i_ecr_componentRef.clone(), i_ecr.clone(), a_preExp.clone(), a_varDecls.clone(), i_ty.clone(), a_context.clone(), i_cr.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_ecr, _, a_preExp, a_varDecls) => {
            let mut txt_1: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_1 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("daeExpCrefRhs2: UNHANDLED EXPRESSION: ")).clone() }))?;
            txt_1 = ExpressionDumpTpl::dumpExp(txt_1.clone(), i_ecr.clone(), (literal!("\"")).clone())?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 2273, 11), (Tpl::textString(txt_1.clone())?).clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_300(mut in_txt: Tpl::Text, mut in_a_dim: Arc<DAE::Dimension>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_dim.clone())) {
        (txt, Deref @ DAE::Dimension::DIM_INTEGER { integer: i_integer }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_integer.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Dimension::DIM_ENUM { size: i_size, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_size.clone())).clone())?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 2292, 22), (literal!("Non-constant dimension in simulation context")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_301(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_301 in &*items.clone() {
        let mut lstElt_301 = lstElt_301.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_301.clone()) {
        i_dim => {
            txt = fun_300(txt.clone(), i_dim.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_302(mut in_txt: Tpl::Text, mut in_a_subrest: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_dimrest: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_subrest.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone(), in_a_dimrest.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecls, a_preExp, _, _) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, i_subrest, a_varDecls, a_preExp, a_context, a_dimrest) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("+")).clone() }))?;
            (txt, a_preExp, a_varDecls) = threadDimSubListXml(txt.clone(), a_dimrest.clone(), i_subrest.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_303(mut in_txt: Tpl::Text, mut in_a_dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut in_a_subrest: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_sub_exp: Arc<DAE::Exp>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_dims.clone(), in_a_subrest.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone(), in_a_sub_exp.clone())) {
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: i_dimrest }, a_subrest, a_varDecls, a_preExp, a_context, a_sub_exp) => {
            let mut l_estr: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (l_estr, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_sub_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("((")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_estr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt = lm_301(txt.clone(), i_dimrest.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt, a_varDecls, a_preExp) = fun_302(txt.clone(), a_subrest.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone(), i_dimrest.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, _, a_varDecls, a_preExp, _, _) => {
            let mut txt = (*txt).clone();
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 2294, 18), (literal!("Less subscripts that dimensions in indexing cref? That's odd!")).clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_304(mut in_txt: Tpl::Text, mut in_a_subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut in_a_dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_subs.clone(), in_a_dims.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 2280, 22), (literal!("Empty dimensions in indexing cref?")).clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: i_sub_exp }, tail: i_subrest }, a_dims, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls, a_preExp) = fun_303(txt.clone(), a_dims.clone(), i_subrest.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone(), i_sub_exp.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 2295, 14), (literal!("Non-index subscript in indexing cref? That's odd!")).clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub(crate) fn threadDimSubListXml(mut txt: Tpl::Text, mut a_dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut a_subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut a_context: SimCodeFunction::Context, mut a_preExp: Tpl::Text, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = fun_304(txt.clone(), a_subs.clone(), a_dims.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_306(mut in_txt: Tpl::Text, mut in_a_sub: Arc<DAE::Subscript>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_sub.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone())) {
        (txt, Deref @ DAE::Subscript::INDEX { exp: i_exp }, a_varDecls, a_preExp, a_context) => {
            let mut l_str: Tpl::Text;
            let mut l_expPart: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (l_expPart, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            l_str = Tpl::writeText(Tpl::emptyTxt.clone(), l_expPart.clone())?;
            txt = Tpl::writeText(txt.clone(), l_str.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, Deref @ DAE::Subscript::WHOLEDIM { .. }, a_varDecls, a_preExp, _) => {
            let mut l_str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_str = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(1), (int*)0, 'W'")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_str.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, Deref @ DAE::Subscript::SLICE { exp: i_exp }, a_varDecls, a_preExp, a_context) => {
            let mut l_tmp: Tpl::Text;
            let mut l_str: Tpl::Text;
            let mut l_expPart: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (l_expPart, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_tmp, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (literal!("modelica_integer")).clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_tmp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = size_of_dimension_integer_array(&")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_expPart.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", 1);")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            l_str = Tpl::writeText(Tpl::emptyTxt.clone(), l_tmp.clone())?;
            l_str = Tpl::writeTok(l_str.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", integer_array_make_index_array(")).clone() }))?;
            l_str = Tpl::writeText(l_str.clone(), l_expPart.clone())?;
            l_str = Tpl::writeTok(l_str.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("), 'A'")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_str.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_varDecls, a_preExp, _) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn lm_307(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut a_varDecls: Tpl::Text, mut a_preExp: Tpl::Text, mut a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    let mut a_preExp: Tpl::Text = a_preExp;
    for mut lstElt_307 in &*items.clone() {
        let mut lstElt_307 = lstElt_307.clone();
        (txt, a_varDecls, a_preExp) = (::match_deref::match_deref! { match &(lstElt_307.clone()) {
        i_sub => {
            (txt, a_varDecls, a_preExp) = fun_306(txt.clone(), i_sub.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_varDecls, a_preExp))
}

pub(crate) fn daeExpCrefRhsIndexSpecXml(mut txt: Tpl::Text, mut a_subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut a_context: SimCodeFunction::Context, mut a_preExp: Tpl::Text, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut l_tmp: Tpl::Text;
    let mut l_idx__str: Tpl::Text;
    let mut ret_1: i32;
    let mut l_nridx__str: Tpl::Text;
    ret_1 = (a_subs.clone().len() as i32);
    l_nridx__str = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_1.clone())).clone())?;
    l_idx__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    (l_idx__str, out_a_varDecls, out_a_preExp) = lm_307(l_idx__str.clone(), a_subs.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone())?;
    l_idx__str = Tpl::popIter(l_idx__str.clone())?;
    (l_tmp, out_a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (literal!("index_spec_t")).clone(), out_a_varDecls.clone())?;
    out_a_preExp = Tpl::writeTok(out_a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("create_index_spec(&")).clone() }))?;
    out_a_preExp = Tpl::writeText(out_a_preExp.clone(), l_tmp.clone())?;
    out_a_preExp = Tpl::writeTok(out_a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
    out_a_preExp = Tpl::writeText(out_a_preExp.clone(), l_nridx__str.clone())?;
    out_a_preExp = Tpl::writeTok(out_a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
    out_a_preExp = Tpl::writeText(out_a_preExp.clone(), l_idx__str.clone())?;
    out_a_preExp = Tpl::writeTok(out_a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
    out_a_preExp = Tpl::writeTok(out_a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
    out_txt = Tpl::writeText(txt.clone(), l_tmp.clone())?;
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn lm_309(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_309 in &*items.clone() {
        let mut lstElt_309 = lstElt_309.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_309.clone()) {
        i_dim => {
            txt = dimensionXml(txt.clone(), i_dim.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_310(mut in_txt: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_ecr_componentRef: Arc<DAE::ComponentRef>, mut in_a_preExp: Tpl::Text, mut in_a_dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut in_a_varDecls: Tpl::Text, mut in_a_aty: Arc<DAE::Type>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_context.clone(), in_a_ecr_componentRef.clone(), in_a_preExp.clone(), in_a_dims.clone(), in_a_varDecls.clone(), in_a_aty.clone())) {
        (txt, SimCodeFunction::Context::FUNCTION_CONTEXT { cref_prefix: _, .. }, _, a_preExp, _, a_varDecls, _) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, a_ecr_componentRef, a_preExp, a_dims, a_varDecls, a_aty) => {
            let mut l_type: Tpl::Text;
            let mut l_dimsValuesStr: Tpl::Text;
            let mut ret_3: i32;
            let mut l_dimsLenStr: Tpl::Text;
            let mut txt_1: Tpl::Text;
            let mut l_tmpArr: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            txt_1 = expTypeArrayXml(Tpl::emptyTxt.clone(), a_aty.clone())?;
            (l_tmpArr, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(txt_1.clone())?).clone(), a_varDecls.clone())?;
            ret_3 = (a_dims.clone().len() as i32);
            l_dimsLenStr = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_3.clone())).clone())?;
            l_dimsValuesStr = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_dimsValuesStr = lm_309(l_dimsValuesStr.clone(), a_dims.clone())?;
            l_dimsValuesStr = Tpl::popIter(l_dimsValuesStr.clone())?;
            l_type = expTypeShortXml(Tpl::emptyTxt.clone(), a_aty.clone())?;
            a_preExp = arrayCrefXmlStr(a_preExp.clone(), a_ecr_componentRef.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), l_tmpArr.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub(crate) fn daeExpCrefRhsArrayBoxXml(mut in_txt: Tpl::Text, mut in_a_ecr: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ecr.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { ty: i_aty, dims: i_dims }, componentRef: i_ecr_componentRef }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = fun_310(txt.clone(), a_context.clone(), i_ecr_componentRef.clone(), a_preExp.clone(), i_dims.clone(), a_varDecls.clone(), i_aty.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn lm_312(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Var>>>, mut a_varDecls: Tpl::Text, mut a_preExp: Tpl::Text, mut a_context: SimCodeFunction::Context, mut a_cr: Arc<DAE::ComponentRef>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    let mut a_preExp: Tpl::Text = a_preExp;
    for mut lstElt_312 in &*items.clone() {
        let mut lstElt_312 = lstElt_312.clone();
        (txt, a_varDecls, a_preExp) = (::match_deref::match_deref! { match &(lstElt_312.clone()) {
        i_v => {
            let mut ret_0: Arc<DAE::Exp>;
            ret_0 = SimCodeFunctionUtil::makeCrefRecordExp(a_cr.clone(), i_v.clone())?;
            (txt, a_preExp, a_varDecls) = daeExpXml(txt.clone(), ret_0.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_varDecls, a_preExp))
}

pub(crate) fn daeExpRecordCrefRhsXml(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_cr: Arc<DAE::ComponentRef>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone(), in_a_cr.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Type::T_COMPLEX { complexClassType: _, varLst: i_var__lst, .. }, a_cr, a_context, a_preExp, a_varDecls) => {
            let mut l_vars: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_vars = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!(" ")).clone()], lastHasNewLine: false })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_vars, a_varDecls, a_preExp) = lm_312(l_vars.clone(), i_var__lst.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone(), a_cr.clone())?;
            l_vars = Tpl::popIter(l_vars.clone())?;
            txt = Tpl::writeText(txt.clone(), l_vars.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_314(mut in_txt: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_cr: Arc<DAE::ComponentRef>, mut in_a_t: Arc<DAE::Type>, mut in_a_varDecls: Tpl::Text, mut in_a_afterExp: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_afterExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_afterExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_context.clone(), in_a_cr.clone(), in_a_t.clone(), in_a_varDecls.clone(), in_a_afterExp.clone(), in_a_exp.clone())) {
        (txt, i_context @ SimCodeFunction::Context::FUNCTION_CONTEXT { cref_prefix: _, .. }, _, _, a_varDecls, a_afterExp, a_exp) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            (txt, a_afterExp, a_varDecls) = daeExpCrefLhs2Xml(txt.clone(), a_exp.clone(), i_context.clone(), a_afterExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        (txt, i_context, a_cr, a_t, a_varDecls, a_afterExp, _) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            (txt, a_afterExp, a_varDecls) = daeExpRecordCrefLhsXml(txt.clone(), a_t.clone(), a_cr.clone(), i_context.clone(), a_afterExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_afterExp))
}

pub(crate) fn daeExpCrefLhsXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_afterExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_afterExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_afterExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_afterExp.clone(), in_a_varDecls.clone())) {
        (txt, i_exp @ Deref @ DAE::Exp::CREF { componentRef: i_cr, ty: i_t @ Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. } }, a_context, a_afterExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls, a_afterExp) = fun_314(txt.clone(), a_context.clone(), i_cr.clone(), i_t.clone(), a_varDecls.clone(), a_afterExp.clone(), i_exp.clone())?;
            (txt.clone(), a_afterExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CREF { componentRef: i_cr, ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { builtin: _, .. } }, _, a_afterExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = crefFunctionNameXml(txt.clone(), i_cr.clone())?;
            (txt.clone(), a_afterExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CREF { componentRef: i_cr, ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_VAR { functionType: _ } }, _, a_afterExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = crefStrXml(txt.clone(), i_cr.clone())?;
            (txt.clone(), a_afterExp.clone(), a_varDecls.clone())
        },
        (txt, i_exp, a_context, a_afterExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_afterExp, a_varDecls) = daeExpCrefLhs2Xml(txt.clone(), i_exp.clone(), a_context.clone(), a_afterExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_afterExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_afterExp, out_a_varDecls))
}

fn lm_316(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut a_varDecls: Tpl::Text, mut a_afterExp: Tpl::Text, mut a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    let mut a_afterExp: Tpl::Text = a_afterExp;
    for mut lstElt_316 in &*items.clone() {
        let mut lstElt_316 = lstElt_316.clone();
        (txt, a_varDecls, a_afterExp) = (::match_deref::match_deref! { match &(lstElt_316.clone()) {
        Deref @ DAE::Subscript::INDEX { exp: i_exp } => {
            (txt, a_afterExp, a_varDecls) = daeExpXml(txt.clone(), i_exp.clone(), a_context.clone(), a_afterExp.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        _ => {
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_varDecls, a_afterExp))
}

fn fun_317(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_dimsValuesStr: Tpl::Text, mut in_a_arrName: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_dimsValuesStr.clone(), in_a_arrName.clone())) {
        (txt, Deref @ "metatype_array", a_dimsValuesStr, a_arrName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("arrayGet(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_arrName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_dimsValuesStr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") /* DAE.CREF */")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_dimsValuesStr, a_arrName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Identifier>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_arrName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\">\n")).clone(), (literal!("  <exp:ArraySubscripts>\n")).clone(), (literal!("    <exp:IndexExpression>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            txt = Tpl::writeText(txt.clone(), a_dimsValuesStr.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("    </exp:IndexExpression>\n")).clone(), (literal!("  </exp:ArraySubscripts>\n")).clone(), (literal!("</exp:QualifiedNamePart>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Identifier>")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_318(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_varDecls: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_context: SimCodeFunction::Context, mut in_a_cr: Arc<DAE::ComponentRef>, mut in_a_ecr: Arc<DAE::Exp>, mut in_a_afterExp: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_afterExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_afterExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_varDecls.clone(), in_a_ty.clone(), in_a_context.clone(), in_a_cr.clone(), in_a_ecr.clone(), in_a_afterExp.clone())) {
        (txt, false, a_varDecls, a_ty, a_context, a_cr, a_ecr, a_afterExp) => {
            let mut ret_4: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
            let mut l_spec1: Tpl::Text;
            let mut l_tmp: Tpl::Text;
            let mut l_arrayType: Tpl::Text;
            let mut l_arrName: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* daeExpCrefLhs2 SLICE(")).clone() }))?;
            a_afterExp = ExpressionDumpTpl::dumpExp(a_afterExp.clone(), a_ecr.clone(), (literal!("\"")).clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") afterExp  */")).clone() }))?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            l_arrName = contextArrayCrefXml(Tpl::emptyTxt.clone(), a_cr.clone(), a_context.clone())?;
            l_arrayType = expTypeArrayXml(Tpl::emptyTxt.clone(), a_ty.clone())?;
            (l_tmp, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_arrayType.clone())?).clone(), a_varDecls.clone())?;
            ret_4 = ComponentReferenceBasics::crefSubs(a_cr.clone())?;
            (l_spec1, a_afterExp, a_varDecls) = daeExpCrefLhsIndexSpecXml(Tpl::emptyTxt.clone(), ret_4.clone(), a_context.clone(), a_afterExp.clone(), a_varDecls.clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("indexed_assign_")).clone() }))?;
            a_afterExp = Tpl::writeText(a_afterExp.clone(), l_arrayType.clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(&")).clone() }))?;
            a_afterExp = Tpl::writeText(a_afterExp.clone(), l_tmp.clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", &")).clone() }))?;
            a_afterExp = Tpl::writeText(a_afterExp.clone(), l_arrName.clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", &")).clone() }))?;
            a_afterExp = Tpl::writeText(a_afterExp.clone(), l_spec1.clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), l_tmp.clone())?;
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        (txt, _, a_varDecls, a_ty, a_context, a_cr, a_ecr, a_afterExp) => {
            let mut str_11: ArcStr;
            let mut ret_10: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
            let mut l_dimsValuesStr: Tpl::Text;
            let mut ret_8: i32;
            let mut ret_7: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
            let mut l_dimsLenStr: Tpl::Text;
            let mut ret_5: Arc<DAE::ComponentRef>;
            let mut l_arrayType: Tpl::Text;
            let mut l_arrName: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* daeExpCrefLhs2 SCALAR(")).clone() }))?;
            a_afterExp = ExpressionDumpTpl::dumpExp(a_afterExp.clone(), a_ecr.clone(), (literal!("\"")).clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") afterExp  */")).clone() }))?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            ret_5 = ComponentReferenceBasics::crefStripLastSubs(a_cr.clone())?;
            l_arrName = contextCrefXml(Tpl::emptyTxt.clone(), ret_5.clone(), a_context.clone())?;
            l_arrayType = expTypeArrayXml(Tpl::emptyTxt.clone(), a_ty.clone())?;
            ret_7 = ComponentReferenceBasics::crefSubs(a_cr.clone())?;
            ret_8 = (ret_7.clone().len() as i32);
            l_dimsLenStr = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_8.clone())).clone())?;
            ret_10 = ComponentReferenceBasics::crefSubs(a_cr.clone())?;
            l_dimsValuesStr = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_dimsValuesStr, a_varDecls, a_afterExp) = lm_316(l_dimsValuesStr.clone(), ret_10.clone(), a_varDecls.clone(), a_afterExp.clone(), a_context.clone())?;
            l_dimsValuesStr = Tpl::popIter(l_dimsValuesStr.clone())?;
            str_11 = (Tpl::textString(l_arrayType.clone())?).clone();
            txt = fun_317(txt.clone(), (str_11.clone()).clone(), l_dimsValuesStr.clone(), l_arrName.clone())?;
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_afterExp))
}

fn fun_319(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_varDecls: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_context: SimCodeFunction::Context, mut in_a_ecr: Arc<DAE::Exp>, mut in_a_afterExp: Tpl::Text, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_afterExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_afterExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_varDecls.clone(), in_a_ty.clone(), in_a_context.clone(), in_a_ecr.clone(), in_a_afterExp.clone(), in_a_cr.clone())) {
        (txt, false, a_varDecls, a_ty, a_context, a_ecr, a_afterExp, a_cr) => {
            let mut ret_0: bool;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            ret_0 = SimCodeFunctionUtil::crefSubIsScalar(a_cr.clone())?;
            (txt, a_varDecls, a_afterExp) = fun_318(txt.clone(), ret_0.clone(), a_varDecls.clone(), a_ty.clone(), a_context.clone(), a_cr.clone(), a_ecr.clone(), a_afterExp.clone())?;
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        (txt, _, a_varDecls, _, a_context, _, a_afterExp, a_cr) => {
            let mut txt = (*txt).clone();
            txt = contextCrefXml(txt.clone(), a_cr.clone(), a_context.clone())?;
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_afterExp))
}

fn fun_320(mut in_txt: Tpl::Text, mut in_a_box: Tpl::Text, mut in_a_varDecls: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_ecr: Arc<DAE::Exp>, mut in_a_afterExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_afterExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_afterExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_box.clone(), in_a_varDecls.clone(), in_a_ty.clone(), in_a_ecr.clone(), in_a_afterExp.clone(), in_a_context.clone(), in_a_cr.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_varDecls, a_ty, a_ecr, a_afterExp, a_context, a_cr) => {
            let mut ret_0: bool;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            ret_0 = SimCodeFunctionUtil::crefIsScalar(a_cr.clone(), a_context.clone())?;
            (txt, a_varDecls, a_afterExp) = fun_319(txt.clone(), ret_0.clone(), a_varDecls.clone(), a_ty.clone(), a_context.clone(), a_ecr.clone(), a_afterExp.clone(), a_cr.clone())?;
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        (txt, i_box, a_varDecls, _, _, a_afterExp, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), i_box.clone())?;
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_afterExp))
}

pub(crate) fn daeExpCrefLhs2Xml(mut in_txt: Tpl::Text, mut in_a_ecr: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_afterExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_afterExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_afterExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ecr.clone(), in_a_context.clone(), in_a_afterExp.clone(), in_a_varDecls.clone())) {
        (txt, i_ecr @ Deref @ DAE::Exp::CREF { componentRef: i_cr, ty: i_ty }, a_context, a_afterExp, a_varDecls) => {
            let mut l_box: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* daeExpCrefLhs2 begin afterExp (")).clone() }))?;
            a_afterExp = ExpressionDumpTpl::dumpExp(a_afterExp.clone(), i_ecr.clone(), (literal!("\"")).clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") */")).clone() }))?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            (l_box, a_afterExp, a_varDecls) = daeExpCrefLhsArrayBoxXml(Tpl::emptyTxt.clone(), i_ecr.clone(), a_context.clone(), a_afterExp.clone(), a_varDecls.clone())?;
            (txt, a_varDecls, a_afterExp) = fun_320(txt.clone(), l_box.clone(), a_varDecls.clone(), i_ty.clone(), i_ecr.clone(), a_afterExp.clone(), a_context.clone(), i_cr.clone())?;
            (txt.clone(), a_afterExp.clone(), a_varDecls.clone())
        },
        (txt, i_ecr, _, a_afterExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* daeExpCrefLhs2 UNHANDLED(")).clone() }))?;
            a_afterExp = ExpressionDumpTpl::dumpExp(a_afterExp.clone(), i_ecr.clone(), (literal!("\"")).clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") afterExp */")).clone() }))?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("/* SimCodeC.tpl template: daeExpCrefLhs2: UNHANDLED EXPRESSION:\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("* ")).clone() }))?;
            txt = ExpressionDumpTpl::dumpExp(txt.clone(), i_ecr.clone(), (literal!("\"")).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*/")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            (txt.clone(), a_afterExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_afterExp, out_a_varDecls))
}

fn fun_322(mut in_txt: Tpl::Text, mut in_a_sub: Arc<DAE::Subscript>, mut in_a_varDecls: Tpl::Text, mut in_a_afterExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_afterExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_afterExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_sub.clone(), in_a_varDecls.clone(), in_a_afterExp.clone(), in_a_context.clone())) {
        (txt, Deref @ DAE::Subscript::INDEX { exp: i_exp }, a_varDecls, a_afterExp, a_context) => {
            let mut l_str: Tpl::Text;
            let mut l_expPart: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            (l_expPart, a_afterExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), a_afterExp.clone(), a_varDecls.clone())?;
            l_str = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(0), make_index_array(1, (int) ")).clone() }))?;
            l_str = Tpl::writeText(l_str.clone(), l_expPart.clone())?;
            l_str = Tpl::writeTok(l_str.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("), 'S'")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_str.clone())?;
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        (txt, Deref @ DAE::Subscript::WHOLEDIM { .. }, a_varDecls, a_afterExp, _) => {
            let mut l_str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_str = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(1), (int*)0, 'W'")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_str.clone())?;
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        (txt, Deref @ DAE::Subscript::SLICE { exp: i_exp }, a_varDecls, a_afterExp, a_context) => {
            let mut l_tmp: Tpl::Text;
            let mut l_str: Tpl::Text;
            let mut l_expPart: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            (l_expPart, a_afterExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), a_afterExp.clone(), a_varDecls.clone())?;
            (l_tmp, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (literal!("modelica_integer")).clone(), a_varDecls.clone())?;
            a_afterExp = Tpl::writeText(a_afterExp.clone(), l_tmp.clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = size_of_dimension_integer_array(&")).clone() }))?;
            a_afterExp = Tpl::writeText(a_afterExp.clone(), l_expPart.clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", 1);")).clone() }))?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            l_str = Tpl::writeText(Tpl::emptyTxt.clone(), l_tmp.clone())?;
            l_str = Tpl::writeTok(l_str.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", integer_array_make_index_array(")).clone() }))?;
            l_str = Tpl::writeText(l_str.clone(), l_expPart.clone())?;
            l_str = Tpl::writeTok(l_str.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("), 'A'")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_str.clone())?;
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        (txt, _, a_varDecls, a_afterExp, _) => {
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_afterExp))
}

fn lm_323(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut a_varDecls: Tpl::Text, mut a_afterExp: Tpl::Text, mut a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    let mut a_afterExp: Tpl::Text = a_afterExp;
    for mut lstElt_323 in &*items.clone() {
        let mut lstElt_323 = lstElt_323.clone();
        (txt, a_varDecls, a_afterExp) = (::match_deref::match_deref! { match &(lstElt_323.clone()) {
        i_sub => {
            (txt, a_varDecls, a_afterExp) = fun_322(txt.clone(), i_sub.clone(), a_varDecls.clone(), a_afterExp.clone(), a_context.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_varDecls, a_afterExp))
}

pub(crate) fn daeExpCrefLhsIndexSpecXml(mut txt: Tpl::Text, mut a_subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut a_context: SimCodeFunction::Context, mut a_afterExp: Tpl::Text, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_afterExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut l_tmp: Tpl::Text;
    let mut l_idx__str: Tpl::Text;
    let mut ret_1: i32;
    let mut l_nridx__str: Tpl::Text;
    ret_1 = (a_subs.clone().len() as i32);
    l_nridx__str = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_1.clone())).clone())?;
    l_idx__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    (l_idx__str, out_a_varDecls, out_a_afterExp) = lm_323(l_idx__str.clone(), a_subs.clone(), a_varDecls.clone(), a_afterExp.clone(), a_context.clone())?;
    l_idx__str = Tpl::popIter(l_idx__str.clone())?;
    (l_tmp, out_a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (literal!("index_spec_t")).clone(), out_a_varDecls.clone())?;
    out_a_afterExp = Tpl::writeTok(out_a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("create_index_spec(&")).clone() }))?;
    out_a_afterExp = Tpl::writeText(out_a_afterExp.clone(), l_tmp.clone())?;
    out_a_afterExp = Tpl::writeTok(out_a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
    out_a_afterExp = Tpl::writeText(out_a_afterExp.clone(), l_nridx__str.clone())?;
    out_a_afterExp = Tpl::writeTok(out_a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
    out_a_afterExp = Tpl::writeText(out_a_afterExp.clone(), l_idx__str.clone())?;
    out_a_afterExp = Tpl::writeTok(out_a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
    out_a_afterExp = Tpl::writeTok(out_a_afterExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
    out_txt = Tpl::writeText(txt.clone(), l_tmp.clone())?;
    Ok((out_txt, out_a_afterExp, out_a_varDecls))
}

fn lm_325(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_325 in &*items.clone() {
        let mut lstElt_325 = lstElt_325.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_325.clone()) {
        i_dim => {
            txt = dimensionXml(txt.clone(), i_dim.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_326(mut in_txt: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_ecr_componentRef: Arc<DAE::ComponentRef>, mut in_a_afterExp: Tpl::Text, mut in_a_dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut in_a_varDecls: Tpl::Text, mut in_a_aty: Arc<DAE::Type>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_afterExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_afterExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_context.clone(), in_a_ecr_componentRef.clone(), in_a_afterExp.clone(), in_a_dims.clone(), in_a_varDecls.clone(), in_a_aty.clone())) {
        (txt, SimCodeFunction::Context::FUNCTION_CONTEXT { cref_prefix: _, .. }, _, a_afterExp, _, a_varDecls, _) => {
            (txt.clone(), a_afterExp.clone(), a_varDecls.clone())
        },
        (txt, _, a_ecr_componentRef, a_afterExp, a_dims, a_varDecls, a_aty) => {
            let mut l_type: Tpl::Text;
            let mut l_dimsValuesStr: Tpl::Text;
            let mut ret_3: i32;
            let mut l_dimsLenStr: Tpl::Text;
            let mut txt_1: Tpl::Text;
            let mut l_tmpArr: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            txt_1 = expTypeArrayXml(Tpl::emptyTxt.clone(), a_aty.clone())?;
            (l_tmpArr, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(txt_1.clone())?).clone(), a_varDecls.clone())?;
            ret_3 = (a_dims.clone().len() as i32);
            l_dimsLenStr = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_3.clone())).clone())?;
            l_dimsValuesStr = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_dimsValuesStr = lm_325(l_dimsValuesStr.clone(), a_dims.clone())?;
            l_dimsValuesStr = Tpl::popIter(l_dimsValuesStr.clone())?;
            l_type = expTypeShortXml(Tpl::emptyTxt.clone(), a_aty.clone())?;
            a_afterExp = Tpl::writeText(a_afterExp.clone(), l_type.clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_array_create(&")).clone() }))?;
            a_afterExp = Tpl::writeText(a_afterExp.clone(), l_tmpArr.clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ((modelica_")).clone() }))?;
            a_afterExp = Tpl::writeText(a_afterExp.clone(), l_type.clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*)&(")).clone() }))?;
            a_afterExp = arrayCrefXmlStr(a_afterExp.clone(), a_ecr_componentRef.clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")), ")).clone() }))?;
            a_afterExp = Tpl::writeText(a_afterExp.clone(), l_dimsLenStr.clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            a_afterExp = Tpl::writeText(a_afterExp.clone(), l_dimsValuesStr.clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), l_tmpArr.clone())?;
            (txt.clone(), a_afterExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_afterExp, out_a_varDecls))
}

pub(crate) fn daeExpCrefLhsArrayBoxXml(mut in_txt: Tpl::Text, mut in_a_ecr: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_afterExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_afterExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_afterExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ecr.clone(), in_a_context.clone(), in_a_afterExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { ty: i_aty, dims: i_dims }, componentRef: i_ecr_componentRef }, a_context, a_afterExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_afterExp, a_varDecls) = fun_326(txt.clone(), a_context.clone(), i_ecr_componentRef.clone(), a_afterExp.clone(), i_dims.clone(), a_varDecls.clone(), i_aty.clone())?;
            (txt.clone(), a_afterExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_afterExp, a_varDecls) => {
            (txt.clone(), a_afterExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_afterExp, out_a_varDecls))
}

fn lm_328(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Var>>>, mut a_varDecls: Tpl::Text, mut a_afterExp: Tpl::Text, mut a_context: SimCodeFunction::Context, mut a_cr: Arc<DAE::ComponentRef>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    let mut a_afterExp: Tpl::Text = a_afterExp;
    for mut lstElt_328 in &*items.clone() {
        let mut lstElt_328 = lstElt_328.clone();
        (txt, a_varDecls, a_afterExp) = (::match_deref::match_deref! { match &(lstElt_328.clone()) {
        i_v => {
            let mut ret_0: Arc<DAE::Exp>;
            ret_0 = SimCodeFunctionUtil::makeCrefRecordExp(a_cr.clone(), i_v.clone())?;
            (txt, a_afterExp, a_varDecls) = daeExpXml(txt.clone(), ret_0.clone(), a_context.clone(), a_afterExp.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_varDecls, a_afterExp))
}

pub(crate) fn daeExpRecordCrefLhsXml(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_cr: Arc<DAE::ComponentRef>, mut in_a_context: SimCodeFunction::Context, mut in_a_afterExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_afterExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_afterExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone(), in_a_cr.clone(), in_a_context.clone(), in_a_afterExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Type::T_COMPLEX { complexClassType: i_record__state, varLst: i_var__lst, .. }, a_cr, a_context, a_afterExp, a_varDecls) => {
            let mut l_ret__var: Tpl::Text;
            let mut l_ret__type: Tpl::Text;
            let mut ret_2: Arc<Absyn::Path>;
            let mut l_record__type__name: Tpl::Text;
            let mut l_vars: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_vars = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_vars, a_varDecls, a_afterExp) = lm_328(l_vars.clone(), i_var__lst.clone(), a_varDecls.clone(), a_afterExp.clone(), a_context.clone(), a_cr.clone())?;
            l_vars = Tpl::popIter(l_vars.clone())?;
            ret_2 = ClassInfUtil::getStateName(i_record__state.clone());
            l_record__type__name = underscorePathXml(Tpl::emptyTxt.clone(), ret_2.clone())?;
            l_ret__type = Tpl::writeText(Tpl::emptyTxt.clone(), l_record__type__name.clone())?;
            l_ret__type = Tpl::writeTok(l_ret__type.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_rettype")).clone() }))?;
            (l_ret__var, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_ret__type.clone())?).clone(), a_varDecls.clone())?;
            a_afterExp = Tpl::writeText(a_afterExp.clone(), l_ret__var.clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = _")).clone() }))?;
            a_afterExp = Tpl::writeText(a_afterExp.clone(), l_record__type__name.clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            a_afterExp = Tpl::writeText(a_afterExp.clone(), l_vars.clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), l_ret__var.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ret__type.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_1")).clone() }))?;
            (txt.clone(), a_afterExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, _, a_afterExp, a_varDecls) => {
            (txt.clone(), a_afterExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_afterExp, out_a_varDecls))
}

fn fun_330(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone())) {
        (txt, Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_INTEGER { varLst: _ }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("integer_array")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_ENUMERATION { index: _, .. }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("integer_array")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("real_array")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_331(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone())) {
        (txt, Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_INTEGER { varLst: _ }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("integer_array")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_ENUMERATION { index: _, .. }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("integer_array")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("real_array")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_332(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone())) {
        (txt, Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_INTEGER { varLst: _ }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("integer_array")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_ENUMERATION { index: _, .. }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("integer_array")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("real_array")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_333(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone())) {
        (txt, Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_INTEGER { varLst: _ }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("integer_scalar")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_ENUMERATION { index: _, .. }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("integer_scalar")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("real_scalar")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_334(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone())) {
        (txt, Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_INTEGER { varLst: _ }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("integer")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_ENUMERATION { index: _, .. }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("integer")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("real")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_335(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone())) {
        (txt, Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_INTEGER { varLst: _ }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("integer_array")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_ENUMERATION { index: _, .. }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("integer_array")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("real_array")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_336(mut in_txt: Tpl::Text, mut in_a_operator: DAE::Operator, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_exp: Arc<DAE::Exp>, mut in_a_e2: Tpl::Text, mut in_a_e1: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_operator.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone(), in_a_exp.clone(), in_a_e2.clone(), in_a_e1.clone())) {
        (txt, DAE::Operator::ADD { ty: _ }, a_varDecls, a_preExp, _, _, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Add>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Add>")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::SUB { ty: _ }, a_varDecls, a_preExp, _, _, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Sub>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Sub>")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::MUL { ty: _ }, a_varDecls, a_preExp, _, _, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Mul>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Mul>")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::DIV { ty: _ }, a_varDecls, a_preExp, _, _, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Div>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Div>")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::POW { ty: _ }, a_varDecls, a_preExp, _, _, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Pow>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Pow>")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::UMINUS { ty: _ }, a_varDecls, a_preExp, a_context, a_exp, _, _) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_preExp, a_varDecls) = daeExpUnaryXml(txt.clone(), a_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::ADD_ARR { ty: i_ty }, a_varDecls, a_preExp, _, _, a_e2, a_e1) => {
            let mut l_var: Tpl::Text;
            let mut l_type: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            l_type = fun_330(Tpl::emptyTxt.clone(), i_ty.clone())?;
            (l_var, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_type.clone())?).clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Add>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Add>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), l_var.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::SUB_ARR { ty: i_ty }, a_varDecls, a_preExp, _, _, a_e2, a_e1) => {
            let mut l_var: Tpl::Text;
            let mut l_type: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            l_type = fun_331(Tpl::emptyTxt.clone(), i_ty.clone())?;
            (l_var, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_type.clone())?).clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Sub>\n")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Sub>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), l_var.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::MUL_ARR { ty: _ }, a_varDecls, a_preExp, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("daeExpBinary:ERR for MUL_ARR")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::DIV_ARR { ty: _ }, a_varDecls, a_preExp, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("daeExpBinary:ERR for DIV_ARR")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::MUL_ARRAY_SCALAR { ty: i_ty }, a_varDecls, a_preExp, _, _, a_e2, a_e1) => {
            let mut l_var: Tpl::Text;
            let mut l_type: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            l_type = fun_332(Tpl::emptyTxt.clone(), i_ty.clone())?;
            (l_var, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_type.clone())?).clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Mul>\n")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Mul>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), l_var.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::ADD_ARRAY_SCALAR { ty: _ }, a_varDecls, a_preExp, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("daeExpBinary:ERR for ADD_ARRAY_SCALAR")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::SUB_SCALAR_ARRAY { ty: _ }, a_varDecls, a_preExp, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("daeExpBinary:ERR for SUB_SCALAR_ARRAY")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::MUL_SCALAR_PRODUCT { ty: i_ty }, a_varDecls, a_preExp, _, _, a_e2, a_e1) => {
            let mut l_type: Tpl::Text;
            let mut txt = (*txt).clone();
            l_type = fun_333(Tpl::emptyTxt.clone(), i_ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mul_")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_type.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_product(&")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", &")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::MUL_MATRIX_PRODUCT { ty: i_ty }, a_varDecls, a_preExp, _, _, a_e2, a_e1) => {
            let mut l_typeShort: Tpl::Text;
            let mut l_var: Tpl::Text;
            let mut l_type: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            l_typeShort = fun_334(Tpl::emptyTxt.clone(), i_ty.clone())?;
            l_type = Tpl::writeText(Tpl::emptyTxt.clone(), l_typeShort.clone())?;
            l_type = Tpl::writeTok(l_type.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_array")).clone() }))?;
            (l_var, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_type.clone())?).clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Mul>\n")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Mul>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), l_var.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::DIV_ARRAY_SCALAR { ty: i_ty }, a_varDecls, a_preExp, _, _, a_e2, a_e1) => {
            let mut l_var: Tpl::Text;
            let mut l_type: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            l_type = fun_335(Tpl::emptyTxt.clone(), i_ty.clone())?;
            (l_var, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_type.clone())?).clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Div>\n")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Div>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), l_var.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::DIV_SCALAR_ARRAY { ty: _ }, a_varDecls, a_preExp, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("daeExpBinary:ERR for DIV_SCALAR_ARRAY")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::POW_ARRAY_SCALAR { ty: _ }, a_varDecls, a_preExp, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("daeExpBinary:ERR for POW_ARRAY_SCALAR")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::POW_SCALAR_ARRAY { ty: _ }, a_varDecls, a_preExp, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("daeExpBinary:ERR for POW_SCALAR_ARRAY")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::POW_ARR { ty: _ }, a_varDecls, a_preExp, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("daeExpBinary:ERR for POW_ARR")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::POW_ARR2 { ty: _ }, a_varDecls, a_preExp, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("daeExpBinary:ERR for POW_ARR2")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_varDecls, a_preExp, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("daeExpBinary:ERR")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

pub(crate) fn daeExpBinaryXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, i_exp @ Deref @ DAE::Exp::BINARY { exp1: i_exp1, exp2: i_exp2, operator: i_operator }, a_context, a_preExp, a_varDecls) => {
            let mut l_e2: Tpl::Text;
            let mut l_e1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_e1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_e2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt, a_varDecls, a_preExp) = fun_336(txt.clone(), i_operator.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone(), i_exp.clone(), l_e2.clone(), l_e1.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_338(mut in_txt: Tpl::Text, mut in_a_operator: DAE::Operator, mut in_a_e: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_operator.clone(), in_a_e.clone())) {
        (txt, DAE::Operator::UMINUS { ty: _ }, a_e) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  <exp:Neg>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), a_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("  </exp:Neg>")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::UMINUS_ARR { ty: Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_REAL { varLst: _ }, .. } }, a_e) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  <exp:Neg>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), a_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("  </exp:Neg>")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::UMINUS_ARR { ty: _ }, _) => {
            let mut txt = (*txt).clone();
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 2675, 34), (literal!("unary minus for non-real arrays not implemented")).clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            let mut txt = (*txt).clone();
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 2676, 14), (literal!("daeExpUnary:ERR")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn daeExpUnaryXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::UNARY { exp: i_exp, operator: i_operator }, a_context, a_preExp, a_varDecls) => {
            let mut l_e: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_e, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = fun_338(txt.clone(), i_operator.clone(), l_e.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_340(mut in_txt: Tpl::Text, mut in_a_operator: DAE::Operator, mut in_a_e2: Tpl::Text, mut in_a_e1: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_operator.clone(), in_a_e2.clone(), in_a_e1.clone()) {
        (mut txt, DAE::Operator::AND { ty: _ }, mut a_e2, mut a_e1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:And>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:And>")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::OR { ty: _ }, mut a_e2, mut a_e1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Or>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Or>")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("daeExpLbinary:ERR")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn daeExpLbinaryXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::LBINARY { exp1: i_exp1, exp2: i_exp2, operator: i_operator }, a_context, a_preExp, a_varDecls) => {
            let mut l_e2: Tpl::Text;
            let mut l_e1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_e1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_e2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = fun_340(txt.clone(), i_operator.clone(), l_e2.clone(), l_e1.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_342(mut in_txt: Tpl::Text, mut in_a_operator: DAE::Operator, mut in_a_e: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_operator.clone(), in_a_e.clone()) {
        (mut txt, DAE::Operator::NOT { ty: _ }, mut a_e) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Not>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Not>")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn daeExpLunaryXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::LUNARY { exp: i_exp, operator: i_operator }, a_context, a_preExp, a_varDecls) => {
            let mut l_e: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_e, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = fun_342(txt.clone(), i_operator.clone(), l_e.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_344(mut in_txt: Tpl::Text, mut in_a_rel_operator: DAE::Operator, mut in_a_e2: Tpl::Text, mut in_a_e1: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_rel_operator.clone(), in_a_e2.clone(), in_a_e1.clone())) {
        (txt, DAE::Operator::LESS { ty: Deref @ DAE::Type::T_BOOL { varLst: _ } }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(!")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" && ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::LESS { ty: Deref @ DAE::Type::T_STRING { varLst: _ } }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(stringCompare(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") < 0)")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::LESS { ty: _ }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:LogLt>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogLt>")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::GREATER { ty: Deref @ DAE::Type::T_BOOL { varLst: _ } }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" && !")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::GREATER { ty: Deref @ DAE::Type::T_STRING { varLst: _ } }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(stringCompare(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") > 0)")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::GREATER { ty: _ }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:LogGt>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogGt>")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::LESSEQ { ty: Deref @ DAE::Type::T_BOOL { varLst: _ } }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(!")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" || ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::LESSEQ { ty: Deref @ DAE::Type::T_STRING { varLst: _ } }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(stringCompare(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") <= 0)")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::LESSEQ { ty: _ }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:LogLeq>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogLeq>")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::GREATEREQ { ty: Deref @ DAE::Type::T_BOOL { varLst: _ } }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" || !")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::GREATEREQ { ty: Deref @ DAE::Type::T_STRING { varLst: _ } }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(stringCompare(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") >= 0)")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::GREATEREQ { ty: _ }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:LogGeq>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogGeq>")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::EQUAL { ty: Deref @ DAE::Type::T_BOOL { varLst: _ } }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("((!")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" && !")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") || (")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" && ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::EQUAL { ty: Deref @ DAE::Type::T_STRING { varLst: _ } }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(stringEqual(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::EQUAL { ty: _ }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:LogEq>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogEq>")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::NEQUAL { ty: Deref @ DAE::Type::T_BOOL { varLst: _ } }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("((!")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" && ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") || (")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" && !")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::NEQUAL { ty: Deref @ DAE::Type::T_STRING { varLst: _ } }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(!stringEqual(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::NEQUAL { ty: _ }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:LogNeq>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogNeq>")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("daeExpRelation:ERR")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_345(mut in_txt: Tpl::Text, mut in_a_simRel: Tpl::Text, mut in_a_rel_operator: DAE::Operator, mut in_a_rel_exp2: Arc<DAE::Exp>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_rel_exp1: Arc<DAE::Exp>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_simRel.clone(), in_a_rel_operator.clone(), in_a_rel_exp2.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone(), in_a_rel_exp1.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_rel_operator, a_rel_exp2, a_varDecls, a_preExp, a_context, a_rel_exp1) => {
            let mut l_e2: Tpl::Text;
            let mut l_e1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (l_e1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_rel_exp1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_e2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_rel_exp2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = fun_344(txt.clone(), a_rel_operator.clone(), l_e2.clone(), l_e1.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, i_simRel, _, _, a_varDecls, a_preExp, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), i_simRel.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

pub(crate) fn daeExpRelationXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, i_rel @ Deref @ DAE::Exp::RELATION { exp1: i_rel_exp1, exp2: i_rel_exp2, operator: i_rel_operator, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_simRel: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_simRel, a_preExp, a_varDecls) = daeExpRelationSimXml(Tpl::emptyTxt.clone(), i_rel.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt, a_varDecls, a_preExp) = fun_345(txt.clone(), l_simRel.clone(), i_rel_operator.clone(), i_rel_exp2.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone(), i_rel_exp1.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_347(mut in_txt: Tpl::Text, mut in_a_rel_operator: DAE::Operator, mut in_a_res: Tpl::Text, mut in_a_e2: Tpl::Text, mut in_a_e1: Tpl::Text, mut in_a_preExp: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_preExp) = (match (in_txt.clone(), in_a_rel_operator.clone(), in_a_res.clone(), in_a_e2.clone(), in_a_e1.clone(), in_a_preExp.clone()) {
        (mut txt, DAE::Operator::LESS { ty: _ }, mut a_res, mut a_e2, mut a_e1, mut a_preExp) => {
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:LogLt>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogLt>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            (txt.clone(), a_preExp.clone())
        },
        (mut txt, DAE::Operator::LESSEQ { ty: _ }, mut a_res, mut a_e2, mut a_e1, mut a_preExp) => {
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:LogLeq>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogLeq>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            (txt.clone(), a_preExp.clone())
        },
        (mut txt, DAE::Operator::GREATER { ty: _ }, mut a_res, mut a_e2, mut a_e1, mut a_preExp) => {
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:LogGt>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogGt>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            (txt.clone(), a_preExp.clone())
        },
        (mut txt, DAE::Operator::GREATEREQ { ty: _ }, mut a_res, mut a_e2, mut a_e1, mut a_preExp) => {
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:LogGeq>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogGeq>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            (txt.clone(), a_preExp.clone())
        },
        (mut txt, _, _, _, _, mut a_preExp) => {
            (txt.clone(), a_preExp.clone())
        },
    });
    Ok((out_txt, out_a_preExp))
}

fn fun_348(mut in_txt: Tpl::Text, mut in_a_rel_operator: DAE::Operator, mut in_a_res: Tpl::Text, mut in_a_e2: Tpl::Text, mut in_a_e1: Tpl::Text, mut in_a_preExp: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_preExp) = (match (in_txt.clone(), in_a_rel_operator.clone(), in_a_res.clone(), in_a_e2.clone(), in_a_e1.clone(), in_a_preExp.clone()) {
        (mut txt, DAE::Operator::LESS { ty: _ }, mut a_res, mut a_e2, mut a_e1, mut a_preExp) => {
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  <exp:LogLt>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogLt>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            (txt.clone(), a_preExp.clone())
        },
        (mut txt, DAE::Operator::LESSEQ { ty: _ }, mut a_res, mut a_e2, mut a_e1, mut a_preExp) => {
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  <exp:LogLeq>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:LogLeq>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            (txt.clone(), a_preExp.clone())
        },
        (mut txt, DAE::Operator::GREATER { ty: _ }, mut a_res, mut a_e2, mut a_e1, mut a_preExp) => {
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  <exp:LogGt>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogGt>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            (txt.clone(), a_preExp.clone())
        },
        (mut txt, DAE::Operator::GREATEREQ { ty: _ }, mut a_res, mut a_e2, mut a_e1, mut a_preExp) => {
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  <exp:LogGeq>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogGeq>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            (txt.clone(), a_preExp.clone())
        },
        (mut txt, _, _, _, _, mut a_preExp) => {
            (txt.clone(), a_preExp.clone())
        },
    });
    Ok((out_txt, out_a_preExp))
}

fn fun_349(mut in_txt: Tpl::Text, mut in_a_rel_optionExpisASUB: Option<(Arc<DAE::Exp>, i32, i32)>, mut in_a_rel_operator: DAE::Operator, mut in_a_rel_exp2: Arc<DAE::Exp>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_rel_exp1: Arc<DAE::Exp>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_rel_optionExpisASUB.clone(), in_a_rel_operator.clone(), in_a_rel_exp2.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone(), in_a_rel_exp1.clone())) {
        (txt, None, a_rel_operator, a_rel_exp2, a_varDecls, a_preExp, a_context, a_rel_exp1) => {
            let mut l_res: Tpl::Text;
            let mut l_e2: Tpl::Text;
            let mut l_e1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (l_e1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_rel_exp1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_e2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_rel_exp2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_res, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (literal!("modelica_boolean")).clone(), a_varDecls.clone())?;
            (txt, a_preExp) = fun_347(txt.clone(), a_rel_operator.clone(), l_res.clone(), l_e2.clone(), l_e1.clone(), a_preExp.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, Some((i_exp, _, _)), a_rel_operator, a_rel_exp2, a_varDecls, a_preExp, a_context, a_rel_exp1) => {
            let mut l_iterator: Tpl::Text;
            let mut l_res: Tpl::Text;
            let mut l_e2: Tpl::Text;
            let mut l_e1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (l_e1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_rel_exp1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_e2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_rel_exp2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_iterator, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_res, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (literal!("modelica_boolean")).clone(), a_varDecls.clone())?;
            (txt, a_preExp) = fun_348(txt.clone(), a_rel_operator.clone(), l_res.clone(), l_e2.clone(), l_e1.clone(), a_preExp.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, _, _, a_varDecls, a_preExp, _, _) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_350(mut in_txt: Tpl::Text, mut in_a_rel_operator: DAE::Operator, mut in_a_res: Tpl::Text, mut in_a_e2: Tpl::Text, mut in_a_e1: Tpl::Text, mut in_a_preExp: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_preExp) = (match (in_txt.clone(), in_a_rel_operator.clone(), in_a_res.clone(), in_a_e2.clone(), in_a_e1.clone(), in_a_preExp.clone()) {
        (mut txt, DAE::Operator::LESS { ty: _ }, mut a_res, mut a_e2, mut a_e1, mut a_preExp) => {
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:LogLt>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogLt>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            (txt.clone(), a_preExp.clone())
        },
        (mut txt, DAE::Operator::LESSEQ { ty: _ }, mut a_res, mut a_e2, mut a_e1, mut a_preExp) => {
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:LogLeq>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogLeq>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            (txt.clone(), a_preExp.clone())
        },
        (mut txt, DAE::Operator::GREATER { ty: _ }, mut a_res, mut a_e2, mut a_e1, mut a_preExp) => {
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:LogGt>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogGt>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            (txt.clone(), a_preExp.clone())
        },
        (mut txt, DAE::Operator::GREATEREQ { ty: _ }, mut a_res, mut a_e2, mut a_e1, mut a_preExp) => {
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:LogGeq>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogGeq>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            (txt.clone(), a_preExp.clone())
        },
        (mut txt, _, _, _, _, mut a_preExp) => {
            (txt.clone(), a_preExp.clone())
        },
    });
    Ok((out_txt, out_a_preExp))
}

fn fun_351(mut in_txt: Tpl::Text, mut in_a_rel_operator: DAE::Operator, mut in_a_res: Tpl::Text, mut in_a_e2: Tpl::Text, mut in_a_e1: Tpl::Text, mut in_a_preExp: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_preExp) = (match (in_txt.clone(), in_a_rel_operator.clone(), in_a_res.clone(), in_a_e2.clone(), in_a_e1.clone(), in_a_preExp.clone()) {
        (mut txt, DAE::Operator::LESS { ty: _ }, mut a_res, mut a_e2, mut a_e1, mut a_preExp) => {
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("    <exp:LogLt>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogLt>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            (txt.clone(), a_preExp.clone())
        },
        (mut txt, DAE::Operator::LESSEQ { ty: _ }, mut a_res, mut a_e2, mut a_e1, mut a_preExp) => {
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("    <exp:LogLeq>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogLeq>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            (txt.clone(), a_preExp.clone())
        },
        (mut txt, DAE::Operator::GREATER { ty: _ }, mut a_res, mut a_e2, mut a_e1, mut a_preExp) => {
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("    <exp:LogGt>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogGt>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            (txt.clone(), a_preExp.clone())
        },
        (mut txt, DAE::Operator::GREATEREQ { ty: _ }, mut a_res, mut a_e2, mut a_e1, mut a_preExp) => {
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("    <exp:LogGeq>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogGeq>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            (txt.clone(), a_preExp.clone())
        },
        (mut txt, _, _, _, _, mut a_preExp) => {
            (txt.clone(), a_preExp.clone())
        },
    });
    Ok((out_txt, out_a_preExp))
}

fn fun_352(mut in_txt: Tpl::Text, mut in_a_rel_optionExpisASUB: Option<(Arc<DAE::Exp>, i32, i32)>, mut in_a_rel_operator: DAE::Operator, mut in_a_rel_exp2: Arc<DAE::Exp>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_rel_exp1: Arc<DAE::Exp>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_rel_optionExpisASUB.clone(), in_a_rel_operator.clone(), in_a_rel_exp2.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone(), in_a_rel_exp1.clone())) {
        (txt, None, a_rel_operator, a_rel_exp2, a_varDecls, a_preExp, a_context, a_rel_exp1) => {
            let mut l_res: Tpl::Text;
            let mut l_e2: Tpl::Text;
            let mut l_e1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (l_e1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_rel_exp1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_e2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_rel_exp2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_res, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (literal!("modelica_boolean")).clone(), a_varDecls.clone())?;
            (txt, a_preExp) = fun_350(txt.clone(), a_rel_operator.clone(), l_res.clone(), l_e2.clone(), l_e1.clone(), a_preExp.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, Some((i_exp, _, _)), a_rel_operator, a_rel_exp2, a_varDecls, a_preExp, a_context, a_rel_exp1) => {
            let mut l_iterator: Tpl::Text;
            let mut l_res: Tpl::Text;
            let mut l_e2: Tpl::Text;
            let mut l_e1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (l_e1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_rel_exp1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_e2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_rel_exp2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_res, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (literal!("modelica_boolean")).clone(), a_varDecls.clone())?;
            (l_iterator, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt, a_preExp) = fun_351(txt.clone(), a_rel_operator.clone(), l_res.clone(), l_e2.clone(), l_e1.clone(), a_preExp.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, _, _, a_varDecls, a_preExp, _, _) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_353(mut in_txt: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_rel_operator: DAE::Operator, mut in_a_rel_exp2: Arc<DAE::Exp>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_rel_exp1: Arc<DAE::Exp>, mut in_a_rel_optionExpisASUB: Option<(Arc<DAE::Exp>, i32, i32)>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_context.clone(), in_a_rel_operator.clone(), in_a_rel_exp2.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_rel_exp1.clone(), in_a_rel_optionExpisASUB.clone())) {
        (txt, i_context @ SimCodeFunction::Context::SIMULATION_CONTEXT { genDiscrete: false }, a_rel_operator, a_rel_exp2, a_varDecls, a_preExp, a_rel_exp1, a_rel_optionExpisASUB) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_varDecls, a_preExp) = fun_349(txt.clone(), a_rel_optionExpisASUB.clone(), a_rel_operator.clone(), a_rel_exp2.clone(), a_varDecls.clone(), a_preExp.clone(), i_context.clone(), a_rel_exp1.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, i_context @ SimCodeFunction::Context::SIMULATION_CONTEXT { genDiscrete: true }, a_rel_operator, a_rel_exp2, a_varDecls, a_preExp, a_rel_exp1, a_rel_optionExpisASUB) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_varDecls, a_preExp) = fun_352(txt.clone(), a_rel_optionExpisASUB.clone(), a_rel_operator.clone(), a_rel_exp2.clone(), a_varDecls.clone(), a_preExp.clone(), i_context.clone(), a_rel_exp1.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, _, _, a_varDecls, a_preExp, _, _) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

pub(crate) fn daeExpRelationSimXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::RELATION { optionExpisASUB: i_rel_optionExpisASUB, exp1: i_rel_exp1, exp2: i_rel_exp2, operator: i_rel_operator, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls, a_preExp) = fun_353(txt.clone(), a_context.clone(), i_rel_operator.clone(), i_rel_exp2.clone(), a_varDecls.clone(), a_preExp.clone(), i_rel_exp1.clone(), i_rel_optionExpisASUB.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_355(mut in_txt: Tpl::Text, mut in_a_rel_operator: DAE::Operator, mut in_a_e2: Tpl::Text, mut in_a_e1: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_rel_operator.clone(), in_a_e2.clone(), in_a_e1.clone()) {
        (mut txt, DAE::Operator::EQUAL { ty: _ }, mut a_e2, mut a_e1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<opt:ConstraintEqu>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</opt:ConstraintEqu>")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::LESSEQ { ty: _ }, mut a_e2, mut a_e1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<opt:ConstraintLeq>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</opt:ConstraintLeq>")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::GREATEREQ { ty: _ }, mut a_e2, mut a_e1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<opt:ConstraintGeq>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</opt:ConstraintGeq>")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("  \"The XML schema does only support =, >= , <=  operators for constraints\"")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_356(mut in_txt: Tpl::Text, mut in_a_rel_optionExpisASUB: Option<(Arc<DAE::Exp>, i32, i32)>, mut in_a_rel_operator: DAE::Operator, mut in_a_rel_exp2: Arc<DAE::Exp>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_rel_exp1: Arc<DAE::Exp>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_rel_optionExpisASUB.clone(), in_a_rel_operator.clone(), in_a_rel_exp2.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone(), in_a_rel_exp1.clone())) {
        (txt, None, a_rel_operator, a_rel_exp2, a_varDecls, a_preExp, a_context, a_rel_exp1) => {
            let mut l_res: Tpl::Text;
            let mut l_e2: Tpl::Text;
            let mut l_e1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (l_e1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_rel_exp1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_e2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_rel_exp2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_res, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (literal!("modelica_boolean")).clone(), a_varDecls.clone())?;
            txt = fun_355(txt.clone(), a_rel_operator.clone(), l_e2.clone(), l_e1.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, _, _, a_varDecls, a_preExp, _, _) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_357(mut in_txt: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_rel_operator: DAE::Operator, mut in_a_rel_exp2: Arc<DAE::Exp>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_rel_exp1: Arc<DAE::Exp>, mut in_a_rel_optionExpisASUB: Option<(Arc<DAE::Exp>, i32, i32)>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_context.clone(), in_a_rel_operator.clone(), in_a_rel_exp2.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_rel_exp1.clone(), in_a_rel_optionExpisASUB.clone())) {
        (txt, i_context @ SimCodeFunction::Context::SIMULATION_CONTEXT { genDiscrete: true }, a_rel_operator, a_rel_exp2, a_varDecls, a_preExp, a_rel_exp1, a_rel_optionExpisASUB) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_varDecls, a_preExp) = fun_356(txt.clone(), a_rel_optionExpisASUB.clone(), a_rel_operator.clone(), a_rel_exp2.clone(), a_varDecls.clone(), a_preExp.clone(), i_context.clone(), a_rel_exp1.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, _, _, a_varDecls, a_preExp, _, _) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

pub(crate) fn daeExpConstraintXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::RELATION { optionExpisASUB: i_rel_optionExpisASUB, exp1: i_rel_exp1, exp2: i_rel_exp2, operator: i_rel_operator, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls, a_preExp) = fun_357(txt.clone(), a_context.clone(), i_rel_operator.clone(), i_rel_exp2.clone(), a_varDecls.clone(), a_preExp.clone(), i_rel_exp1.clone(), i_rel_optionExpisASUB.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub(crate) fn daeExpIfXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::IFEXP { expCond: i_expCond, expThen: i_expThen, expElse: i_expElse }, a_context, a_preExp, a_varDecls) => {
            let mut l_eElse: Tpl::Text;
            let mut l_preExpElse: Tpl::Text;
            let mut l_eThen: Tpl::Text;
            let mut l_preExpThen: Tpl::Text;
            let mut l_resVar: Tpl::Text;
            let mut l_condExp: Tpl::Text;
            let mut l_preExpCond: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExpCond = Tpl::emptyTxt.clone();
            (l_condExp, l_preExpCond, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_expCond.clone(), a_context.clone(), l_preExpCond.clone(), a_varDecls.clone())?;
            l_resVar = Tpl::emptyTxt.clone();
            l_preExpThen = Tpl::emptyTxt.clone();
            (l_eThen, l_preExpThen, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_expThen.clone(), a_context.clone(), l_preExpThen.clone(), a_varDecls.clone())?;
            l_preExpElse = Tpl::emptyTxt.clone();
            (l_eElse, l_preExpElse, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_expElse.clone(), a_context.clone(), l_preExpElse.clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:If>\n")).clone(), (literal!("  <fun:Condition>\n")).clone()], lastHasNewLine: true }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_condExp.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Condition>\n")).clone(), (literal!("  <fun:Statements>\n")).clone()], lastHasNewLine: true }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_eThen.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Statements>\n")).clone(), (literal!("  <fun:Else>\n")).clone()], lastHasNewLine: true }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_eElse.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Else>\n")).clone(), (literal!("</fun:If>")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_resVar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_360(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone())) {
        (txt, Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_INTEGER { varLst: _ }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("integer_array")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_ENUMERATION { index: _, .. }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("integer_array")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("real_array")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_361(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_var1: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_var1.clone()) {
        (mut txt, false, mut a_var1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("puts(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_var1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, mut a_var1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("print(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_var1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_362(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut a_varDecls: Tpl::Text, mut a_preExp: Tpl::Text, mut a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    let mut a_preExp: Tpl::Text = a_preExp;
    for mut lstElt_362 in &*items.clone() {
        let mut lstElt_362 = lstElt_362.clone();
        (txt, a_varDecls, a_preExp) = (::match_deref::match_deref! { match &(lstElt_362.clone()) {
        i_dim => {
            (txt, a_preExp, a_varDecls) = daeExpXml(txt.clone(), i_dim.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_varDecls, a_preExp))
}

fn lm_363(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut a_varDecls: Tpl::Text, mut a_preExp: Tpl::Text, mut a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    let mut a_preExp: Tpl::Text = a_preExp;
    for mut lstElt_363 in &*items.clone() {
        let mut lstElt_363 = lstElt_363.clone();
        (txt, a_varDecls, a_preExp) = (::match_deref::match_deref! { match &(lstElt_363.clone()) {
        i_array => {
            (txt, a_preExp, a_varDecls) = daeExpXml(txt.clone(), i_array.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_varDecls, a_preExp))
}

fn lm_364(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut a_varDecls: Tpl::Text, mut a_preExp: Tpl::Text, mut a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    let mut a_preExp: Tpl::Text = a_preExp;
    for mut lstElt_364 in &*items.clone() {
        let mut lstElt_364 = lstElt_364.clone();
        (txt, a_varDecls, a_preExp) = (::match_deref::match_deref! { match &(lstElt_364.clone()) {
        i_exp => {
            (txt, a_preExp, a_varDecls) = daeExpXml(txt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_varDecls, a_preExp))
}

fn fun_365(mut in_txt: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_argStr: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_preExp.clone(), in_a_argStr.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_argStr) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_argStr.clone())?;
            txt.clone()
        },
        (txt, i_preExp, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), i_preExp.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_366(mut in_txt: Tpl::Text, mut in_a_attr_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_attr_ty.clone())) {
        (txt, Deref @ DAE::Type::T_NORETCALL { .. }) => {
            txt.clone()
        },
        (txt, i_attr_ty) => {
            let mut txt = (*txt).clone();
            txt = expTypeModelicaXml(txt.clone(), i_attr_ty.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_367(mut in_txt: Tpl::Text, mut in_a_attr_builtin: bool, mut in_a_attr_ty: Arc<DAE::Type>, mut in_a_funName: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_attr_builtin.clone(), in_a_attr_ty.clone(), in_a_funName.clone())) {
        (txt, false, _, a_funName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_funName.clone())?;
            txt.clone()
        },
        (txt, _, a_attr_ty, _) => {
            let mut txt = (*txt).clone();
            txt = fun_366(txt.clone(), a_attr_ty.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_368(mut in_txt: Tpl::Text, mut in_a_attr_ty: Arc<DAE::Type>, mut in_a_varDecls: Tpl::Text, mut in_a_retType: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_attr_ty.clone(), in_a_varDecls.clone(), in_a_retType.clone())) {
        (txt, Deref @ DAE::Type::T_NORETCALL { .. }, a_varDecls, _) => {
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, a_varDecls, a_retType) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = tempDeclXml(txt.clone(), (Tpl::textString(a_retType.clone())?).clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

fn fun_369(mut in_txt: Tpl::Text, mut in_a_attr_builtin: bool, mut in_a_builtinFunctionName: Tpl::Text, mut in_a_result: Tpl::Text, mut in_a_funName: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_attr_builtin.clone(), in_a_builtinFunctionName.clone(), in_a_result.clone(), in_a_funName.clone()) {
        (mut txt, false, _, mut a_result, mut a_funName) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<exp:FunctionCall>\n")).clone(), (literal!("  <exp:Name>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), a_funName.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </exp:Name>\n")).clone(), (literal!("  <exp:Arguments>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), a_result.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </exp:Arguments>\n")).clone(), (literal!("</exp:FunctionCall>")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (mut txt, _, mut a_builtinFunctionName, mut a_result, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_builtinFunctionName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(">\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_result.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_builtinFunctionName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(">")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_370(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_builtinFunctionName: Tpl::Text, mut in_a_result: Tpl::Text, mut in_a_funName: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_builtinFunctionName.clone(), in_a_result.clone(), in_a_funName.clone())) {
        (txt, Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_NORETCALL { .. }, .. }, .. }, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* NORETCALL */")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { tuple_: false, builtin: i_attr_builtin, .. }, .. }, a_builtinFunctionName, a_result, a_funName) => {
            let mut txt = (*txt).clone();
            txt = fun_369(txt.clone(), i_attr_builtin.clone(), a_builtinFunctionName.clone(), a_result.clone(), a_funName.clone())?;
            txt.clone()
        },
        (txt, _, _, a_result, a_funName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<exp:FunctionCall>\n")).clone(), (literal!("  <exp:Name>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), a_funName.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </exp:Name>\n")).clone(), (literal!("  <exp:Arguments>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), a_result.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </exp:Arguments>\n")).clone(), (literal!("</exp:FunctionCall>")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn daeExpCallXml(mut in_txt: Tpl::Text, mut in_a_call: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_call.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "DIVISION" }, expLst: Deref @ metamodelica::List::Cons { head: i_e1, tail: Deref @ metamodelica::List::Cons { head: i_e2, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::SCONST { string: i_string }, tail: Deref @ metamodelica::List::Nil } } }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut ret_3: ArcStr;
            let mut l_var3: Tpl::Text;
            let mut l_var2: Tpl::Text;
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_var2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            ret_3 = (Util::escapeModelicaStringToXmlString((i_string.clone()).clone())?).clone();
            l_var3 = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_3.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Div>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_var1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_var2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Div>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: i_ty, .. }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "DIVISION_ARRAY_SCALAR" }, expLst: Deref @ metamodelica::List::Cons { head: i_e1, tail: Deref @ metamodelica::List::Cons { head: i_e2, tail: Deref @ metamodelica::List::Cons { head: i_e3 @ Deref @ DAE::Exp::SHARED_LITERAL { index: _, .. }, tail: Deref @ metamodelica::List::Nil } } } }, a_context, a_preExp, a_varDecls) => {
            let mut l_var: Tpl::Text;
            let mut l_type: Tpl::Text;
            let mut l_var3: Tpl::Text;
            let mut l_var2: Tpl::Text;
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_type = fun_360(Tpl::emptyTxt.clone(), i_ty.clone())?;
            (l_var, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_type.clone())?).clone(), a_varDecls.clone())?;
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_var2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_var3, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e3.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("division_alloc_")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_type.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_scalar(&")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_var1.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_var2.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", &")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_var.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_var3.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), l_var.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_exp @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "DIVISION_ARRAY_SCALAR" }, .. }, _, a_preExp, a_varDecls) => {
            let mut txt_6: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_6 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Code generation does not support ")).clone() }))?;
            txt_6 = ExpressionDumpTpl::dumpExp(txt_6.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 3088, 11), (Tpl::textString(txt_6.clone())?).clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: i_arg_componentRef, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Der>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = crefXml(txt.clone(), i_arg_componentRef.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Der>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: i_exp, tail: Deref @ metamodelica::List::Nil }, .. }, _, a_preExp, a_varDecls) => {
            let mut txt_7: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_7 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Code generation does not support der(")).clone() }))?;
            txt_7 = ExpressionDumpTpl::dumpExp(txt_7.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt_7 = Tpl::writeTok(txt_7.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 3097, 11), (Tpl::textString(txt_7.clone())?).clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, expLst: Deref @ metamodelica::List::Cons { head: i_arg, tail: Deref @ metamodelica::List::Nil }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpCallPreXml(txt.clone(), i_arg.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: i_arg_componentRef, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = crefXml(txt.clone(), i_arg_componentRef.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, expLst: Deref @ metamodelica::List::Cons { head: i_exp, tail: Deref @ metamodelica::List::Nil }, .. }, _, a_preExp, a_varDecls) => {
            let mut txt_8: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_8 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Code generation does not support edge(")).clone() }))?;
            txt_8 = ExpressionDumpTpl::dumpExp(txt_8.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt_8 = Tpl::writeTok(txt_8.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 3105, 11), (Tpl::textString(txt_8.clone())?).clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: i_arg_componentRef, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = crefXml(txt.clone(), i_arg_componentRef.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, expLst: Deref @ metamodelica::List::Cons { head: i_exp, tail: Deref @ metamodelica::List::Nil }, .. }, _, a_preExp, a_varDecls) => {
            let mut txt_9: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_9 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Code generation does not support change(")).clone() }))?;
            txt_9 = ExpressionDumpTpl::dumpExp(txt_9.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt_9 = Tpl::writeTok(txt_9.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 3111, 11), (Tpl::textString(txt_9.clone())?).clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "print" }, expLst: Deref @ metamodelica::List::Cons { head: i_e1, tail: Deref @ metamodelica::List::Nil }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut ret_10: bool;
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            ret_10 = Config::acceptMetaModelicaGrammar()?;
            txt = fun_361(txt.clone(), ret_10.clone(), l_var1.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "max" }, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_REAL { varLst: _ }, .. }, expLst: Deref @ metamodelica::List::Cons { head: i_e1, tail: Deref @ metamodelica::List::Cons { head: i_e2, tail: Deref @ metamodelica::List::Nil } } }, a_context, a_preExp, a_varDecls) => {
            let mut l_var2: Tpl::Text;
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_var2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Max>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_var1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_var2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Max>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "max" }, expLst: Deref @ metamodelica::List::Cons { head: i_e1, tail: Deref @ metamodelica::List::Cons { head: i_e2, tail: Deref @ metamodelica::List::Nil } }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_var2: Tpl::Text;
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_var2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Max>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_var1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_var2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Max>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sum" }, attr: Deref @ DAE::CallAttributes { ty: i_ty, .. }, expLst: Deref @ metamodelica::List::Cons { head: i_e, tail: Deref @ metamodelica::List::Nil } }, a_context, a_preExp, a_varDecls) => {
            let mut l_ty__str: Tpl::Text;
            let mut l_arr: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_arr, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            l_ty__str = expTypeArrayXml(Tpl::emptyTxt.clone(), i_ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("sum_")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ty__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(&")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_arr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "min" }, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_REAL { varLst: _ }, .. }, expLst: Deref @ metamodelica::List::Cons { head: i_e1, tail: Deref @ metamodelica::List::Cons { head: i_e2, tail: Deref @ metamodelica::List::Nil } } }, a_context, a_preExp, a_varDecls) => {
            let mut l_var2: Tpl::Text;
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_var2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Min>\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_var1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_var2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Min>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "min" }, expLst: Deref @ metamodelica::List::Cons { head: i_e1, tail: Deref @ metamodelica::List::Cons { head: i_e2, tail: Deref @ metamodelica::List::Nil } }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_var2: Tpl::Text;
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_var2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Min>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_var1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_var2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Min>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "abs" }, expLst: Deref @ metamodelica::List::Cons { head: i_e1, tail: Deref @ metamodelica::List::Nil }, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_INTEGER { varLst: _ }, .. } }, a_context, a_preExp, a_varDecls) => {
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Abs>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_var1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Abs>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "abs" }, expLst: Deref @ metamodelica::List::Cons { head: i_e1, tail: Deref @ metamodelica::List::Nil }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Abs>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_var1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Abs>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, expLst: Deref @ metamodelica::List::Cons { head: i_e1, tail: Deref @ metamodelica::List::Nil }, attr: Deref @ DAE::CallAttributes { ty: _, .. } }, a_context, a_preExp, a_varDecls) => {
            let mut l_argStr: Tpl::Text;
            let mut ret_15: Arc<DAE::Exp>;
            let mut ret_14: Arc<DAE::Exp>;
            let mut l_retPre: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            ret_14 = SimCodeFunctionUtil::createAssertforSqrt(i_e1.clone())?;
            ret_15 = SimCodeFunctionUtil::createDAEString((literal!("Model error: Argument of sqrt should be >= 0")).clone());
            (l_retPre, a_varDecls) = assertCommonXml(Tpl::emptyTxt.clone(), ret_14.clone(), ret_15.clone(), a_context.clone(), a_varDecls.clone(), Absyn::dummyInfo.clone())?;
            (l_argStr, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_retPre.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Sqrt>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_argStr.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Sqrt>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "div" }, expLst: Deref @ metamodelica::List::Cons { head: i_e1, tail: Deref @ metamodelica::List::Cons { head: i_e2, tail: Deref @ metamodelica::List::Nil } }, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_INTEGER { varLst: _ }, .. } }, a_context, a_preExp, a_varDecls) => {
            let mut l_var2: Tpl::Text;
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_var2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Div>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_var1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_var2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Div>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "div" }, expLst: Deref @ metamodelica::List::Cons { head: i_e1, tail: Deref @ metamodelica::List::Cons { head: i_e2, tail: Deref @ metamodelica::List::Nil } }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_var2: Tpl::Text;
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_var2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Div>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_var1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_var2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Div>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "mod" }, expLst: Deref @ metamodelica::List::Cons { head: i_e1, tail: Deref @ metamodelica::List::Cons { head: i_e2, tail: Deref @ metamodelica::List::Nil } }, attr: Deref @ DAE::CallAttributes { ty: i_ty, .. } }, a_context, a_preExp, a_varDecls) => {
            let mut l_var2: Tpl::Text;
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_var2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_mod_")).clone() }))?;
            txt = expTypeShortXml(txt.clone(), i_ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_var1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_var2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "max" }, attr: Deref @ DAE::CallAttributes { ty: i_ty, .. }, expLst: Deref @ metamodelica::List::Cons { head: i_array, tail: Deref @ metamodelica::List::Nil } }, a_context, a_preExp, a_varDecls) => {
            let mut txt_20: Tpl::Text;
            let mut l_tvar: Tpl::Text;
            let mut l_arr__tp__str: Tpl::Text;
            let mut l_expVar: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_expVar, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_array.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            l_arr__tp__str = expTypeArrayXml(Tpl::emptyTxt.clone(), i_ty.clone())?;
            txt_20 = expTypeModelicaXml(Tpl::emptyTxt.clone(), i_ty.clone())?;
            (l_tvar, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(txt_20.clone())?).clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_tvar.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = max_")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_arr__tp__str.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(&")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_expVar.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), l_tvar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "min" }, attr: Deref @ DAE::CallAttributes { ty: i_ty, .. }, expLst: Deref @ metamodelica::List::Cons { head: i_array, tail: Deref @ metamodelica::List::Nil } }, a_context, a_preExp, a_varDecls) => {
            let mut txt_21: Tpl::Text;
            let mut l_tvar: Tpl::Text;
            let mut l_arr__tp__str: Tpl::Text;
            let mut l_expVar: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_expVar, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_array.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            l_arr__tp__str = expTypeArrayXml(Tpl::emptyTxt.clone(), i_ty.clone())?;
            txt_21 = expTypeModelicaXml(Tpl::emptyTxt.clone(), i_ty.clone())?;
            (l_tvar, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(txt_21.clone())?).clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_tvar.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = min_")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_arr__tp__str.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(&")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_expVar.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), l_tvar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fill" }, expLst: Deref @ metamodelica::List::Cons { head: i_val, tail: i_dims }, attr: Deref @ DAE::CallAttributes { ty: i_ty, .. } }, a_context, a_preExp, a_varDecls) => {
            let mut ret_24: i32;
            let mut l_dimsExp: Tpl::Text;
            let mut l_valExp: Tpl::Text;
            let mut l_tvar: Tpl::Text;
            let mut l_ty__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_valExp, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_val.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            l_dimsExp = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_dimsExp, a_varDecls, a_preExp) = lm_362(l_dimsExp.clone(), i_dims.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone())?;
            l_dimsExp = Tpl::popIter(l_dimsExp.clone())?;
            l_ty__str = expTypeArrayXml(Tpl::emptyTxt.clone(), i_ty.clone())?;
            (l_tvar, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_ty__str.clone())?).clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fill_alloc_")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_ty__str.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(&")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_tvar.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_valExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            ret_24 = (i_dims.clone().len() as i32);
            a_preExp = Tpl::writeStr(a_preExp.clone(), (intString(ret_24.clone())).clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_dimsExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), l_tvar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_call @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "vector" }, .. }, _, a_preExp, a_varDecls) => {
            let mut txt_25: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_25 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("vector() call does not have a C implementation ")).clone() }))?;
            txt_25 = ExpressionDumpTpl::dumpExp(txt_25.clone(), i_call.clone(), (literal!("\"")).clone())?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 3236, 11), (Tpl::textString(txt_25.clone())?).clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cat" }, expLst: Deref @ metamodelica::List::Cons { head: i_dim, tail: i_arrays }, attr: Deref @ DAE::CallAttributes { ty: i_ty, .. } }, a_context, a_preExp, a_varDecls) => {
            let mut ret_28: i32;
            let mut l_arrays__exp: Tpl::Text;
            let mut l_dim__exp: Tpl::Text;
            let mut l_tvar: Tpl::Text;
            let mut l_ty__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_dim__exp, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_dim.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            l_arrays__exp = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", &")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_arrays__exp, a_varDecls, a_preExp) = lm_363(l_arrays__exp.clone(), i_arrays.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone())?;
            l_arrays__exp = Tpl::popIter(l_arrays__exp.clone())?;
            l_ty__str = expTypeArrayXml(Tpl::emptyTxt.clone(), i_ty.clone())?;
            (l_tvar, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_ty__str.clone())?).clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("cat_alloc_")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_ty__str.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_dim__exp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", &")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_tvar.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            ret_28 = (i_arrays.clone().len() as i32);
            a_preExp = Tpl::writeStr(a_preExp.clone(), (intString(ret_28.clone())).clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", &")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_arrays__exp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" where is cat2")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_tvar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "promote" }, expLst: Deref @ metamodelica::List::Cons { head: i_A, tail: Deref @ metamodelica::List::Cons { head: i_n, tail: Deref @ metamodelica::List::Nil } }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_tvar: Tpl::Text;
            let mut l_arr__tp__str: Tpl::Text;
            let mut l_var2: Tpl::Text;
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_A.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_var2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_n.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            l_arr__tp__str = expTypeFromExpArrayXml(Tpl::emptyTxt.clone(), i_A.clone())?;
            (l_tvar, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_arr__tp__str.clone())?).clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("promote_alloc_")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_arr__tp__str.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(&")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_var1.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_var2.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", &")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_tvar.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), l_tvar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "transpose" }, expLst: Deref @ metamodelica::List::Cons { head: i_A, tail: Deref @ metamodelica::List::Nil }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_tvar: Tpl::Text;
            let mut l_arr__tp__str: Tpl::Text;
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_A.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            l_arr__tp__str = expTypeFromExpArrayXml(Tpl::emptyTxt.clone(), i_A.clone())?;
            (l_tvar, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_arr__tp__str.clone())?).clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("transpose_alloc_")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_arr__tp__str.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(&")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_var1.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", &")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_tvar.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), l_tvar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cross" }, expLst: Deref @ metamodelica::List::Cons { head: i_v1, tail: Deref @ metamodelica::List::Cons { head: i_v2, tail: Deref @ metamodelica::List::Nil } }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_tvar: Tpl::Text;
            let mut l_arr__tp__str: Tpl::Text;
            let mut l_var2: Tpl::Text;
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_v1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_var2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_v2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            l_arr__tp__str = expTypeFromExpArrayXml(Tpl::emptyTxt.clone(), i_v1.clone())?;
            (l_tvar, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_arr__tp__str.clone())?).clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("cross_alloc_")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_arr__tp__str.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(&")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_var1.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", &")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_var2.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", &")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_tvar.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), l_tvar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "identity" }, expLst: Deref @ metamodelica::List::Cons { head: i_A, tail: Deref @ metamodelica::List::Nil }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_tvar: Tpl::Text;
            let mut l_arr__tp__str: Tpl::Text;
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_A.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            l_arr__tp__str = expTypeFromExpArrayXml(Tpl::emptyTxt.clone(), i_A.clone())?;
            (l_tvar, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_arr__tp__str.clone())?).clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("identity_alloc_")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_arr__tp__str.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_var1.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", &")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_tvar.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), l_tvar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "rem" }, expLst: Deref @ metamodelica::List::Cons { head: i_e1, tail: Deref @ metamodelica::List::Cons { head: i_e2, tail: Deref @ metamodelica::List::Nil } }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_typeStr: Tpl::Text;
            let mut l_var2: Tpl::Text;
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_var2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            l_typeStr = expTypeFromExpShortXml(Tpl::emptyTxt.clone(), i_e1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_rem_")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_typeStr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_var1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_var2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "delay" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: _ }, tail: Deref @ metamodelica::List::Cons { head: i_e, tail: Deref @ metamodelica::List::Cons { head: i_d, tail: Deref @ metamodelica::List::Cons { head: i_delayMax, tail: Deref @ metamodelica::List::Nil } } } }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_var3: Tpl::Text;
            let mut l_var2: Tpl::Text;
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_var2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_d.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_var3, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_delayMax.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Delay>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_var1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_var2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_var3.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Delay>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "integer" }, expLst: Deref @ metamodelica::List::Cons { head: i_toBeCasted, tail: Deref @ metamodelica::List::Nil }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_castedVar: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_castedVar, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_toBeCasted.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeText(txt.clone(), l_castedVar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "Integer" }, expLst: Deref @ metamodelica::List::Cons { head: i_toBeCasted, tail: Deref @ metamodelica::List::Nil }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_castedVar: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_castedVar, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_toBeCasted.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeText(txt.clone(), l_castedVar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "clock" }, expLst: Deref @ metamodelica::List::Nil, .. }, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_clock()")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "noEvent" }, expLst: Deref @ metamodelica::List::Cons { head: i_e1, tail: Deref @ metamodelica::List::Nil }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpXml(txt.clone(), i_e1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "anyString" }, expLst: Deref @ metamodelica::List::Cons { head: i_e1, tail: Deref @ metamodelica::List::Nil }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpXml(txt.clone(), i_e1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "mmc_get_field" }, expLst: Deref @ metamodelica::List::Cons { head: i_s1, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: i_i }, tail: Deref @ metamodelica::List::Nil } }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_expPart: Tpl::Text;
            let mut l_tvar: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_tvar, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (literal!("modelica_metatype")).clone(), a_varDecls.clone())?;
            (l_expPart, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_s1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_tvar.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = MMC_FETCH(MMC_OFFSET(MMC_UNTAGPTR(")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_expPart.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("), ")).clone() }))?;
            a_preExp = Tpl::writeStr(a_preExp.clone(), (intString(i_i.clone())).clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("));")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), l_tvar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "mmc_unbox_record" }, expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, attr: Deref @ DAE::CallAttributes { ty: _, .. } }, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("  \"mmc_unbox_record\" is not necessary")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_exp @ Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { tailCall: DAE::TailCall::TAIL { vars: i_tail_vars, .. }, .. }, expLst: i_expLst, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_res: Tpl::Text;
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_res = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* Tail recursive call ")).clone() }))?;
            l_res = ExpressionDumpTpl::dumpExp(l_res.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            l_res = Tpl::writeTok(l_res.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" */\n")).clone() }))?;
            (l_res, a_preExp, a_varDecls) = daeExpTailCallXml(l_res.clone(), i_expLst.clone(), i_tail_vars.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            l_res = Tpl::writeTok(l_res.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("goto _tailrecursive;\n")).clone(), (literal!("/* TODO: Make sure any eventual dead code below is never generated */")).clone()], lastHasNewLine: false }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_res.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_exp @ Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { builtin: i_attr_builtin, ty: i_attr_ty, .. }, expLst: i_expLst, path: i_path }, a_context, a_preExp, a_varDecls) => {
            let mut l_retVar: Tpl::Text;
            let mut l_retType: Tpl::Text;
            let mut l_funName: Tpl::Text;
            let mut l_builtinFunctionName: Tpl::Text;
            let mut l_result: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut l_argStr: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            l_argStr = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_argStr, a_varDecls, l_preExp) = lm_364(l_argStr.clone(), i_expLst.clone(), a_varDecls.clone(), l_preExp.clone(), a_context.clone())?;
            l_argStr = Tpl::popIter(l_argStr.clone())?;
            l_result = fun_365(Tpl::emptyTxt.clone(), l_preExp.clone(), l_argStr.clone())?;
            l_builtinFunctionName = builtinFunctionNameXml(Tpl::emptyTxt.clone(), i_path.clone())?;
            l_funName = underscorePathXml(Tpl::emptyTxt.clone(), i_path.clone())?;
            l_retType = fun_367(Tpl::emptyTxt.clone(), i_attr_builtin.clone(), i_attr_ty.clone(), l_funName.clone())?;
            (l_retVar, a_varDecls) = fun_368(Tpl::emptyTxt.clone(), i_attr_ty.clone(), a_varDecls.clone(), l_retType.clone())?;
            txt = fun_370(txt.clone(), i_exp.clone(), l_builtinFunctionName.clone(), l_result.clone(), l_funName.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub(crate) fn builtinFunctionNameXml(mut in_txt: Tpl::Text, mut in_a_path: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_path.clone())) {
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "DIVISION" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Div")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "ADDITION" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Add")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "SUBTRACTION" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Sub")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "POWER" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Pow")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "sin" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Sin")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "cos" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Cos")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "tan" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Tan")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "asin" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Asin")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "acos" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Acos")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "atan" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Atan")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "sinh" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Sinh")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "cosh" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Cosh")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "tanh" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Tanh")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Exp")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "log" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Log")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "log10" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Log10")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Sqrt")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "atan2" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Atan2")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "abs" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Abs")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "sign" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Sign")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "min" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Min")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "max" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Max")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "noEvent" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NoEvent")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "array" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Array")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Sample")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "smooth" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Smooth")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Homotopy")).clone() }))?;
            txt.clone()
        },
        (txt, i_path) => {
            let mut txt = (*txt).clone();
            txt = dotPathXml(txt.clone(), i_path.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_373(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_vrest: Arc<metamodelica::List<ArcStr>>, mut in_a_erest: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut in_a_exp: Tpl::Text, mut in_a_v: ArcStr) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone(), in_a_vrest.clone(), in_a_erest.clone(), in_a_exp.clone(), in_a_v.clone())) {
        (txt, false, a_varDecls, a_preExp, a_context, a_vrest, a_erest, a_exp, a_v) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_v.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_exp.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            (txt, a_preExp, a_varDecls) = daeExpTailCallXml(txt.clone(), a_erest.clone(), a_vrest.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_varDecls, a_preExp, a_context, a_vrest, a_erest, _, _) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_preExp, a_varDecls) = daeExpTailCallXml(txt.clone(), a_erest.clone(), a_vrest.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_374(mut in_txt: Tpl::Text, mut in_a_e: Arc<DAE::Exp>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_vrest: Arc<metamodelica::List<ArcStr>>, mut in_a_erest: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut in_a_exp: Tpl::Text, mut in_a_v: ArcStr) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_e.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone(), in_a_vrest.clone(), in_a_erest.clone(), in_a_exp.clone(), in_a_v.clone())) {
        (txt, Deref @ DAE::Exp::CREF { componentRef: i_cr, ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_VAR { functionType: _ } }, a_varDecls, a_preExp, a_context, a_vrest, a_erest, a_exp, a_v) => {
            let mut ret_1: bool;
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            txt_0 = crefStrXml(Tpl::emptyTxt.clone(), i_cr.clone())?;
            ret_1 = stringEq((a_v.clone()).clone(), (Tpl::textString(txt_0.clone())?).clone());
            (txt, a_varDecls, a_preExp) = fun_373(txt.clone(), ret_1.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone(), a_vrest.clone(), a_erest.clone(), a_exp.clone(), (a_v.clone()).clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_varDecls, a_preExp, a_context, a_vrest, a_erest, a_exp, a_v) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_v.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_exp.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            (txt, a_preExp, a_varDecls) = daeExpTailCallXml(txt.clone(), a_erest.clone(), a_vrest.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_375(mut in_txt: Tpl::Text, mut in_a_vs: Arc<metamodelica::List<ArcStr>>, mut in_a_erest: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_e: Arc<DAE::Exp>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_vs.clone(), in_a_erest.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone(), in_a_e.clone())) {
        (txt, Deref @ metamodelica::List::Cons { head: i_v, tail: i_vrest }, a_erest, a_varDecls, a_preExp, a_context, a_e) => {
            let mut l_exp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (l_exp, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt, a_varDecls, a_preExp) = fun_374(txt.clone(), a_e.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone(), i_vrest.clone(), a_erest.clone(), l_exp.clone(), (i_v.clone()).clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, _, a_varDecls, a_preExp, _, _) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

pub(crate) fn daeExpTailCallXml(mut in_txt: Tpl::Text, mut in_a_es: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut in_a_vs: Arc<metamodelica::List<ArcStr>>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_es.clone(), in_a_vs.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: i_erest }, a_vs, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls, a_preExp) = fun_375(txt.clone(), a_vs.clone(), i_erest.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone(), i_e.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub(crate) fn daeExpCallBuiltinPrefixXml(mut in_txt: Tpl::Text, mut in_a_builtin: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_builtin.clone()) {
        (mut txt, true) => {
            txt.clone()
        },
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_378(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut a_varDecls: Tpl::Text, mut a_preExp: Tpl::Text, mut a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    let mut a_preExp: Tpl::Text = a_preExp;
    for mut lstElt_378 in &*items.clone() {
        let mut lstElt_378 = lstElt_378.clone();
        (txt, a_varDecls, a_preExp) = (::match_deref::match_deref! { match &(lstElt_378.clone()) {
        i_e => {
            (txt, a_preExp, a_varDecls) = daeExpXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_varDecls, a_preExp))
}

pub(crate) fn daeExpArrayXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::ARRAY { array: i_array, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_params: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_params = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_params, a_varDecls, a_preExp) = lm_378(l_params.clone(), i_array.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone())?;
            l_params = Tpl::popIter(l_params.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Array>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_params.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Array>")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_params.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn lm_380(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, mut a_vars2: Tpl::Text, mut a_promote: Tpl::Text, mut a_context: SimCodeFunction::Context, mut a_varDecls: Tpl::Text, mut a_arrayTypeStr: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_vars2: Tpl::Text = a_vars2;
    let mut a_promote: Tpl::Text = a_promote;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_380 in &*items.clone() {
        let mut lstElt_380 = lstElt_380.clone();
        (txt, a_vars2, a_promote, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_380.clone()) {
        i_row => {
            let mut l_vars: Tpl::Text;
            let mut l_tmp: Tpl::Text;
            (l_tmp, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(a_arrayTypeStr.clone())?).clone(), a_varDecls.clone())?;
            (l_vars, a_promote, a_varDecls) = daeExpMatrixRowXml(Tpl::emptyTxt.clone(), i_row.clone(), (Tpl::textString(a_arrayTypeStr.clone())?).clone(), a_context.clone(), a_promote.clone(), a_varDecls.clone())?;
            a_vars2 = Tpl::writeTok(a_vars2.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", &")).clone() }))?;
            a_vars2 = Tpl::writeText(a_vars2.clone(), l_tmp.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_vars2.clone(), a_promote.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_vars2, a_promote, a_varDecls))
}

pub(crate) fn daeExpMatrixXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::MATRIX { matrix: Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Nil, tail: Deref @ metamodelica::List::Nil }, .. }, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::MATRIX { matrix: Deref @ metamodelica::List::Nil, .. }, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::MATRIX { ty: i_m_ty, matrix: i_m_matrix, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_tmp: Tpl::Text;
            let mut l_catAlloc: Tpl::Text;
            let mut l_promote: Tpl::Text;
            let mut l_vars2: Tpl::Text;
            let mut l_arrayTypeStr: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_arrayTypeStr = expTypeArrayXml(Tpl::emptyTxt.clone(), i_m_ty.clone())?;
            l_vars2 = Tpl::emptyTxt.clone();
            l_promote = Tpl::emptyTxt.clone();
            l_catAlloc = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_catAlloc, l_vars2, l_promote, a_varDecls) = lm_380(l_catAlloc.clone(), i_m_matrix.clone(), l_vars2.clone(), l_promote.clone(), a_context.clone(), a_varDecls.clone(), l_arrayTypeStr.clone())?;
            l_catAlloc = Tpl::popIter(l_catAlloc.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_promote.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_catAlloc.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            (l_tmp, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_arrayTypeStr.clone())?).clone(), a_varDecls.clone())?;
            txt = Tpl::writeText(txt.clone(), l_tmp.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn lm_382(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut a_varLstStr: Tpl::Text, mut a_arrayTypeStr: ArcStr, mut a_varDecls: Tpl::Text, mut a_preExp: Tpl::Text, mut a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_varLstStr: Tpl::Text = a_varLstStr;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    let mut a_preExp: Tpl::Text = a_preExp;
    for mut lstElt_382 in &*items.clone() {
        let mut lstElt_382 = lstElt_382.clone();
        (txt, a_varLstStr, a_varDecls, a_preExp) = (::match_deref::match_deref! { match &(lstElt_382.clone()) {
        i_e => {
            let mut l_tmp: Tpl::Text;
            let mut l_expVar: Tpl::Text;
            (l_expVar, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_tmp, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (a_arrayTypeStr.clone()).clone(), a_varDecls.clone())?;
            a_varLstStr = Tpl::writeTok(a_varLstStr.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", &")).clone() }))?;
            a_varLstStr = Tpl::writeText(a_varLstStr.clone(), l_tmp.clone())?;
            txt = Tpl::writeText(txt.clone(), l_expVar.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_varLstStr.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_varLstStr, a_varDecls, a_preExp))
}

pub(crate) fn daeExpMatrixRowXml(mut txt: Tpl::Text, mut a_row: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut a_arrayTypeStr: ArcStr, mut a_context: SimCodeFunction::Context, mut a_preExp: Tpl::Text, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut l_preExp2: Tpl::Text;
    let mut l_varLstStr: Tpl::Text;
    l_varLstStr = Tpl::emptyTxt.clone();
    l_preExp2 = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    (l_preExp2, l_varLstStr, out_a_varDecls, out_a_preExp) = lm_382(l_preExp2.clone(), a_row.clone(), l_varLstStr.clone(), (a_arrayTypeStr.clone()).clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone())?;
    l_preExp2 = Tpl::popIter(l_preExp2.clone())?;
    l_preExp2 = Tpl::writeTok(l_preExp2.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
    out_a_preExp = Tpl::writeText(out_a_preExp.clone(), l_preExp2.clone())?;
    out_txt = Tpl::writeText(txt.clone(), l_varLstStr.clone())?;
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_384(mut in_txt: Tpl::Text, mut in_a_step: Option<Arc<DAE::Exp>>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_step.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone())) {
        (txt, Some(i_stepExp), a_varDecls, a_preExp, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_preExp, a_varDecls) = daeExpXml(txt.clone(), i_stepExp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_varDecls, a_preExp, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("1")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

pub(crate) fn daeExpRangeXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::RANGE { ty: i_ty, start: i_start, stop: i_stop, step: i_step }, a_context, a_preExp, a_varDecls) => {
            let mut l_step__exp: Tpl::Text;
            let mut l_tmp: Tpl::Text;
            let mut l_stop__exp: Tpl::Text;
            let mut l_start__exp: Tpl::Text;
            let mut l_ty__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_ty__str = expTypeArrayXml(Tpl::emptyTxt.clone(), i_ty.clone())?;
            (l_start__exp, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_start.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_stop__exp, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_stop.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_tmp, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_ty__str.clone())?).clone(), a_varDecls.clone())?;
            (l_step__exp, a_varDecls, a_preExp) = fun_384(Tpl::emptyTxt.clone(), i_step.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Range>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_start__exp.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_step__exp.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_stop__exp.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Range>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), l_tmp.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_386(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_preExp: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_varDecls: Tpl::Text, mut in_a_expVar: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone(), in_a_preExp.clone(), in_a_exp.clone(), in_a_varDecls.clone(), in_a_expVar.clone())) {
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }, a_preExp, _, a_varDecls, a_expVar) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_expVar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }, a_preExp, _, a_varDecls, a_expVar) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_expVar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Type::T_ENUMERATION { index: _, .. }, a_preExp, _, a_varDecls, a_expVar) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_expVar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }, a_preExp, _, a_varDecls, a_expVar) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_expVar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Type::T_ARRAY { ty: i_ty, .. }, a_preExp, a_exp, a_varDecls, a_expVar) => {
            let mut l_from: Tpl::Text;
            let mut l_to: Tpl::Text;
            let mut l_tvar: Tpl::Text;
            let mut l_arrayTypeStr: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_arrayTypeStr = expTypeArrayXml(Tpl::emptyTxt.clone(), i_ty.clone())?;
            (l_tvar, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_arrayTypeStr.clone())?).clone(), a_varDecls.clone())?;
            l_to = expTypeShortXml(Tpl::emptyTxt.clone(), i_ty.clone())?;
            l_from = expTypeFromExpShortXml(Tpl::emptyTxt.clone(), a_exp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("cast_")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_from.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_array_to_")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_to.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(&")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_expVar.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", &")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_tvar.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), l_tvar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, a_preExp, _, a_varDecls, a_expVar) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_expVar.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" /* could not cast, using the variable as it is */")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub(crate) fn daeExpCastXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::CAST { exp: i_exp, ty: i_ty }, a_context, a_preExp, a_varDecls) => {
            let mut l_expVar: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_expVar, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt, a_preExp, a_varDecls) = fun_386(txt.clone(), i_ty.clone(), a_preExp.clone(), i_exp.clone(), a_varDecls.clone(), l_expVar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub(crate) fn daeSubscriptXML(mut in_txt: Tpl::Text, mut in_a_sub: Arc<DAE::Subscript>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_sub.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Subscript::INDEX { exp: i_exp }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpXml(txt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 3580, 14), (literal!("non INDEX(_) (i.e., slice) subscripts probably should not reach here. Check indexedAssign template.")).clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_389(mut in_txt: Tpl::Text, mut in_a_inExp: Arc<DAE::Exp>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_inExp.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone())) {
        (txt, Deref @ DAE::Exp::ASUB { exp: i_e, sub: Deref @ metamodelica::List::Cons { head: i_idx, tail: Deref @ metamodelica::List::Nil } }, a_varDecls, a_preExp, a_context) => {
            let mut l_idx1: Tpl::Text;
            let mut l_e1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (l_e1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_idx1, a_preExp, a_varDecls) = daeSubscriptXML(Tpl::emptyTxt.clone(), i_idx.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("arrayGet(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_e1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_idx1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") /* DAE.ASUB */")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_varDecls, a_preExp, _) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn lm_390(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut a_res: Tpl::Text, mut a_context: SimCodeFunction::Context) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_390 in &*items.clone() {
        let mut lstElt_390 = lstElt_390.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_390.clone()) {
        i_e => {
            let mut x_i1: i32;
            let mut l_v: Tpl::Text;
            let mut l_casePreExp: Tpl::Text;
            let mut l_caseVarDecls: Tpl::Text;
            x_i1 = Tpl::getIteri_i0(txt.clone())?;
            l_caseVarDecls = Tpl::emptyTxt.clone();
            l_casePreExp = Tpl::emptyTxt.clone();
            (l_v, l_casePreExp, l_caseVarDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e.clone(), a_context.clone(), l_casePreExp.clone(), l_caseVarDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(x_i1.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(": {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_caseVarDecls.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_casePreExp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_v.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";\n")).clone(), (literal!("break;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_391(mut in_txt: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut in_a_ecr_ty: Arc<DAE::Type>, mut in_a_arrName: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_context.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_subs.clone(), in_a_ecr_ty.clone(), in_a_arrName.clone())) {
        (txt, SimCodeFunction::Context::FUNCTION_CONTEXT { cref_prefix: _, .. }, a_varDecls, a_preExp, _, _, a_arrName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_arrName.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, i_context, a_varDecls, a_preExp, a_subs, a_ecr_ty, a_arrName) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_preExp, a_varDecls) = arrayScalarRhsXml(txt.clone(), a_ecr_ty.clone(), a_subs.clone(), (Tpl::textString(a_arrName.clone())?).clone(), i_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Asub array scalar RHS")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_392(mut in_txt: Tpl::Text, mut in_a_inExp: Arc<DAE::Exp>, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_inExp.clone(), in_a_preExp.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::ASUB { exp: i_exp @ Deref @ DAE::Exp::ASUB { exp: _, .. }, .. }, a_preExp, _, a_varDecls) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Nested array subscripting *should* have been handled by the routine creating the asub, but for some reason it was not: ")).clone() }))?;
            txt_0 = ExpressionDumpTpl::dumpExp(txt_0.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 3600, 11), (Tpl::textString(txt_0.clone())?).clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::ASUB { exp: i_exp @ Deref @ DAE::Exp::ARRAY { scalar: true, array: i_exp_array, .. }, sub: Deref @ metamodelica::List::Cons { head: i_idx, tail: Deref @ metamodelica::List::Nil } }, a_preExp, a_context, a_varDecls) => {
            let mut l_expl: Tpl::Text;
            let mut l_idx1: Tpl::Text;
            let mut txt_2: Tpl::Text;
            let mut l_res: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            txt_2 = expTypeFromExpModelicaXml(Tpl::emptyTxt.clone(), i_exp.clone())?;
            (l_res, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(txt_2.clone())?).clone(), a_varDecls.clone())?;
            l_res = Tpl::writeTok(l_res.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" asub tmp test")).clone() }))?;
            (l_idx1, a_preExp, a_varDecls) = daeSubscriptXML(Tpl::emptyTxt.clone(), i_idx.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            l_expl = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_expl = lm_390(l_expl.clone(), i_exp_array.clone(), l_res.clone(), a_context.clone())?;
            l_expl = Tpl::popIter(l_expl.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("switch (")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_idx1.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(") { /* ASUB */\n")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_expl.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("default:\n")).clone(), (literal!("  assert(NULL == \"index out of bounds\");\n")).clone(), (literal!("}")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_res.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::ASUB { exp: i_exp @ Deref @ DAE::Exp::RANGE { ty: _, .. }, sub: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, a_preExp, _, a_varDecls) => {
            let mut txt_5: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_5 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ASUB_EASY_CASE ")).clone() }))?;
            txt_5 = ExpressionDumpTpl::dumpExp(txt_5.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 3629, 11), (Tpl::textString(txt_5.clone())?).clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::ASUB { exp: i_ecr @ Deref @ DAE::Exp::CREF { ty: i_ecr_ty, .. }, sub: i_subs }, a_preExp, a_context, a_varDecls) => {
            let mut ret_7: Arc<DAE::Exp>;
            let mut l_arrName: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            ret_7 = SimCodeFunctionUtil::buildCrefExpFromSubs(i_ecr.clone(), i_subs.clone())?;
            (l_arrName, a_preExp, a_varDecls) = daeExpCrefRhsXml(Tpl::emptyTxt.clone(), ret_7.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt, a_varDecls, a_preExp) = fun_391(txt.clone(), a_context.clone(), a_varDecls.clone(), a_preExp.clone(), i_subs.clone(), i_ecr_ty.clone(), l_arrName.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::ASUB { exp: i_e, sub: _ }, a_preExp, a_context, a_varDecls) => {
            let mut l_exp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_exp, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeText(txt.clone(), l_exp.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_exp, a_preExp, _, a_varDecls) => {
            let mut txt_9: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_9 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OTHER_ASUB ")).clone() }))?;
            txt_9 = ExpressionDumpTpl::dumpExp(txt_9.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 3644, 11), (Tpl::textString(txt_9.clone())?).clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_393(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_inExp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_inExp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ "metatype", a_inExp, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls, a_preExp) = fun_389(txt.clone(), a_inExp.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, a_inExp, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = fun_392(txt.clone(), a_inExp.clone(), a_preExp.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub(crate) fn daeExpAsubXml(mut txt: Tpl::Text, mut a_inExp: Arc<DAE::Exp>, mut a_context: SimCodeFunction::Context, mut a_preExp: Tpl::Text, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut str_1: ArcStr;
    let mut txt_0: Tpl::Text;
    txt_0 = expTypeFromExpShortXml(Tpl::emptyTxt.clone(), a_inExp.clone())?;
    str_1 = (Tpl::textString(txt_0.clone())?).clone();
    (out_txt, out_a_preExp, out_a_varDecls) = fun_393(txt.clone(), (str_1.clone()).clone(), a_inExp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub(crate) fn daeExpASubIndexXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::ICONST { integer: i_integer }, _, a_preExp, a_varDecls) => {
            let mut ret_0: i32;
            let mut txt = (*txt).clone();
            ret_0 = SimCodeFunctionUtil::incrementInt(i_integer.clone(), -1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::ENUM_LITERAL { index: i_index, .. }, _, a_preExp, a_varDecls) => {
            let mut ret_1: i32;
            let mut txt = (*txt).clone();
            ret_1 = SimCodeFunctionUtil::incrementInt(i_index.clone(), -1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_1.clone())).clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_exp, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpXml(txt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_396(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone())) {
        (txt, Deref @ DAE::Exp::CREF { componentRef: i_cr_componentRef, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Pre>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = crefXml(txt.clone(), i_cr_componentRef.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Pre>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::ASUB { exp: Deref @ DAE::Exp::CREF { componentRef: _, .. }, sub: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" \"case ASUB(exp = cr as CREF(__), sub = {sub_exp}) is not yet implemented\"")).clone() }))?;
            txt.clone()
        },
        (txt, i_exp) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Code generation does not support pre(")).clone() }))?;
            txt_0 = ExpressionDumpTpl::dumpExp(txt_0.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt_0 = Tpl::writeTok(txt_0.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 3671, 11), (Tpl::textString(txt_0.clone())?).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn daeExpCallPreXml(mut txt: Tpl::Text, mut a_exp: Arc<DAE::Exp>, mut a_context: SimCodeFunction::Context, mut a_preExp: Tpl::Text, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    out_txt = fun_396(txt.clone(), a_exp.clone())?;
    out_a_preExp = a_preExp.clone();
    out_a_varDecls = a_varDecls.clone();
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub(crate) fn daeExpSizeXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::SIZE { exp: i_exp @ Deref @ DAE::Exp::CREF { componentRef: _, .. }, sz: Some(i_dim) }, a_context, a_preExp, a_varDecls) => {
            let mut l_dimPart: Tpl::Text;
            let mut l_expPart: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_expPart, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_dimPart, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_dim.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Size>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_expPart.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_dimPart.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Size>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("size(X) not implemented")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub(crate) fn daeExpBoxXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::BOX { exp: i_exp_exp }, a_context, a_preExp, a_varDecls) => {
            let mut l_res: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_res, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeText(txt.clone(), l_res.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub(crate) fn daeExpUnboxXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::UNBOX { exp: i_exp_exp, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_res: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_res, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeText(txt.clone(), l_res.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_401(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>) -> Tpl::Text {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone())) {
        (txt, Deref @ DAE::Exp::SHARED_LITERAL { index: _, .. }) => {
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out_txt
}

pub(crate) fn daeExpSharedLiteralXml(mut txt: Tpl::Text, mut a_exp: Arc<DAE::Exp>, mut a_context: SimCodeFunction::Context, mut a_preExp: Tpl::Text, mut a_varDecls: Tpl::Text) -> (Tpl::Text, Tpl::Text, Tpl::Text) {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    out_txt = fun_401(txt.clone(), a_exp.clone());
    out_a_preExp = a_preExp.clone();
    out_a_varDecls = a_varDecls.clone();
    (out_txt, out_a_preExp, out_a_varDecls)
}

fn lm_403(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut a_varDecls: Tpl::Text, mut a_preExp: Tpl::Text, mut a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    let mut a_preExp: Tpl::Text = a_preExp;
    for mut lstElt_403 in &*items.clone() {
        let mut lstElt_403 = lstElt_403.clone();
        (txt, a_varDecls, a_preExp) = (::match_deref::match_deref! { match &(lstElt_403.clone()) {
        i_sub => {
            (txt, a_preExp, a_varDecls) = daeSubscriptXML(txt.clone(), i_sub.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_varDecls, a_preExp))
}

fn fun_404(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_dimsValuesStr: Tpl::Text, mut in_a_arrName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_dimsValuesStr.clone(), in_a_arrName.clone())) {
        (txt, Deref @ "metatype_array", a_dimsValuesStr, a_arrName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("arrayGet(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_arrName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_dimsValuesStr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") /*arrayScalarRhs*/")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_dimsValuesStr, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" wrong LHS\n")).clone(), (literal!("    <exp:ArraySubscripts>\n")).clone(), (literal!("      <exp:IndexExpression>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 8 }))?;
            txt = Tpl::writeText(txt.clone(), a_dimsValuesStr.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("      </exp:IndexExpression>\n")).clone(), (literal!("    </exp:ArraySubscripts>\n")).clone(), (literal!("  </exp:QualifiedNamepart>\n")).clone(), (literal!("</exp:QualifiedName>")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn arrayScalarRhsXml(mut txt: Tpl::Text, mut a_ty: Arc<DAE::Type>, mut a_subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut a_arrName: ArcStr, mut a_context: SimCodeFunction::Context, mut a_preExp: Tpl::Text, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut str_4: ArcStr;
    let mut l_dimsValuesStr: Tpl::Text;
    let mut ret_2: i32;
    let mut l_dimsLenStr: Tpl::Text;
    let mut l_arrayType: Tpl::Text;
    l_arrayType = expTypeArrayXml(Tpl::emptyTxt.clone(), a_ty.clone())?;
    ret_2 = (a_subs.clone().len() as i32);
    l_dimsLenStr = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_2.clone())).clone())?;
    l_dimsValuesStr = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    (l_dimsValuesStr, out_a_varDecls, out_a_preExp) = lm_403(l_dimsValuesStr.clone(), a_subs.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone())?;
    l_dimsValuesStr = Tpl::popIter(l_dimsValuesStr.clone())?;
    str_4 = (Tpl::textString(l_arrayType.clone())?).clone();
    out_txt = fun_404(txt.clone(), (str_4.clone()).clone(), l_dimsValuesStr.clone(), (a_arrName.clone()).clone())?;
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub(crate) fn outDeclXml(mut txt: Tpl::Text, mut a_ty: ArcStr, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut l_newVar: Tpl::Text;
    l_newVar = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("out")).clone() }))?;
    out_a_varDecls = Tpl::writeStr(a_varDecls.clone(), (a_ty.clone()).clone())?;
    out_a_varDecls = Tpl::writeTok(out_a_varDecls.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
    out_a_varDecls = Tpl::writeText(out_a_varDecls.clone(), l_newVar.clone())?;
    out_a_varDecls = Tpl::writeTok(out_a_varDecls.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
    out_a_varDecls = Tpl::writeTok(out_a_varDecls.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
    out_txt = Tpl::writeText(txt.clone(), l_newVar.clone())?;
    Ok((out_txt, out_a_varDecls))
}

fn fun_407(mut in_txt: Tpl::Text, mut in_a_ty: ArcStr, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ "modelica_metatype", a_varDecls) => {
            let mut ret_0: i32;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("tmpMeta[")).clone() }))?;
            ret_0 = System::tmpTickIndex(1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ "metamodelica_string", a_varDecls) => {
            let mut ret_1: i32;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("tmpMeta[")).clone() }))?;
            ret_1 = System::tmpTickIndex(1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_1.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ "metamodelica_string_const", a_varDecls) => {
            let mut ret_2: i32;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("tmpMeta[")).clone() }))?;
            ret_2 = System::tmpTickIndex(1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_2.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, i_ty, a_varDecls) => {
            let mut ret_4: i32;
            let mut l_newVarIx: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_newVarIx = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("tmp")).clone() }))?;
            ret_4 = System::tmpTick();
            l_newVarIx = Tpl::writeStr(l_newVarIx.clone(), (intString(ret_4.clone())).clone())?;
            a_varDecls = Tpl::writeStr(a_varDecls.clone(), (i_ty.clone()).clone())?;
            a_varDecls = Tpl::writeTok(a_varDecls.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            a_varDecls = Tpl::writeText(a_varDecls.clone(), l_newVarIx.clone())?;
            a_varDecls = Tpl::writeTok(a_varDecls.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            a_varDecls = Tpl::writeTok(a_varDecls.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), l_newVarIx.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub(crate) fn tempDeclXml(mut txt: Tpl::Text, mut a_ty: ArcStr, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut l_newVar: Tpl::Text;
    (l_newVar, out_a_varDecls) = fun_407(Tpl::emptyTxt.clone(), (a_ty.clone()).clone(), a_varDecls.clone())?;
    out_txt = Tpl::writeText(txt.clone(), l_newVar.clone())?;
    Ok((out_txt, out_a_varDecls))
}

pub(crate) fn tempDeclConstXml(mut txt: Tpl::Text, mut a_ty: ArcStr, mut a_val: ArcStr, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut ret_1: i32;
    let mut l_newVar: Tpl::Text;
    l_newVar = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("tmp")).clone() }))?;
    ret_1 = System::tmpTick();
    l_newVar = Tpl::writeStr(l_newVar.clone(), (intString(ret_1.clone())).clone())?;
    out_a_varDecls = Tpl::writeStr(a_varDecls.clone(), (a_ty.clone()).clone())?;
    out_a_varDecls = Tpl::writeTok(out_a_varDecls.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
    out_a_varDecls = Tpl::writeText(out_a_varDecls.clone(), l_newVar.clone())?;
    out_a_varDecls = Tpl::writeTok(out_a_varDecls.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
    out_a_varDecls = Tpl::writeStr(out_a_varDecls.clone(), (a_val.clone()).clone())?;
    out_a_varDecls = Tpl::writeTok(out_a_varDecls.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
    out_a_varDecls = Tpl::writeTok(out_a_varDecls.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
    out_txt = Tpl::writeText(txt.clone(), l_newVar.clone())?;
    Ok((out_txt, out_a_varDecls))
}

fn fun_410(mut in_txt: Tpl::Text, mut in_a_instDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut in_a_var_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_instDims.clone(), in_a_var_ty.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_var_ty) => {
            let mut txt = (*txt).clone();
            txt = expTypeArrayIfXml(txt.clone(), a_var_ty.clone())?;
            txt.clone()
        },
        (txt, _, a_var_ty) => {
            let mut txt = (*txt).clone();
            txt = expTypeArrayXml(txt.clone(), a_var_ty.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn varTypeXml(mut in_txt: Tpl::Text, mut in_a_var: Arc<SimCodeFunction::Variable::Variable>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_var.clone())) {
        (txt, Deref @ SimCodeFunction::Variable::VARIABLE { instDims: i_instDims, ty: i_var_ty, .. }) => {
            let mut txt = (*txt).clone();
            txt = fun_410(txt.clone(), i_instDims.clone(), i_var_ty.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn varTypeBoxedXml(mut in_txt: Tpl::Text, mut in_a_var: Arc<SimCodeFunction::Variable::Variable>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_var.clone())) {
        (txt, Deref @ SimCodeFunction::Variable::VARIABLE { name: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ SimCodeFunction::Variable::FUNCTION_PTR { name: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_fnptr")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn expTypeRWXml(mut in_txt: Tpl::Text, mut in_a_type: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_type.clone())) {
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("TYPE_DESC_INT")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("TYPE_DESC_REAL")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("TYPE_DESC_STRING")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("TYPE_DESC_BOOL")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ENUMERATION { index: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("TYPE_DESC_INT")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ARRAY { ty: i_ty, .. }) => {
            let mut txt = (*txt).clone();
            txt = expTypeRWXml(txt.clone(), i_ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_ARRAY")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("TYPE_DESC_RECORD")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METATYPE { ty: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("TYPE_DESC_MMC")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METABOXED { ty: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("TYPE_DESC_MMC")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_414(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("String")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("MetaType")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn expTypeShortXml(mut in_txt: Tpl::Text, mut in_a_type: Arc<DAE::Type>) -> Result<Tpl::Text> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((in_txt.clone(), in_a_type.clone())) {
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }) => {
            let mut txt = (*txt).clone();
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Integer")).clone() }))?)
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }) => {
            let mut txt = (*txt).clone();
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Real")).clone() }))?)
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }) => {
            let mut ret_0: bool;
            let mut txt = (*txt).clone();
            ret_0 = Config::acceptMetaModelicaGrammar()?;
            return Ok(fun_414(txt.clone(), ret_0.clone())?)
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }) => {
            let mut txt = (*txt).clone();
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Boolean")).clone() }))?)
        },
        (txt, Deref @ DAE::Type::T_ENUMERATION { index: _, .. }) => {
            let mut txt = (*txt).clone();
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Integer")).clone() }))?)
        },
        (txt, Deref @ DAE::Type::T_ARRAY { ty: i_ty, .. }) => {
            let mut txt = (*txt).clone();
            { (in_txt, in_a_type) = (txt.clone(), i_ty.clone()); continue '__tco; }
        },
        (txt, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { path: _ }, .. }) => {
            let mut txt = (*txt).clone();
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Complex")).clone() }))?)
        },
        (txt, Deref @ DAE::Type::T_COMPLEX { complexClassType: i_complexClassType, .. }) => {
            let mut ret_1: Arc<Absyn::Path>;
            let mut txt = (*txt).clone();
            ret_1 = ClassInfUtil::getStateName(i_complexClassType.clone());
            return Ok(underscorePathXml(txt.clone(), ret_1.clone())?)
        },
        (txt, Deref @ DAE::Type::T_METATYPE { ty: _ }) => {
            let mut txt = (*txt).clone();
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("MetaType")).clone() }))?)
        },
        (txt, Deref @ DAE::Type::T_METABOXED { ty: _ }) => {
            let mut txt = (*txt).clone();
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("MetaType")).clone() }))?)
        },
        (txt, Deref @ DAE::Type::T_FUNCTION_REFERENCE_VAR { functionType: _ }) => {
            let mut txt = (*txt).clone();
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fnptr")).clone() }))?)
        },
        (txt, Deref @ DAE::Type::T_UNKNOWN { .. }) => {
            let mut txt = (*txt).clone();
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Complex")).clone() }))?)
        },
        (txt, Deref @ DAE::Type::T_ANYTYPE { anyClassType: _ }) => {
            let mut txt = (*txt).clone();
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Complex")).clone() }))?)
        },
        (txt, i_type) => {
            let mut txt_2: Tpl::Text;
            let mut ret_2: ArcStr;
            let mut txt = (*txt).clone();
            txt_2 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("expTypeShortXml:")).clone() }))?;
            ret_2 = (TypesDump::unparseType(i_type.clone())?).clone();
            txt_2 = Tpl::writeStr(txt_2.clone(), (ret_2.clone()).clone())?;
            return Ok(error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 3835, 14), (Tpl::textString(txt_2.clone())?).clone())?)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn fun_416(mut in_txt: Tpl::Text, mut in_a_array: bool, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_array.clone(), in_a_ty.clone())) {
        (txt, true, a_ty) => {
            let mut txt = (*txt).clone();
            txt = expTypeArrayXml(txt.clone(), a_ty.clone())?;
            txt.clone()
        },
        (txt, false, a_ty) => {
            let mut txt = (*txt).clone();
            txt = expTypeModelicaXml(txt.clone(), a_ty.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn expTypeXml(mut txt: Tpl::Text, mut a_ty: Arc<DAE::Type>, mut a_array: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_416(txt.clone(), a_array.clone(), a_ty.clone())?;
    Ok(out_txt)
}

pub(crate) fn expTypeModelicaXml(mut txt: Tpl::Text, mut a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = expTypeFlagXml(txt.clone(), a_ty.clone(), 2)?;
    Ok(out_txt)
}

pub(crate) fn expTypeArrayXml(mut txt: Tpl::Text, mut a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = expTypeFlagXml(txt.clone(), a_ty.clone(), 3)?;
    Ok(out_txt)
}

pub(crate) fn expTypeArrayIfXml(mut txt: Tpl::Text, mut a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = expTypeFlagXml(txt.clone(), a_ty.clone(), 4)?;
    Ok(out_txt)
}

pub(crate) fn expTypeFromExpShortXml(mut txt: Tpl::Text, mut a_exp: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = expTypeFromExpFlagXml(txt.clone(), a_exp.clone(), 1)?;
    Ok(out_txt)
}

pub(crate) fn expTypeFromExpModelicaXml(mut txt: Tpl::Text, mut a_exp: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = expTypeFromExpFlagXml(txt.clone(), a_exp.clone(), 2)?;
    Ok(out_txt)
}

pub(crate) fn expTypeFromExpArrayXml(mut txt: Tpl::Text, mut a_exp: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = expTypeFromExpFlagXml(txt.clone(), a_exp.clone(), 3)?;
    Ok(out_txt)
}

pub(crate) fn expTypeFromExpArrayIfXml(mut txt: Tpl::Text, mut a_exp: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = expTypeFromExpFlagXml(txt.clone(), a_exp.clone(), 4)?;
    Ok(out_txt)
}

fn fun_425(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone())) {
        (txt, Deref @ DAE::Type::T_COMPLEX { complexClassType: i_complexClassType, .. }) => {
            let mut ret_0: Arc<Absyn::Path>;
            let mut txt = (*txt).clone();
            ret_0 = ClassInfUtil::getStateName(i_complexClassType.clone());
            txt = underscorePathXml(txt.clone(), ret_0.clone())?;
            txt.clone()
        },
        (txt, i_ty) => {
            let mut txt = (*txt).clone();
            txt = expTypeShortXml(txt.clone(), i_ty.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_426(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone())) {
        (txt, i_ty @ Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { path: _ }, .. }) => {
            let mut txt = (*txt).clone();
            txt = expTypeShortXml(txt.clone(), i_ty.clone())?;
            txt.clone()
        },
        (txt, i_ty) => {
            let mut txt = (*txt).clone();
            txt = fun_425(txt.clone(), i_ty.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_427(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone())) {
        (txt, Deref @ DAE::Type::T_ARRAY { ty: i_ty, .. }) => {
            let mut txt = (*txt).clone();
            txt = expTypeShortXml(txt.clone(), i_ty.clone())?;
            txt.clone()
        },
        (txt, i_ty) => {
            let mut txt = (*txt).clone();
            txt = expTypeFlagXml(txt.clone(), i_ty.clone(), 2)?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_428(mut in_txt: Tpl::Text, mut in_a_flag: i32, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_flag.clone(), in_a_ty.clone())) {
        (txt, 1, a_ty) => {
            let mut txt = (*txt).clone();
            txt = expTypeShortXml(txt.clone(), a_ty.clone())?;
            txt.clone()
        },
        (txt, 2, a_ty) => {
            let mut txt = (*txt).clone();
            txt = fun_426(txt.clone(), a_ty.clone())?;
            txt.clone()
        },
        (txt, 3, a_ty) => {
            let mut txt = (*txt).clone();
            txt = expTypeShortXml(txt.clone(), a_ty.clone())?;
            txt.clone()
        },
        (txt, 4, a_ty) => {
            let mut txt = (*txt).clone();
            txt = fun_427(txt.clone(), a_ty.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn expTypeFlagXml(mut txt: Tpl::Text, mut a_ty: Arc<DAE::Type>, mut a_flag: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_428(txt.clone(), a_flag.clone(), a_ty.clone())?;
    Ok(out_txt)
}

fn fun_430(mut in_txt: Tpl::Text, mut in_a_flag: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_flag.clone()) {
        (mut txt, 8) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("int")).clone() }))?;
            txt.clone()
        },
        (mut txt, 1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("integer")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_integer")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_431(mut in_txt: Tpl::Text, mut in_a_flag: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_flag.clone()) {
        (mut txt, 1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("real")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_real")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_432(mut in_txt: Tpl::Text, mut in_a_flag: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_flag.clone()) {
        (mut txt, 1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("string")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_string")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_433(mut in_txt: Tpl::Text, mut in_a_flag: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_flag.clone()) {
        (mut txt, 1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("metatype")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_434(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_flag: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_flag.clone()) {
        (mut txt, false, mut a_flag) => {
            txt = fun_432(txt.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_flag) => {
            txt = fun_433(txt.clone(), a_flag.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_435(mut in_txt: Tpl::Text, mut in_a_flag: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_flag.clone()) {
        (mut txt, 1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("boolean")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_boolean")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_436(mut in_txt: Tpl::Text, mut in_a_flag: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_flag.clone()) {
        (mut txt, 8) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("int")).clone() }))?;
            txt.clone()
        },
        (mut txt, 1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("integer")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_integer")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_437(mut in_txt: Tpl::Text, mut in_a_flag: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_flag.clone()) {
        (mut txt, 1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("metatype")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_438(mut in_txt: Tpl::Text, mut in_a_flag: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_flag.clone()) {
        (mut txt, 1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("metatype")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_439(mut in_txt: Tpl::Text, mut in_a_flag: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_flag.clone()) {
        (mut txt, 1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("metatype")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_440(mut in_txt: Tpl::Text, mut in_a_flag: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_flag.clone()) {
        (mut txt, 1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("metatype")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_441(mut in_txt: Tpl::Text, mut in_a_flag: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_flag.clone()) {
        (mut txt, 1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("metatype")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn expTypeFromExpFlagXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_flag: i32) -> Result<Tpl::Text> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_flag.clone())) {
        (txt, Deref @ DAE::Exp::ICONST { integer: _ }, a_flag) => {
            let mut txt = (*txt).clone();
            return Ok(fun_430(txt.clone(), a_flag.clone())?)
        },
        (txt, Deref @ DAE::Exp::RCONST { real: _ }, a_flag) => {
            let mut txt = (*txt).clone();
            return Ok(fun_431(txt.clone(), a_flag.clone())?)
        },
        (txt, Deref @ DAE::Exp::SCONST { string: _ }, a_flag) => {
            let mut ret_0: bool;
            let mut txt = (*txt).clone();
            ret_0 = Config::acceptMetaModelicaGrammar()?;
            return Ok(fun_434(txt.clone(), ret_0.clone(), a_flag.clone())?)
        },
        (txt, Deref @ DAE::Exp::BCONST { bool: _ }, a_flag) => {
            let mut txt = (*txt).clone();
            return Ok(fun_435(txt.clone(), a_flag.clone())?)
        },
        (txt, Deref @ DAE::Exp::ENUM_LITERAL { name: _, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            return Ok(fun_436(txt.clone(), a_flag.clone())?)
        },
        (txt, Deref @ DAE::Exp::BINARY { operator: i_e_operator, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            return Ok(expTypeFromOpFlagXml(txt.clone(), i_e_operator.clone(), a_flag.clone())?)
        },
        (txt, Deref @ DAE::Exp::UNARY { operator: i_e_operator, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            return Ok(expTypeFromOpFlagXml(txt.clone(), i_e_operator.clone(), a_flag.clone())?)
        },
        (txt, Deref @ DAE::Exp::LBINARY { operator: i_e_operator, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            return Ok(expTypeFromOpFlagXml(txt.clone(), i_e_operator.clone(), a_flag.clone())?)
        },
        (txt, Deref @ DAE::Exp::LUNARY { operator: i_e_operator, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            return Ok(expTypeFromOpFlagXml(txt.clone(), i_e_operator.clone(), a_flag.clone())?)
        },
        (txt, Deref @ DAE::Exp::RELATION { operator: i_e_operator, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            return Ok(expTypeFromOpFlagXml(txt.clone(), i_e_operator.clone(), a_flag.clone())?)
        },
        (txt, Deref @ DAE::Exp::IFEXP { expThen: i_expThen, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            { (in_txt, in_a_exp, in_a_flag) = (txt.clone(), i_expThen.clone(), a_flag.clone()); continue '__tco; }
        },
        (txt, Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: i_attr_ty, .. }, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            return Ok(expTypeFlagXml(txt.clone(), i_attr_ty.clone(), a_flag.clone())?)
        },
        (txt, Deref @ DAE::Exp::ARRAY { ty: i_c_ty, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            return Ok(expTypeFlagXml(txt.clone(), i_c_ty.clone(), a_flag.clone())?)
        },
        (txt, Deref @ DAE::Exp::MATRIX { ty: i_c_ty, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            return Ok(expTypeFlagXml(txt.clone(), i_c_ty.clone(), a_flag.clone())?)
        },
        (txt, Deref @ DAE::Exp::RANGE { ty: i_c_ty, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            return Ok(expTypeFlagXml(txt.clone(), i_c_ty.clone(), a_flag.clone())?)
        },
        (txt, Deref @ DAE::Exp::CAST { ty: i_c_ty, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            return Ok(expTypeFlagXml(txt.clone(), i_c_ty.clone(), a_flag.clone())?)
        },
        (txt, Deref @ DAE::Exp::CREF { ty: i_c_ty, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            return Ok(expTypeFlagXml(txt.clone(), i_c_ty.clone(), a_flag.clone())?)
        },
        (txt, Deref @ DAE::Exp::CODE { ty: i_c_ty, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            return Ok(expTypeFlagXml(txt.clone(), i_c_ty.clone(), a_flag.clone())?)
        },
        (txt, i_c @ Deref @ DAE::Exp::ASUB { exp: _, .. }, a_flag) => {
            let mut ret_1: Arc<DAE::Type>;
            let mut txt = (*txt).clone();
            ret_1 = Expression::r#typeof(i_c.clone())?;
            return Ok(expTypeFlagXml(txt.clone(), ret_1.clone(), a_flag.clone())?)
        },
        (txt, i_exp @ Deref @ DAE::Exp::REDUCTION { reductionInfo: _, .. }, a_flag) => {
            let mut ret_2: Arc<DAE::Type>;
            let mut txt = (*txt).clone();
            ret_2 = Expression::r#typeof(i_exp.clone())?;
            return Ok(expTypeFlagXml(txt.clone(), ret_2.clone(), a_flag.clone())?)
        },
        (txt, i_e @ Deref @ DAE::Exp::BOX { exp: _ }, a_flag) => {
            let mut ret_3: Arc<DAE::Type>;
            let mut txt = (*txt).clone();
            ret_3 = Expression::r#typeof(i_e.clone())?;
            return Ok(expTypeFlagXml(txt.clone(), ret_3.clone(), a_flag.clone())?)
        },
        (txt, i_e @ Deref @ DAE::Exp::CONS { car: _, .. }, a_flag) => {
            let mut ret_4: Arc<DAE::Type>;
            let mut txt = (*txt).clone();
            ret_4 = Expression::r#typeof(i_e.clone())?;
            return Ok(expTypeFlagXml(txt.clone(), ret_4.clone(), a_flag.clone())?)
        },
        (txt, i_e @ Deref @ DAE::Exp::LIST { valList: _ }, a_flag) => {
            let mut ret_5: Arc<DAE::Type>;
            let mut txt = (*txt).clone();
            ret_5 = Expression::r#typeof(i_e.clone())?;
            return Ok(expTypeFlagXml(txt.clone(), ret_5.clone(), a_flag.clone())?)
        },
        (txt, i_e @ Deref @ DAE::Exp::SIZE { exp: _, .. }, a_flag) => {
            let mut ret_6: Arc<DAE::Type>;
            let mut txt = (*txt).clone();
            ret_6 = Expression::r#typeof(i_e.clone())?;
            return Ok(expTypeFlagXml(txt.clone(), ret_6.clone(), a_flag.clone())?)
        },
        (txt, Deref @ DAE::Exp::META_TUPLE { listExp: _ }, a_flag) => {
            let mut txt = (*txt).clone();
            return Ok(fun_437(txt.clone(), a_flag.clone())?)
        },
        (txt, Deref @ DAE::Exp::META_OPTION { exp: _ }, a_flag) => {
            let mut txt = (*txt).clone();
            return Ok(fun_438(txt.clone(), a_flag.clone())?)
        },
        (txt, Deref @ DAE::Exp::MATCHEXPRESSION { matchType: _, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            return Ok(fun_439(txt.clone(), a_flag.clone())?)
        },
        (txt, Deref @ DAE::Exp::METARECORDCALL { path: _, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            return Ok(fun_440(txt.clone(), a_flag.clone())?)
        },
        (txt, Deref @ DAE::Exp::BOX { exp: _ }, a_flag) => {
            let mut txt = (*txt).clone();
            return Ok(fun_441(txt.clone(), a_flag.clone())?)
        },
        (txt, Deref @ DAE::Exp::UNBOX { ty: i_c_ty, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            return Ok(expTypeFlagXml(txt.clone(), i_c_ty.clone(), a_flag.clone())?)
        },
        (txt, Deref @ DAE::Exp::SHARED_LITERAL { exp: i_c_exp, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            { (in_txt, in_a_exp, in_a_flag) = (txt.clone(), i_c_exp.clone(), a_flag.clone()); continue '__tco; }
        },
        (txt, i_exp, _) => {
            let mut txt_7: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_7 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("expTypeFromExpFlag:")).clone() }))?;
            txt_7 = ExpressionDumpTpl::dumpExp(txt_7.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            return Ok(error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 3951, 14), (Tpl::textString(txt_7.clone())?).clone())?)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn fun_443(mut in_txt: Tpl::Text, mut in_a_flag: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_flag.clone()) {
        (mut txt, 1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("boolean")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_boolean")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_444(mut in_txt: Tpl::Text, mut in_a_flag: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_flag.clone()) {
        (mut txt, 1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("boolean")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_boolean")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_445(mut in_txt: Tpl::Text, mut in_a_flag: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_flag.clone()) {
        (mut txt, 1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("boolean")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_boolean")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn expTypeFromOpFlagXml(mut in_txt: Tpl::Text, mut in_a_op: DAE::Operator, mut in_a_flag: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_op.clone(), in_a_flag.clone()) {
        (mut txt, DAE::Operator::ADD { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::SUB { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::MUL { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::DIV { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::POW { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::UMINUS { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::UMINUS_ARR { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::ADD_ARR { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::SUB_ARR { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::MUL_ARR { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::DIV_ARR { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::MUL_ARRAY_SCALAR { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::ADD_ARRAY_SCALAR { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::SUB_SCALAR_ARRAY { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::MUL_SCALAR_PRODUCT { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::MUL_MATRIX_PRODUCT { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::DIV_ARRAY_SCALAR { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::DIV_SCALAR_ARRAY { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::POW_ARRAY_SCALAR { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::POW_SCALAR_ARRAY { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::POW_ARR { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::POW_ARR2 { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::LESS { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::LESSEQ { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::GREATER { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::GREATEREQ { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::EQUAL { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::NEQUAL { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::AND { ty: _ }, mut a_flag) => {
            txt = fun_443(txt.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::OR { ty: _ }, mut a_flag) => {
            txt = fun_444(txt.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::NOT { ty: _ }, mut a_flag) => {
            txt = fun_445(txt.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("expTypeFromOpFlag:ERROR")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn dimensionXml(mut in_txt: Tpl::Text, mut in_a_d: Arc<DAE::Dimension>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_d.clone())) {
        (txt, Deref @ DAE::Dimension::DIM_INTEGER { integer: i_integer }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_integer.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Dimension::DIM_ENUM { size: i_size, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_size.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Dimension::DIM_UNKNOWN { .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(":")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("INVALID_DIMENSION")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn assertCommonXml(mut txt: Tpl::Text, mut a_condition: Arc<DAE::Exp>, mut a_message: Arc<DAE::Exp>, mut a_context: SimCodeFunction::Context, mut a_varDecls: Tpl::Text, mut a_info: SourceInfo) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut l_msgVar: Tpl::Text;
    let mut l_condVar: Tpl::Text;
    let mut l_preExpMsg: Tpl::Text;
    let mut l_preExpCond: Tpl::Text;
    l_preExpCond = Tpl::emptyTxt.clone();
    l_preExpMsg = Tpl::emptyTxt.clone();
    (l_condVar, l_preExpCond, out_a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_condition.clone(), a_context.clone(), l_preExpCond.clone(), a_varDecls.clone())?;
    (l_msgVar, l_preExpMsg, out_a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_message.clone(), a_context.clone(), l_preExpMsg.clone(), out_a_varDecls.clone())?;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:Assertion>\n")).clone(), (literal!("  <fun:Condition>\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_condVar.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Condition>\n")).clone(), (literal!("  <fun:Message>\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_msgVar.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Message>\n")).clone(), (literal!("</fun:Assertion>")).clone()], lastHasNewLine: false }))?;
    Ok((out_txt, out_a_varDecls))
}

pub(crate) fn error(mut txt: Tpl::Text, mut a_srcInfo: SourceInfo, mut a_errMessage: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_0: ArcStr;
    Tpl::addSourceTemplateError((a_errMessage.clone()).clone(), a_srcInfo.clone())?;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("#error \"")).clone()], lastHasNewLine: false }))?;
    ret_0 = (Error::infoStr(a_srcInfo.clone())?).clone();
    out_txt = Tpl::writeStr(out_txt.clone(), (ret_0.clone()).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
    out_txt = Tpl::writeStr(out_txt.clone(), (a_errMessage.clone()).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
    out_txt = Tpl::writeTok(out_txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
    Ok(out_txt)
}

