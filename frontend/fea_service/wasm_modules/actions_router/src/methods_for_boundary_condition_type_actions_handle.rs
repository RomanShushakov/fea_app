use serde_json::Value;
use wasm_bindgen::prelude::*;

use crate::ActionsRouter;
use crate::action::{Action, ActionType, BoundaryConditionsActionType, PointBoundaryCondition};

use crate::types::FEFloat;


impl ActionsRouter
{
    pub(super) fn handle_add_point_boundary_condition_message(&mut self, boundary_condition_data: &Value)
        -> Result<(), JsValue>
    {
        let action_id = boundary_condition_data["action_id"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from(
                "Actions router: Add point boundary condition action: Action id could not be converted \
                to u32!")))?;
        let point_number = boundary_condition_data["point_number"].as_str()
            .ok_or(JsValue::from("Actions router: Add point boundary condition action: \
                Point number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Add point boundary condition action: \
                Point number could not be converted to u32!")))?;

        if point_number > self.props.max_point_number as u32
        {
            return Err(JsValue::from(format!("Actions router: Add point boundary condition action: Point \
                number could not be greater than {}!", self.props.max_point_number)));
        }

        let optional_ux =
            {
                if boundary_condition_data["optional_ux"].is_null() || boundary_condition_data["optional_ux"] == "''"
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["optional_ux"].as_str()
                        .ok_or(JsValue::from("Actions router: Add point boundary condition \
                            action: Displacement x component could not be extracted!"))?
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Add point boundary condition \
                            action: Displacement x component could not be converted to \
                            FEFloat!")))?)
                }
            };
        let optional_uy =
            {
                if boundary_condition_data["optional_uy"].is_null() || boundary_condition_data["optional_uy"] == "''"
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["optional_uy"].as_str()
                        .ok_or(JsValue::from(
                            "Actions router: Add boundary point condition action: Displacement y \
                            component could not be extracted!"))?
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Add boundary point condition \
                            action: Displacement y component could not be converted to \
                            FEFloat!")))?)
                }
            };
        let optional_uz =
            {
                if boundary_condition_data["optional_uz"].is_null() || boundary_condition_data["optional_uz"] == "''"
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["optional_uz"].as_str()
                        .ok_or(JsValue::from(
                            "Actions router: Add point boundary condition action: Displacement z \
                            component could not be extracted!"))?
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Add point boundary condition \
                            action: Displacement z component could not be converted to \
                            FEFloat!")))?)
                }
            };
        let optional_rx =
            {
                if boundary_condition_data["optional_rx"].is_null() || boundary_condition_data["optional_rx"] == "''"
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["optional_rx"].as_str()
                        .ok_or(JsValue::from(
                            "Actions router: Add point boundary condition action: Rotation x \
                            component could not be extracted!"))?
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Add point boundary condition \
                            action: Rotation x component could not be converted to \
                            FEFloat!")))?)
                }
            };
        let optional_ry =
            {
                if boundary_condition_data["optional_ry"].is_null() || boundary_condition_data["optional_ry"] == "''"
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["optional_ry"].as_str()
                        .ok_or(JsValue::from(
                            "Actions router: Add point boundary condition action: Rotation y \
                            component could not be extracted!"))?
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Add point boundary condition \
                            action: Rotation y component could not be converted to \
                            FEFloat!")))?)
                }
            };
        let optional_rz =
            {
                if boundary_condition_data["optional_rz"].is_null() || boundary_condition_data["optional_rz"] == "''"
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["optional_rz"].as_str()
                        .ok_or(JsValue::from(
                            "Actions router: Add point boundary condition action: Rotation z \
                            component could not be extracted!"))?
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Add point boundary condition \
                            action: Rotation z component could not be converted to \
                            FEFloat!")))?)
                }
            };

        
        let boundary_condition = PointBoundaryCondition::create(optional_ux,
            optional_uy, optional_uz, optional_rx, optional_ry, optional_rz);
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(
            BoundaryConditionsActionType::AddPointBoundaryCondition(
                point_number, boundary_condition,
                is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_update_point_boundary_condition_message(&mut self,
        boundary_condition_data: &Value) -> Result<(), JsValue>
    {
        let action_id = boundary_condition_data["action_id"].to_string()
                .parse::<u32>()
                .or(Err(JsValue::from("Actions router: Update point boundary condition action: \
                    Action id could not be converted to u32!")))?;
        let point_number = boundary_condition_data["point_number"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Update point boundary condition action: Point number could not be \
                extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Update point boundary condition action: \
                Point number could not be converted to u32!")))?;

        let old_optional_ux_value =
            {
                if boundary_condition_data["old_boundary_condition_values"]["optional_ux"].is_null() ||
                    boundary_condition_data["old_boundary_condition_values"]["optional_ux"] == "''"
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["old_boundary_condition_values"]["optional_ux"]
                        .to_string()
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Update point boundary condition \
                            action: Old displacement x component value could not be converted to \
                            FEFloat!")))?)
                }
            };
        let old_optional_uy_value =
            {
                if boundary_condition_data["old_boundary_condition_values"]["optional_uy"].is_null() ||
                    boundary_condition_data["old_boundary_condition_values"]["optional_uy"] == "''"
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["old_boundary_condition_values"]["optional_uy"]
                        .to_string()
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Update point boundary condition \
                            action: Old displacement y component value could not be converted to \
                            FEFloat!")))?)
                }
            };
        let old_optional_uz_value =
            {
                if boundary_condition_data["old_boundary_condition_values"]["optional_uz"].is_null() ||
                    boundary_condition_data["old_boundary_condition_values"]["optional_uz"] == "''"
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["old_boundary_condition_values"]["optional_uz"]
                        .to_string()
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Update point boundary condition \
                            action: Old displacement z component value could not be converted to \
                            FEFloat!")))?)
                }
            };
        let old_optional_rx_value =
            {
                if boundary_condition_data["old_boundary_condition_values"]["optional_rx"].is_null() ||
                    boundary_condition_data["old_boundary_condition_values"]["optional_rx"] == "''"
                
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["old_boundary_condition_values"]["optional_rx"]
                        .to_string()
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Update point boundary condition \
                            action: Old rotation x component value could not be converted to \
                            FEFloat!")))?)
                }
            };
        let old_optional_ry_value =
            {
                if boundary_condition_data["old_boundary_condition_values"]["optional_ry"].is_null() ||
                    boundary_condition_data["old_boundary_condition_values"]["optional_ry"] == "''"
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["old_boundary_condition_values"]["optional_ry"]
                        .to_string()
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Update point boundary condition \
                            action: Old rotation y component value could not be converted to \
                            FEFloat!")))?)
                }
            };
        let old_optional_rz_value =
            {
                if boundary_condition_data["old_boundary_condition_values"]["optional_rz"].is_null() ||
                    boundary_condition_data["old_boundary_condition_values"]["optional_rz"] == "''"
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["old_boundary_condition_values"]["optional_rz"]
                        .to_string()
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Update point boundary condition \
                            action: Old rotation z component value could not be converted to \
                            FEFloat!")))?)
                }
            };

        let new_optional_ux_value =
            {
                if boundary_condition_data["new_boundary_condition_values"]["optional_ux"].is_null() ||
                    boundary_condition_data["new_boundary_condition_values"]["optional_ux"] == "''"
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["new_boundary_condition_values"]["optional_ux"]
                        .as_str()
                        .ok_or(JsValue::from("Actions router: Update point boundary condition \
                            action: New displacement x component value could not be extracted!"))?
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Update point boundary condition \
                            action: New displacement x component value could not be converted to \
                            FEFloat!")))?)
                }
            };
        let new_optional_uy_value =
            {
                if boundary_condition_data["new_boundary_condition_values"]["optional_uy"].is_null() ||
                    boundary_condition_data["new_boundary_condition_values"]["optional_uy"] == "''"
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["new_boundary_condition_values"]["optional_uy"]
                        .as_str()
                        .ok_or(JsValue::from("Actions router: Update point boundary condition \
                            action: New displacement y component value could not be extracted!"))?
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Update point boundary condition \
                            action: New displacement y component value could not be converted to \
                            FEFloat!")))?)
                }
            };
        let new_optional_uz_value =
            {
                if boundary_condition_data["new_boundary_condition_values"]["optional_uz"].is_null() ||
                    boundary_condition_data["new_boundary_condition_values"]["optional_uz"] == "''"
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["new_boundary_condition_values"]["optional_uz"]
                        .as_str()
                        .ok_or(JsValue::from("Actions router: Update point boundary condition \
                            action: New displacement z component value could not be extracted!"))?
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Update point boundary condition \
                            action: New displacement z component value could not be converted to \
                            FEFloat!")))?)
                }
            };
        let new_optional_rx_value =
            {
                if boundary_condition_data["new_boundary_condition_values"]["optional_rx"].is_null() ||
                    boundary_condition_data["new_boundary_condition_values"]["optional_rx"] == "''"
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["new_boundary_condition_values"]["optional_rx"]
                        .as_str()
                        .ok_or(JsValue::from("Actions router: Update point boundary condition \
                            action: New rotation x component value could not be extracted!"))?
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Update point boundary condition \
                            action: New rotation x component value could not be converted to \
                            FEFloat!")))?)
                }
            };
        let new_optional_ry_value =
            {
                if boundary_condition_data["new_boundary_condition_values"]["optional_ry"].is_null() ||
                    boundary_condition_data["new_boundary_condition_values"]["optional_ry"] == "''"
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["new_boundary_condition_values"]["optional_ry"]
                        .as_str()
                        .ok_or(JsValue::from("Actions router: Update point boundary condition \
                            action: New rotation y component value could not be extracted!"))?
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Update point boundary condition \
                            action: New rotation y component value could not be converted to \
                            FEFloat!")))?)
                }
            };
        let new_optional_rz_value =
            {
                if boundary_condition_data["new_boundary_condition_values"]["optional_rz"].is_null() ||
                    boundary_condition_data["new_boundary_condition_values"]["optional_rz"] == "''"
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["new_boundary_condition_values"]["optional_rz"]
                        .as_str()
                        .ok_or(JsValue::from("Actions router: Update point boundary condition \
                            action: New rotation z component value could not be extracted!"))?
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Update point boundary condition \
                            action: New rotation z component value could not be converted to \
                            FEFloat!")))?)
                }
            };
        
        let old_boundary_condition = PointBoundaryCondition::create(
            old_optional_ux_value, old_optional_uy_value,
            old_optional_uz_value, old_optional_rx_value,
            old_optional_ry_value, old_optional_rz_value);
        let new_boundary_condition = PointBoundaryCondition::create(
            new_optional_ux_value, new_optional_uy_value,
            new_optional_uz_value, new_optional_rx_value,
            new_optional_ry_value, new_optional_rz_value);
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(
            BoundaryConditionsActionType::UpdatePointBoundaryCondition(point_number,
            old_boundary_condition, new_boundary_condition,
            is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_delete_point_boundary_condition_message(&mut self,
        boundary_condition_data: &Value) -> Result<(), JsValue>
    {
        let action_id = boundary_condition_data["action_id"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from( "Actions router: Delete point boundary condition action: \
                Action id could not be converted to u32!")))?;
        let point_number = boundary_condition_data["point_number"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Delete point boundary condition action: \
                Point number could not be converted to u32!")))?;
        
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(
            BoundaryConditionsActionType::DeletePointBoundaryCondition(
                point_number, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }
}
