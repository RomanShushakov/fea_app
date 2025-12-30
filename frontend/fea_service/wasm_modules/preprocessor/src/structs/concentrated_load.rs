use serde_json::{Value, json};

use crate::props::Props;
use crate::traits::{StatusTrait, ServerNotificationTrait};
use crate::types::FEFloat;
use crate::enums::{Status, NotificationType};


#[derive(Debug, Clone)]
pub struct ConcentratedLoad
{
    uid: u32,
    status: Status<u32>,
    fx: FEFloat,
    fy: FEFloat,
    fz: FEFloat,
    mx: FEFloat,
    my: FEFloat,
    mz: FEFloat,
}


impl ConcentratedLoad
{
    pub fn create(fx: FEFloat, fy: FEFloat, fz: FEFloat, mx: FEFloat, my: FEFloat, mz: FEFloat, uid: u32) -> Self
    {
        ConcentratedLoad { fx, fy, fz, mx, my, mz, uid, status: Status::Active }
    }


    pub fn get_load_components(&self) -> [FEFloat; 6]
    {
        [self.fx, self.fy, self.fz, self.mx, self.my, self.mz]
    }


    pub fn set_load_components(&mut self, fx: FEFloat, fy: FEFloat, fz: FEFloat, mx: FEFloat, my: FEFloat, mz: FEFloat)
    {
        self.fx = fx;
        self.fy = fy;
        self.fz = fz;
        self.mx = mx;
        self.my = my;
        self.mz = mz;
    }


    pub fn get_uid(&self) -> u32
    {
        self.uid
    }
}


impl StatusTrait for ConcentratedLoad
{
    type Key = u32;
    fn get_mut_ref_status(&mut self) -> &mut Status<Self::Key> 
    {
        &mut self.status
    }
}


impl ServerNotificationTrait for ConcentratedLoad
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
                    "concentrated_load_data": 
                    { 
                        "point_number": key, 
                        "uid": self.uid,
                        "fx": self.fx, 
                        "fy": self.fy,
                        "fz": self.fz, 
                        "mx": self.mx,
                        "my": self.my,
                        "mz": self.mz
                    },
                    "is_action_id_should_be_increased": is_action_id_should_be_increased 
                })
            },
            NotificationType::Delete(is_action_id_should_be_increased) =>
            {
                json!({ 
                    "concentrated_load_data": { "point_number": key, "uid": self.uid },
                    "is_action_id_should_be_increased": is_action_id_should_be_increased 
                })
            }
        }
    }


    fn get_event_name(&self, notification_type: &NotificationType, props: &Props) -> String 
    {
        match notification_type
        {
            NotificationType::Add(_) => props.add_concentrated_load_event_name.clone(),
            NotificationType::Update(_) => props.update_concentrated_load_event_name.clone(),
            NotificationType::Delete(_) => props.delete_concentrated_load_event_name.clone(),
        }
    }


    fn get_event_target(&self, props: &Props) -> String 
    {
        props.event_target.clone()
    }
}
