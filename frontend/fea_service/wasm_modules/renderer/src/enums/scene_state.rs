use crate::enums::ResultPlot;
use crate::structs::{ExtremeGlobalAnalysisValues, ExtremeElementsAnalysisValues};


#[derive(Clone, Copy)]
pub enum SceneState
{
    Preprocessor,
    Postprocessor((
        Option<ResultPlot>, ExtremeGlobalAnalysisValues, ExtremeGlobalAnalysisValues, ExtremeElementsAnalysisValues,
    )),
}
