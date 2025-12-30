use crate::structs::QuadElementData;
use crate::traits::RotationMatrixTrait;


pub trait QuadElementDataTrait: RotationMatrixTrait
{
    fn get_ref_quad_element_data(&self) -> &QuadElementData;

    fn get_mut_ref_quad_element_data(&mut self) -> &mut QuadElementData;

    fn get_uid(&self) -> u32
    {
        self.get_ref_quad_element_data().uid
    }

    fn get_node_1_coordinates(&self) -> [f32; 3]
    {
        self.get_ref_quad_element_data().node_1_coordinates
    }

    fn get_node_2_coordinates(&self) -> [f32; 3]
    {
        self.get_ref_quad_element_data().node_2_coordinates
    }

    fn get_node_3_coordinates(&self) -> [f32; 3]
    {
        self.get_ref_quad_element_data().node_3_coordinates
    }

    fn get_node_4_coordinates(&self) -> [f32; 3]
    {
        self.get_ref_quad_element_data().node_4_coordinates
    }

    fn get_node_1_displacement(&self) -> [f32; 3]
    {
        self.get_ref_quad_element_data().node_1_displacement
    }

    fn get_node_2_displacement(&self) -> [f32; 3]
    {
        self.get_ref_quad_element_data().node_2_displacement
    }

    fn get_node_3_displacement(&self) -> [f32; 3]
    {
        self.get_ref_quad_element_data().node_3_displacement
    }

    fn get_node_4_displacement(&self) -> [f32; 3]
    {
        self.get_ref_quad_element_data().node_4_displacement
    }

    fn get_optional_node_1_u_result_coeff(&self) -> Option<[f32; 5]>
    {
        self.get_ref_quad_element_data().optional_node_1_u_result_coeff
    }

    fn get_optional_node_2_u_result_coeff(&self) -> Option<[f32; 5]>
    {
        self.get_ref_quad_element_data().optional_node_2_u_result_coeff
    }

    fn get_optional_node_3_u_result_coeff(&self) -> Option<[f32; 5]>
    {
        self.get_ref_quad_element_data().optional_node_3_u_result_coeff
    }

    fn get_optional_node_4_u_result_coeff(&self) -> Option<[f32; 5]>
    {
        self.get_ref_quad_element_data().optional_node_4_u_result_coeff
    }
}
