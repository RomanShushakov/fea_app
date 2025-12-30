use crate::structs::Denotation;


pub struct DenotationsData
{
    point_objects_denotations: Vec<Denotation>,
    line_objects_denotations: Vec<Denotation>,
    surface_objects_denotations: Vec<Denotation>,
}


impl DenotationsData
{
    pub fn create() -> Self
    {
        DenotationsData 
        { 
            point_objects_denotations: Vec::new(), 
            line_objects_denotations: Vec::new(), 
            surface_objects_denotations: Vec::new() 
        }
    }


    pub fn add_point_object_denotation(&mut self, denotation: Denotation)
    {
        self.point_objects_denotations.push(denotation);
    }


    pub fn get_ref_point_objects_denotations(&self) -> &Vec<Denotation>
    {
        &self.point_objects_denotations
    }


    pub fn add_line_object_denotation(&mut self, denotation: Denotation)
    {
        self.line_objects_denotations.push(denotation);
    }


    pub fn get_ref_line_objects_denotations(&self) -> &Vec<Denotation>
    {
        &self.line_objects_denotations
    }


    pub fn add_surface_object_denotation(&mut self, denotation: Denotation)
    {
        self.surface_objects_denotations.push(denotation);
    }


    pub fn get_ref_surface_objects_denotations(&self) -> &Vec<Denotation>
    {
        &self.surface_objects_denotations
    }
}
