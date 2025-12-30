#[derive(Debug)]
pub struct QuadElementData
{
    pub uid: u32,
    pub node_1_coordinates: [f32; 3],
    pub node_2_coordinates: [f32; 3],
    pub node_3_coordinates: [f32; 3],
    pub node_4_coordinates: [f32; 3],
    pub node_1_displacement: [f32; 3],
    pub node_2_displacement: [f32; 3],
    pub node_3_displacement: [f32; 3],
    pub node_4_displacement: [f32; 3],
    pub optional_node_1_u_result_coeff: Option<[f32; 5]>,
    pub optional_node_2_u_result_coeff: Option<[f32; 5]>,
    pub optional_node_3_u_result_coeff: Option<[f32; 5]>,
    pub optional_node_4_u_result_coeff: Option<[f32; 5]>,
}


impl QuadElementData
{
    pub fn create(
        uid: u32,
        node_1_coordinates: [f32; 3],
        node_2_coordinates: [f32; 3],
        node_3_coordinates: [f32; 3],
        node_4_coordinates: [f32; 3],
        node_1_displacement: [f32; 3],
        node_2_displacement: [f32; 3],
        node_3_displacement: [f32; 3],
        node_4_displacement: [f32; 3],
        optional_node_1_u_result_coeff: Option<[f32; 5]>,
        optional_node_2_u_result_coeff: Option<[f32; 5]>,
        optional_node_3_u_result_coeff: Option<[f32; 5]>,
        optional_node_4_u_result_coeff: Option<[f32; 5]>,
    ) 
        -> Self
    {
        QuadElementData 
        { 
            uid,
            node_1_coordinates,
            node_2_coordinates,
            node_3_coordinates,
            node_4_coordinates,
            node_1_displacement,
            node_2_displacement,
            node_3_displacement,
            node_4_displacement,
            optional_node_1_u_result_coeff,
            optional_node_2_u_result_coeff,
            optional_node_3_u_result_coeff,
            optional_node_4_u_result_coeff,
        }
    }
}
