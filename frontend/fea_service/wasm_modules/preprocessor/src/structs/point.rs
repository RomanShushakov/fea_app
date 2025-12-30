use serde_json::{Value, json};

use crate::props::Props;
use crate::types::FEFloat;
use crate::enums::{Status, NotificationType};
use crate::traits::{StatusTrait, ServerNotificationTrait};


#[derive(Debug, Clone)]
pub struct Point
{
    x: FEFloat,
    y: FEFloat,
    z: FEFloat,
    uid: u32,
    status: Status<u32>,
}


impl Point
{
    pub fn create(x: FEFloat, y: FEFloat, z: FEFloat, uid: u32) -> Self
    {
        Point { x, y, z, uid, status: Status::Active }
    }


    pub fn are_coordinates_same(&self, x: FEFloat, y: FEFloat, z: FEFloat) -> bool
    {
        self.x == x && self.y == y && self.z == z
    }


    pub fn get_coordinates(&self) -> [FEFloat; 3]
    {
        [self.x, self.y, self.z]
    }


    pub fn set_coordinates(&mut self, x: FEFloat, y: FEFloat, z: FEFloat)
    {
        self.x = x;
        self.y = y;
        self.z = z;
    }


    pub fn get_uid(&self) -> u32
    {
        self.uid
    }
}


impl StatusTrait for Point
{
    type Key = u32;
    fn get_mut_ref_status(&mut self) -> &mut Status<Self::Key> 
    {
        &mut self.status
    }
}


impl ServerNotificationTrait for Point
{
    type Key = u32;
    fn get_event_detail(&self, notification_type: &NotificationType, key: Self::Key) -> Value 
    {
        match notification_type
        {
            NotificationType::Add(is_action_id_should_be_increased) | 
            NotificationType::Update(is_action_id_should_be_increased) =>
            {
                json!({ 
                    "point_data": 
                    { 
                        "number": key, "uid": self.uid, "x": self.x, "y": self.y, "z": self.z 
                    },
                    "is_action_id_should_be_increased": is_action_id_should_be_increased 
                })
            },
            NotificationType::Delete(is_action_id_should_be_increased) =>
            {
                json!({ 
                    "point_data": { "number": key, "uid": self.uid },
                    "is_action_id_should_be_increased": is_action_id_should_be_increased 
                })
            }
        }
    }


    fn get_event_name(&self, notification_type: &NotificationType, props: &Props) -> String 
    {
        match notification_type
        {
            NotificationType::Add(_) => props.add_point_event_name.clone(),
            NotificationType::Update(_) => props.update_point_event_name.clone(),
            NotificationType::Delete(_) => props.delete_point_event_name.clone(),
        }
    }


    fn get_event_target(&self, props: &Props) -> String 
    {
        props.event_target.clone()
    }
}
