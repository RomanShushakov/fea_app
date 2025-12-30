use wasm_bindgen::JsValue;
use serde_json::{Value, json};

use crate::props::Props;
use crate::traits::{StatusTrait, ServerNotificationTrait};
use crate::types::FEFloat;
use crate::enums::{Status, NotificationType};
use crate::functions::{check_value_positive, check_optional_value_positive};


#[derive(Debug, Clone)]
pub struct TrussSection
{
    area: FEFloat,
    optional_area2: Option<FEFloat>,
    status: Status<String>,
}


impl TrussSection
{
    pub fn create(area: FEFloat, optional_area2: Option<FEFloat>) -> Result<Self, JsValue>
    {
        check_value_positive("Area", area)?;
        check_optional_value_positive("Area 2", optional_area2)?;

        Ok(TrussSection { area, optional_area2, status: Status::Active })
    }


    pub fn is_data_same(&self, area: FEFloat, optional_area2: Option<FEFloat>) -> bool
    {
        self.area == area && self.optional_area2 == optional_area2
    }


    pub fn get_data(&self) -> (FEFloat, Option<FEFloat>)
    {
        (self.area, self.optional_area2)
    }


    pub fn set_data(&mut self, area: FEFloat, optional_area2: Option<FEFloat>) -> Result<(), JsValue>
    {
        check_value_positive("Area", area)?;
        check_optional_value_positive("Area 2", optional_area2)?;

        self.area = area;
        self.optional_area2 = optional_area2;

        Ok(())
    }
}



impl StatusTrait for TrussSection
{
    type Key = String;
    fn get_mut_ref_status(&mut self) -> &mut Status<Self::Key> 
    {
       &mut self.status 
    }
}


impl ServerNotificationTrait for TrussSection
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
                    "truss_section_data": 
                    { 
                        "name": key, "area": self.area, "area2": self.optional_area2 
                    },
                    "is_action_id_should_be_increased": is_action_id_should_be_increased 
                })
            },
            NotificationType::Delete(is_action_id_should_be_increased) =>
            {
                json!({ 
                    "truss_section_data": { "name": key },
                    "is_action_id_should_be_increased": is_action_id_should_be_increased 
                })
            }
        }
    }


    fn get_event_name(&self, notification_type: &NotificationType, props: &Props) -> String 
    {
        match notification_type
        {
            NotificationType::Add(_) => props.add_truss_section_event_name.clone(),
            NotificationType::Update(_) => props.update_truss_section_event_name.clone(),
            NotificationType::Delete(_) => props.delete_truss_section_event_name.clone(),
        }
    }


    fn get_event_target(&self, props: &Props) -> String 
    {
        props.event_target.clone()
    }
}
