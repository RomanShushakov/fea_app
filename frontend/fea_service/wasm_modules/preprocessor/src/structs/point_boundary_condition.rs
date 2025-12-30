use serde_json::{Value, json};

use crate::props::Props;
use crate::traits::{StatusTrait, ServerNotificationTrait};
use crate::types::FEFloat;
use crate::enums::{Status, NotificationType};


#[derive(Debug, Clone)]
pub struct PointBoundaryCondition
{
    uid: u32,
    status: Status<u32>,
    optional_ux: Option<FEFloat>,
    optional_uy: Option<FEFloat>,
    optional_uz: Option<FEFloat>,
    optional_rx: Option<FEFloat>,
    optional_ry: Option<FEFloat>,
    optional_rz: Option<FEFloat>,
}


impl PointBoundaryCondition
{
    pub fn create(optional_ux: Option<FEFloat>, optional_uy: Option<FEFloat>, optional_uz: Option<FEFloat>, 
        optional_rx: Option<FEFloat>, optional_ry: Option<FEFloat>, optional_rz: Option<FEFloat>, uid: u32) -> Self
    {
        PointBoundaryCondition { optional_ux, optional_uy, optional_uz, optional_rx, optional_ry, optional_rz, uid, 
            status: Status::Active }
    }


    pub fn get_displacement_components(&self) -> [Option<FEFloat>; 6]
    {
        [self.optional_ux, self.optional_uy, self.optional_uz, self.optional_rx, self.optional_ry, self.optional_rz]
    }


    pub fn set_displacement_components(&mut self, optional_ux: Option<FEFloat>, optional_uy: Option<FEFloat>, 
        optional_uz: Option<FEFloat>, optional_rx: Option<FEFloat>, optional_ry: Option<FEFloat>, 
        optional_rz: Option<FEFloat>)
    {
        self.optional_ux = optional_ux;
        self.optional_uy = optional_uy;
        self.optional_uz = optional_uz;
        self.optional_rx = optional_rx;
        self.optional_ry = optional_ry;
        self.optional_rz = optional_rz;
    }


    pub fn get_uid(&self) -> u32
    {
        self.uid
    }
}


impl StatusTrait for PointBoundaryCondition
{
    type Key = u32;
    fn get_mut_ref_status(&mut self) -> &mut Status<Self::Key> 
    {
        &mut self.status
    }
}


impl ServerNotificationTrait for PointBoundaryCondition
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
                    "point_boundary_condition_data": 
                    { 
                        "point_number": key, 
                        "uid": self.uid,
                        "optional_ux": self.optional_ux, 
                        "optional_uy": self.optional_uy,
                        "optional_uz": self.optional_uz,
                        "optional_rx": self.optional_rx, 
                        "optional_ry": self.optional_ry,
                        "optional_rz": self.optional_rz
                    },
                    "is_action_id_should_be_increased": is_action_id_should_be_increased 
                })
            },
            NotificationType::Delete(is_action_id_should_be_increased) =>
            {
                json!({ 
                    "point_boundary_condition_data": { "point_number": key, "uid": self.uid },
                    "is_action_id_should_be_increased": is_action_id_should_be_increased 
                })
            }
        }
    }


    fn get_event_name(&self, notification_type: &NotificationType, props: &Props) -> String 
    {
        match notification_type
        {
            NotificationType::Add(_) => props.add_point_boundary_condition_event_name.clone(),
            NotificationType::Update(_) => props.update_point_boundary_condition_event_name.clone(),
            NotificationType::Delete(_) => props.delete_point_boundary_condition_event_name.clone(),
        }
    }


    fn get_event_target(&self, props: &Props) -> String 
    {
        props.event_target.clone()
    }
}
