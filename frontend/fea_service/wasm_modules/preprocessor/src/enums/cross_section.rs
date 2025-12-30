use wasm_bindgen::prelude::JsValue;
use serde::Serialize;


#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum CrossSection
{
    Truss(String),
    Beam(String),
    Plate(String),
}


impl CrossSection
{
    pub fn create(cross_section_name: &str, cross_section_type: &str) -> Result<Self, JsValue>
    {
        if cross_section_type.replace('"', "") == "truss"
        {
            Ok(CrossSection::Truss(cross_section_name.to_string()))
        }
        else if cross_section_type.replace('"', "") == "beam"
        {
            Ok(CrossSection::Beam(cross_section_name.to_string()))
        }
        else if cross_section_type.replace('"', "") == "plate"
        {
            Ok(CrossSection::Plate(cross_section_name.to_string()))
        }
        else
        {
            Err(JsValue::from("You input unknown cross section type!"))
        }
    }


    pub fn create_truss(name: &str) -> Self
    {
        CrossSection::Truss(name.to_string())
    }


    pub fn create_beam(name: &str) -> Self
    {
        CrossSection::Beam(name.to_string())
    }


    pub fn create_plate(name: &str) -> Self
    {
        CrossSection::Plate(name.to_string())
    }


    pub fn is_beam(&self) -> bool
    {
        match self 
        {
            CrossSection::Beam(_) => true,
            _ => false,   
        }
    }


    pub fn is_truss(&self) -> bool
    {
        match self 
        {
            CrossSection::Truss(_) => true,
            _ => false,   
        }
    }


    pub fn is_plate(&self) -> bool
    {
        match self 
        {
            CrossSection::Plate(_) => true,
            _ => false,   
        }
    }
}
