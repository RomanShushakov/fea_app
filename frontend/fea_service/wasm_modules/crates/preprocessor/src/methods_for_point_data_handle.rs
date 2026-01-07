use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

use crate::enums::{NotificationType, Status, ParentKey};
use crate::types::FEFloat;
use crate::structs::Point as NewPoint;
use crate::functions::{are_all_numbers_unique, move_objects_to_dst, clear_deleted_objects_by_action_id};
use crate::traits::{StatusTrait, ServerNotificationTrait, MeshSeedTrait};
use crate::Preprocessor;


#[wasm_bindgen]
impl Preprocessor
{
    fn check_point_number_absence(&self, number: u32) -> Result<(), JsValue>
    {
        if self.points.contains_key(&number)
        {
            let error_message = format!("Point with number {number} already exists!");
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    pub(super) fn check_point_number_existence(&self, number: u32) -> Result<(), JsValue>
    {
        if !self.points.contains_key(&number)
        {
            let error_message = format!("Point with number {number} does not exist!");
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    fn check_point_coordinates_absence(&self, x: FEFloat, y: FEFloat, z: FEFloat) -> Result<(), JsValue>
    {
        if self.points.values().any(|point| point.are_coordinates_same(x, y, z))
        {
            let error_message = &format!("Point with coordinates {:?}, {:?}, {:?} already exists!", x, y, z);
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    fn check_deleted_point_existence(&mut self, action_id: u32, number: u32) -> Result<(), JsValue>
    {
        if let Some(point) = self.deleted_points.get_mut(&action_id)
        {
            match point.get_status()
            {
                Status::Active | Status::Changed(_) =>
                {
                    let error_message = &format!("Incorrect status for point deleted by action {}!",
                        action_id);
                    Err(JsValue::from(error_message))
                },
                Status::Deleted(n) =>
                {
                    if n != number
                    {
                        let error_message = &format!("Incorrect number for point deleted by action {}!",
                            action_id);
                        Err(JsValue::from(error_message))
                    }
                    else
                    {
                        Ok(())
                    }
                }
            }
        }
        else
        {
            let error_message = &format!("No points deleted by action {}!", action_id);
            Err(JsValue::from(error_message))
        }
    }


    pub(super) fn check_point_numbers_existence(&self, point_numbers: &[u32]) -> Result<(), JsValue>
    {
        for point_number in point_numbers.iter()
        {
            self.check_point_number_existence(*point_number)?;
        }
        Ok(())
    }


    pub(super) fn check_point_numbers_unique(point_numbers: &[u32]) -> Result<(), JsValue>
    {
        if !are_all_numbers_unique(point_numbers)
        {
            let error_message = &format!("Point numbers should be unique {:?}!", 
                point_numbers);
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    pub(super) fn get_points_coordinates(&self, point_numbers: &[u32]) -> Vec<[FEFloat; 3]>
    {
        let mut points_coordinates = Vec::new();
        for point_number in point_numbers.iter()
        {
            let point_coordinates = self.points.get(point_number).expect("Point is absent!")
                .get_coordinates();
            points_coordinates.push(point_coordinates);
        }
        points_coordinates
    }


    pub fn add_point(&mut self, action_id: u32, number: u32, x: FEFloat, y: FEFloat, z: FEFloat, 
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_point_number_absence(number)?;
        self.check_point_coordinates_absence(x, y, z)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let uid = self.generate_uid();
        let point = NewPoint::create(x, y, z, uid);

        self.points.insert(number, point.clone());
        point.notify_server(NotificationType::Add(is_action_id_should_be_increased), number, &self.props)?;

        self.logging();

        Ok(())
    }


    pub fn update_point(&mut self, action_id: u32, number: u32, x: FEFloat, y: FEFloat,
        z: FEFloat, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_point_number_existence(number)?;
        self.check_point_coordinates_absence(x, y, z)?;

        clear_deleted_objects_by_action_id(&mut self.deleted_points, action_id);
        clear_deleted_objects_by_action_id(&mut self.deleted_lines, action_id);
        clear_deleted_objects_by_action_id(&mut self.deleted_materials, action_id);
        clear_deleted_objects_by_action_id(&mut self.deleted_truss_sections, action_id);
        clear_deleted_objects_by_action_id(&mut self.deleted_beam_sections, action_id);
        clear_deleted_objects_by_action_id(&mut self.deleted_plate_sections, action_id);
        clear_deleted_objects_by_action_id(&mut self.deleted_new_properties, action_id);
        clear_deleted_objects_by_action_id(&mut self.deleted_beam_sections_local_axis_1_directions, action_id);
        clear_deleted_objects_by_action_id(&mut self.deleted_concentrated_loads, action_id);
        clear_deleted_objects_by_action_id(&mut self.deleted_point_boundary_conditions, action_id);

        let point = self.points.get_mut(&number).expect("Point is absent!");
        point.set_coordinates(x, y, z);
        point.notify_server(NotificationType::Update(is_action_id_should_be_increased), number, &self.props)?;

        if is_action_id_should_be_increased
        {
            clear_deleted_objects_by_action_id(&mut self.deleted_surfaces, action_id);
            self.delete_non_planar_surfaces_passing_through_point(number, x, y, z, action_id)?;
            self.delete_concave_planar_surfaces_passing_through_point(number, x, y, z, action_id)?;
            let line_numbers = self.get_numbers_of_lines_passing_through_point(number);
            let mut changed_lines = Vec::new();
            for line_number in line_numbers.into_iter()
            {
                let mut line = self.lines.get(&line_number).expect("Line is absent!").clone();
                let mut changed_line = line.clone();
                changed_line.set_status(Status::Changed(line_number));
                let optional_local_axis_1_direction = 
                    changed_line.get_ref_optional_local_axis_1_direction();
                if let Some(local_axis_1_direction) = optional_local_axis_1_direction
                {
                    match self.check_local_axis_1_direction_not_parallel_to_line(line_number, local_axis_1_direction)
                    {
                        Ok(transformed_local_axis_1_direction) => 
                            line.set_transformed_local_axis_1_direction(
                                Some(transformed_local_axis_1_direction)),
                        Err(_) => 
                        {
                            line.set_local_axis_1_direction(None);
                            line.set_transformed_local_axis_1_direction(None);
                            line.set_mesh_seed(None);
                            line.set_uniformly_distributed_line_load(None);
                        }
                    }
                    changed_lines.push(changed_line);
                    self.lines.insert(line_number, line.clone());
                    line.notify_server(NotificationType::Update(false), number, &self.props)?;
                }
            }
            move_objects_to_dst(changed_lines, &mut self.changed_lines, action_id);
        }
        else
        {
            self.restore_surfaces_passing_through_point(number, action_id)?;
            clear_deleted_objects_by_action_id(&mut self.deleted_surfaces, action_id);
            self.move_lines_with_parent_key_to_active(&ParentKey::Point(number), action_id)?;
        }

        self.logging();

        Ok(())
    }


    pub fn delete_point(&mut self, action_id: u32, number: u32,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_point_number_existence(number)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let line_numbers_through_point = self.get_numbers_of_lines_passing_through_point(number);
        self.delete_lines_passing_through_point(&line_numbers_through_point, action_id)?;
        self.delete_surfaces_passing_through_point(number, action_id)?;
        self.delete_concentrated_load_applied_at_point(number, action_id)?;
        self.delete_point_boundary_condition_applied_at_point(number, action_id)?;

        let mut point = self.points.remove(&number).expect("Point is absent!");
        point.set_status(Status::Deleted(number));

        self.deleted_points.insert(action_id, point.clone());
    
        point.notify_server(NotificationType::Delete(is_action_id_should_be_increased), number, &self.props)?;

        self.logging();

        Ok(())
    }


    pub fn restore_point(&mut self, action_id: u32, number: u32, is_action_id_should_be_increased: bool) 
        -> Result<(), JsValue>
    {
        self.check_deleted_point_existence(action_id, number)?;
        self.check_point_number_absence(number)?;

        let mut point = self.deleted_points.remove(&action_id).expect("Point is absent!");
        point.set_status(Status::Active);

        self.points.insert(number, point.clone());

        point.notify_server(NotificationType::Add(is_action_id_should_be_increased), number, &self.props)?;

        self.restore_lines_passing_through_point(number, action_id)?;
        self.restore_surfaces_passing_through_point(number, action_id)?;
        self.restore_concentrated_load_applied_at_point(number, action_id)?;
        self.restore_point_boundary_condition_applied_at_point(number, action_id)?;

        self.logging();

        Ok(())
    }
}
