#[derive(Debug)]
pub struct LineElementData
{
    pub uid: u32,
    pub node_1_coordinates: [f32; 3],
    pub node_2_coordinates: [f32; 3],
    pub node_1_displacement: [f32; 3],
    pub node_2_displacement: [f32; 3],
    pub optional_node_1_u_result_coeff: Option<[f32; 5]>,
    pub optional_node_2_u_result_coeff: Option<[f32; 5]>,
}


impl LineElementData
{
    pub fn create(
        uid: u32,
        node_1_coordinates: [f32; 3],
        node_2_coordinates: [f32; 3],
        node_1_displacement: [f32; 3],
        node_2_displacement: [f32; 3],
        optional_node_1_u_result_coeff: Option<[f32; 5]>,
        optional_node_2_u_result_coeff: Option<[f32; 5]>,
    ) 
        -> Self
    {
        LineElementData 
        { 
            uid,
            node_1_coordinates,
            node_2_coordinates,
            node_1_displacement,
            node_2_displacement,
            optional_node_1_u_result_coeff,
            optional_node_2_u_result_coeff,
        }
    }
}
