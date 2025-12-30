use serde_json::{Value, json};
use serde::Serialize;

use crate::props::Props;
use crate::enums::{Status, NotificationType, CrossSection, ParentKey};
use crate::traits::{StatusTrait, ServerNotificationTrait, ChildTrait};


#[derive(Debug, Clone, Serialize)]
pub struct Property
{
    material_name: String,
    cross_section: CrossSection,
    status: Status<String>,
}


impl Property
{
    pub fn create(material_name: String, cross_section: CrossSection) -> Self
    {
        Property { material_name, cross_section, status: Status::Active }
    }


    pub fn is_data_same(&self, material_name: String, cross_section: CrossSection) -> bool
    {
        self.material_name == material_name && self.cross_section == cross_section
    }


    pub fn get_data(&self) -> (String,  CrossSection)
    {
        (self.material_name.clone(), self.cross_section.clone())
    }


    pub fn set_data(&mut self, material_name: String, cross_section: CrossSection)
    {
        self.material_name = material_name;
        self.cross_section = cross_section;
    }


    pub fn is_plate_cross_section(&self) -> bool
    {
        self.cross_section.is_plate()
    }
}


impl ChildTrait for Property
{
    fn get_parent_keys(&self) -> Vec<ParentKey> 
    {
        vec![ParentKey::Material(self.material_name.clone()), ParentKey::CrossSection(self.cross_section.clone())]
    }
}


impl StatusTrait for Property
{
    type Key = String;
    fn get_mut_ref_status(&mut self) -> &mut Status<Self::Key> 
    {
        &mut self.status
    }
}


impl ServerNotificationTrait for Property
{
    type Key = String;
    fn get_event_detail(&self, notification_type: &NotificationType, key: Self::Key) -> Value 
    {
        match notification_type
        {
            NotificationType::Add(is_action_id_should_be_increased) | 
            NotificationType::Update(is_action_id_should_be_increased) =>
            {
                let (cross_section_name, cross_section_type) = match &self.cross_section 
                    {
                        CrossSection::Truss(name) => (name, "truss"),
                        CrossSection::Beam(name) => (name, "beam"),
                        CrossSection::Plate(name) => (name, "plate"),
                    };
                json!({ 
                    "properties_data": 
                    { 
                        "name": key,
                        "material_name": self.material_name, 
                        "cross_section_name": cross_section_name,
                        "cross_section_type": cross_section_type 
                    },
                    "is_action_id_should_be_increased": is_action_id_should_be_increased 
                })
            },
            NotificationType::Delete(is_action_id_should_be_increased) =>
            {
                json!({ 
                    "properties_data": { "name": key },
                    "is_action_id_should_be_increased": is_action_id_should_be_increased 
                })
            }
        }
    }


    fn get_event_name(&self, notification_type: &NotificationType, props: &Props) -> String 
    {
        match notification_type
        {
            NotificationType::Add(_) => props.add_properties_event_name.clone(),
            NotificationType::Update(_) => props.update_properties_event_name.clone(),
            NotificationType::Delete(_) => props.delete_properties_event_name.clone(),
        }
    }


    fn get_event_target(&self, props: &Props) -> String 
    {
        props.event_target.clone()
    }
}
