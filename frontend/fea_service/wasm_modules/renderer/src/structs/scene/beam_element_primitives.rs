use crate::structs::Primitives;


pub struct BeamElementPrimitives 
{
    pub basic_primitives: Primitives,
    pub optional_primitives_result_u_displacement: Option<Primitives>,
    pub optional_primitives_local_axes: Option<Primitives>,
    pub optional_primitives_force_r: Option<Primitives>,
    pub optional_primitives_force_s: Option<Primitives>,
    pub optional_primitives_force_t: Option<Primitives>,
    pub optional_primitives_moment_r: Option<Primitives>,
    pub optional_primitives_moment_s: Option<Primitives>,
    pub optional_primitives_moment_t: Option<Primitives>,
}


impl BeamElementPrimitives
{
    pub fn init(basic_primitives: Primitives) -> Self
    {
        BeamElementPrimitives 
        { 
            basic_primitives,
            optional_primitives_result_u_displacement: None,
            optional_primitives_local_axes: None,
            optional_primitives_force_r: None,
            optional_primitives_force_s:None,
            optional_primitives_force_t: None,
            optional_primitives_moment_r: None,
            optional_primitives_moment_s: None,
            optional_primitives_moment_t: None,
        }
    }
}
