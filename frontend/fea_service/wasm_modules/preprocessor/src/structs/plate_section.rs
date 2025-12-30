use wasm_bindgen::JsValue;
use serde_json::{Value, json};

use crate::props::Props;
use crate::traits::{StatusTrait, ServerNotificationTrait};
use crate::types::FEFloat;
use crate::enums::{Status, NotificationType};
use crate::functions::check_value_positive;


#[derive(Debug, Clone)]
pub struct PlateSection
{
    thickness: FEFloat,
    shear_factor: FEFloat,
    status: Status<String>,
}


impl PlateSection
{
    pub fn create(thickness: FEFloat, shear_factor: FEFloat) -> Result<Self, JsValue>
    {
        check_value_positive("Thickness", thickness)?;
        check_value_positive("Shear factor", shear_factor)?;

        Ok(PlateSection { thickness, shear_factor, status: Status::Active })
    }


    pub fn is_data_same(&self, thickness: FEFloat, shear_factor: FEFloat) -> bool
    {
        self.thickness == thickness && self.shear_factor == shear_factor
    }


    pub fn get_data(&self) -> [FEFloat; 2]
    {
        [self.thickness, self.shear_factor]
    }


    pub fn set_data(&mut self, thickness: FEFloat, shear_factor: FEFloat) -> Result<(), JsValue>
    {
        check_value_positive("Thickness", thickness)?;
        check_value_positive("Shear factor", shear_factor)?;

        self.thickness = thickness;
        self.shear_factor = shear_factor;

        Ok(())
    }
}



impl StatusTrait for PlateSection
{
    type Key = String;
    fn get_mut_ref_status(&mut self) -> &mut Status<Self::Key> 
    {
       &mut self.status 
    }
}


impl ServerNotificationTrait for PlateSection
{
    type Key = String;
    fn get_event_detail(&self, notification_type: &NotificationType, key: Self::Key) -> Value 
    {
        match notification_type
        {
            NotificationType::Add(is_action_id_should_be_increased) | 
            NotificationType::Update(is_action_id_should_be_increased) =>
            {
                json!({ 
                    "plate_section_data": 
                    { 
                        "name": key, 
                        "thickness": self.thickness, 
                        "shear_factor": self.shear_factor 
                    },
                    "is_action_id_should_be_increased": is_action_id_should_be_increased 
                })
            },
            NotificationType::Delete(is_action_id_should_be_increased) =>
            {
                json!({ 
                    "plate_section_data": { "name": key },
                    "is_action_id_should_be_increased": is_action_id_should_be_increased 
                })
            }
        }
    }


    fn get_event_name(&self, notification_type: &NotificationType, props: &Props) -> String 
    {
        match notification_type
        {
            NotificationType::Add(_) => props.add_plate_section_event_name.clone(),
            NotificationType::Update(_) => props.update_plate_section_event_name.clone(),
            NotificationType::Delete(_) => props.delete_plate_section_event_name.clone(),
        }
    }


    fn get_event_target(&self, props: &Props) -> String 
    {
        props.event_target.clone()
    }
}
