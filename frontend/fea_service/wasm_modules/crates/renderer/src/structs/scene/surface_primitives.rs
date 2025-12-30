use crate::structs::Primitives;


pub struct SurfacePrimitives 
{
    pub optional_primitives_for_selection: Option<Primitives>,
    pub optional_primitives_normal: Option<Primitives>,
    pub optional_primitives_edges_1_3: Option<Primitives>,
    pub optional_primitives_edges_2_4: Option<Primitives>,
    pub optional_primitives_mesh_seed_edges_1_3: Option<Primitives>,
    pub optional_primitives_mesh_seed_edges_2_4: Option<Primitives>,
}


impl SurfacePrimitives
{
    pub fn init() -> Self
    {
        SurfacePrimitives 
        { 
            optional_primitives_for_selection: None, 
            optional_primitives_normal: None,
            optional_primitives_edges_1_3: None, 
            optional_primitives_edges_2_4: None, 
            optional_primitives_mesh_seed_edges_1_3: None, 
            optional_primitives_mesh_seed_edges_2_4: None,
        }
    }
}
