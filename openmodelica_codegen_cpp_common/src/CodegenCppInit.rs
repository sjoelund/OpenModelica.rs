// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::CodegenCppCommon;
use openmodelica_ast::Absyn;
use openmodelica_backend::CodegenUtil;
use openmodelica_backend::SimCodeUtil;
use openmodelica_backend_types::BackendDAE;
use openmodelica_codegen_fmu::CodegenFMU2;
use openmodelica_codegen_fmu::CodegenFMUCommon;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::HashTableCrIListArray;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_simcode_types::SimCode;
use openmodelica_simcode_types::SimCodeFunction;
use openmodelica_simcode_types::SimCodeVar;
use openmodelica_susan::Tpl;
use openmodelica_util::FMI;
use openmodelica_util::Settings;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

fn fun_52(mut in_txt: Tpl::Text, mut in_a_generateFMUModelDescription: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_generateFMUModelDescription.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ModelDescription")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fmiModelDescription")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_53(mut in_txt: Tpl::Text, mut in_a_generateFMUModelDescription: bool, mut in_a_FMUGuid: ArcStr, mut in_a_FMUType: ArcStr, mut in_a_FMUVersion: ArcStr, mut in_a_simCode: SimCode::SimCode, mut in_a_modelInfo_name: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_generateFMUModelDescription.clone(), in_a_FMUGuid.clone(), in_a_FMUType.clone(), in_a_FMUVersion.clone(), in_a_simCode.clone(), in_a_modelInfo_name.clone())) {
        (txt, false, _, _, _, _, a_modelInfo_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelName=\"")).clone() }))?;
            txt = CodegenUtil::dotPath(txt.clone(), a_modelInfo_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_FMUGuid, a_FMUType, a_FMUVersion, a_simCode, _) => {
            let mut txt = (*txt).clone();
            txt = fmiDescriptionAttributes(txt.clone(), a_simCode.clone(), (a_FMUVersion.clone()).clone(), (a_FMUType.clone()).clone(), (a_FMUGuid.clone()).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_54(mut in_txt: Tpl::Text, mut in_a_generateFMUModelDescription: bool, mut in_a_FMUVersion: ArcStr, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_generateFMUModelDescription.clone(), in_a_FMUVersion.clone(), in_a_simCode.clone()) {
        (mut txt, false, _, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_FMUVersion, mut a_simCode) => {
            txt = CodegenFMUCommon::fmiTypeDefinitions(txt.clone(), a_simCode.clone(), (a_FMUVersion.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_55(mut in_txt: Tpl::Text, mut in_a_generateFMUModelDescription: bool, mut in_a_FMUVersion: ArcStr, mut in_a_simulationSettingsOpt: Option<SimCode::SimulationSettings>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_generateFMUModelDescription.clone(), in_a_FMUVersion.clone(), in_a_simulationSettingsOpt.clone()) {
        (mut txt, false, _, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_FMUVersion, mut a_simulationSettingsOpt) => {
            txt = CodegenFMUCommon::DefaultExperiment(txt.clone(), a_simulationSettingsOpt.clone(), (a_FMUVersion.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn modelInitXMLFile(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_numRealVars: ArcStr, mut in_a_numIntVars: ArcStr, mut in_a_numBoolVars: ArcStr, mut in_a_numStringVars: ArcStr, mut in_a_FMUVersion: ArcStr, mut in_a_FMUType: ArcStr, mut in_a_FMUGuid: ArcStr, mut in_a_generateFMUModelDescription: bool, mut in_a_generatorComments: ArcStr, mut in_a_complexStartExpressions: Tpl::Text, mut in_a_stateDerVectorName: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_complexStartExpressions: Tpl::Text;
    let mut out_a_stateDerVectorName: Tpl::Text;
    (out_txt, out_a_complexStartExpressions, out_a_stateDerVectorName) = (match (in_txt.clone(), in_a_simCode.clone(), in_a_numRealVars.clone(), in_a_numIntVars.clone(), in_a_numBoolVars.clone(), in_a_numStringVars.clone(), in_a_FMUVersion.clone(), in_a_FMUType.clone(), in_a_FMUGuid.clone(), in_a_generateFMUModelDescription.clone(), in_a_generatorComments.clone(), in_a_complexStartExpressions.clone(), in_a_stateDerVectorName.clone()) {
        (mut txt, ref i_simCode @ SimCode::SimCode { modelInfo: ref i_modelInfo @ SimCode::ModelInfo { name: ref i_modelInfo_name, .. }, varToArrayIndexMapping: ref i_varToArrayIndexMapping, simulationSettingsOpt: ref i_simulationSettingsOpt, .. }, mut a_numRealVars, mut a_numIntVars, mut a_numBoolVars, mut a_numStringVars, mut a_FMUVersion, mut a_FMUType, mut a_FMUGuid, mut a_generateFMUModelDescription, mut a_generatorComments, mut a_complexStartExpressions, mut a_stateDerVectorName) => {
            let mut l_fmiDefaultExperiment: Tpl::Text;
            let mut l_fmiTypeDefinitions: Tpl::Text;
            let mut l_fmiDescriptionAttributes: Tpl::Text;
            let mut l_descriptionTag: Tpl::Text;
            let mut txt_4: Tpl::Text;
            let mut txt_3: Tpl::Text;
            let mut txt_2: Tpl::Text;
            let mut txt_1: Tpl::Text;
            let mut l_variables: Tpl::Text;
            txt_1 = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_numRealVars.clone()).clone())?;
            txt_1 = Tpl::writeTok(txt_1.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" - 1")).clone() }))?;
            txt_2 = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_numIntVars.clone()).clone())?;
            txt_2 = Tpl::writeTok(txt_2.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" - 1")).clone() }))?;
            txt_3 = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_numBoolVars.clone()).clone())?;
            txt_3 = Tpl::writeTok(txt_3.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" - 1")).clone() }))?;
            txt_4 = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_numStringVars.clone()).clone())?;
            txt_4 = Tpl::writeTok(txt_4.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" - 1")).clone() }))?;
            (l_variables, a_complexStartExpressions, a_stateDerVectorName) = modelVariablesXML(Tpl::emptyTxt.clone(), i_simCode.clone(), i_modelInfo.clone(), i_varToArrayIndexMapping.clone(), (Tpl::textString(txt_1.clone())?).clone(), (Tpl::textString(txt_2.clone())?).clone(), (Tpl::textString(txt_3.clone())?).clone(), (Tpl::textString(txt_4.clone())?).clone(), a_generateFMUModelDescription.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())?;
            l_descriptionTag = fun_52(Tpl::emptyTxt.clone(), a_generateFMUModelDescription.clone())?;
            l_fmiDescriptionAttributes = fun_53(Tpl::emptyTxt.clone(), a_generateFMUModelDescription.clone(), (a_FMUGuid.clone()).clone(), (a_FMUType.clone()).clone(), (a_FMUVersion.clone()).clone(), i_simCode.clone(), i_modelInfo_name.clone())?;
            l_fmiTypeDefinitions = fun_54(Tpl::emptyTxt.clone(), a_generateFMUModelDescription.clone(), (a_FMUVersion.clone()).clone(), i_simCode.clone())?;
            l_fmiDefaultExperiment = fun_55(Tpl::emptyTxt.clone(), a_generateFMUModelDescription.clone(), (a_FMUVersion.clone()).clone(), i_simulationSettingsOpt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n")).clone(), (literal!("<!--Generated with the modifications: ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_generatorComments.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" -->\n")).clone(), (literal!("<!--Take care about array indices, they are stored in column major layout.-->\n")).clone(), (literal!("<")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_descriptionTag.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fmiDescriptionAttributes.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(">\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_fmiTypeDefinitions.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_fmiDefaultExperiment.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<ModelVariables>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_variables.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</ModelVariables>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_descriptionTag.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(">")).clone() }))?;
            (txt.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())
        },
        (mut txt, _, _, _, _, _, _, _, _, _, _, mut a_complexStartExpressions, mut a_stateDerVectorName) => {
            (txt.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())
        },
    });
    Ok((out_txt, out_a_complexStartExpressions, out_a_stateDerVectorName))
}

fn fun_57(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_simCode: SimCode::SimCode, mut in_a_FMUGuid: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_simCode.clone(), in_a_FMUGuid.clone()) {
        (mut txt, false, mut a_simCode, mut a_FMUGuid) => {
            txt = fmiModelDescriptionAttributes(txt.clone(), a_simCode.clone(), (a_FMUGuid.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_simCode, mut a_FMUGuid) => {
            txt = CodegenFMU2::fmiModelDescriptionAttributes(txt.clone(), a_simCode.clone(), (a_FMUGuid.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn fmiDescriptionAttributes(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_FMUVersion: ArcStr, mut a_FMUType: ArcStr, mut a_FMUGuid: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_0: bool;
    ret_0 = FMI::isFMIVersion20((a_FMUVersion.clone()).clone())?;
    out_txt = fun_57(txt.clone(), ret_0.clone(), a_simCode.clone(), (a_FMUGuid.clone()).clone())?;
    Ok(out_txt)
}

pub fn fmiModelDescriptionAttributes(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_guid: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone(), in_a_guid.clone()) {
        (mut txt, ref i_simCode @ SimCode::SimCode { modelInfo: SimCode::ModelInfo { varInfo: SimCode::VarInfo { numStateVars: ref i_vi_numStateVars, .. }, vars: SimCodeVar::SimVars { stateVars: _, .. }, name: ref i_modelInfo_name, .. }, fileNamePrefix: ref i_fileNamePrefix, .. }, mut a_guid) => {
            let mut l_numberOfEventIndicators: Tpl::Text;
            let mut l_numberOfContinuousStates: Tpl::Text;
            let mut l_variableNamingConvention: Tpl::Text;
            let mut ret_12: Util::DateTime;
            let mut l_generationDateAndTime: Tpl::Text;
            let mut ret_10: ArcStr;
            let mut l_generationTool: Tpl::Text;
            let mut l_version: Tpl::Text;
            let mut l_author: Tpl::Text;
            let mut l_description: Tpl::Text;
            let mut ret_5: ArcStr;
            let mut l_modelIdentifier: Tpl::Text;
            let mut ret_3: ArcStr;
            let mut txt_2: Tpl::Text;
            let mut l_modelName: Tpl::Text;
            let mut l_fmiVersion: Tpl::Text;
            l_fmiVersion = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("1.0")).clone() }))?;
            txt_2 = CodegenUtil::dotPath(Tpl::emptyTxt.clone(), i_modelInfo_name.clone())?;
            ret_3 = (System::stringReplace((Tpl::textString(txt_2.clone())?).clone(), (literal!("$")).clone(), (literal!("_D_")).clone())?).clone();
            l_modelName = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_3.clone()).clone())?;
            ret_5 = (System::stringReplace((i_fileNamePrefix.clone()).clone(), (literal!(".")).clone(), (literal!("_")).clone())?).clone();
            l_modelIdentifier = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_5.clone()).clone())?;
            l_description = Tpl::emptyTxt.clone();
            l_author = Tpl::emptyTxt.clone();
            l_version = Tpl::emptyTxt.clone();
            l_generationTool = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OpenModelica Compiler ")).clone() }))?;
            ret_10 = (Settings::getVersionNr()).clone();
            l_generationTool = Tpl::writeStr(l_generationTool.clone(), (ret_10.clone()).clone())?;
            ret_12 = Util::getCurrentDateTime();
            l_generationDateAndTime = CodegenFMUCommon::xsdateTime(Tpl::emptyTxt.clone(), ret_12.clone())?;
            l_variableNamingConvention = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("structured")).clone() }))?;
            l_numberOfContinuousStates = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(i_vi_numStateVars.clone())).clone())?;
            l_numberOfEventIndicators = CodegenFMUCommon::getNumberOfEventIndicators(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fmiVersion=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fmiVersion.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("modelName=\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_modelName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("modelIdentifier=\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_modelIdentifier.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("guid=\"{")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_guid.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\"\n")).clone(), (literal!("generationTool=\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_generationTool.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("generationDateAndTime=\"")).clone()], lastHasNewLine: false }))?;
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

fn lm_60(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut a_stateDerVectorName: Tpl::Text, mut a_complexStartExpressions: Tpl::Text, mut a_generateFMUModelDescription: bool, mut a_indexForUndefinedReferencesReal: ArcStr, mut a_varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), mut a_simCode: SimCode::SimCode) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_stateDerVectorName: Tpl::Text = a_stateDerVectorName;
    let mut a_complexStartExpressions: Tpl::Text = a_complexStartExpressions;
    for mut lstElt_60 in &*items.clone() {
        let mut lstElt_60 = lstElt_60.clone();
        (txt, a_stateDerVectorName, a_complexStartExpressions) = (match lstElt_60.clone() {
        mut i_var => {
            (txt, a_complexStartExpressions, a_stateDerVectorName) = scalarVariableXML(txt.clone(), a_simCode.clone(), i_var.clone(), a_varToArrayIndexMapping.clone(), (a_indexForUndefinedReferencesReal.clone()).clone(), a_generateFMUModelDescription.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone())
        },
    });
    }
    Ok((txt, a_stateDerVectorName, a_complexStartExpressions))
}

fn lm_61(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut a_stateDerVectorName: Tpl::Text, mut a_complexStartExpressions: Tpl::Text, mut a_generateFMUModelDescription: bool, mut a_indexForUndefinedReferencesReal: ArcStr, mut a_varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), mut a_simCode: SimCode::SimCode) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_stateDerVectorName: Tpl::Text = a_stateDerVectorName;
    let mut a_complexStartExpressions: Tpl::Text = a_complexStartExpressions;
    for mut lstElt_61 in &*items.clone() {
        let mut lstElt_61 = lstElt_61.clone();
        (txt, a_stateDerVectorName, a_complexStartExpressions) = (match lstElt_61.clone() {
        mut i_var => {
            (txt, a_complexStartExpressions, a_stateDerVectorName) = scalarVariableXML(txt.clone(), a_simCode.clone(), i_var.clone(), a_varToArrayIndexMapping.clone(), (a_indexForUndefinedReferencesReal.clone()).clone(), a_generateFMUModelDescription.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone())
        },
    });
    }
    Ok((txt, a_stateDerVectorName, a_complexStartExpressions))
}

fn lm_62(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut a_stateDerVectorName: Tpl::Text, mut a_complexStartExpressions: Tpl::Text, mut a_generateFMUModelDescription: bool, mut a_indexForUndefinedReferencesReal: ArcStr, mut a_varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), mut a_simCode: SimCode::SimCode) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_stateDerVectorName: Tpl::Text = a_stateDerVectorName;
    let mut a_complexStartExpressions: Tpl::Text = a_complexStartExpressions;
    for mut lstElt_62 in &*items.clone() {
        let mut lstElt_62 = lstElt_62.clone();
        (txt, a_stateDerVectorName, a_complexStartExpressions) = (match lstElt_62.clone() {
        mut i_var => {
            (txt, a_complexStartExpressions, a_stateDerVectorName) = scalarVariableXML(txt.clone(), a_simCode.clone(), i_var.clone(), a_varToArrayIndexMapping.clone(), (a_indexForUndefinedReferencesReal.clone()).clone(), a_generateFMUModelDescription.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone())
        },
    });
    }
    Ok((txt, a_stateDerVectorName, a_complexStartExpressions))
}

fn lm_63(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut a_stateDerVectorName: Tpl::Text, mut a_complexStartExpressions: Tpl::Text, mut a_generateFMUModelDescription: bool, mut a_indexForUndefinedReferencesReal: ArcStr, mut a_varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), mut a_simCode: SimCode::SimCode) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_stateDerVectorName: Tpl::Text = a_stateDerVectorName;
    let mut a_complexStartExpressions: Tpl::Text = a_complexStartExpressions;
    for mut lstElt_63 in &*items.clone() {
        let mut lstElt_63 = lstElt_63.clone();
        (txt, a_stateDerVectorName, a_complexStartExpressions) = (match lstElt_63.clone() {
        mut i_var => {
            (txt, a_complexStartExpressions, a_stateDerVectorName) = scalarVariableXML(txt.clone(), a_simCode.clone(), i_var.clone(), a_varToArrayIndexMapping.clone(), (a_indexForUndefinedReferencesReal.clone()).clone(), a_generateFMUModelDescription.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone())
        },
    });
    }
    Ok((txt, a_stateDerVectorName, a_complexStartExpressions))
}

fn lm_64(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut a_stateDerVectorName: Tpl::Text, mut a_complexStartExpressions: Tpl::Text, mut a_generateFMUModelDescription: bool, mut a_indexForUndefinedReferencesReal: ArcStr, mut a_varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), mut a_simCode: SimCode::SimCode) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_stateDerVectorName: Tpl::Text = a_stateDerVectorName;
    let mut a_complexStartExpressions: Tpl::Text = a_complexStartExpressions;
    for mut lstElt_64 in &*items.clone() {
        let mut lstElt_64 = lstElt_64.clone();
        (txt, a_stateDerVectorName, a_complexStartExpressions) = (match lstElt_64.clone() {
        mut i_var => {
            (txt, a_complexStartExpressions, a_stateDerVectorName) = scalarVariableXML(txt.clone(), a_simCode.clone(), i_var.clone(), a_varToArrayIndexMapping.clone(), (a_indexForUndefinedReferencesReal.clone()).clone(), a_generateFMUModelDescription.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone())
        },
    });
    }
    Ok((txt, a_stateDerVectorName, a_complexStartExpressions))
}

fn lm_65(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut a_stateDerVectorName: Tpl::Text, mut a_complexStartExpressions: Tpl::Text, mut a_generateFMUModelDescription: bool, mut a_indexForUndefinedReferencesReal: ArcStr, mut a_varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), mut a_simCode: SimCode::SimCode) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_stateDerVectorName: Tpl::Text = a_stateDerVectorName;
    let mut a_complexStartExpressions: Tpl::Text = a_complexStartExpressions;
    for mut lstElt_65 in &*items.clone() {
        let mut lstElt_65 = lstElt_65.clone();
        (txt, a_stateDerVectorName, a_complexStartExpressions) = (match lstElt_65.clone() {
        mut i_var => {
            (txt, a_complexStartExpressions, a_stateDerVectorName) = scalarVariableXML(txt.clone(), a_simCode.clone(), i_var.clone(), a_varToArrayIndexMapping.clone(), (a_indexForUndefinedReferencesReal.clone()).clone(), a_generateFMUModelDescription.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone())
        },
    });
    }
    Ok((txt, a_stateDerVectorName, a_complexStartExpressions))
}

fn lm_66(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut a_stateDerVectorName: Tpl::Text, mut a_complexStartExpressions: Tpl::Text, mut a_generateFMUModelDescription: bool, mut a_indexForUndefinedReferencesInt: ArcStr, mut a_varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), mut a_simCode: SimCode::SimCode) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_stateDerVectorName: Tpl::Text = a_stateDerVectorName;
    let mut a_complexStartExpressions: Tpl::Text = a_complexStartExpressions;
    for mut lstElt_66 in &*items.clone() {
        let mut lstElt_66 = lstElt_66.clone();
        (txt, a_stateDerVectorName, a_complexStartExpressions) = (match lstElt_66.clone() {
        mut i_var => {
            (txt, a_complexStartExpressions, a_stateDerVectorName) = scalarVariableXML(txt.clone(), a_simCode.clone(), i_var.clone(), a_varToArrayIndexMapping.clone(), (a_indexForUndefinedReferencesInt.clone()).clone(), a_generateFMUModelDescription.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone())
        },
    });
    }
    Ok((txt, a_stateDerVectorName, a_complexStartExpressions))
}

fn lm_67(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut a_stateDerVectorName: Tpl::Text, mut a_complexStartExpressions: Tpl::Text, mut a_generateFMUModelDescription: bool, mut a_indexForUndefinedReferencesInt: ArcStr, mut a_varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), mut a_simCode: SimCode::SimCode) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_stateDerVectorName: Tpl::Text = a_stateDerVectorName;
    let mut a_complexStartExpressions: Tpl::Text = a_complexStartExpressions;
    for mut lstElt_67 in &*items.clone() {
        let mut lstElt_67 = lstElt_67.clone();
        (txt, a_stateDerVectorName, a_complexStartExpressions) = (match lstElt_67.clone() {
        mut i_var => {
            (txt, a_complexStartExpressions, a_stateDerVectorName) = scalarVariableXML(txt.clone(), a_simCode.clone(), i_var.clone(), a_varToArrayIndexMapping.clone(), (a_indexForUndefinedReferencesInt.clone()).clone(), a_generateFMUModelDescription.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone())
        },
    });
    }
    Ok((txt, a_stateDerVectorName, a_complexStartExpressions))
}

fn lm_68(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut a_stateDerVectorName: Tpl::Text, mut a_complexStartExpressions: Tpl::Text, mut a_generateFMUModelDescription: bool, mut a_indexForUndefinedReferencesInt: ArcStr, mut a_varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), mut a_simCode: SimCode::SimCode) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_stateDerVectorName: Tpl::Text = a_stateDerVectorName;
    let mut a_complexStartExpressions: Tpl::Text = a_complexStartExpressions;
    for mut lstElt_68 in &*items.clone() {
        let mut lstElt_68 = lstElt_68.clone();
        (txt, a_stateDerVectorName, a_complexStartExpressions) = (match lstElt_68.clone() {
        mut i_var => {
            (txt, a_complexStartExpressions, a_stateDerVectorName) = scalarVariableXML(txt.clone(), a_simCode.clone(), i_var.clone(), a_varToArrayIndexMapping.clone(), (a_indexForUndefinedReferencesInt.clone()).clone(), a_generateFMUModelDescription.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone())
        },
    });
    }
    Ok((txt, a_stateDerVectorName, a_complexStartExpressions))
}

fn lm_69(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut a_stateDerVectorName: Tpl::Text, mut a_complexStartExpressions: Tpl::Text, mut a_generateFMUModelDescription: bool, mut a_indexForUndefinedReferencesBool: ArcStr, mut a_varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), mut a_simCode: SimCode::SimCode) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_stateDerVectorName: Tpl::Text = a_stateDerVectorName;
    let mut a_complexStartExpressions: Tpl::Text = a_complexStartExpressions;
    for mut lstElt_69 in &*items.clone() {
        let mut lstElt_69 = lstElt_69.clone();
        (txt, a_stateDerVectorName, a_complexStartExpressions) = (match lstElt_69.clone() {
        mut i_var => {
            (txt, a_complexStartExpressions, a_stateDerVectorName) = scalarVariableXML(txt.clone(), a_simCode.clone(), i_var.clone(), a_varToArrayIndexMapping.clone(), (a_indexForUndefinedReferencesBool.clone()).clone(), a_generateFMUModelDescription.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone())
        },
    });
    }
    Ok((txt, a_stateDerVectorName, a_complexStartExpressions))
}

fn lm_70(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut a_stateDerVectorName: Tpl::Text, mut a_complexStartExpressions: Tpl::Text, mut a_generateFMUModelDescription: bool, mut a_indexForUndefinedReferencesBool: ArcStr, mut a_varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), mut a_simCode: SimCode::SimCode) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_stateDerVectorName: Tpl::Text = a_stateDerVectorName;
    let mut a_complexStartExpressions: Tpl::Text = a_complexStartExpressions;
    for mut lstElt_70 in &*items.clone() {
        let mut lstElt_70 = lstElt_70.clone();
        (txt, a_stateDerVectorName, a_complexStartExpressions) = (match lstElt_70.clone() {
        mut i_var => {
            (txt, a_complexStartExpressions, a_stateDerVectorName) = scalarVariableXML(txt.clone(), a_simCode.clone(), i_var.clone(), a_varToArrayIndexMapping.clone(), (a_indexForUndefinedReferencesBool.clone()).clone(), a_generateFMUModelDescription.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone())
        },
    });
    }
    Ok((txt, a_stateDerVectorName, a_complexStartExpressions))
}

fn lm_71(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut a_stateDerVectorName: Tpl::Text, mut a_complexStartExpressions: Tpl::Text, mut a_generateFMUModelDescription: bool, mut a_indexForUndefinedReferencesBool: ArcStr, mut a_varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), mut a_simCode: SimCode::SimCode) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_stateDerVectorName: Tpl::Text = a_stateDerVectorName;
    let mut a_complexStartExpressions: Tpl::Text = a_complexStartExpressions;
    for mut lstElt_71 in &*items.clone() {
        let mut lstElt_71 = lstElt_71.clone();
        (txt, a_stateDerVectorName, a_complexStartExpressions) = (match lstElt_71.clone() {
        mut i_var => {
            (txt, a_complexStartExpressions, a_stateDerVectorName) = scalarVariableXML(txt.clone(), a_simCode.clone(), i_var.clone(), a_varToArrayIndexMapping.clone(), (a_indexForUndefinedReferencesBool.clone()).clone(), a_generateFMUModelDescription.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone())
        },
    });
    }
    Ok((txt, a_stateDerVectorName, a_complexStartExpressions))
}

fn lm_72(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut a_stateDerVectorName: Tpl::Text, mut a_complexStartExpressions: Tpl::Text, mut a_generateFMUModelDescription: bool, mut a_indexForUndefinedReferencesString: ArcStr, mut a_varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), mut a_simCode: SimCode::SimCode) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_stateDerVectorName: Tpl::Text = a_stateDerVectorName;
    let mut a_complexStartExpressions: Tpl::Text = a_complexStartExpressions;
    for mut lstElt_72 in &*items.clone() {
        let mut lstElt_72 = lstElt_72.clone();
        (txt, a_stateDerVectorName, a_complexStartExpressions) = (match lstElt_72.clone() {
        mut i_var => {
            (txt, a_complexStartExpressions, a_stateDerVectorName) = scalarVariableXML(txt.clone(), a_simCode.clone(), i_var.clone(), a_varToArrayIndexMapping.clone(), (a_indexForUndefinedReferencesString.clone()).clone(), a_generateFMUModelDescription.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone())
        },
    });
    }
    Ok((txt, a_stateDerVectorName, a_complexStartExpressions))
}

fn lm_73(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut a_stateDerVectorName: Tpl::Text, mut a_complexStartExpressions: Tpl::Text, mut a_generateFMUModelDescription: bool, mut a_indexForUndefinedReferencesString: ArcStr, mut a_varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), mut a_simCode: SimCode::SimCode) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_stateDerVectorName: Tpl::Text = a_stateDerVectorName;
    let mut a_complexStartExpressions: Tpl::Text = a_complexStartExpressions;
    for mut lstElt_73 in &*items.clone() {
        let mut lstElt_73 = lstElt_73.clone();
        (txt, a_stateDerVectorName, a_complexStartExpressions) = (match lstElt_73.clone() {
        mut i_var => {
            (txt, a_complexStartExpressions, a_stateDerVectorName) = scalarVariableXML(txt.clone(), a_simCode.clone(), i_var.clone(), a_varToArrayIndexMapping.clone(), (a_indexForUndefinedReferencesString.clone()).clone(), a_generateFMUModelDescription.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone())
        },
    });
    }
    Ok((txt, a_stateDerVectorName, a_complexStartExpressions))
}

fn lm_74(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut a_stateDerVectorName: Tpl::Text, mut a_complexStartExpressions: Tpl::Text, mut a_generateFMUModelDescription: bool, mut a_indexForUndefinedReferencesString: ArcStr, mut a_varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), mut a_simCode: SimCode::SimCode) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_stateDerVectorName: Tpl::Text = a_stateDerVectorName;
    let mut a_complexStartExpressions: Tpl::Text = a_complexStartExpressions;
    for mut lstElt_74 in &*items.clone() {
        let mut lstElt_74 = lstElt_74.clone();
        (txt, a_stateDerVectorName, a_complexStartExpressions) = (match lstElt_74.clone() {
        mut i_var => {
            (txt, a_complexStartExpressions, a_stateDerVectorName) = scalarVariableXML(txt.clone(), a_simCode.clone(), i_var.clone(), a_varToArrayIndexMapping.clone(), (a_indexForUndefinedReferencesString.clone()).clone(), a_generateFMUModelDescription.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone())
        },
    });
    }
    Ok((txt, a_stateDerVectorName, a_complexStartExpressions))
}

fn fun_75(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo, mut in_a_simCode: SimCode::SimCode, mut in_a_varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), mut in_a_indexForUndefinedReferencesReal: ArcStr, mut in_a_indexForUndefinedReferencesInt: ArcStr, mut in_a_indexForUndefinedReferencesBool: ArcStr, mut in_a_indexForUndefinedReferencesString: ArcStr, mut in_a_generateFMUModelDescription: bool, mut in_a_complexStartExpressions: Tpl::Text, mut in_a_stateDerVectorName: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_complexStartExpressions: Tpl::Text;
    let mut out_a_stateDerVectorName: Tpl::Text;
    (out_txt, out_a_complexStartExpressions, out_a_stateDerVectorName) = (match (in_txt.clone(), in_a_modelInfo.clone(), in_a_simCode.clone(), in_a_varToArrayIndexMapping.clone(), in_a_indexForUndefinedReferencesReal.clone(), in_a_indexForUndefinedReferencesInt.clone(), in_a_indexForUndefinedReferencesBool.clone(), in_a_indexForUndefinedReferencesString.clone(), in_a_generateFMUModelDescription.clone(), in_a_complexStartExpressions.clone(), in_a_stateDerVectorName.clone()) {
        (mut txt, SimCode::ModelInfo { vars: SimCodeVar::SimVars { stateVars: ref i_vars_stateVars, derivativeVars: ref i_vars_derivativeVars, algVars: ref i_vars_algVars, discreteAlgVars: ref i_vars_discreteAlgVars, paramVars: ref i_vars_paramVars, aliasVars: ref i_vars_aliasVars, intAlgVars: ref i_vars_intAlgVars, intParamVars: ref i_vars_intParamVars, intAliasVars: ref i_vars_intAliasVars, boolAlgVars: ref i_vars_boolAlgVars, boolParamVars: ref i_vars_boolParamVars, boolAliasVars: ref i_vars_boolAliasVars, stringAlgVars: ref i_vars_stringAlgVars, stringParamVars: ref i_vars_stringParamVars, stringAliasVars: ref i_vars_stringAliasVars, .. }, varInfo: SimCode::VarInfo { numAlgVars: _, numDiscreteReal: _, numOptimizeConstraints: _, .. }, .. }, mut a_simCode, mut a_varToArrayIndexMapping, mut a_indexForUndefinedReferencesReal, mut a_indexForUndefinedReferencesInt, mut a_indexForUndefinedReferencesBool, mut a_indexForUndefinedReferencesString, mut a_generateFMUModelDescription, mut a_complexStartExpressions, mut a_stateDerVectorName) => {
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (txt, a_stateDerVectorName, a_complexStartExpressions) = lm_60(txt.clone(), i_vars_stateVars.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone(), a_generateFMUModelDescription.clone(), (a_indexForUndefinedReferencesReal.clone()).clone(), a_varToArrayIndexMapping.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (txt, a_stateDerVectorName, a_complexStartExpressions) = lm_61(txt.clone(), i_vars_derivativeVars.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone(), a_generateFMUModelDescription.clone(), (a_indexForUndefinedReferencesReal.clone()).clone(), a_varToArrayIndexMapping.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (txt, a_stateDerVectorName, a_complexStartExpressions) = lm_62(txt.clone(), i_vars_algVars.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone(), a_generateFMUModelDescription.clone(), (a_indexForUndefinedReferencesReal.clone()).clone(), a_varToArrayIndexMapping.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (txt, a_stateDerVectorName, a_complexStartExpressions) = lm_63(txt.clone(), i_vars_discreteAlgVars.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone(), a_generateFMUModelDescription.clone(), (a_indexForUndefinedReferencesReal.clone()).clone(), a_varToArrayIndexMapping.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (txt, a_stateDerVectorName, a_complexStartExpressions) = lm_64(txt.clone(), i_vars_paramVars.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone(), a_generateFMUModelDescription.clone(), (a_indexForUndefinedReferencesReal.clone()).clone(), a_varToArrayIndexMapping.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (txt, a_stateDerVectorName, a_complexStartExpressions) = lm_65(txt.clone(), i_vars_aliasVars.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone(), a_generateFMUModelDescription.clone(), (a_indexForUndefinedReferencesReal.clone()).clone(), a_varToArrayIndexMapping.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (txt, a_stateDerVectorName, a_complexStartExpressions) = lm_66(txt.clone(), i_vars_intAlgVars.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone(), a_generateFMUModelDescription.clone(), (a_indexForUndefinedReferencesInt.clone()).clone(), a_varToArrayIndexMapping.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (txt, a_stateDerVectorName, a_complexStartExpressions) = lm_67(txt.clone(), i_vars_intParamVars.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone(), a_generateFMUModelDescription.clone(), (a_indexForUndefinedReferencesInt.clone()).clone(), a_varToArrayIndexMapping.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (txt, a_stateDerVectorName, a_complexStartExpressions) = lm_68(txt.clone(), i_vars_intAliasVars.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone(), a_generateFMUModelDescription.clone(), (a_indexForUndefinedReferencesInt.clone()).clone(), a_varToArrayIndexMapping.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (txt, a_stateDerVectorName, a_complexStartExpressions) = lm_69(txt.clone(), i_vars_boolAlgVars.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone(), a_generateFMUModelDescription.clone(), (a_indexForUndefinedReferencesBool.clone()).clone(), a_varToArrayIndexMapping.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (txt, a_stateDerVectorName, a_complexStartExpressions) = lm_70(txt.clone(), i_vars_boolParamVars.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone(), a_generateFMUModelDescription.clone(), (a_indexForUndefinedReferencesBool.clone()).clone(), a_varToArrayIndexMapping.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (txt, a_stateDerVectorName, a_complexStartExpressions) = lm_71(txt.clone(), i_vars_boolAliasVars.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone(), a_generateFMUModelDescription.clone(), (a_indexForUndefinedReferencesBool.clone()).clone(), a_varToArrayIndexMapping.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (txt, a_stateDerVectorName, a_complexStartExpressions) = lm_72(txt.clone(), i_vars_stringAlgVars.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone(), a_generateFMUModelDescription.clone(), (a_indexForUndefinedReferencesString.clone()).clone(), a_varToArrayIndexMapping.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (txt, a_stateDerVectorName, a_complexStartExpressions) = lm_73(txt.clone(), i_vars_stringParamVars.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone(), a_generateFMUModelDescription.clone(), (a_indexForUndefinedReferencesString.clone()).clone(), a_varToArrayIndexMapping.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (txt, a_stateDerVectorName, a_complexStartExpressions) = lm_74(txt.clone(), i_vars_stringAliasVars.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone(), a_generateFMUModelDescription.clone(), (a_indexForUndefinedReferencesString.clone()).clone(), a_varToArrayIndexMapping.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            (txt.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())
        },
        (mut txt, _, _, _, _, _, _, _, _, mut a_complexStartExpressions, mut a_stateDerVectorName) => {
            (txt.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())
        },
    });
    Ok((out_txt, out_a_complexStartExpressions, out_a_stateDerVectorName))
}

pub fn modelVariablesXML(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_modelInfo: SimCode::ModelInfo, mut a_varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), mut a_indexForUndefinedReferencesReal: ArcStr, mut a_indexForUndefinedReferencesInt: ArcStr, mut a_indexForUndefinedReferencesBool: ArcStr, mut a_indexForUndefinedReferencesString: ArcStr, mut a_generateFMUModelDescription: bool, mut a_complexStartExpressions: Tpl::Text, mut a_stateDerVectorName: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_complexStartExpressions: Tpl::Text;
    let mut out_a_stateDerVectorName: Tpl::Text;
    (out_txt, out_a_complexStartExpressions, out_a_stateDerVectorName) = fun_75(txt.clone(), a_modelInfo.clone(), a_simCode.clone(), a_varToArrayIndexMapping.clone(), (a_indexForUndefinedReferencesReal.clone()).clone(), (a_indexForUndefinedReferencesInt.clone()).clone(), (a_indexForUndefinedReferencesBool.clone()).clone(), (a_indexForUndefinedReferencesString.clone()).clone(), a_generateFMUModelDescription.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())?;
    Ok((out_txt, out_a_complexStartExpressions, out_a_stateDerVectorName))
}

fn fun_77(mut in_txt: Tpl::Text, mut in_a_generateFMUModelDescription: bool, mut in_a_stateDerVectorName: Tpl::Text, mut in_a_complexStartExpressions: Tpl::Text, mut in_a_type__: Arc<DAE::Type>, mut in_a_isFixed: bool, mut in_a_nominalValue: Option<Arc<DAE::Exp>>, mut in_a_initialValue: Option<Arc<DAE::Exp>>, mut in_a_maxValue: Option<Arc<DAE::Exp>>, mut in_a_minValue: Option<Arc<DAE::Exp>>, mut in_a_displayUnit: ArcStr, mut in_a_unit: ArcStr, mut in_a_aliasvar: SimCodeVar::AliasVariable, mut in_a_name: Arc<DAE::ComponentRef>, mut in_a_simCode: SimCode::SimCode) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_stateDerVectorName: Tpl::Text;
    let mut out_a_complexStartExpressions: Tpl::Text;
    (out_txt, out_a_stateDerVectorName, out_a_complexStartExpressions) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_generateFMUModelDescription.clone(), in_a_stateDerVectorName.clone(), in_a_complexStartExpressions.clone(), in_a_type__.clone(), in_a_isFixed.clone(), in_a_nominalValue.clone(), in_a_initialValue.clone(), in_a_maxValue.clone(), in_a_minValue.clone(), in_a_displayUnit.clone(), in_a_unit.clone(), in_a_aliasvar.clone(), in_a_name.clone(), in_a_simCode.clone())) {
        (txt, false, a_stateDerVectorName, a_complexStartExpressions, a_type__, a_isFixed, a_nominalValue, a_initialValue, a_maxValue, a_minValue, a_displayUnit, a_unit, a_aliasvar, a_name, a_simCode) => {
            let mut ret_0: Arc<DAE::Type>;
            let mut txt = (*txt).clone();
            let mut a_stateDerVectorName = (*a_stateDerVectorName).clone();
            let mut a_complexStartExpressions = (*a_complexStartExpressions).clone();
            ret_0 = Types::arrayElementType(a_type__.clone());
            (txt, a_complexStartExpressions, a_stateDerVectorName) = ScalarVariableType(txt.clone(), a_simCode.clone(), a_name.clone(), a_aliasvar.clone(), (a_unit.clone()).clone(), (a_displayUnit.clone()).clone(), a_minValue.clone(), a_maxValue.clone(), a_initialValue.clone(), a_nominalValue.clone(), a_isFixed.clone(), ret_0.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())?;
            (txt.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone())
        },
        (txt, _, a_stateDerVectorName, a_complexStartExpressions, _, _, _, _, _, _, _, _, _, _, _) => {
            (txt.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_stateDerVectorName, out_a_complexStartExpressions))
}

fn lm_78(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut a_stateDerVectorName: Tpl::Text, mut a_complexStartExpressionsForScalarsUnused: Tpl::Text, mut a_generateFMUModelDescription: bool, mut a_indexForUndefinedReferences: ArcStr, mut a_varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), mut a_simCode: SimCode::SimCode) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_stateDerVectorName: Tpl::Text = a_stateDerVectorName;
    let mut a_complexStartExpressionsForScalarsUnused: Tpl::Text = a_complexStartExpressionsForScalarsUnused;
    for mut lstElt_78 in &*items.clone() {
        let mut lstElt_78 = lstElt_78.clone();
        (txt, a_stateDerVectorName, a_complexStartExpressionsForScalarsUnused) = (match lstElt_78.clone() {
        mut i_var => {
            (txt, a_complexStartExpressionsForScalarsUnused, a_stateDerVectorName) = scalarVariableXML(txt.clone(), a_simCode.clone(), i_var.clone(), a_varToArrayIndexMapping.clone(), (a_indexForUndefinedReferences.clone()).clone(), a_generateFMUModelDescription.clone(), a_complexStartExpressionsForScalarsUnused.clone(), a_stateDerVectorName.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_stateDerVectorName.clone(), a_complexStartExpressionsForScalarsUnused.clone())
        },
    });
    }
    Ok((txt, a_stateDerVectorName, a_complexStartExpressionsForScalarsUnused))
}

fn fun_79(mut in_txt: Tpl::Text, mut in_a_generateFMUModelDescription: bool, mut in_a_simVar: SimCodeVar::SimVar, mut in_a_stateDerVectorName: Tpl::Text, mut in_a_complexStartExpressions: Tpl::Text, mut in_a_type__: Arc<DAE::Type>, mut in_a_isFixed: bool, mut in_a_nominalValue: Option<Arc<DAE::Exp>>, mut in_a_initialValue: Option<Arc<DAE::Exp>>, mut in_a_maxValue: Option<Arc<DAE::Exp>>, mut in_a_minValue: Option<Arc<DAE::Exp>>, mut in_a_displayUnit: ArcStr, mut in_a_unit: ArcStr, mut in_a_aliasvar: SimCodeVar::AliasVariable, mut in_a_name: Arc<DAE::ComponentRef>, mut in_a_simCode: SimCode::SimCode) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_stateDerVectorName: Tpl::Text;
    let mut out_a_complexStartExpressions: Tpl::Text;
    (out_txt, out_a_stateDerVectorName, out_a_complexStartExpressions) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_generateFMUModelDescription.clone(), in_a_simVar.clone(), in_a_stateDerVectorName.clone(), in_a_complexStartExpressions.clone(), in_a_type__.clone(), in_a_isFixed.clone(), in_a_nominalValue.clone(), in_a_initialValue.clone(), in_a_maxValue.clone(), in_a_minValue.clone(), in_a_displayUnit.clone(), in_a_unit.clone(), in_a_aliasvar.clone(), in_a_name.clone(), in_a_simCode.clone())) {
        (txt, false, _, a_stateDerVectorName, a_complexStartExpressions, a_type__, a_isFixed, a_nominalValue, a_initialValue, a_maxValue, a_minValue, a_displayUnit, a_unit, a_aliasvar, a_name, a_simCode) => {
            let mut txt = (*txt).clone();
            let mut a_stateDerVectorName = (*a_stateDerVectorName).clone();
            let mut a_complexStartExpressions = (*a_complexStartExpressions).clone();
            (txt, a_complexStartExpressions, a_stateDerVectorName) = ScalarVariableType(txt.clone(), a_simCode.clone(), a_name.clone(), a_aliasvar.clone(), (a_unit.clone()).clone(), (a_displayUnit.clone()).clone(), a_minValue.clone(), a_maxValue.clone(), a_initialValue.clone(), a_nominalValue.clone(), a_isFixed.clone(), a_type__.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())?;
            (txt.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone())
        },
        (txt, _, a_simVar, a_stateDerVectorName, a_complexStartExpressions, _, _, _, _, _, _, _, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = CodegenFMUCommon::ScalarVariableType(txt.clone(), a_simVar.clone())?;
            (txt.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_stateDerVectorName, out_a_complexStartExpressions))
}

fn fun_80(mut in_txt: Tpl::Text, mut in_a_simVar: SimCodeVar::SimVar, mut in_a_simCode: SimCode::SimCode, mut in_a_varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), mut in_a_indexForUndefinedReferences: ArcStr, mut in_a_generateFMUModelDescription: bool, mut in_a_complexStartExpressions: Tpl::Text, mut in_a_stateDerVectorName: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_complexStartExpressions: Tpl::Text;
    let mut out_a_stateDerVectorName: Tpl::Text;
    (out_txt, out_a_complexStartExpressions, out_a_stateDerVectorName) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_simVar.clone(), in_a_simCode.clone(), in_a_varToArrayIndexMapping.clone(), in_a_indexForUndefinedReferences.clone(), in_a_generateFMUModelDescription.clone(), in_a_complexStartExpressions.clone(), in_a_stateDerVectorName.clone())) {
        (txt, i_simVar @ SimCodeVar::SimVar { type_: i_type__ @ Deref @ DAE::Type::T_ARRAY { ty: _, .. }, name: i_name, aliasvar: i_aliasvar, unit: i_unit, displayUnit: i_displayUnit, minValue: i_minValue, maxValue: i_maxValue, initialValue: i_initialValue, nominalValue: i_nominalValue, isFixed: i_isFixed, .. }, a_simCode, a_varToArrayIndexMapping, a_indexForUndefinedReferences, a_generateFMUModelDescription, a_complexStartExpressions, a_stateDerVectorName) => {
            let mut ret_2: Arc<metamodelica::List<SimCodeVar::SimVar>>;
            let mut l_complexStartExpressionsForScalarsUnused: Tpl::Text;
            let mut l_0__: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_complexStartExpressions = (*a_complexStartExpressions).clone();
            let mut a_stateDerVectorName = (*a_stateDerVectorName).clone();
            (l_0__, a_stateDerVectorName, a_complexStartExpressions) = fun_77(Tpl::emptyTxt.clone(), a_generateFMUModelDescription.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone(), i_type__.clone(), i_isFixed.clone(), i_nominalValue.clone(), i_initialValue.clone(), i_maxValue.clone(), i_minValue.clone(), (i_displayUnit.clone()).clone(), (i_unit.clone()).clone(), i_aliasvar.clone(), i_name.clone(), a_simCode.clone())?;
            l_complexStartExpressionsForScalarsUnused = Tpl::emptyTxt.clone();
            ret_2 = SimCodeUtil::getScalarElements(i_simVar.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (txt, a_stateDerVectorName, l_complexStartExpressionsForScalarsUnused) = lm_78(txt.clone(), ret_2.clone(), a_stateDerVectorName.clone(), l_complexStartExpressionsForScalarsUnused.clone(), a_generateFMUModelDescription.clone(), (a_indexForUndefinedReferences.clone()).clone(), a_varToArrayIndexMapping.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            (txt.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())
        },
        (txt, i_simVar @ SimCodeVar::SimVar { name: i_name, aliasvar: i_aliasvar, unit: i_unit, displayUnit: i_displayUnit, minValue: i_minValue, maxValue: i_maxValue, initialValue: i_initialValue, nominalValue: i_nominalValue, isFixed: i_isFixed, type_: i_type__, .. }, a_simCode, _, a_indexForUndefinedReferences, a_generateFMUModelDescription, a_complexStartExpressions, a_stateDerVectorName) => {
            let mut l_variableCode: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_complexStartExpressions = (*a_complexStartExpressions).clone();
            let mut a_stateDerVectorName = (*a_stateDerVectorName).clone();
            (l_variableCode, a_stateDerVectorName, a_complexStartExpressions) = fun_79(Tpl::emptyTxt.clone(), a_generateFMUModelDescription.clone(), i_simVar.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone(), i_type__.clone(), i_isFixed.clone(), i_nominalValue.clone(), i_initialValue.clone(), i_maxValue.clone(), i_minValue.clone(), (i_displayUnit.clone()).clone(), (i_unit.clone()).clone(), i_aliasvar.clone(), i_name.clone(), a_simCode.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<ScalarVariable ")).clone() }))?;
            txt = scalarVariableAttributeXML(txt.clone(), i_simVar.clone(), a_simCode.clone(), (a_indexForUndefinedReferences.clone()).clone(), a_generateFMUModelDescription.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(">\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_variableCode.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</ScalarVariable>")).clone() }))?;
            (txt.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())
        },
        (txt, _, _, _, _, _, a_complexStartExpressions, a_stateDerVectorName) => {
            (txt.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_complexStartExpressions, out_a_stateDerVectorName))
}

pub fn scalarVariableXML(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_simVar: SimCodeVar::SimVar, mut a_varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), mut a_indexForUndefinedReferences: ArcStr, mut a_generateFMUModelDescription: bool, mut a_complexStartExpressions: Tpl::Text, mut a_stateDerVectorName: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_complexStartExpressions: Tpl::Text;
    let mut out_a_stateDerVectorName: Tpl::Text;
    (out_txt, out_a_complexStartExpressions, out_a_stateDerVectorName) = fun_80(txt.clone(), a_simVar.clone(), a_simCode.clone(), a_varToArrayIndexMapping.clone(), (a_indexForUndefinedReferences.clone()).clone(), a_generateFMUModelDescription.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())?;
    Ok((out_txt, out_a_complexStartExpressions, out_a_stateDerVectorName))
}

fn fun_82(mut in_txt: Tpl::Text, mut in_a_comment: ArcStr) -> Result<Tpl::Text> {
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

fn fun_83(mut in_txt: Tpl::Text, mut in_a_hideResult: Option<bool>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_hideResult.clone()) {
        (mut txt, Some(mut i_bval)) => {
            txt = Tpl::writeStr(txt.clone(), (Tpl::booleanString(i_bval.clone())).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_84(mut in_txt: Tpl::Text, mut in_a_generateFMUModelDescription: bool, mut in_a_isValueChangeable: bool, mut in_a_isDiscrete: bool, mut in_a_hr: Tpl::Text, mut in_a_isProtected: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_generateFMUModelDescription.clone(), in_a_isValueChangeable.clone(), in_a_isDiscrete.clone(), in_a_hr.clone(), in_a_isProtected.clone()) {
        (mut txt, false, mut a_isValueChangeable, mut a_isDiscrete, mut a_hr, mut a_isProtected) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("isProtected=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (Tpl::booleanString(a_isProtected.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" hideResult=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_hr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" isDiscrete=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (Tpl::booleanString(a_isDiscrete.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" isValueChangeable=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (Tpl::booleanString(a_isValueChangeable.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_85(mut in_txt: Tpl::Text, mut in_a_simVar: SimCodeVar::SimVar, mut in_a_simCode: SimCode::SimCode, mut in_a_generateFMUModelDescription: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_simVar.clone(), in_a_simCode.clone(), in_a_generateFMUModelDescription.clone())) {
        (txt, i_simVar @ SimCodeVar::SimVar { source: Deref @ DAE::ElementSource { info: _, .. }, aliasvar: i_aliasvar, causality: i_causality, varKind: i_varKind, comment: i_comment, hideResult: i_hideResult, isProtected: i_isProtected, isDiscrete: i_isDiscrete, isValueChangeable: i_isValueChangeable, name: i_name, .. }, a_simCode, a_generateFMUModelDescription) => {
            let mut ret_10: ArcStr;
            let mut ret_9: ArcStr;
            let mut txt_8: Tpl::Text;
            let mut l_additionalAttributes: Tpl::Text;
            let mut l_hr: Tpl::Text;
            let mut l_description: Tpl::Text;
            let mut l_variability: Tpl::Text;
            let mut l_causalityAtt: Tpl::Text;
            let mut l_alias: Tpl::Text;
            let mut ret_1: ArcStr;
            let mut l_valueReference: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_1 = (SimCodeUtil::getValueReference(i_simVar.clone(), a_simCode.clone(), true)?).clone();
            l_valueReference = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1.clone()).clone())?;
            l_alias = getAliasAttribute(Tpl::emptyTxt.clone(), i_aliasvar.clone())?;
            l_causalityAtt = CodegenFMUCommon::getCausality(Tpl::emptyTxt.clone(), i_causality.clone())?;
            l_variability = CodegenUtil::getVariablity(Tpl::emptyTxt.clone(), i_varKind.clone())?;
            l_description = fun_82(Tpl::emptyTxt.clone(), (i_comment.clone()).clone())?;
            l_hr = fun_83(Tpl::emptyTxt.clone(), i_hideResult.clone())?;
            l_additionalAttributes = fun_84(Tpl::emptyTxt.clone(), a_generateFMUModelDescription.clone(), i_isValueChangeable.clone(), i_isDiscrete.clone(), l_hr.clone(), i_isProtected.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("name=\"")).clone() }))?;
            txt_8 = CodegenUtil::crefStrNoUnderscore(Tpl::emptyTxt.clone(), i_name.clone())?;
            ret_9 = (Util::escapeModelicaStringToXmlString((Tpl::textString(txt_8.clone())?).clone())?).clone();
            ret_10 = (System::stringReplace((ret_9.clone()).clone(), (literal!("$")).clone(), (literal!("_D_")).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_10.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" valueReference=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_valueReference.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_description.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" variability=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_variability.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" causality=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_causalityAtt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" alias=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_alias.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_additionalAttributes.clone())?;
            txt.clone()
        },
        (txt, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn scalarVariableAttributeXML(mut txt: Tpl::Text, mut a_simVar: SimCodeVar::SimVar, mut a_simCode: SimCode::SimCode, mut a_indexForUndefinedReferences: ArcStr, mut a_generateFMUModelDescription: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_85(txt.clone(), a_simVar.clone(), a_simCode.clone(), a_generateFMUModelDescription.clone())?;
    Ok(out_txt)
}

pub fn getAliasAttribute(mut in_txt: Tpl::Text, mut in_a_aliasvar: SimCodeVar::AliasVariable) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_aliasvar.clone()) {
        (mut txt, SimCodeVar::AliasVariable::NOALIAS { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("noAlias")).clone() }))?;
            txt.clone()
        },
        (mut txt, SimCodeVar::AliasVariable::ALIAS { varName: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("alias")).clone() }))?;
            txt.clone()
        },
        (mut txt, SimCodeVar::AliasVariable::NEGATEDALIAS { varName: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("negatedAlias")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("undefinedAliasType")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_88(mut in_txt: Tpl::Text, mut in_a_type__: Arc<DAE::Type>, mut in_a_simCode: SimCode::SimCode, mut in_a_simVarCref: Arc<DAE::ComponentRef>, mut in_a_simVarAlias: SimCodeVar::AliasVariable, mut in_a_unit: ArcStr, mut in_a_displayUnit: ArcStr, mut in_a_minValue: Option<Arc<DAE::Exp>>, mut in_a_maxValue: Option<Arc<DAE::Exp>>, mut in_a_startValue: Option<Arc<DAE::Exp>>, mut in_a_nominalValue: Option<Arc<DAE::Exp>>, mut in_a_isFixed: bool, mut in_a_complexStartExpressions: Tpl::Text, mut in_a_stateDerVectorName: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_complexStartExpressions: Tpl::Text;
    let mut out_a_stateDerVectorName: Tpl::Text;
    (out_txt, out_a_complexStartExpressions, out_a_stateDerVectorName) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_type__.clone(), in_a_simCode.clone(), in_a_simVarCref.clone(), in_a_simVarAlias.clone(), in_a_unit.clone(), in_a_displayUnit.clone(), in_a_minValue.clone(), in_a_maxValue.clone(), in_a_startValue.clone(), in_a_nominalValue.clone(), in_a_isFixed.clone(), in_a_complexStartExpressions.clone(), in_a_stateDerVectorName.clone())) {
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }, a_simCode, a_simVarCref, a_simVarAlias, a_unit, a_displayUnit, a_minValue, a_maxValue, a_startValue, _, a_isFixed, a_complexStartExpressions, a_stateDerVectorName) => {
            let mut l_disp__: Tpl::Text;
            let mut l_unit__: Tpl::Text;
            let mut l_max__: Tpl::Text;
            let mut l_min__: Tpl::Text;
            let mut l_fixed__: Tpl::Text;
            let mut l_start__: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_complexStartExpressions = (*a_complexStartExpressions).clone();
            let mut a_stateDerVectorName = (*a_stateDerVectorName).clone();
            (l_start__, _, a_complexStartExpressions, a_stateDerVectorName) = ScalarVariableTypeStartAttribute(Tpl::emptyTxt.clone(), a_simCode.clone(), a_simVarCref.clone(), a_simVarAlias.clone(), a_startValue.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Int")).clone() })), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())?;
            l_fixed__ = Tpl::pushBlock(Tpl::emptyTxt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            l_fixed__ = Tpl::writeTok(l_fixed__.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fixed=\"")).clone() }))?;
            l_fixed__ = Tpl::writeStr(l_fixed__.clone(), (Tpl::booleanString(a_isFixed.clone())).clone())?;
            l_fixed__ = Tpl::writeTok(l_fixed__.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            l_fixed__ = Tpl::popBlock(l_fixed__.clone())?;
            l_min__ = attributeOptionString(Tpl::emptyTxt.clone(), a_minValue.clone(), (literal!("min")).clone())?;
            l_max__ = attributeOptionString(Tpl::emptyTxt.clone(), a_maxValue.clone(), (literal!("max")).clone())?;
            l_unit__ = unitString(Tpl::emptyTxt.clone(), (a_unit.clone()).clone(), (literal!("unit")).clone())?;
            l_disp__ = unitString(Tpl::emptyTxt.clone(), (a_displayUnit.clone()).clone(), (literal!("displayUnit")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Integer ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_start__.clone())?;
            txt = Tpl::writeText(txt.clone(), l_fixed__.clone())?;
            txt = Tpl::writeText(txt.clone(), l_min__.clone())?;
            txt = Tpl::writeText(txt.clone(), l_max__.clone())?;
            txt = Tpl::writeText(txt.clone(), l_unit__.clone())?;
            txt = Tpl::writeText(txt.clone(), l_disp__.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" />")).clone() }))?;
            (txt.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }, a_simCode, a_simVarCref, a_simVarAlias, a_unit, a_displayUnit, a_minValue, a_maxValue, a_startValue, a_nominalValue, a_isFixed, a_complexStartExpressions, a_stateDerVectorName) => {
            let mut ret_7: bool;
            let mut l_nom__: Tpl::Text;
            let mut l_disp__: Tpl::Text;
            let mut l_unit__: Tpl::Text;
            let mut l_max__: Tpl::Text;
            let mut l_min__: Tpl::Text;
            let mut l_fixed__: Tpl::Text;
            let mut l_start__: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_complexStartExpressions = (*a_complexStartExpressions).clone();
            let mut a_stateDerVectorName = (*a_stateDerVectorName).clone();
            (l_start__, _, a_complexStartExpressions, a_stateDerVectorName) = ScalarVariableTypeStartAttribute(Tpl::emptyTxt.clone(), a_simCode.clone(), a_simVarCref.clone(), a_simVarAlias.clone(), a_startValue.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Real")).clone() })), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())?;
            l_fixed__ = Tpl::pushBlock(Tpl::emptyTxt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            l_fixed__ = Tpl::writeTok(l_fixed__.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fixed=\"")).clone() }))?;
            l_fixed__ = Tpl::writeStr(l_fixed__.clone(), (Tpl::booleanString(a_isFixed.clone())).clone())?;
            l_fixed__ = Tpl::writeTok(l_fixed__.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            l_fixed__ = Tpl::popBlock(l_fixed__.clone())?;
            l_nom__ = Tpl::pushBlock(Tpl::emptyTxt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            l_nom__ = Tpl::writeTok(l_nom__.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("useNominal=\"")).clone() }))?;
            ret_7 = isSome(a_nominalValue.clone());
            l_nom__ = Tpl::writeStr(l_nom__.clone(), (Tpl::booleanString(ret_7.clone())).clone())?;
            l_nom__ = Tpl::writeTok(l_nom__.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            l_nom__ = attributeOptionString(l_nom__.clone(), a_nominalValue.clone(), (literal!("nominal")).clone())?;
            l_nom__ = Tpl::popBlock(l_nom__.clone())?;
            l_min__ = attributeOptionString(Tpl::emptyTxt.clone(), a_minValue.clone(), (literal!("min")).clone())?;
            l_max__ = attributeOptionString(Tpl::emptyTxt.clone(), a_maxValue.clone(), (literal!("max")).clone())?;
            l_unit__ = unitString(Tpl::emptyTxt.clone(), (a_unit.clone()).clone(), (literal!("unit")).clone())?;
            l_disp__ = unitString(Tpl::emptyTxt.clone(), (a_displayUnit.clone()).clone(), (literal!("displayUnit")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Real ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_start__.clone())?;
            txt = Tpl::writeText(txt.clone(), l_fixed__.clone())?;
            txt = Tpl::writeText(txt.clone(), l_nom__.clone())?;
            txt = Tpl::writeText(txt.clone(), l_min__.clone())?;
            txt = Tpl::writeText(txt.clone(), l_max__.clone())?;
            txt = Tpl::writeText(txt.clone(), l_unit__.clone())?;
            txt = Tpl::writeText(txt.clone(), l_disp__.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" />")).clone() }))?;
            (txt.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }, a_simCode, a_simVarCref, a_simVarAlias, a_unit, a_displayUnit, _, _, a_startValue, _, a_isFixed, a_complexStartExpressions, a_stateDerVectorName) => {
            let mut l_disp__: Tpl::Text;
            let mut l_unit__: Tpl::Text;
            let mut l_fixed__: Tpl::Text;
            let mut l_start__: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_complexStartExpressions = (*a_complexStartExpressions).clone();
            let mut a_stateDerVectorName = (*a_stateDerVectorName).clone();
            (l_start__, _, a_complexStartExpressions, a_stateDerVectorName) = ScalarVariableTypeStartAttribute(Tpl::emptyTxt.clone(), a_simCode.clone(), a_simVarCref.clone(), a_simVarAlias.clone(), a_startValue.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Bool")).clone() })), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())?;
            l_fixed__ = Tpl::pushBlock(Tpl::emptyTxt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            l_fixed__ = Tpl::writeTok(l_fixed__.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fixed=\"")).clone() }))?;
            l_fixed__ = Tpl::writeStr(l_fixed__.clone(), (Tpl::booleanString(a_isFixed.clone())).clone())?;
            l_fixed__ = Tpl::writeTok(l_fixed__.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            l_fixed__ = Tpl::popBlock(l_fixed__.clone())?;
            l_unit__ = unitString(Tpl::emptyTxt.clone(), (a_unit.clone()).clone(), (literal!("unit")).clone())?;
            l_disp__ = unitString(Tpl::emptyTxt.clone(), (a_displayUnit.clone()).clone(), (literal!("displayUnit")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Boolean ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_start__.clone())?;
            txt = Tpl::writeText(txt.clone(), l_fixed__.clone())?;
            txt = Tpl::writeText(txt.clone(), l_unit__.clone())?;
            txt = Tpl::writeText(txt.clone(), l_disp__.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" />")).clone() }))?;
            (txt.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }, a_simCode, a_simVarCref, a_simVarAlias, a_unit, a_displayUnit, _, _, a_startValue, _, a_isFixed, a_complexStartExpressions, a_stateDerVectorName) => {
            let mut l_disp__: Tpl::Text;
            let mut l_unit__: Tpl::Text;
            let mut l_fixed__: Tpl::Text;
            let mut l_start__: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_complexStartExpressions = (*a_complexStartExpressions).clone();
            let mut a_stateDerVectorName = (*a_stateDerVectorName).clone();
            (l_start__, _, a_complexStartExpressions, a_stateDerVectorName) = ScalarVariableTypeStartAttribute(Tpl::emptyTxt.clone(), a_simCode.clone(), a_simVarCref.clone(), a_simVarAlias.clone(), a_startValue.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("String")).clone() })), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())?;
            l_fixed__ = Tpl::pushBlock(Tpl::emptyTxt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            l_fixed__ = Tpl::writeTok(l_fixed__.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fixed=\"")).clone() }))?;
            l_fixed__ = Tpl::writeStr(l_fixed__.clone(), (Tpl::booleanString(a_isFixed.clone())).clone())?;
            l_fixed__ = Tpl::writeTok(l_fixed__.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            l_fixed__ = Tpl::popBlock(l_fixed__.clone())?;
            l_unit__ = unitString(Tpl::emptyTxt.clone(), (a_unit.clone()).clone(), (literal!("unit")).clone())?;
            l_disp__ = unitString(Tpl::emptyTxt.clone(), (a_displayUnit.clone()).clone(), (literal!("displayUnit")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<String ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_start__.clone())?;
            txt = Tpl::writeText(txt.clone(), l_fixed__.clone())?;
            txt = Tpl::writeText(txt.clone(), l_unit__.clone())?;
            txt = Tpl::writeText(txt.clone(), l_disp__.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" />")).clone() }))?;
            (txt.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())
        },
        (txt, Deref @ DAE::Type::T_ENUMERATION { index: _, .. }, a_simCode, a_simVarCref, a_simVarAlias, a_unit, a_displayUnit, _, _, a_startValue, _, a_isFixed, a_complexStartExpressions, a_stateDerVectorName) => {
            let mut l_disp__: Tpl::Text;
            let mut l_unit__: Tpl::Text;
            let mut l_fixed__: Tpl::Text;
            let mut l_start__: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_complexStartExpressions = (*a_complexStartExpressions).clone();
            let mut a_stateDerVectorName = (*a_stateDerVectorName).clone();
            (l_start__, _, a_complexStartExpressions, a_stateDerVectorName) = ScalarVariableTypeStartAttribute(Tpl::emptyTxt.clone(), a_simCode.clone(), a_simVarCref.clone(), a_simVarAlias.clone(), a_startValue.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Int")).clone() })), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())?;
            l_fixed__ = Tpl::pushBlock(Tpl::emptyTxt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            l_fixed__ = Tpl::writeTok(l_fixed__.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fixed=\"")).clone() }))?;
            l_fixed__ = Tpl::writeStr(l_fixed__.clone(), (Tpl::booleanString(a_isFixed.clone())).clone())?;
            l_fixed__ = Tpl::writeTok(l_fixed__.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            l_fixed__ = Tpl::popBlock(l_fixed__.clone())?;
            l_unit__ = unitString(Tpl::emptyTxt.clone(), (a_unit.clone()).clone(), (literal!("unit")).clone())?;
            l_disp__ = unitString(Tpl::emptyTxt.clone(), (a_displayUnit.clone()).clone(), (literal!("displayUnit")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Integer ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_start__.clone())?;
            txt = Tpl::writeText(txt.clone(), l_fixed__.clone())?;
            txt = Tpl::writeText(txt.clone(), l_unit__.clone())?;
            txt = Tpl::writeText(txt.clone(), l_disp__.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" />")).clone() }))?;
            (txt.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())
        },
        (txt, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { path: i_ci_path }, .. }, _, _, _, _, _, _, _, _, _, _, a_complexStartExpressions, a_stateDerVectorName) => {
            let mut ret_9: ArcStr;
            let mut txt_8: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<ExternalObject path=\"")).clone() }))?;
            txt_8 = CodegenUtil::dotPath(Tpl::emptyTxt.clone(), i_ci_path.clone())?;
            ret_9 = (Util::escapeModelicaStringToXmlString((Tpl::textString(txt_8.clone())?).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_9.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" />")).clone() }))?;
            (txt.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())
        },
        (txt, i_type__, _, _, _, _, _, _, _, _, _, _, a_complexStartExpressions, a_stateDerVectorName) => {
            let mut txt_10: Tpl::Text;
            let mut ret_10: ArcStr;
            let mut txt = (*txt).clone();
            txt_10 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ScalarVariableType: ")).clone() }))?;
            ret_10 = (TypesDump::unparseType(i_type__.clone())?).clone();
            txt_10 = Tpl::writeStr(txt_10.clone(), (ret_10.clone()).clone())?;
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenCppInit.tpl")).clone(), 243, 16), (Tpl::textString(txt_10.clone())?).clone())?;
            (txt.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_complexStartExpressions, out_a_stateDerVectorName))
}

pub fn ScalarVariableType(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_simVarCref: Arc<DAE::ComponentRef>, mut a_simVarAlias: SimCodeVar::AliasVariable, mut a_unit: ArcStr, mut a_displayUnit: ArcStr, mut a_minValue: Option<Arc<DAE::Exp>>, mut a_maxValue: Option<Arc<DAE::Exp>>, mut a_startValue: Option<Arc<DAE::Exp>>, mut a_nominalValue: Option<Arc<DAE::Exp>>, mut a_isFixed: bool, mut a_type__: Arc<DAE::Type>, mut a_complexStartExpressions: Tpl::Text, mut a_stateDerVectorName: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_complexStartExpressions: Tpl::Text;
    let mut out_a_stateDerVectorName: Tpl::Text;
    (out_txt, out_a_complexStartExpressions, out_a_stateDerVectorName) = fun_88(txt.clone(), a_type__.clone(), a_simCode.clone(), a_simVarCref.clone(), a_simVarAlias.clone(), (a_unit.clone()).clone(), (a_displayUnit.clone()).clone(), a_minValue.clone(), a_maxValue.clone(), a_startValue.clone(), a_nominalValue.clone(), a_isFixed.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())?;
    Ok((out_txt, out_a_complexStartExpressions, out_a_stateDerVectorName))
}

fn lm_90(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_90 in &*items.clone() {
        let mut lstElt_90 = lstElt_90.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_90.clone()) {
        i_elem => {
            txt = CodegenUtil::initValXml(txt.clone(), i_elem.clone(), (literal!("&quot;")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_91(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_array: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut in_a_attr__name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_array.clone(), in_a_attr__name.clone())) {
        (txt, false, _, _) => {
            txt.clone()
        },
        (txt, _, a_array, a_attr__name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeStr(txt.clone(), (a_attr__name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("=\"")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_90(txt.clone(), a_array.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_92(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_attr__name: ArcStr, mut in_a_expr: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_attr__name.clone(), in_a_expr.clone())) {
        (txt, false, _, _) => {
            txt.clone()
        },
        (txt, _, a_attr__name, a_expr) => {
            let mut txt = (*txt).clone();
            txt = attributeString(txt.clone(), a_expr.clone(), (a_attr__name.clone()).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn attributeString(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_attr__name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_attr__name.clone())) {
        (txt, i_exp @ Deref @ DAE::Exp::ICONST { integer: _ }, a_attr__name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeStr(txt.clone(), (a_attr__name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("=\"")).clone() }))?;
            txt = CodegenUtil::initValXml(txt.clone(), i_exp.clone(), (literal!("")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, i_exp @ Deref @ DAE::Exp::RCONST { real: _ }, a_attr__name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeStr(txt.clone(), (a_attr__name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("=\"")).clone() }))?;
            txt = CodegenUtil::initValXml(txt.clone(), i_exp.clone(), (literal!("")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, i_exp @ Deref @ DAE::Exp::SCONST { string: _ }, a_attr__name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeStr(txt.clone(), (a_attr__name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("=\"")).clone() }))?;
            txt = CodegenUtil::initValXml(txt.clone(), i_exp.clone(), (literal!("")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, i_exp @ Deref @ DAE::Exp::BCONST { bool: _ }, a_attr__name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeStr(txt.clone(), (a_attr__name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("=\"")).clone() }))?;
            txt = CodegenUtil::initValXml(txt.clone(), i_exp.clone(), (literal!("")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, i_exp @ Deref @ DAE::Exp::ENUM_LITERAL { name: _, .. }, a_attr__name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeStr(txt.clone(), (a_attr__name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("=\"")).clone() }))?;
            txt = CodegenUtil::initValXml(txt.clone(), i_exp.clone(), (literal!("")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, i_exp @ Deref @ DAE::Exp::ARRAY { array: i_array, .. }, a_attr__name) => {
            let mut ret_0: bool;
            let mut txt = (*txt).clone();
            ret_0 = Expression::isSimpleLiteralValue(i_exp.clone(), true)?;
            txt = fun_91(txt.clone(), ret_0.clone(), i_array.clone(), (a_attr__name.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::REDUCTION { expr: i_expr, .. }, a_attr__name) => {
            let mut ret_1: bool;
            let mut txt = (*txt).clone();
            ret_1 = Expression::isSimpleLiteralValue(i_expr.clone(), true)?;
            txt = fun_92(txt.clone(), ret_1.clone(), (a_attr__name.clone()).clone(), i_expr.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn attributeOptionString(mut in_txt: Tpl::Text, mut in_a_exp__opt: Option<Arc<DAE::Exp>>, mut in_a_attr__name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp__opt.clone(), in_a_attr__name.clone())) {
        (txt, Some(i_exp), a_attr__name) => {
            let mut txt = (*txt).clone();
            txt = attributeString(txt.clone(), i_exp.clone(), (a_attr__name.clone()).clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn unitString(mut in_txt: Tpl::Text, mut in_a_unit: ArcStr, mut in_a_attr__name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_unit.clone(), in_a_attr__name.clone())) {
        (txt, Deref @ "", _) => {
            txt.clone()
        },
        (txt, i_unit, a_attr__name) => {
            let mut ret_0: ArcStr;
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeStr(txt.clone(), (a_attr__name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("=\"")).clone() }))?;
            ret_0 = (Util::escapeModelicaStringToXmlString((i_unit.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_96(mut in_txt: Tpl::Text, mut in_a_simVarAlias: SimCodeVar::AliasVariable, mut in_a_type: Tpl::Text, mut in_a_complexStartExpressions: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_stateDerVectorName: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_simVarCref: Arc<DAE::ComponentRef>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_complexStartExpressions: Tpl::Text;
    let mut out_a_stateDerVectorName: Tpl::Text;
    (out_txt, out_a_complexStartExpressions, out_a_stateDerVectorName) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_simVarAlias.clone(), in_a_type.clone(), in_a_complexStartExpressions.clone(), in_a_exp.clone(), in_a_stateDerVectorName.clone(), in_a_simCode.clone(), in_a_simVarCref.clone())) {
        (txt, SimCodeVar::AliasVariable::NOALIAS { .. }, a_type, a_complexStartExpressions, a_exp, a_stateDerVectorName, a_simCode, a_simVarCref) => {
            let mut l_expression: Tpl::Text;
            let mut l_crefStr: Tpl::Text;
            let mut l_extraFuncsDecl: Tpl::Text;
            let mut l_extraFuncs: Tpl::Text;
            let mut l_varDecls: Tpl::Text;
            let mut l_complexPreExpression: Tpl::Text;
            let mut a_complexStartExpressions = (*a_complexStartExpressions).clone();
            let mut a_stateDerVectorName = (*a_stateDerVectorName).clone();
            l_complexPreExpression = Tpl::emptyTxt.clone();
            l_varDecls = Tpl::emptyTxt.clone();
            l_extraFuncs = Tpl::emptyTxt.clone();
            l_extraFuncsDecl = Tpl::emptyTxt.clone();
            (l_crefStr, l_extraFuncs, l_extraFuncsDecl, _, l_varDecls, a_stateDerVectorName) = CodegenCppCommon::cref1(Tpl::emptyTxt.clone(), a_simVarCref.clone(), a_simCode.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), SimCodeFunction::contextOther().clone(), l_varDecls.clone(), a_stateDerVectorName.clone(), false)?;
            (l_expression, l_complexPreExpression, l_varDecls, l_extraFuncs, l_extraFuncsDecl, _, a_stateDerVectorName) = CodegenCppCommon::daeExp(Tpl::emptyTxt.clone(), a_exp.clone(), SimCodeFunction::contextOther().clone(), l_complexPreExpression.clone(), l_varDecls.clone(), a_simCode.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), a_stateDerVectorName.clone(), false)?;
            a_complexStartExpressions = Tpl::writeText(a_complexStartExpressions.clone(), l_varDecls.clone())?;
            a_complexStartExpressions = Tpl::writeText(a_complexStartExpressions.clone(), l_complexPreExpression.clone())?;
            a_complexStartExpressions = Tpl::writeTok(a_complexStartExpressions.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SystemDefaultImplementation::set")).clone() }))?;
            a_complexStartExpressions = Tpl::writeText(a_complexStartExpressions.clone(), a_type.clone())?;
            a_complexStartExpressions = Tpl::writeTok(a_complexStartExpressions.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("StartValue(")).clone() }))?;
            a_complexStartExpressions = Tpl::writeText(a_complexStartExpressions.clone(), l_crefStr.clone())?;
            a_complexStartExpressions = Tpl::writeTok(a_complexStartExpressions.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() }))?;
            a_complexStartExpressions = Tpl::writeText(a_complexStartExpressions.clone(), l_expression.clone())?;
            a_complexStartExpressions = Tpl::writeTok(a_complexStartExpressions.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            a_complexStartExpressions = Tpl::writeTok(a_complexStartExpressions.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
            (txt.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())
        },
        (txt, _, _, a_complexStartExpressions, _, a_stateDerVectorName, _, _) => {
            (txt.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_complexStartExpressions, out_a_stateDerVectorName))
}

fn fun_97(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_startString: Tpl::Text, mut in_a_type: Tpl::Text, mut in_a_complexStartExpressions: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_stateDerVectorName: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_simVarCref: Arc<DAE::ComponentRef>, mut in_a_simVarAlias: SimCodeVar::AliasVariable) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_complexStartExpressions: Tpl::Text;
    let mut out_a_stateDerVectorName: Tpl::Text;
    (out_txt, out_a_complexStartExpressions, out_a_stateDerVectorName) = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_startString.clone(), in_a_type.clone(), in_a_complexStartExpressions.clone(), in_a_exp.clone(), in_a_stateDerVectorName.clone(), in_a_simCode.clone(), in_a_simVarCref.clone(), in_a_simVarAlias.clone())) {
        (txt, Deref @ "", _, a_type, a_complexStartExpressions, a_exp, a_stateDerVectorName, a_simCode, a_simVarCref, a_simVarAlias) => {
            let mut l_unsued: Tpl::Text;
            let mut a_complexStartExpressions = (*a_complexStartExpressions).clone();
            let mut a_stateDerVectorName = (*a_stateDerVectorName).clone();
            (l_unsued, a_complexStartExpressions, a_stateDerVectorName) = fun_96(Tpl::emptyTxt.clone(), a_simVarAlias.clone(), a_type.clone(), a_complexStartExpressions.clone(), a_exp.clone(), a_stateDerVectorName.clone(), a_simCode.clone(), a_simVarCref.clone())?;
            (txt.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())
        },
        (txt, _, a_startString, _, a_complexStartExpressions, _, a_stateDerVectorName, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_startString.clone())?;
            (txt.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_complexStartExpressions, out_a_stateDerVectorName))
}

fn fun_98(mut in_txt: Tpl::Text, mut in_a_startValue: Option<Arc<DAE::Exp>>, mut in_a_simCode: SimCode::SimCode, mut in_a_simVarCref: Arc<DAE::ComponentRef>, mut in_a_simVarAlias: SimCodeVar::AliasVariable, mut in_a_type: Tpl::Text, mut in_a_complexStartExpressions: Tpl::Text, mut in_a_stateDerVectorName: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_complexStartExpressions: Tpl::Text;
    let mut out_a_stateDerVectorName: Tpl::Text;
    (out_txt, out_a_complexStartExpressions, out_a_stateDerVectorName) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_startValue.clone(), in_a_simCode.clone(), in_a_simVarCref.clone(), in_a_simVarAlias.clone(), in_a_type.clone(), in_a_complexStartExpressions.clone(), in_a_stateDerVectorName.clone())) {
        (txt, Some(i_exp), a_simCode, a_simVarCref, a_simVarAlias, a_type, a_complexStartExpressions, a_stateDerVectorName) => {
            let mut str_1: ArcStr;
            let mut l_startString: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_complexStartExpressions = (*a_complexStartExpressions).clone();
            let mut a_stateDerVectorName = (*a_stateDerVectorName).clone();
            l_startString = attributeString(Tpl::emptyTxt.clone(), i_exp.clone(), (literal!("start")).clone())?;
            str_1 = (Tpl::textString(l_startString.clone())?).clone();
            (txt, a_complexStartExpressions, a_stateDerVectorName) = fun_97(txt.clone(), (str_1.clone()).clone(), l_startString.clone(), a_type.clone(), a_complexStartExpressions.clone(), i_exp.clone(), a_stateDerVectorName.clone(), a_simCode.clone(), a_simVarCref.clone(), a_simVarAlias.clone())?;
            (txt.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())
        },
        (txt, None, _, _, _, _, a_complexStartExpressions, a_stateDerVectorName) => {
            (txt.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())
        },
        (txt, _, _, _, _, _, a_complexStartExpressions, a_stateDerVectorName) => {
            (txt.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_complexStartExpressions, out_a_stateDerVectorName))
}

pub fn ScalarVariableTypeStartAttribute(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_simVarCref: Arc<DAE::ComponentRef>, mut a_simVarAlias: SimCodeVar::AliasVariable, mut a_startValue: Option<Arc<DAE::Exp>>, mut a_type: Tpl::Text, mut a_complexStartExpressions: Tpl::Text, mut a_stateDerVectorName: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_type: Tpl::Text;
    let mut out_a_complexStartExpressions: Tpl::Text;
    let mut out_a_stateDerVectorName: Tpl::Text;
    (out_txt, out_a_complexStartExpressions, out_a_stateDerVectorName) = fun_98(txt.clone(), a_startValue.clone(), a_simCode.clone(), a_simVarCref.clone(), a_simVarAlias.clone(), a_type.clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())?;
    out_a_type = a_type.clone();
    Ok((out_txt, out_a_type, out_a_complexStartExpressions, out_a_stateDerVectorName))
}

fn lm_100(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_100 in &*items.clone() {
        let mut lstElt_100 = lstElt_100.clone();
        txt = (match lstElt_100.clone() {
        mut i_it => {
            txt = Tpl::writeStr(txt.clone(), (i_it.clone()).clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_101(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut a_indexForUndefinedReferences: ArcStr, mut a_varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>))) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_101 in &*items.clone() {
        let mut lstElt_101 = lstElt_101.clone();
        txt = (match lstElt_101.clone() {
        SimCodeVar::SimVar { name: ref i_v_name, .. } => {
            let mut ret_0: Arc<metamodelica::List<ArcStr>>;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Var type=\"double\" index=\"")).clone() }))?;
            ret_0 = SimCodeUtil::getVarIndexListByMapping(a_varToArrayIndexMapping.clone(), i_v_name.clone(), true, (a_indexForUndefinedReferences.clone()).clone())?;
            txt = lm_100(txt.clone(), ret_0.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" />")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => {
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_102(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_102 in &*items.clone() {
        let mut lstElt_102 = lstElt_102.clone();
        txt = (match lstElt_102.clone() {
        mut i_it => {
            txt = Tpl::writeStr(txt.clone(), (i_it.clone()).clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_103(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut a_indexForUndefinedReferences: ArcStr, mut a_varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>))) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_103 in &*items.clone() {
        let mut lstElt_103 = lstElt_103.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_103.clone()) {
        i_name => {
            let mut ret_0: Arc<metamodelica::List<ArcStr>>;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Var type=\"double\" index=\"")).clone() }))?;
            ret_0 = SimCodeUtil::getVarIndexListByMapping(a_varToArrayIndexMapping.clone(), i_name.clone(), true, (a_indexForUndefinedReferences.clone()).clone())?;
            txt = lm_102(txt.clone(), ret_0.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" />")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_104(mut in_txt: Tpl::Text, mut in_a_eqs: Arc<SimCode::SimEqSystem>, mut in_a_varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), mut in_a_indexForUndefinedReferences: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eqs.clone(), in_a_varToArrayIndexMapping.clone(), in_a_indexForUndefinedReferences.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: Deref @ SimCode::LinearSystem { index: i_ls_index, vars: i_ls_vars, .. }, .. }, a_varToArrayIndexMapping, a_indexForUndefinedReferences) => {
            let mut ret_0: i32;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Linear eqIdx=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_ls_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" sparse=\"true\" size=\"")).clone() }))?;
            ret_0 = (i_ls_vars.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\">\n")).clone(), (literal!("  <Vars>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_101(txt.clone(), i_ls_vars.clone(), (a_indexForUndefinedReferences.clone()).clone(), a_varToArrayIndexMapping.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </Vars>\n")).clone(), (literal!("</Linear>")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem: Deref @ SimCode::NonlinearSystem { index: i_nls_index, crefs: i_nls_crefs, .. }, .. }, a_varToArrayIndexMapping, a_indexForUndefinedReferences) => {
            let mut ret_1: i32;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<NonLinear eqIdx=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_nls_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" size=\"")).clone() }))?;
            ret_1 = (i_nls_crefs.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_1.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\">\n")).clone(), (literal!("  <Vars>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_103(txt.clone(), i_nls_crefs.clone(), (a_indexForUndefinedReferences.clone()).clone(), a_varToArrayIndexMapping.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </Vars>\n")).clone(), (literal!("  <NominalVars>\n")).clone(), (literal!("  <!-- Maybe Expressions here -->\n")).clone(), (literal!("  </NominalVars>\n")).clone(), (literal!("</NonLinear>")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn algLoopXML(mut txt: Tpl::Text, mut a_eqs: Arc<SimCode::SimEqSystem>, mut a_simCode: SimCode::SimCode, mut a_varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), mut a_indexForUndefinedReferences: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_104(txt.clone(), a_eqs.clone(), a_varToArrayIndexMapping.clone(), (a_indexForUndefinedReferences.clone()).clone())?;
    Ok(out_txt)
}

fn lm_106(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCode::JacobianMatrix>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_106 in &*items.clone() {
        let mut lstElt_106 = lstElt_106.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_106.clone()) {
        Deref @ SimCode::JacobianMatrix { columns: i_mat, seedVars: i_vars, matrixName: i_name, sparsity: i_sparsepattern, coloredCols: i_colorList, maxColorCols: i_maxColor, jacobianIndex: i_jacIndex, .. } => {
            txt = jacobianMatrixXML(txt.clone(), i_jacIndex.clone(), i_mat.clone(), i_vars.clone(), (i_name.clone()).clone(), i_sparsepattern.clone(), i_colorList.clone(), i_maxColor.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub fn jacobianMatricesXML(mut txt: Tpl::Text, mut a_JacobianMatrices: Arc<metamodelica::List<Arc<SimCode::JacobianMatrix>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_jacMats: Tpl::Text;
    l_jacMats = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    l_jacMats = lm_106(l_jacMats.clone(), a_JacobianMatrices.clone())?;
    l_jacMats = Tpl::popIter(l_jacMats.clone())?;
    out_txt = Tpl::writeText(txt.clone(), l_jacMats.clone())?;
    Ok(out_txt)
}

fn lm_108(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCode::JacobianColumn>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_108 in &*items.clone() {
        let mut lstElt_108 = lstElt_108.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_108.clone()) {
        Deref @ SimCode::JacobianColumn { numberOfResultVars: i_nRows, .. } => {
            txt = Tpl::writeStr(txt.clone(), (intString(i_nRows.clone())).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_109(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_i__index: i32, mut in_a_index: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_i__index.clone(), in_a_index.clone())) {
        (txt, Deref @ "1", _, a_index) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Entry indexX=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" indexY=\"0\" valueIndex=\"0\"/>")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_i__index, a_index) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Entry indexX=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" indexY=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_i__index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" valueIndex=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_i__index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"/>")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_110(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<i32>>, mut a_index: i32, mut a_indexColumn: Tpl::Text) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_110 in &*items.clone() {
        let mut lstElt_110 = lstElt_110.clone();
        txt = (match lstElt_110.clone() {
        mut i_i__index => {
            let mut str_0: ArcStr;
            str_0 = (Tpl::textString(a_indexColumn.clone())?).clone();
            txt = fun_109(txt.clone(), (str_0.clone()).clone(), i_i__index.clone(), a_index.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_111(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>, mut a_indexColumn: Tpl::Text) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_111 in &*items.clone() {
        let mut lstElt_111 = lstElt_111.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_111.clone()) {
        (i_index, i_indexes) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<Column>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 19 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_110(txt.clone(), i_indexes.clone(), i_index.clone(), a_indexColumn.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</Column>")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub fn jacobianMatrixXML(mut txt: Tpl::Text, mut a_indexJacobian: i32, mut a_jacobianColumn: Arc<metamodelica::List<Arc<SimCode::JacobianColumn>>>, mut a_seedVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut a_matrixName: ArcStr, mut a_sparsepattern: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>, mut a_colorList: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut a_maxColor: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_jacvals: Tpl::Text;
    let mut l_indexColumn: Tpl::Text;
    l_indexColumn = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    l_indexColumn = lm_108(l_indexColumn.clone(), a_jacobianColumn.clone())?;
    l_indexColumn = Tpl::popIter(l_indexColumn.clone())?;
    l_jacvals = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    l_jacvals = lm_111(l_jacvals.clone(), a_sparsepattern.clone(), l_indexColumn.clone())?;
    l_jacvals = Tpl::popIter(l_jacvals.clone())?;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Matrix name=\"")).clone() }))?;
    out_txt = Tpl::writeStr(out_txt.clone(), (a_matrixName.clone()).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\">\n")).clone(), (literal!("  <Column>\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_jacvals.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </Column>\n")).clone(), (literal!("</Matrix>")).clone()], lastHasNewLine: false }))?;
    Ok(out_txt)
}

