use serde_json::{Value, json};

use crate::props::Props;
use crate::traits::{ChildTrait, StatusTrait, ServerNotificationTrait, RelativeTrait, PropertyTrait, MeshSeedTrait};
use crate::enums::{Status, NotificationType, ParentKey, RelativeKey, MeshSeed};
use crate::structs::{ExtendedProperty, LocalAxis1Direction, UniformlyDistributedLineLoad};


#[derive(Debug, Clone)]
pub struct Line
{
    point_1_number: u32,
    point_2_number: u32,
    uid: u32,
    status: Status<u32>,
    optional_property: Option<ExtendedProperty>,
    optional_local_axis_1_direction: Option<LocalAxis1Direction>,
    optional_transformed_local_axis_1_direction: Option<LocalAxis1Direction>,
    optional_mesh_seed: Option<MeshSeed>,
    optional_uniformly_distributed_line_load: Option<UniformlyDistributedLineLoad>,
}


impl Line
{
    pub fn create(point_1_number: u32, point_2_number: u32, uid: u32) -> Self
    {
        Line { point_1_number, point_2_number, uid, status: Status::Active, optional_property: None,
            optional_local_axis_1_direction: None, optional_transformed_local_axis_1_direction: None, 
            optional_mesh_seed: None, optional_uniformly_distributed_line_load: None }
    }


    pub fn are_points_same(&self, point_1_number: u32, point_2_number: u32) -> bool
    {
        (self.point_1_number == point_1_number &&
        self.point_2_number == point_2_number) ||
        (self.point_1_number == point_2_number &&
        self.point_2_number == point_1_number)
    }


    pub fn get_point_numbers(&self) -> [u32; 2]
    {
        [self.point_1_number, self.point_2_number]
    }


    pub fn set_point_numbers(&mut self, point_1_number: u32, point_2_number: u32)
    {
        self.point_1_number = point_1_number;
        self.point_2_number = point_2_number;
    }


    pub fn get_uid(&self) -> u32
    {
        self.uid
    }


    pub fn is_local_axis_1_direction_assigned(&self) -> bool
    {
        self.optional_local_axis_1_direction.is_some()
    }


    pub fn is_local_axis_1_direction_same(&self, local_axis_1_direction: &LocalAxis1Direction) -> bool
    {
        if let Some(loc_axis_1_direction) = &self.optional_local_axis_1_direction
        {
            return loc_axis_1_direction == local_axis_1_direction;
        }
        false
    }


    pub fn get_ref_optional_local_axis_1_direction(&self) -> &Option<LocalAxis1Direction>
    {
        &self.optional_local_axis_1_direction
    }


    pub fn set_local_axis_1_direction(&mut self, optional_local_axis_1_direction: Option<LocalAxis1Direction>)
    {
        self.optional_local_axis_1_direction = optional_local_axis_1_direction;
    }


    pub fn set_transformed_local_axis_1_direction(&mut self, 
        optional_transformed_local_axis_1_direction: Option<LocalAxis1Direction>)
    {
        self.optional_transformed_local_axis_1_direction = optional_transformed_local_axis_1_direction;
    }


    pub fn get_ref_optional_transformed_local_axis_1_direction(&self) -> &Option<LocalAxis1Direction>
    {
        &self.optional_transformed_local_axis_1_direction
    }


    pub fn is_uniformly_distributed_line_load_assigned(&self) -> bool
    {
        self.optional_uniformly_distributed_line_load.is_some()
    }


    pub fn get_ref_optional_uniformly_distributed_line_load(&self) -> &Option<UniformlyDistributedLineLoad>
    {
        &self.optional_uniformly_distributed_line_load
    }


    pub fn get_mut_ref_optional_uniformly_distributed_line_load(&mut self) -> &mut Option<UniformlyDistributedLineLoad>
    {
        &mut self.optional_uniformly_distributed_line_load
    }


    pub fn set_uniformly_distributed_line_load(&mut self, 
        optional_uniformly_distributed_line_load: Option<UniformlyDistributedLineLoad>)
    {
        self.optional_uniformly_distributed_line_load = optional_uniformly_distributed_line_load;
    }


    pub fn get_ref_optional_property(&self) -> &Option<ExtendedProperty>
    {
        &self.optional_property
    }


    pub fn get_ref_optional_mesh_seed(&self) -> &Option<MeshSeed>
    {
        &self.optional_mesh_seed
    }
}


impl ChildTrait for Line
{
    fn get_parent_keys(&self) -> Vec<ParentKey> 
    {
        vec![ParentKey::Point(self.point_1_number), ParentKey::Point(self.point_2_number)]
    }
}


impl StatusTrait for Line
{
    type Key = u32;
    fn get_mut_ref_status(&mut self) -> &mut Status<Self::Key> 
    {
        &mut self.status
    }
}


impl ServerNotificationTrait for Line
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
                    "line_data": 
                    { 
                        "number": key, 
                        "uid": self.uid,
                        "point_1_number": self.point_1_number, 
                        "point_2_number": self.point_2_number,
                        "optional_property": self.optional_property,
                        "optional_local_axis_1_direction": self.optional_local_axis_1_direction,
                        "optional_transformed_local_axis_1_direction": self.optional_transformed_local_axis_1_direction,
                        "optional_mesh_seed": self.optional_mesh_seed, 
                        "optional_uniformly_distributed_line_load": self.optional_uniformly_distributed_line_load,
                    },
                    "is_action_id_should_be_increased": is_action_id_should_be_increased 
                })
            },
            NotificationType::Delete(is_action_id_should_be_increased) =>
            {
                json!({ 
                    "line_data": { "number": key, "uid": self.uid },
                    "is_action_id_should_be_increased": is_action_id_should_be_increased 
                })
            }
        }
    }


    fn get_event_name(&self, notification_type: &NotificationType, props: &Props) -> String 
    {
        match notification_type
        {
            NotificationType::Add(_) => props.add_line_event_name.clone(),
            NotificationType::Update(_) => props.update_line_event_name.clone(),
            NotificationType::Delete(_) => props.delete_line_event_name.clone(),
        }
    }


    fn get_event_target(&self, props: &Props) -> String 
    {
        props.event_target.clone()
    }
}


impl RelativeTrait for Line
{
    fn get_relative_keys(&self) -> Vec<RelativeKey> 
    {
        let mut relative_keys = Vec::new();
        if let Some(property) = self.optional_property.as_ref()
        {
            let property_name = property.get_name();
            relative_keys.push(RelativeKey::Property(property_name));
        }
        if let Some(local_axis_1_direction) = self.optional_local_axis_1_direction.as_ref()
        {
            relative_keys.push(RelativeKey::LocalAxis1Direction(local_axis_1_direction.clone()));
        }
        relative_keys
    }
    fn set_relative_to_none(&mut self, relative_key: &RelativeKey)
    {
        match relative_key
        {
            RelativeKey::Property(_) => 
            {
                self.optional_property = None;
                self.optional_local_axis_1_direction = None;
                self.optional_transformed_local_axis_1_direction = None;
                self.optional_mesh_seed = None;
                self.optional_uniformly_distributed_line_load = None;
            },
            RelativeKey::LocalAxis1Direction(_) => 
            {
                self.optional_local_axis_1_direction = None;
                self.optional_transformed_local_axis_1_direction = None;
                self.optional_mesh_seed = None;
                self.optional_uniformly_distributed_line_load = None;
            }
        }
    }
}


impl PropertyTrait for Line
{
    fn get_ref_property(&self) -> &Option<ExtendedProperty>
    {
        &self.optional_property
    }


    fn get_mut_ref_property(&mut self) -> &mut Option<ExtendedProperty>
    {
        &mut self.optional_property
    }
}


impl MeshSeedTrait for Line
{
    fn get_ref_mesh_seed(&self) -> &Option<MeshSeed> 
    {
        &self.optional_mesh_seed
    }


    fn get_mut_ref_mesh_seed(&mut self) -> &mut Option<MeshSeed> 
    {
        &mut self.optional_mesh_seed
    }
}
