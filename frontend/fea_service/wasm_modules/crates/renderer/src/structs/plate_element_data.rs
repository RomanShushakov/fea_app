use extended_matrix::SquareMatrix;

use crate::structs::{ExtremeElementsAnalysisValues, QuadElementData};
use crate::props::Props;
use crate::traits::{RotationMatrixTrait, QuadElementDataTrait};


#[derive(Debug)]
pub struct PlateElementData
{
    optional_rotation_matrix: Option<SquareMatrix<f32>>,
    quad_element_data: QuadElementData,
    optional_mem_force_r_coeff: Option<[f32; 5]>,
    optional_mem_force_s_coeff: Option<[f32; 5]>,
    optional_mem_force_r_s_coeff: Option<[f32; 5]>,
    optional_bend_moment_r_coeff: Option<[f32; 5]>,
    optional_bend_moment_s_coeff: Option<[f32; 5]>,
    optional_bend_moment_r_s_coeff: Option<[f32; 5]>,
    optional_shear_force_r_t_coeff: Option<[f32; 5]>,
    optional_shear_force_s_t_coeff: Option<[f32; 5]>,
}


impl RotationMatrixTrait for PlateElementData
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


impl QuadElementDataTrait for PlateElementData
{
    fn get_ref_quad_element_data(&self) -> &QuadElementData 
    {
        &self.quad_element_data
    }


    fn get_mut_ref_quad_element_data(&mut self) -> &mut QuadElementData 
    {
        &mut self.quad_element_data
    }
}


impl PlateElementData
{
    pub fn init(quad_element_data: QuadElementData) 
        -> Self
    {
        PlateElementData 
        { 
            optional_rotation_matrix: None,
            quad_element_data,
            optional_mem_force_r_coeff: None,
            optional_mem_force_s_coeff: None,
            optional_mem_force_r_s_coeff: None,
            optional_bend_moment_r_coeff: None,
            optional_bend_moment_s_coeff: None,
            optional_bend_moment_r_s_coeff: None,
            optional_shear_force_r_t_coeff: None,
            optional_shear_force_s_t_coeff: None,
        }
    }


    pub fn update(
        &mut self,
        rotation_matrix_elements: [f32; 9],
        mem_force_r: f32,
        mem_force_s: f32,
        mem_force_r_s: f32,
        bend_moment_r: f32,
        bend_moment_s: f32,
        bend_moment_r_s: f32,
        shear_force_r_t: f32,
        shear_force_s_t: f32,
        extreme_elements_loads_data: &ExtremeElementsAnalysisValues,
        props: &Props,
    )
    {
        self.set_optional_rotation_matrix(rotation_matrix_elements);
        self.optional_mem_force_r_coeff = extreme_elements_loads_data.get_mem_f_r_coeff(mem_force_r, props);
        self.optional_mem_force_s_coeff = extreme_elements_loads_data.get_mem_f_s_coeff(mem_force_s, props);
        self.optional_mem_force_r_s_coeff = extreme_elements_loads_data.get_mem_f_r_s_coeff(
            mem_force_r_s, props,
        );
        self.optional_bend_moment_r_coeff = extreme_elements_loads_data.get_bend_m_r_coeff(
            bend_moment_r, props,
        );
        self.optional_bend_moment_s_coeff = extreme_elements_loads_data.get_bend_m_s_coeff(
            bend_moment_s, props,
        );
        self.optional_bend_moment_r_s_coeff = extreme_elements_loads_data.get_bend_m_r_s_coeff(
            bend_moment_r_s, props,
        );
        self.optional_shear_force_r_t_coeff = extreme_elements_loads_data.get_shear_f_r_t_coeff(
            shear_force_r_t, props,
        );
        self.optional_shear_force_s_t_coeff = extreme_elements_loads_data.get_shear_f_s_t_coeff(
            shear_force_s_t, props,
        );
    }


    pub fn get_optional_mem_force_r_coeff(&self) -> Option<[f32; 5]>
    {
        self.optional_mem_force_r_coeff
    }


    pub fn get_optional_mem_force_s_coeff(&self) -> Option<[f32; 5]>
    {
        self.optional_mem_force_s_coeff
    }


    pub fn get_optional_mem_force_r_s_coeff(&self) -> Option<[f32; 5]>
    {
        self.optional_mem_force_r_s_coeff
    }


    pub fn get_optional_bend_moment_r_coeff(&self) -> Option<[f32; 5]>
    {
        self.optional_bend_moment_r_coeff
    }


    pub fn get_optional_bend_moment_s_coeff(&self) -> Option<[f32; 5]>
    {
        self.optional_bend_moment_s_coeff
    }


    pub fn get_optional_bend_moment_r_s_coeff(&self) -> Option<[f32; 5]>
    {
        self.optional_bend_moment_r_s_coeff
    }


    pub fn get_optional_shear_force_r_t_coeff(&self) -> Option<[f32; 5]>
    {
        self.optional_shear_force_r_t_coeff
    }


    pub fn get_optional_shear_force_s_t_coeff(&self) -> Option<[f32; 5]>
    {
        self.optional_shear_force_s_t_coeff
    }
}
