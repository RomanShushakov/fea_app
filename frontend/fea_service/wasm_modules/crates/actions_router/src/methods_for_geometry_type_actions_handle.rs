use serde_json::Value;
use wasm_bindgen::prelude::*;

use crate::ActionsRouter;
use crate::action::{Coordinates, Action, ActionType, GeometryActionType};

use crate::types::FEFloat;


impl ActionsRouter
{
    pub(super) fn handle_add_point_message(&mut self, point_data: &Value) -> Result<(), JsValue>
    {
        let action_id = point_data["action_id"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from(
                "Actions router: Add point action: Action id could not be converted to u32!")))?;
        let number = point_data["number"].as_str()
            .ok_or(JsValue::from("Actions router: Add point action: \
                Point number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Add point action: \
                Point number could not be converted to u32!")))?;

        if number > self.props.max_point_number
        {
            return Err(JsValue::from(format!("Actions router: Add point action: Point number could \
                not be greater than {}!", self.props.max_point_number)));
        }

        let x = point_data["x"].as_str()
            .ok_or(JsValue::from("Actions router: Add point action: \
                Point x coordinate could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Add point action: \
                Point x coordinate could not be converted to FEFloat!")))?;
        let y = point_data["y"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Add point action: Point y coordinate could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Add point action: \
                Point y coordinate could not be converted to FEFloat!")))?;
        let z = point_data["z"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Add point action: Point z coordinate could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Add point action: \
                Point z coordinate could not be converted to FEFloat!")))?;
        
        let coordinates = Coordinates::create(x, y, z);
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(GeometryActionType::AddPoint(
            number, coordinates, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_update_point_message(&mut self, point_data: &Value) -> Result<(), JsValue>
    {
        let action_id = point_data["action_id"].to_string()
                .parse::<u32>()
                .or(Err(JsValue::from("Actions router: Update point action: \
                    Action id could not be converted to u32!")))?;
        let number = point_data["number"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Update point action: Point number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Update point action: \
                Point number could not be converted to u32!")))?;
        let old_x_value = point_data["old_point_values"]["x"].to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update point action: \
                Point old x coordinate could not be converted to FEFloat!")))?;
        let old_y_value = point_data["old_point_values"]["y"].to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update point action: \
                Point old y coordinate could not be converted to FEFloat!")))?;
        let old_z_value = point_data["old_point_values"]["z"].to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from(
                "Actions router: Update point action: \
                Point old z coordinate could not be converted to FEFloat!")))?;
        let new_x_value = point_data["new_point_values"]["x"].as_str()
            .ok_or(JsValue::from("Actions router: Update point action: \
                Point new x coordinate could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update point action: \
                Point new x value could not be converted to FEFloat!")))?;
        let new_y_value = point_data["new_point_values"]["y"].as_str()
            .ok_or(JsValue::from("Actions router: Update point action: \
                Point new y coordinate could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update point action: \
                Point new y value could not be converted to FEFloat!")))?;
        let new_z_value = point_data["new_point_values"]["z"].as_str()
            .ok_or(JsValue::from("Actions router: Update point action: \
                Point new z coordinate could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update point action: \
                Point new z value could not be converted to FEFloat!")))?;
        
        let old_coordinates = Coordinates::create(old_x_value,
            old_y_value, old_z_value);
        let new_coordinates = Coordinates::create(new_x_value,
            new_y_value, new_z_value);
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(GeometryActionType::UpdatePoint(
            number, old_coordinates, new_coordinates, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_delete_point_message(&mut self, point_data: &Value) -> Result<(), JsValue>
    {
        let action_id = point_data["action_id"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from( "Actions router: Delete point action: \
                Action id could not be converted to u32!")))?;
        let number = point_data["number"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Delete point action: \
                Point number could not be converted to u32!")))?;
        
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(GeometryActionType::DeletePoint(
            number, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_add_line_message(&mut self, line_data: &Value) -> Result<(), JsValue>
    {
        let action_id = line_data["action_id"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from(
                "Actions router: Add line action: Action id could not be converted to u32!")))?;
        let number = line_data["number"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Add line action: Line number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Add line action: \
                Line number could not be converted to u32!")))?;

        if number > self.props.max_line_number
        {
            return Err(JsValue::from(format!("Actions router: Add line action: Line number could \
                not be greater than {}!", self.props.max_line_number)));
        }

        let point_1_number = line_data["point_1_number"].as_str()
            .ok_or(JsValue::from("Actions router: Add line action: \
                Line start point number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Add line action: \
                Line start point number could not be converted to u32!")))?;
        let point_2_number = line_data["point_2_number"].as_str()
            .ok_or(JsValue::from("Actions router: Add line action: \
                Line end point number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Add line action: \
                Line end point number could not be converted to u32!")))?;
        
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(GeometryActionType::AddLine(
            number, point_1_number, point_2_number, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_update_line_message(&mut self, line_data: &Value) -> Result<(), JsValue>
    {
        let action_id = line_data["action_id"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Update line action: \
                Action id could not be converted to u32!")))?;
        let number = line_data["number"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Update line action: Line number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Update line action: \
                Line number could not be converted to u32!")))?;
        let old_point_1_number = line_data["old_line_values"]["point_1_number"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Update line action: \
                Line old start point number could not be converted to u32!")))?;
        let old_point_2_number = line_data["old_line_values"]["point_2_number"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Update line action: \
                Line old end point number could not be converted to u32!")))?;
        let new_point_1_number = line_data["new_line_values"]["point_1_number"].as_str()
            .ok_or(JsValue::from("Actions router: Update line action: \
                Line new start point number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Update line action: \
                Line new start point number could not be converted to u32!")))?;
        let new_point_2_number = line_data["new_line_values"]["point_2_number"].as_str()
            .ok_or(JsValue::from("Actions router: Update line action: \
                Line new end point number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Update line action: \
                Line new end point number could not be converted to u32!")))?;
        
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(GeometryActionType::UpdateLine(
            number, old_point_1_number, old_point_2_number, new_point_1_number,
            new_point_2_number, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_delete_line_message(&mut self, line_data: &Value) -> Result<(), JsValue>
    {
        let action_id = line_data["action_id"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Delete line action: \
                Action id could not be converted to u32!")))?;
        let number = line_data["number"].as_str()
            .ok_or(JsValue::from("Actions router: Delete line action: \
                Line number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Delete line action: \
                Line number could not be converted to u32!")))?;
        
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(GeometryActionType::DeleteLine(
            number, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_add_surface_message(&mut self, surface_data: &Value) -> Result<(), JsValue>
    {
        let action_id = surface_data["action_id"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from(
                "Actions router: Add surface action: Action id could not be converted to u32!")))?;
        let number = surface_data["number"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Add surface action: Surface number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Add surface action: \
                Surface number could not be converted to u32!")))?;

        if number > self.props.max_surface_number
        {
            return Err(JsValue::from(format!("Actions router: Add surface action: Surface number could \
                not be greater than {}!", self.props.max_surface_number)));
        }

        let point_1_number = surface_data["point_1_number"].as_str()
            .ok_or(JsValue::from("Actions router: Add surface action: \
                Surface point 1 number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Add Surface action: \
                Surface point 1 number could not be converted to u32!")))?;
        let point_2_number = surface_data["point_2_number"].as_str()
            .ok_or(JsValue::from("Actions router: Add surface action: \
                Surface point 2 number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Add Surface action: \
                Surface point 2 number could not be converted to u32!")))?;
        let point_3_number = surface_data["point_3_number"].as_str()
            .ok_or(JsValue::from("Actions router: Add surface action: \
                Surface point 3 number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Add Surface action: \
                Surface point 3 number could not be converted to u32!")))?;
        let point_4_number = surface_data["point_4_number"].as_str()
            .ok_or(JsValue::from("Actions router: Add surface action: \
                Surface point 4 number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Add Surface action: \
                Surface point 4 number could not be converted to u32!")))?;
        
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(GeometryActionType::AddSurface(
            number, point_1_number, point_2_number, point_3_number, point_4_number, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_update_surface_message(&mut self, surface_data: &Value) -> Result<(), JsValue>
    {
        let action_id = surface_data["action_id"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Update surface action: \
                Action id could not be converted to u32!")))?;
        let number = surface_data["number"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Update surface action: Surface number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Update Surface action: \
                Surface number could not be converted to u32!")))?;

        let old_point_1_number = surface_data["old_surface_values"]["point_1_number"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Update surface action: \
                Surface old point 1 number could not be converted to u32!")))?;
        let old_point_2_number = surface_data["old_surface_values"]["point_2_number"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Update surface action: \
                Surface old point 2 number could not be converted to u32!")))?;
        let old_point_3_number = surface_data["old_surface_values"]["point_3_number"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Update surface action: \
                Surface old point 3 number could not be converted to u32!")))?;
        let old_point_4_number = surface_data["old_surface_values"]["point_4_number"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Update surface action: \
                Surface old point 4 number could not be converted to u32!")))?;

        let new_point_1_number = surface_data["new_surface_values"]["point_1_number"].as_str()
            .ok_or(JsValue::from("Actions router: Update surface action: \
                Surface new point 1 number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Update surface action: \
                Surface new point 1 number could not be converted to u32!")))?;
        let new_point_2_number = surface_data["new_surface_values"]["point_2_number"].as_str()
            .ok_or(JsValue::from("Actions router: Update surface action: \
                Surface new point 2 number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Update surface action: \
                Surface new point 2 number could not be converted to u32!")))?;
        let new_point_3_number = surface_data["new_surface_values"]["point_3_number"].as_str()
            .ok_or(JsValue::from("Actions router: Update surface action: \
                Surface new point 3 number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Update surface action: \
                Surface new point 3 number could not be converted to u32!")))?;
        let new_point_4_number = surface_data["new_surface_values"]["point_4_number"].as_str()
            .ok_or(JsValue::from("Actions router: Update surface action: \
                Surface new point 4 number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Update surface action: \
                Surface new point 4 number could not be converted to u32!")))?;
        
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(GeometryActionType::UpdateSurface(
            number, old_point_1_number, old_point_2_number, old_point_3_number, old_point_4_number, 
            new_point_1_number, new_point_2_number, new_point_3_number, new_point_4_number,
            is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_rotate_surface_vertices_clockwise_message(&mut self, surface_data: &Value) -> Result<(), JsValue>
    {
        let action_id = surface_data["action_id"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Rotate surface vertices clockwise action: \
                Action id could not be converted to u32!")))?;
        let number = surface_data["number"].as_str()
            .ok_or(JsValue::from("Actions router: Rotate surface vertices clockwise action: \
                Surface number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Rotate surface vertices clockwise action: \
                Surface number could not be converted to u32!")))?;
        
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(GeometryActionType::RotateSurfaceVerticesClockwise(
            number, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_rotate_surface_vertices_counter_clockwise_message(&mut self, surface_data: &Value) 
        -> Result<(), JsValue>
    {
        let action_id = surface_data["action_id"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Rotate surface vertices counter clockwise action: \
                Action id could not be converted to u32!")))?;
        let number = surface_data["number"].as_str()
            .ok_or(JsValue::from("Actions router: Rotate surface vertices counter clockwise action: \
                Surface number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Rotate surface vertices counter clockwise action: \
                Surface number could not be converted to u32!")))?;
        
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(GeometryActionType::RotateSurfaceVerticesCounterClockwise(
            number, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_flip_surface_normal_axis_message(&mut self, surface_data: &Value) -> Result<(), JsValue>
    {
        let action_id = surface_data["action_id"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Flip surface normal axis action: \
                Action id could not be converted to u32!")))?;
        let number = surface_data["number"].as_str()
            .ok_or(JsValue::from("Actions router: Flip surface normal axis action: \
                Surface number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Flip surface normal axis action: \
                Surface number could not be converted to u32!")))?;
        
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(GeometryActionType::FlipSurfaceNormalAxis(
            number, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_delete_surface_message(&mut self, surface_data: &Value) -> Result<(), JsValue>
    {
        let action_id = surface_data["action_id"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Delete surface action: \
                Action id could not be converted to u32!")))?;
        let number = surface_data["number"].as_str()
            .ok_or(JsValue::from("Actions router: Delete surface action: \
                Surface number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Delete surface action: \
                Surface number could not be converted to u32!")))?;
        
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(GeometryActionType::DeleteSurface(
            number, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }
}
