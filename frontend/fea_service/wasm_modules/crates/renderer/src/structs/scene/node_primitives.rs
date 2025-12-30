use crate::structs::Primitives;


pub struct NodePrimitives 
{
    pub basic_primitives: Primitives,
    pub optional_primitives_result_u_displacement: Option<Primitives>,
    pub optional_primitives_global_forces: Option<Primitives>,
    pub optional_primitives_global_moments: Option<Primitives>,
}


impl NodePrimitives
{
    pub fn init(basic_primitives: Primitives) -> Self
    {
        NodePrimitives 
        { 
            basic_primitives, 
            optional_primitives_result_u_displacement: None,
            optional_primitives_global_forces: None, 
            optional_primitives_global_moments: None,
        }
    }
}
