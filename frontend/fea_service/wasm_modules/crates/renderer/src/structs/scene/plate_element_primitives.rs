use crate::structs::Primitives;


pub struct PlateElementPrimitives 
{
    pub basic_primitives: Primitives,
    pub optional_primitives_result_u_displacement: Option<Primitives>,
    pub optional_primitives_local_axes: Option<Primitives>,
    pub optional_primitives_mem_force_r: Option<Primitives>,
    pub optional_primitives_mem_force_s: Option<Primitives>,
    pub optional_primitives_mem_force_r_s: Option<Primitives>,
    pub optional_primitives_bend_moment_r: Option<Primitives>,
    pub optional_primitives_bend_moment_s: Option<Primitives>,
    pub optional_primitives_bend_moment_r_s: Option<Primitives>,
    pub optional_primitives_shear_force_r_t: Option<Primitives>,
    pub optional_primitives_shear_force_s_t: Option<Primitives>,
}


impl PlateElementPrimitives
{
    pub fn init(basic_primitives: Primitives) -> Self
    {
        PlateElementPrimitives 
        { 
            basic_primitives,
            optional_primitives_result_u_displacement: None,
            optional_primitives_local_axes: None,
            optional_primitives_mem_force_r: None,
            optional_primitives_mem_force_s: None,
            optional_primitives_mem_force_r_s: None,
            optional_primitives_bend_moment_r: None,
            optional_primitives_bend_moment_s: None,
            optional_primitives_bend_moment_r_s: None,
            optional_primitives_shear_force_r_t: None,
            optional_primitives_shear_force_s_t: None,
        }
    }
}
