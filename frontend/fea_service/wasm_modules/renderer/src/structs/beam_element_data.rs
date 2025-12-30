use extended_matrix::SquareMatrix;

use crate::structs::{ExtremeElementsAnalysisValues, LineElementData};
use crate::props::Props;
use crate::traits::{RotationMatrixTrait, LineElementDataTrait};


#[derive(Debug)]
pub struct BeamElementData
{
    optional_rotation_matrix: Option<SquareMatrix<f32>>,
    line_element_data: LineElementData,
    optional_force_r_coeff: Option<[f32; 5]>,
    optional_force_s_coeff: Option<[f32; 5]>,
    optional_force_t_coeff: Option<[f32; 5]>,
    optional_moment_r_coeff: Option<[f32; 5]>,
    optional_moment_s_node_1_coeff: Option<[f32; 5]>,
    optional_moment_s_coeff: Option<[f32; 5]>,
    optional_moment_s_node_2_coeff: Option<[f32; 5]>,
    optional_moment_t_node_1_coeff: Option<[f32; 5]>,
    optional_moment_t_coeff: Option<[f32; 5]>,
    optional_moment_t_node_2_coeff: Option<[f32; 5]>,
}


impl RotationMatrixTrait for BeamElementData
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


impl LineElementDataTrait for BeamElementData
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


impl BeamElementData
{
    pub fn init(line_element_data: LineElementData) 
        -> Self
    {
        BeamElementData 
        { 
            optional_rotation_matrix: None,
            line_element_data,
            optional_force_r_coeff: None,
            optional_force_s_coeff: None,
            optional_force_t_coeff: None,
            optional_moment_r_coeff: None,
            optional_moment_s_node_1_coeff: None,
            optional_moment_s_coeff: None,
            optional_moment_s_node_2_coeff: None,
            optional_moment_t_node_1_coeff: None,
            optional_moment_t_coeff: None,
            optional_moment_t_node_2_coeff: None,
        }
    }


    pub fn update(
        &mut self,
        rotation_matrix_elements: [f32; 9],
        force_r: f32,
        force_s: f32,
        force_t: f32,
        moment_r: f32,
        moment_s_node_1: f32,
        moment_s: f32,
        moment_s_node_2: f32,
        moment_t_node_1: f32,
        moment_t: f32,
        moment_t_node_2: f32,
        extreme_elements_loads_data: &ExtremeElementsAnalysisValues,
        props: &Props,
    )
    {
        self.set_optional_rotation_matrix(rotation_matrix_elements);
        self.optional_force_r_coeff = extreme_elements_loads_data.get_f_r_coeff(force_r, props);
        self.optional_force_s_coeff = extreme_elements_loads_data.get_f_s_coeff(force_s, props);
        self.optional_force_t_coeff = extreme_elements_loads_data.get_f_t_coeff(force_t, props);
        self.optional_moment_r_coeff = extreme_elements_loads_data.get_m_r_coeff(moment_r, props);
        self.optional_moment_s_node_1_coeff = extreme_elements_loads_data.get_m_s_coeff(moment_s_node_1, props);
        self.optional_moment_s_coeff = extreme_elements_loads_data.get_m_s_coeff(moment_s, props);
        self.optional_moment_s_node_2_coeff = extreme_elements_loads_data.get_m_s_coeff(moment_s_node_2, props);
        self.optional_moment_t_node_1_coeff = extreme_elements_loads_data.get_m_t_coeff(moment_t_node_1, props);
        self.optional_moment_t_coeff = extreme_elements_loads_data.get_m_t_coeff(moment_t, props);
        self.optional_moment_t_node_2_coeff = extreme_elements_loads_data.get_m_t_coeff(moment_t_node_2, props);
    }


    pub fn get_optional_force_r_coeff(&self) -> Option<[f32; 5]>
    {
        self.optional_force_r_coeff
    }


    pub fn get_optional_force_s_coeff(&self) -> Option<[f32; 5]>
    {
        self.optional_force_s_coeff
    }


    pub fn get_optional_force_t_coeff(&self) -> Option<[f32; 5]>
    {
        self.optional_force_t_coeff
    }


    pub fn get_optional_moment_r_coeff(&self) -> Option<[f32; 5]>
    {
        self.optional_moment_r_coeff
    }


    pub fn get_optional_moment_s_node_1_coeff(&self) -> Option<[f32; 5]>
    {
        self.optional_moment_s_node_1_coeff
    }


    pub fn get_optional_moment_s_coeff(&self) -> Option<[f32; 5]>
    {
        self.optional_moment_s_coeff
    }


    pub fn get_optional_moment_s_node_2_coeff(&self) -> Option<[f32; 5]>
    {
        self.optional_moment_s_node_2_coeff
    }


    pub fn get_optional_moment_t_node_1_coeff(&self) -> Option<[f32; 5]>
    {
        self.optional_moment_t_node_1_coeff
    }


    pub fn get_optional_moment_t_coeff(&self) -> Option<[f32; 5]>
    {
        self.optional_moment_t_coeff
    }


    pub fn get_optional_moment_t_node_2_coeff(&self) -> Option<[f32; 5]>
    {
        self.optional_moment_t_node_2_coeff
    }
}
