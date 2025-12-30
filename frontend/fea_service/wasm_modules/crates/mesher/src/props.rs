use crate::types::FEFloat;


pub struct Props
{
    pub abs_tol: FEFloat,
    pub rel_tol: FEFloat,
    pub max_point_number: u32,
    pub max_line_number: u32,
    pub max_surface_number: u32,
    pub truss_elements_group_number: u32,
    pub beam_elements_group_number: u32,
    pub plate_elements_group_number: u32,
}


impl Props
{
    pub(super) fn create(
        abs_tol: FEFloat,
        rel_tol: FEFloat,
        max_point_number: u32,
        max_line_number: u32,
        max_surface_number: u32,
        truss_elements_group_number: u32,
        beam_elements_group_number: u32,
        plate_elements_group_number: u32,
    ) 
        -> Self
    {
        Props 
        { 
            abs_tol,
            rel_tol,
            max_point_number,
            max_line_number,
            max_surface_number,
            truss_elements_group_number,
            beam_elements_group_number,
            plate_elements_group_number,
        }
    }
}
