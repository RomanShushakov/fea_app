use serde_json::Value;
use wasm_bindgen::prelude::*;

use crate::ActionsRouter;
use crate::action::{Action, ActionType, MeshSeedActionType};


impl ActionsRouter
{
    pub(super) fn handle_update_global_mesh_seed_message(&mut self, global_mesh_seed_data: &Value) -> Result<(), JsValue>
    {
        let action_id = global_mesh_seed_data["action_id"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Update global mesh seed action: \
                Action id could not be converted to u32!")))?;
        let old_global_mesh_seed_value = global_mesh_seed_data["old_global_mesh_seed_value"].to_string()
            .parse::<u8>()
            .or(Err(JsValue::from("Actions router: Update global mesh seed action: \
                Old global mesh seed value could not be converted to u32!")))?;
        let new_global_mesh_seed_value = global_mesh_seed_data["new_global_mesh_seed_value"].to_string()
            .parse::<u8>()
            .or(Err(JsValue::from("Actions router: Update global mesh seed action: \
                New global mesh seed value could not be converted to u32!")))?;

        
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(MeshSeedActionType::UpdateGlobalMeshSeed(
            old_global_mesh_seed_value, new_global_mesh_seed_value, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_update_lines_mesh_seed_message(&mut self, lines_mesh_seed_data: &Value) -> Result<(), JsValue>
    {
        let action_id = lines_mesh_seed_data["action_id"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Update lines mesh seed action: \
                Action id could not be converted to u32!")))?;
        let lines_mesh_seed_value = lines_mesh_seed_data["lines_mesh_seed_value"].to_string()
            .parse::<u8>()
            .or(Err(JsValue::from("Actions router: Update lines mesh seed action: \
                Lines mesh seed value could not be converted to u32!")))?;
        let line_numbers = serde_json::from_value::<Vec<u32>>(
            lines_mesh_seed_data["line_numbers"].clone())
            .or(Err(JsValue::from("Actions router: Update lines mesh seed action: \
                Line numbers could not be converted to u32!")))?;
        
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(MeshSeedActionType::UpdateLinesMeshSeed(
            lines_mesh_seed_value, line_numbers, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_update_surfaces_mesh_seed_message(&mut self, surfaces_mesh_seed_data: &Value) -> Result<(), JsValue>
    {
        let action_id = surfaces_mesh_seed_data["action_id"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Update surfaces mesh seed action: \
                Action id could not be converted to u32!")))?;
        let edges_1_3_mesh_seed_value = surfaces_mesh_seed_data["surfaces_edges_1_3_mesh_seed_value"].to_string()
            .parse::<u8>()
            .or(Err(JsValue::from("Actions router: Update surfaces mesh seed action: \
                Edges 1 and 3 mesh seed value could not be converted to u32!")))?;
        let edges_2_4_mesh_seed_value = surfaces_mesh_seed_data["surfaces_edges_2_4_mesh_seed_value"].to_string()
            .parse::<u8>()
            .or(Err(JsValue::from("Actions router: Update surfaces mesh seed action: \
                Edges 2 and 4 mesh seed value could not be converted to u32!")))?;
        let surface_numbers = serde_json::from_value::<Vec<u32>>(
            surfaces_mesh_seed_data["surface_numbers"].clone())
            .or(Err(JsValue::from("Actions router: Update surfaces mesh seed action: \
                Surface numbers could not be converted to u32!")))?;
        
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(MeshSeedActionType::UpdateSurfacesMeshSeed(
            edges_1_3_mesh_seed_value, edges_2_4_mesh_seed_value, surface_numbers, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }
}
