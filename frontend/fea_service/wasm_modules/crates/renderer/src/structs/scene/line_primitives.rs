use crate::structs::Primitives;


pub struct LinePrimitives 
{
    pub basic_primitives: Primitives,
    pub optional_primitives_local_axis_1_direction: Option<Primitives>,
    pub optional_primitives_mesh_seed: Option<Primitives>,
}


impl LinePrimitives
{
    pub fn init(basic_primitives: Primitives) -> Self
    {
        LinePrimitives 
        { 
            basic_primitives, optional_primitives_local_axis_1_direction: None, optional_primitives_mesh_seed: None,
        }
    }
}
