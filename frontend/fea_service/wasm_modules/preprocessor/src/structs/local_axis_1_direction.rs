use serde_json::{Value, json};
use serde::Serialize;
use wasm_bindgen::JsValue;

use crate::types::FEFloat;
use crate::enums::NotificationType;
use crate::Props;
use crate::traits::ServerNotificationTrait;


#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct LocalAxis1Direction(FEFloat, FEFloat, FEFloat);


impl LocalAxis1Direction
{
    pub fn create(components: &[FEFloat], props: &Props) -> Result<Self, JsValue>
    {
        if components.len() != 3
        {
            let error_message = &format!("{} Incorrect number of components in {components:?}!",
                props.local_axis_1_direction_input_info_message_header);
            return Err(JsValue::from(error_message));
        }

        if components[0] == 0 as FEFloat && components[1] == 0 as FEFloat && components[2] == 0 as FEFloat
        {
            let error_message = &format!("{} All components in {components:?} are equal to zero!",
                props.local_axis_1_direction_input_info_message_header);
            return Err(JsValue::from(error_message));
        }

        Ok(LocalAxis1Direction(components[0], components[1], components[2]))
    }


    pub fn get_components(&self) -> [FEFloat; 3]
    {
        [self.0, self.1, self.2]
    }    
}


impl ServerNotificationTrait for LocalAxis1Direction
{
    type Key = u32;
    fn get_event_detail(&self, notification_type: &NotificationType, _key: Self::Key) -> Value 
    {
        match notification_type
        {
            NotificationType::Add(is_action_id_should_be_increased) => 
            {
                json!({ "local_axis_1_direction_data":
                {
                    "local_axis_1_direction": self,
                },
                "is_action_id_should_be_increased": is_action_id_should_be_increased })
            },
            NotificationType::Update(_is_action_id_should_be_increased) => unreachable!(),
            NotificationType::Delete(is_action_id_should_be_increased) =>
            {
                json!({ "local_axis_1_direction_data":
                {
                    "local_axis_1_direction": self,
                },
                "is_action_id_should_be_increased": is_action_id_should_be_increased })
            }
        }
    }


    fn get_event_name(&self, notification_type: &NotificationType, props: &Props) -> String 
    {
        match notification_type
        {
            NotificationType::Add(_) => props.add_beam_local_axis_1_direction_event_name.clone(),
            NotificationType::Update(_) => unreachable!(),
            NotificationType::Delete(_) => props.delete_beam_local_axis_1_direction_event_name.clone(),
        }
    }


    fn get_event_target(&self, props: &Props) -> String 
    {
        props.event_target.clone()
    }
}
