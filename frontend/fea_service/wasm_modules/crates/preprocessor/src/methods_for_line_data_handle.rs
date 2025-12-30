use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use serde_json::json;

use crate::traits::{ChildTrait, StatusTrait, ServerNotificationTrait, PropertyTrait, MeshSeedTrait};
use crate::enums::{NotificationType, Status, ParentKey, RelativeKey, MeshSeed};
use crate::types::FEFloat;
use crate::structs::{Line as NewLine, ExtendedProperty, LocalAxis1Direction, UniformlyDistributedLineLoad};
use crate::Preprocessor;

use crate::functions::
{
    dispatch_custom_event, find_vector_length, find_projection_of_vector_a_perpendicular_to_vector_b,
    get_objects_numbers_with_relative_keys, move_objects_to_dst
};


#[wasm_bindgen]
impl Preprocessor
{
    fn check_line_number_absence(&self, number: u32) -> Result<(), JsValue>
    {
        if self.lines.contains_key(&number)
        {
            let error_message = format!("Line with number {number} already exists!");
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    pub(super) fn check_line_number_existence(&self, number: u32) -> Result<(), JsValue>
    {
        if !self.lines.contains_key(&number)
        {
            let error_message = format!("Line with number {number} does not exist!");
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    fn check_line_through_points_absence(&self, point_1_number: u32, point_2_number: u32) 
        -> Result<(), JsValue>
    {
        if self.lines.values().position(|line| line.are_points_same(point_1_number, point_2_number)).is_some()
        {
            let error_message = &format!("Line through points {:?}, {:?} already exists!", point_1_number, 
                point_2_number);
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    fn check_deleted_line_existence(&mut self, action_id: u32, number: u32) -> Result<(), JsValue>
    {
        if let Some(lines) = self.deleted_lines.get_mut(&action_id)
        {
            if lines.len() != 1
            {
                let error_message = &format!("Incorrect number of lines deleted by action {}!",
                    action_id);
                return Err(JsValue::from(error_message));
            }

            match lines[0].get_status()
            {
                Status::Active | Status::Changed(_) =>
                {
                    let error_message = &format!("Incorrect status for line deleted by action {}!",
                        action_id);
                    return Err(JsValue::from(error_message));
                },
                Status::Deleted(n) =>
                {
                    if n != number
                    {
                        let error_message = &format!("Incorrect number for line deleted by action {}!",
                            action_id);
                        return Err(JsValue::from(error_message));
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
            let error_message = &format!("No lines deleted by action {}!", action_id);
            return Err(JsValue::from(error_message));
        }
    }


    pub(super) fn get_numbers_of_lines_passing_through_point(&self, point_number: u32) -> Vec<u32>
    {
        let line_numbers = self.lines.iter()
            .filter(|(_, line)| 
                line.is_child_of_parent(&ParentKey::Point(point_number)))
            .map(|(line_number, _)| *line_number)
            .collect::<Vec<u32>>();
        line_numbers
    }


    pub(super) fn delete_lines_passing_through_point(&mut self, line_numbers: &[u32], action_id: u32)
        -> Result<(), JsValue>
    {
        if !line_numbers.is_empty()
        {
            let mut lines = Vec::new();
            for line_number in line_numbers.into_iter()
            {
                let mut line = self.lines.remove(&line_number).expect("Line is absent!");
                line.set_status(Status::Deleted(*line_number));
                lines.push(line);
            }
            move_objects_to_dst(lines.clone(), &mut self.deleted_lines, action_id);
            for mut line in lines.into_iter()
            {
                let line_number = match line.get_status()
                    {
                        Status::Deleted(number) => number,
                        _ => unreachable!(),
                    };
                line.notify_server(NotificationType::Delete(false), line_number, &self.props)?;
            }
        }
        Ok(())
    }


    pub(super) fn restore_lines_passing_through_point(&mut self, point_number: u32, action_id: u32)
        -> Result<(), JsValue>
    {   
        if let Some(lines) = self.deleted_lines.remove(&action_id)
        {
            for mut line in lines.into_iter()
            {
                if !line.is_child_of_parent(&ParentKey::Point(point_number))
                {
                    let error_message = &format!("Line does not pass through the point {}!",
                        point_number);
                    return Err(JsValue::from(error_message));
                }
                let line_number = match line.get_status()
                    {
                        Status::Deleted(number) => Ok(number),
                        _ => 
                        {
                            let error_message = &format!("Incorrect status for line deleted by action {}!",
                                action_id);
                            Err(JsValue::from(error_message))
                        }
                    }?;
                self.check_line_number_absence(line_number)?;
                line.set_status(Status::Active);
                self.lines.insert(line_number, line.clone());
                line.notify_server(NotificationType::Add(false), line_number, &self.props)?;
            }
        }
        Ok(())
    }


    // pub(super) fn check_line_at_surface_edges_absence(&self, point_1_number: u32, point_2_number: u32, 
    //     point_3_number: u32, point_4_number: u32) -> Result<(), JsValue>
    // {
    //     self.check_line_through_points_absence(point_1_number, point_2_number)?;
    //     self.check_line_through_points_absence(point_2_number, point_3_number)?;
    //     self.check_line_through_points_absence(point_3_number, point_4_number)?;
    //     self.check_line_through_points_absence(point_4_number, point_1_number)?;
    //     Ok(())
    // }


    pub(super) fn check_line_through_surface_diagonals_absence(&self, point_1_number: u32, point_2_number: u32, 
        point_3_number: u32, point_4_number: u32) -> Result<(), JsValue>
    {
        self.check_line_through_points_absence(point_1_number, point_3_number)?;
        self.check_line_through_points_absence(point_2_number, point_4_number)?;
        Ok(())
    }


    pub fn add_line(&mut self, action_id: u32, number: u32, point_1_number: u32,
        point_2_number: u32, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_line_number_absence(number)?;
        self.check_point_numbers_existence(&[point_1_number, point_2_number])?;
        Self::check_point_numbers_unique(&[point_1_number, point_2_number])?;
        self.check_line_through_points_absence(point_1_number, point_2_number)?;
        // self.check_surface_edge_through_points_absence(point_1_number, point_2_number)?;
        self.check_surface_diagonals_through_points_absence(point_1_number, point_2_number)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let uid = self.generate_uid();
        let line = NewLine::create(point_1_number, point_2_number, uid);

        self.lines.insert(number, line.clone());
        line.notify_server(NotificationType::Add(is_action_id_should_be_increased), number, &self.props)?;

        self.logging();

        Ok(())
    }


     pub fn update_line(&mut self, action_id: u32, number: u32, point_1_number: u32,
        point_2_number: u32, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_line_number_existence(number)?;
        self.check_point_numbers_existence(&[point_1_number, point_2_number])?;
        Self::check_point_numbers_unique(&[point_1_number, point_2_number])?;
        self.check_line_through_points_absence(point_1_number, point_2_number)?;
        // self.check_surface_edge_through_points_absence(point_1_number, point_2_number)?;
        self.check_surface_diagonals_through_points_absence(point_1_number, point_2_number)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let mut line = self.lines.get(&number).expect("Line is absent!").clone();

        let mut changed_line = line.clone();
        changed_line.set_status(Status::Changed(number));
        let optional_local_axis_1_direction = 
            changed_line.get_ref_optional_local_axis_1_direction();

        line.set_point_numbers(point_1_number, point_2_number);
        self.lines.insert(number, line.clone());

        if is_action_id_should_be_increased
        {   
            if let Some(local_axis_1_direction) = optional_local_axis_1_direction
            {
                match self.check_local_axis_1_direction_not_parallel_to_line(number, &local_axis_1_direction)
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
                move_objects_to_dst(vec![changed_line], &mut self.changed_lines, action_id);
            }
            self.lines.insert(number, line.clone());
            line.notify_server(NotificationType::Update(is_action_id_should_be_increased), number, &self.props)?;
        }
        else
        {
            if let Some(mut lines) = self.changed_lines.remove(&action_id)
            {
                if lines.len() != 1
                {
                    let error_message = &format!("Incorrect number of lines changed by action {}!",
                        action_id);
                    return Err(JsValue::from(error_message));
                }
                let mut line = lines.remove(0);
                match line.get_status()
                {
                    Status::Active | Status::Deleted(_) =>
                    {
                        let error_message = &format!("Incorrect status for line changed by action {}!",
                            action_id);
                        return Err(JsValue::from(error_message));
                    },
                    Status::Changed(n) =>
                    {
                        if number != n
                        {
                            let error_message = &format!("Incorrect number for line changed by \
                                action {}!", action_id);
                            return Err(JsValue::from(error_message));
                        }
                        if !line.is_local_axis_1_direction_assigned()
                        {
                            let error_message = &format!("Incorrect local axis 1 direction for line \
                                changed by action {action_id}!");
                            return Err(JsValue::from(error_message));
                        }
                        line.set_status(Status::Active);
                        self.lines.insert(n, line.clone());
                        line.notify_server(NotificationType::Update(is_action_id_should_be_increased), number, 
                            &self.props)?;
                    }
                }
            }
        }

        self.logging();

        Ok(())
    }


    pub fn delete_line(&mut self, action_id: u32, number: u32, is_action_id_should_be_increased: bool) 
        -> Result<(), JsValue>
    {
        self.check_line_number_existence(number)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let mut line = self.lines.remove(&number).expect("Line is absent!");
        line.set_status(Status::Deleted(number));

        self.deleted_lines.insert(action_id, vec![line.clone()]);
    
        line.notify_server(NotificationType::Delete(is_action_id_should_be_increased), number, &self.props)?;

        self.logging();

        Ok(())
    }


    pub fn restore_line(&mut self, action_id: u32, number: u32, is_action_id_should_be_increased: bool) 
        -> Result<(), JsValue>
    {
        self.check_deleted_line_existence(action_id, number)?;
        self.check_line_number_absence(number)?;

        let mut line = self.deleted_lines.remove(&action_id).expect("Line is absent!").remove(0);
        line.set_status(Status::Active);

        self.lines.insert(number, line.clone());

        line.notify_server(NotificationType::Add(is_action_id_should_be_increased), number, &self.props)?;

        self.logging();

        Ok(())
    }


    fn check_line_without_other_property(&self, number: u32, property_name: &str) -> Result<(), JsValue>
    {
        if let Some(line) = self.lines.get(&number)
        {
            if line.is_property_assigned()
            {
                if !line.is_property_name_same(property_name.to_string()) 
                {
                    let error_message = format!("Another property has been already assigned to line with \
                        number {number}!");
                    return Err(JsValue::from(error_message));
                }
            }
            Ok(())
        }
        else
        {
            let error_message = format!("Line with number {number} does not exist!");
            return Err(JsValue::from(error_message));
        }
    }


    fn check_line_without_any_local_axis_1_direction(&self, number: u32) -> Result<(), JsValue>
    {
        if let Some(line) = self.lines.get(&number)
        {
            if line.is_local_axis_1_direction_assigned()
            {
                let error_message = format!("Local axis 1 direction has been already assigned to line with \
                    number {number}!");
                return Err(JsValue::from(error_message));
            }
            Ok(())
        }
        else
        {
            let error_message = format!("Line with number {number} does not exist!");
            return Err(JsValue::from(error_message));
        }
    }


    pub fn assign_properties_to_lines(&mut self, action_id: u32, name: &str, line_numbers: &[u32], 
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_property_to_lines_applicability(name)?;
        for number in line_numbers.iter()
        {
            self.check_line_number_existence(*number)?;
            self.check_line_without_other_property(*number, name)?;
        }

        let other_line_numbers_with_same_properties = get_objects_numbers_with_relative_keys(&self.lines, 
            &[RelativeKey::Property(name.to_string())])
            .into_iter()
            .map(|(number, _rk)| number)
            .filter(|n| !line_numbers.contains(n))
            .collect::<Vec<u32>>();
        for number in other_line_numbers_with_same_properties.into_iter()
        {
            self.check_line_without_any_local_axis_1_direction(number)?;
        }
        
        self.clear_all_deleted_objects_by_action_id(action_id);

        for (number, line) in self.lines.iter_mut()
        {
            if line.is_property_name_same(name.to_string())
            {
                line.set_propery(None);
                line.notify_server(NotificationType::Update(false), *number, &self.props)?;
            }
        }

        let property = self.new_properties.get(name).cloned().expect("Property is absent!");
        let extended_property = ExtendedProperty::create(name.to_string(), property); 

        for number in line_numbers.iter()
        {
            let line = self.lines.get_mut(number).expect("Line is absent!");
            line.set_propery(Some(extended_property.clone()));
            line.notify_server(NotificationType::Update(false), *number, &self.props)?;
        }

        if is_action_id_should_be_increased
        {
            let detail = json!(null);
            dispatch_custom_event(detail, &self.props.increase_action_id_event_name, 
                &self.props.event_target)?;
        }

        self.logging();

        Ok(())
    }


    fn get_line_direction_vector(&self, number: u32) -> Result<[FEFloat; 3], JsValue>
    {
        self.check_line_number_existence(number)?;
        let line = self.lines.get(&number).expect("Line is absent!");
        let [point_1_number, point_2_number] = line.get_point_numbers();
        self.check_point_numbers_existence(&[point_1_number, point_2_number])?;
        let endpoints_coordinates = self.get_points_coordinates(
            &[point_1_number, point_2_number]);
        Ok([
            endpoints_coordinates[1][0] - endpoints_coordinates[0][0], 
            endpoints_coordinates[1][1] - endpoints_coordinates[0][1],
            endpoints_coordinates[1][2] - endpoints_coordinates[0][2],
        ])
    }


    pub(super) fn check_local_axis_1_direction_not_parallel_to_line(&self, number: u32, 
        local_axis_1_direction: &LocalAxis1Direction) -> Result<LocalAxis1Direction, JsValue>
    {
        let line_direction_vector = self.get_line_direction_vector(number)?;

        let projection_of_beam_section_orientation_vector = 
            find_projection_of_vector_a_perpendicular_to_vector_b(&local_axis_1_direction.get_components(), 
                &line_direction_vector);

        let projection_of_beam_section_orientation_length = find_vector_length(
            &projection_of_beam_section_orientation_vector);

        if projection_of_beam_section_orientation_length == 0 as FEFloat
        {
            let error_message = format!("Projection of local axis 1 direction \
                on line number {number} equals to zero!");
            return Err(JsValue::from(error_message));
        }

        let transformed_local_axis_1_direction = LocalAxis1Direction::create(
            &projection_of_beam_section_orientation_vector, &self.props)?;
        Ok(transformed_local_axis_1_direction)
    }


    fn check_local_axis_1_direction_can_be_assigned(&self, number: u32) -> Result<(), JsValue>
    {
        if let Some(line) = self.lines.get(&number)
        {
            if line.is_property_assigned()
            {
                let cross_section = line.get_optional_cross_section()
                    .expect("Cross section is absent!");
                if cross_section.is_beam()
                {
                    return Ok(());
                }
                let error_message = format!("Local axis 1 direction could not be assigned to \
                    line with number {number}!");
                return Err(JsValue::from(error_message));
            }
            else
            {
                let error_message = format!("Local axis 1 direction could not be assigned to \
                    line with number {number}!");
                return Err(JsValue::from(error_message));
            }
        }
        else
        {
            let error_message = format!("Line with number {number} does not exist!");
            return Err(JsValue::from(error_message));
        }
    }


    fn check_line_without_other_local_axis_1_direction(&self, number: u32, local_axis_1_direction: &LocalAxis1Direction) 
        -> Result<(), JsValue>
    {
        if let Some(line) = self.lines.get(&number)
        {
            if line.is_local_axis_1_direction_assigned()
            {
                if !line.is_local_axis_1_direction_same(local_axis_1_direction) 
                {
                    let error_message = format!("Another local axis 1 direction has been already assigned to \
                        line with number {number}!");
                    return Err(JsValue::from(error_message));
                }
            }
            Ok(())
        }
        else
        {
            let error_message = format!("Line with number {number} does not exist!");
            return Err(JsValue::from(error_message));
        }
    }


    pub fn assign_beam_section_local_axis_1_direction(&mut self, action_id: u32, components: &[FEFloat], 
        line_numbers: &[u32], is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        let local_axis_1_direction = LocalAxis1Direction::create(components, &self.props)?;
        self.check_beam_section_local_axis_1_direction_existence(&local_axis_1_direction)?;
        let mut transformed_local_axis_1_directions = Vec::new();
        for number in line_numbers.iter()
        {
            self.check_line_number_existence(*number)?;
            let transformed_local_axis_1_direction = 
                self.check_local_axis_1_direction_not_parallel_to_line(*number, &local_axis_1_direction)?;
            transformed_local_axis_1_directions.push((*number, transformed_local_axis_1_direction));
            self.check_local_axis_1_direction_can_be_assigned(*number)?;
            self.check_line_without_other_local_axis_1_direction(*number, &local_axis_1_direction)?;
        }

        self.clear_all_deleted_objects_by_action_id(action_id);

        for (number, line) in self.lines.iter_mut()
        {
            if line.is_local_axis_1_direction_same(&local_axis_1_direction)
            {
                line.set_local_axis_1_direction(None);
                line.set_transformed_local_axis_1_direction(None);
                line.set_mesh_seed(None);
                line.set_uniformly_distributed_line_load(None);
                line.notify_server(NotificationType::Update(false), *number, &self.props)?;
            }
        }

        for (number, transformed_local_axis_1_direction) in transformed_local_axis_1_directions.into_iter()
        {
            let line = self.lines.get_mut(&number).expect("Line is absent!");
            line.set_local_axis_1_direction(Some(local_axis_1_direction.clone()));
            line.set_transformed_local_axis_1_direction(Some(transformed_local_axis_1_direction));
            line.set_mesh_seed(Some(MeshSeed::Global(self.global_mesh_seed_value)));
            line.notify_server(NotificationType::Update(false), number, &self.props)?;
        }

        if is_action_id_should_be_increased
        {
            let detail = json!(null);
            dispatch_custom_event(detail, &self.props.increase_action_id_event_name, 
                &self.props.event_target)?;
        }

        self.logging();

        Ok(())
    }


    pub(super) fn move_lines_with_local_axis_1_direction_to_changed(&mut self, line_numbers: &[u32], 
        action_id: u32)
    {
        let mut changed_lines = Vec::new();
        for line_number in line_numbers.iter()
        {
            let line = self.lines.get_mut(line_number).expect("Line is absent!");
            if line.is_local_axis_1_direction_assigned()
            {
                let mut changed_line = line.clone();
                changed_line.set_status(Status::Changed(*line_number));
                changed_lines.push(changed_line);
                line.set_local_axis_1_direction(None);
                line.set_transformed_local_axis_1_direction(None);
                line.set_mesh_seed(None);
                line.set_uniformly_distributed_line_load(None);
            }
        }
        if !changed_lines.is_empty()
        {
            if let Some(ch_lines) = self.changed_lines.get_mut(&action_id)
            {
                for line in changed_lines.into_iter()
                {
                    ch_lines.push(line);
                }
            }
            else
            {
                self.changed_lines.insert(action_id, changed_lines);
            }
        }
    }


    pub(super) fn move_lines_with_local_axis_1_direction_to_active(&mut self, line_numbers: &[u32], 
        action_id: u32) -> Result<(), JsValue>
    {
        if let Some(lines) = self.changed_lines.remove(&action_id)
        {
            for mut line in lines.into_iter()
            {
                match line.get_status()
                {
                    Status::Active | Status::Deleted(_) =>
                    {
                        let error_message = &format!("Incorrect status for line changed by action {}!",
                            action_id);
                        return Err(JsValue::from(error_message));
                    },
                    Status::Changed(n) =>
                    {
                        if !line_numbers.contains(&n)
                        {
                            let error_message = &format!("Incorrect property for line changed by action {}!",
                                action_id);
                            return Err(JsValue::from(error_message));
                        }
                        if !line.is_local_axis_1_direction_assigned()
                        {
                            let error_message = &format!("Incorrect local axis 1 direction for line \
                                changed by action {action_id}!");
                            return Err(JsValue::from(error_message));
                        }
                        line.set_status(Status::Active);
                        self.lines.insert(n, line);
                    }
                }
            }
        }
        Ok(())
    }


    pub(super) fn move_lines_with_parent_key_to_active(&mut self, parent_key: &ParentKey, action_id: u32)
        -> Result<(), JsValue>
    {   
        if let Some(lines) = self.changed_lines.remove(&action_id)
        {
            for mut line in lines.into_iter()
            {
                match line.get_status()
                {
                    Status::Active | Status::Deleted(_) =>
                    {
                        let error_message = &format!("Incorrect status for line changed by action {}!",
                            action_id);
                        return Err(JsValue::from(error_message));
                    },
                    Status::Changed(n) =>
                    {
                        if !line.is_child_of_parent(parent_key)
                        {
                            let error_message = &format!("There are no line with parent key {parent_key:?}!");
                            return Err(JsValue::from(error_message));
                        }
                        line.set_status(Status::Active);
                        line.notify_server(NotificationType::Update(false), n, &self.props)?;
                        self.lines.insert(n, line);
                    }
                }
            }
        }
        Ok(())
    }


    pub fn update_lines_mesh_seed(&mut self, action_id: u32, lines_mesh_seed_value: u8,
        line_numbers: &[u32], is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        for number in line_numbers.iter()
        {
            self.check_local_axis_1_direction_can_be_assigned(*number)
                .map_err(|_| JsValue::from(format!("Mesh seed could not be set for line {number}")))?;
        }

        self.clear_all_deleted_objects_by_action_id(action_id);

        let mut changed_lines = Vec::new();
        for number in line_numbers.into_iter()
        {
            let line = self.lines.get_mut(&number).expect("Line is absent!");
            let mut changed_line = line.clone();
            changed_line.set_status(Status::Changed(*number));
            changed_lines.push(changed_line);
            line.set_mesh_seed(Some(MeshSeed::Line(lines_mesh_seed_value)));
            line.notify_server(NotificationType::Update(false), *number, &self.props)?;
        }

        move_objects_to_dst(changed_lines, &mut self.changed_lines, action_id);

        if is_action_id_should_be_increased
        {
            let detail = json!(null);
            dispatch_custom_event(detail, &self.props.increase_action_id_event_name, 
                &self.props.event_target)?;
        }

        self.logging();

        Ok(())
    }


    pub fn undo_lines_mesh_seed_update(&mut self, action_id: u32, line_numbers: &[u32],
        _is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        if let Some(lines) = self.changed_lines.remove(&action_id)
        {
            for mut line in lines.into_iter()
            {
                match line.get_status()
                {
                    Status::Active | Status::Deleted(_) =>
                    {
                        let error_message = &format!("Incorrect status for line changed by action {}!",
                            action_id);
                        return Err(JsValue::from(error_message));
                    },
                    Status::Changed(n) =>
                    {
                        if !line_numbers.contains(&n)
                        {
                            let error_message = &format!("Incorrect property for line changed by action {}!",
                                action_id);
                            return Err(JsValue::from(error_message));
                        }
                        if !line.is_mesh_seed_assigned()
                        {
                            let error_message = &format!("Incorrect mesh seed for line \
                                changed by action {action_id}!");
                            return Err(JsValue::from(error_message));
                        }
                        line.set_status(Status::Active);
                        self.lines.insert(n, line.clone());
                        line.notify_server(NotificationType::Update(false), n, &self.props)?;
                    }
                }
            }
        }
        else
        {
            let error_message = &format!("There are no lines chanded by action id {action_id:?}!");
            return Err(JsValue::from(error_message));
        }

        self.logging();

        Ok(())
    }


    fn check_uniformly_distributed_line_load_on_line_absence(&self, line_number: u32) -> Result<(), JsValue>
    {
        self.check_line_number_existence(line_number)?;
        let line = self.lines.get(&line_number).expect("Line is absent!");
        if line.is_uniformly_distributed_line_load_assigned()
        {
            let error_message = format!("Uniformly distributed line load applied at line with number {} \
                already exists!", line_number);
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    fn check_uniformly_distributed_line_load_on_line_existence(&self, line_number: u32) -> Result<(), JsValue>
    {
        self.check_line_number_existence(line_number)?;
        let line = self.lines.get(&line_number).expect("Line is absent!");
        if !line.is_uniformly_distributed_line_load_assigned()
        {
            let error_message = format!("Uniformly distributed line load applied at line with number {} \
                does not exist!", line_number);
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    fn check_deleted_uniformly_distributed_line_load_on_line_existence(&mut self, action_id: u32, line_number: u32) 
        -> Result<(), JsValue>
    {
        if let Some(lines) = self.changed_lines.get_mut(&action_id)
        {
            if lines.len() != 1
            {
                let error_message = &format!("Incorrect number of lines changed by action {}!",
                    action_id);
                return Err(JsValue::from(error_message));
            }

            let changed_line = &mut lines[0];
            match changed_line.get_status()
            {
                Status::Active | Status::Deleted(_) =>
                {
                    let error_message = &format!("Incorrect status for line changed by action {}!",
                        action_id);
                    return Err(JsValue::from(error_message));
                },
                Status::Changed(n) =>
                {
                    if n != line_number
                    {
                        let error_message = &format!("Incorrect number for line changed by action {}!",
                            action_id);
                        return Err(JsValue::from(error_message));
                    }
                    else
                    {
                        if !changed_line.is_uniformly_distributed_line_load_assigned()
                        {
                            let error_message = &format!("No uniformly distributed line loads deleted \
                                by action {}!", action_id);
                            return Err(JsValue::from(error_message));
                        }
                        return Ok(());
                    }
                }
            }
        }
        else
        {
            let error_message = &format!("No lines deleted by action {}!", action_id);
            return Err(JsValue::from(error_message));
        }
    }


    fn check_uniformly_distributed_line_load_can_be_added(&self, number: u32) -> Result<(), JsValue>
    {
        if let Some(line) = self.lines.get(&number)
        {
            if line.is_local_axis_1_direction_assigned()
            {
                return Ok(());
            }
            else
            {
                let error_message = format!("Uniformly distributed line load could not be added to \
                    line with number {number}!");
                return Err(JsValue::from(error_message));
            }
        }
        else
        {
            let error_message = format!("Line with number {number} does not exist!");
            return Err(JsValue::from(error_message));
        }
    }


    pub fn add_uniformly_distributed_line_load(&mut self, action_id: u32, line_number: u32, qx: FEFloat, 
        qy: FEFloat, qz: FEFloat, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_uniformly_distributed_line_load_on_line_absence(line_number)?;
        self.check_uniformly_distributed_line_load_can_be_added(line_number)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let uid = self.generate_uid();
        let uniformly_distributed_line_load = UniformlyDistributedLineLoad::create(
            qx, qy, qz, uid);

        let line = self.lines.get_mut(&line_number).expect("Line is absent!");
        line.set_uniformly_distributed_line_load(Some(uniformly_distributed_line_load));

        line.notify_server(NotificationType::Update(is_action_id_should_be_increased), line_number, &self.props)?;

        self.logging();

        Ok(())
    }


    pub fn update_uniformly_distributed_line_load(&mut self, action_id: u32, line_number: u32, qx: FEFloat, 
        qy: FEFloat, qz: FEFloat, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_uniformly_distributed_line_load_on_line_existence(line_number)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let line = self.lines.get_mut(&line_number).expect("Line is absent!");
        let uniformly_distributed_line_load = 
            line.get_mut_ref_optional_uniformly_distributed_line_load()
                .as_mut()
                .expect("Uniformly distributed line load is absent!");
        uniformly_distributed_line_load.set_load_components(qx, qy, qz);

        line.notify_server(NotificationType::Update(is_action_id_should_be_increased), line_number, &self.props)?;

        self.logging();

        Ok(())
    }


    pub fn delete_uniformly_distributed_line_load(&mut self, action_id: u32, line_number: u32,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_uniformly_distributed_line_load_on_line_existence(line_number)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let line = self.lines.get_mut(&line_number).expect("Line is absent!");
        let mut changed_line = line.clone();
        changed_line.set_status(Status::Changed(line_number));
        line.set_uniformly_distributed_line_load(None);

        move_objects_to_dst(vec![changed_line], &mut self.changed_lines, action_id);

        line.notify_server(NotificationType::Update(is_action_id_should_be_increased), line_number, &self.props)?;

        self.logging();

        Ok(())
    }


    pub fn restore_uniformly_distributed_line_load(&mut self, action_id: u32, line_number: u32,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_deleted_uniformly_distributed_line_load_on_line_existence(action_id, line_number)?;
        self.check_uniformly_distributed_line_load_on_line_absence(line_number)?;

        let mut line = self.changed_lines.remove(&action_id).expect("Line is absent!").remove(0);
        line.set_status(Status::Active);

        self.lines.insert(line_number, line.clone());

        line.notify_server(NotificationType::Update(is_action_id_should_be_increased), line_number, &self.props)?;

        self.logging();

        Ok(())
    }
}
