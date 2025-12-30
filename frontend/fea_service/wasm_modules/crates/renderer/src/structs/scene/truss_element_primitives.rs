use crate::structs::Primitives;


pub struct TrussElementPrimitives 
{
    pub basic_primitives: Primitives,
    pub optional_primitives_result_u_displacement: Option<Primitives>,
    pub optional_primitives_local_axes: Option<Primitives>,
    pub optional_primitives_force_r: Option<Primitives>,
}


impl TrussElementPrimitives
{
    pub fn init(basic_primitives: Primitives) -> Self
    {
        TrussElementPrimitives 
        { 
            basic_primitives,
            optional_primitives_result_u_displacement: None,
            optional_primitives_local_axes: None,
            optional_primitives_force_r: None,
        }
    }
}
