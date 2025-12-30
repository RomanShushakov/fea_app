use serde::Serialize;

use crate::structs::Property;
use crate::enums::CrossSection;


#[derive(Debug, Clone, Serialize)]
pub struct ExtendedProperty
{
    name: String,
    property: Property,
}


impl ExtendedProperty
{
    pub fn create(name: String, property: Property) -> Self
    {
        ExtendedProperty { name, property }
    }


    pub fn get_name(&self) -> String
    {
        self.name.clone()
    }


    pub fn is_name_same(&self, name: String) -> bool
    {
        self.name == name
    }


    pub fn get_cross_section(&self) -> CrossSection
    {
        self.property.get_data().1
    }
}
