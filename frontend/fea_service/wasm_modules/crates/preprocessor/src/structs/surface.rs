use serde_json::{Value, json};

use crate::props::Props;
use crate::traits::{ChildTrait, StatusTrait, ServerNotificationTrait, RelativeTrait, PropertyTrait, MeshSeedTrait};
use crate::enums::{Status, NotificationType, ParentKey, RelativeKey, MeshSeed};
use crate::structs::{ExtendedProperty, Normal, UniformlyDistributedSurfaceLoad};
use crate::functions::recursive_permutations;


#[derive(Debug, Clone)]
pub struct Surface
{
        point_1_number: u32,
        point_2_number: u32,
        point_3_number: u32,
        point_4_number: u32,
        normal: Normal,
        uid: u32,
        status: Status<u32>,
        optional_property: Option<ExtendedProperty>,
        optional_mesh_seed: Option<MeshSeed>,
        optional_uniformly_distributed_surface_load: Option<UniformlyDistributedSurfaceLoad>,
}


impl Surface
{
    pub fn create(point_1_number: u32, point_2_number: u32, point_3_number: u32, point_4_number: u32, 
        normal: Normal, uid: u32) -> Self
    {
        Surface { point_1_number, point_2_number, point_3_number, point_4_number, status: Status::Active, normal, uid, 
            optional_property: None, optional_mesh_seed: None, optional_uniformly_distributed_surface_load: None }
    }


    pub fn are_points_same(&self, point_1_number: u32, point_2_number: u32, point_3_number: u32, 
        point_4_number: u32) -> bool
    {
        let sequence = vec![point_1_number, point_2_number, point_3_number, point_4_number];
        let permutations = recursive_permutations(sequence);
        if permutations.iter()
            .position(|permutation| 
                *permutation == vec![self.point_1_number, self.point_2_number, self.point_3_number, 
                    self.point_4_number])
            .is_some()
        {
            true
        }
        else
        {
            false
        }
    }


    pub fn are_edge_1_points_same(&self, point_1_number: u32, point_2_number: u32) -> bool
    {
        (self.point_1_number == point_1_number &&
        self.point_2_number == point_2_number) ||
        (self.point_1_number == point_2_number &&
        self.point_2_number == point_1_number)
    }


    pub fn are_edge_2_points_same(&self, point_1_number: u32, point_2_number: u32) -> bool
    {
        (self.point_2_number == point_1_number &&
        self.point_3_number == point_2_number) ||
        (self.point_2_number == point_2_number &&
        self.point_3_number == point_1_number)
    }


    pub fn are_edge_3_points_same(&self, point_1_number: u32, point_2_number: u32) -> bool
    {
        (self.point_3_number == point_1_number &&
        self.point_4_number == point_2_number) ||
        (self.point_3_number == point_2_number &&
        self.point_4_number == point_1_number)
    }


    pub fn are_edge_4_points_same(&self, point_1_number: u32, point_2_number: u32) -> bool
    {
        (self.point_4_number == point_1_number &&
        self.point_1_number == point_2_number) ||
        (self.point_4_number == point_2_number &&
        self.point_1_number == point_1_number)
    }


    pub fn are_diagonal_1_3_points_same(&self, point_1_number: u32, point_2_number: u32) -> bool
    {
        (self.point_1_number == point_1_number &&
        self.point_3_number == point_2_number) ||
        (self.point_1_number == point_2_number &&
        self.point_3_number == point_1_number)
    }


    pub fn are_diagonal_2_4_points_same(&self, point_1_number: u32, point_2_number: u32) -> bool
    {
        (self.point_2_number == point_1_number &&
        self.point_4_number == point_2_number) ||
        (self.point_2_number == point_2_number &&
        self.point_4_number == point_1_number)
    }


    pub fn get_point_numbers(&self) -> [u32; 4]
    {
        [self.point_1_number, self.point_2_number, self.point_3_number, self.point_4_number]
    }


    pub fn set_point_numbers(&mut self, point_1_number: u32, point_2_number: u32, point_3_number: u32, 
        point_4_number: u32)
    {
        self.point_1_number = point_1_number;
        self.point_2_number = point_2_number;
        self.point_3_number = point_3_number;
        self.point_4_number = point_4_number;
    }


    pub fn set_normal(&mut self, normal: Normal)
    {
        self.normal = normal;
    }


    pub fn rotate_vertices_clockwise(&mut self)
    {
        let (point_1_number, point_2_number, point_3_number, point_4_number) = 
            (self.point_1_number, self.point_2_number, self.point_3_number, self.point_4_number);
        self.point_1_number = point_4_number;
        self.point_2_number = point_1_number;
        self.point_3_number = point_2_number;
        self.point_4_number = point_3_number;
    }


    pub fn rotate_vertices_counter_clockwise(&mut self)
    {
        let (point_1_number, point_2_number, point_3_number, point_4_number) = 
            (self.point_1_number, self.point_2_number, self.point_3_number, self.point_4_number);
        self.point_1_number = point_2_number;
        self.point_2_number = point_3_number;
        self.point_3_number = point_4_number;
        self.point_4_number = point_1_number;
    }


    pub fn flip_normal_axis(&mut self)
    {
        let (point_1_number, point_2_number, point_3_number, point_4_number) = 
            (self.point_1_number, self.point_2_number, self.point_3_number, self.point_4_number);
        self.point_1_number = point_2_number;
        self.point_2_number = point_1_number;
        self.point_3_number = point_4_number;
        self.point_4_number = point_3_number;
        self.normal.flip();
    }


    pub fn get_uid(&self) -> u32
    {
        self.uid
    }


    pub fn is_uniformly_distributed_surface_load_assigned(&self) -> bool
    {
        self.optional_uniformly_distributed_surface_load.is_some()
    }


    pub fn get_ref_optional_uniformly_distributed_surface_load(&self) -> &Option<UniformlyDistributedSurfaceLoad>
    {
        &self.optional_uniformly_distributed_surface_load
    }


    pub fn get_mut_ref_optional_uniformly_distributed_surface_load(&mut self) 
        -> &mut Option<UniformlyDistributedSurfaceLoad>
    {
        &mut self.optional_uniformly_distributed_surface_load
    }


    pub fn set_uniformly_distributed_surface_load(&mut self, 
        optional_uniformly_distributed_surface_load: Option<UniformlyDistributedSurfaceLoad>)
    {
        self.optional_uniformly_distributed_surface_load = optional_uniformly_distributed_surface_load;
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


impl ChildTrait for Surface
{
    fn get_parent_keys(&self) -> Vec<ParentKey> 
    {
        vec![ParentKey::Point(self.point_1_number), ParentKey::Point(self.point_2_number), 
            ParentKey::Point(self.point_3_number), ParentKey::Point(self.point_4_number)]
    }
}


impl StatusTrait for Surface
{
    type Key = u32;
    fn get_mut_ref_status(&mut self) -> &mut Status<Self::Key> 
    {
        &mut self.status
    }
}


impl ServerNotificationTrait for Surface
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
                    "surface_data": 
                    { 
                        "number": key, 
                        "uid": self.uid,
                        "point_1_number": self.point_1_number, 
                        "point_2_number": self.point_2_number, 
                        "point_3_number": self.point_3_number, 
                        "point_4_number": self.point_4_number,
                        "normal": self.normal,
                        "optional_property": self.optional_property,
                        "optional_mesh_seed": self.optional_mesh_seed,
                        "optional_uniformly_distributed_surface_load": 
                            self.optional_uniformly_distributed_surface_load,
                    },
                    "is_action_id_should_be_increased": is_action_id_should_be_increased 
                })
            },
            NotificationType::Delete(is_action_id_should_be_increased) =>
            {
                json!({ 
                    "surface_data": { "number": key, "uid": self.uid },
                    "is_action_id_should_be_increased": is_action_id_should_be_increased 
                })
            }
        }
    }


    fn get_event_name(&self, notification_type: &NotificationType, props: &Props) -> String 
    {
        match notification_type
        {
            NotificationType::Add(_) => props.add_surface_event_name.clone(),
            NotificationType::Update(_) => props.update_surface_event_name.clone(),
            NotificationType::Delete(_) => props.delete_surface_event_name.clone(),
        }
    }


    fn get_event_target(&self, props: &Props) -> String 
    {
        props.event_target.clone()
    }
}


impl RelativeTrait for Surface
{
    fn get_relative_keys(&self) -> Vec<RelativeKey> 
    {
        let mut relative_keys = Vec::new();
        if let Some(property) = self.optional_property.as_ref()
        {
            let property_name = property.get_name();
            relative_keys.push(RelativeKey::Property(property_name));
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
                self.optional_mesh_seed = None;
                self.optional_uniformly_distributed_surface_load = None;
            },
            RelativeKey::LocalAxis1Direction(_) => unreachable!(),
        }
    }
}


impl PropertyTrait for Surface
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


impl MeshSeedTrait for Surface
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
