use extended_matrix::SquareMatrix;

use crate::structs::{ExtremeElementsAnalysisValues, LineElementData};
use crate::props::Props;
use crate::traits::{RotationMatrixTrait, LineElementDataTrait};


#[derive(Debug)]
pub struct TrussElementData
{
    optional_rotation_matrix: Option<SquareMatrix<f32>>,
    line_element_data: LineElementData,
    optional_force_r_coeff: Option<[f32; 5]>,
}


impl RotationMatrixTrait for TrussElementData
{
    fn get_ref_optional_rotation_matrix(&self) -> &Option<SquareMatrix<f32>>
    {
        &self.optional_rotation_matrix
    }

    fn get_mut_ref_optional_rotation_matrix(&mut self) -> &mut Option<SquareMatrix<f32>>
    {
        &mut self.optional_rotation_matrix
    }
}


impl LineElementDataTrait for TrussElementData
{
    fn get_ref_line_element_data(&self) -> &LineElementData 
    {
        &self.line_element_data
    }


    fn get_mut_ref_line_element_data(&mut self) -> &mut LineElementData 
    {
        &mut self.line_element_data
    }
}


impl TrussElementData
{
    pub fn init(line_element_data: LineElementData) 
        -> Self
    {
        TrussElementData 
        { 
            optional_rotation_matrix: None,
            line_element_data,
            optional_force_r_coeff: None,
        }
    }


    pub fn update(
        &mut self,
        rotation_matrix_elements: [f32; 9],
        force_r: f32,
        extreme_elements_loads_data: &ExtremeElementsAnalysisValues,
        props: &Props,
    )
    {
        self.set_optional_rotation_matrix(rotation_matrix_elements);
        self.optional_force_r_coeff = extreme_elements_loads_data.get_f_r_coeff(force_r, props);
    }


    pub fn get_optional_force_r_coeff(&self) -> Option<[f32; 5]>
    {
        self.optional_force_r_coeff
    }
}
