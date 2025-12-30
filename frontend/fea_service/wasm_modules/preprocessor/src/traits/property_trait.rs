use crate::structs::ExtendedProperty;
use crate::enums::CrossSection;


pub trait PropertyTrait
{
    fn get_ref_property(&self) -> &Option<ExtendedProperty>;
    fn get_mut_ref_property(&mut self) -> &mut Option<ExtendedProperty>;
    fn set_propery(&mut self, optional_property: Option<ExtendedProperty>)
    {
        *self.get_mut_ref_property() = optional_property;
    }


    fn is_property_name_same(&self, property_name: String) -> bool
    {
        if let Some(property) = &self.get_ref_property()
        {
            return property.is_name_same(property_name);
        }
        false
    }


    fn is_property_assigned(&self) -> bool
    {
        self.get_ref_property().is_some()
    }


    fn get_optional_cross_section(&self) -> Option<CrossSection>
    {
        if let Some(property) = self.get_ref_property().as_ref()
        {
            return Some(property.get_cross_section());
        }
        None
    }
}
