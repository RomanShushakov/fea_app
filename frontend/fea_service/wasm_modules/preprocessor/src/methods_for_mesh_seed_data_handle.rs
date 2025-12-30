use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use serde_json::json;

use crate::traits::{ServerNotificationTrait, MeshSeedTrait};
use crate::functions::dispatch_custom_event;
use crate::enums::{MeshSeed, NotificationType};

use crate::Preprocessor;


#[wasm_bindgen]
impl Preprocessor
{
    pub fn update_global_mesh_seed_value(&mut self, action_id: u32, global_mesh_seed_value: u8,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_all_deleted_objects_by_action_id(action_id);

        self.global_mesh_seed_value = global_mesh_seed_value;

        for (line_number, line) in self.lines.iter_mut()
        {
            if line.is_mesh_seed_global()
            {
                line.set_mesh_seed(Some(MeshSeed::Global(global_mesh_seed_value)));
                line.notify_server(NotificationType::Update(false), *line_number, &self.props)?;
            }
        }

        for (surface_number, surface) in self.surfaces.iter_mut()
        {
            if surface.is_mesh_seed_global()
            {
                surface.set_mesh_seed(Some(MeshSeed::Global(global_mesh_seed_value)));
                surface.notify_server(NotificationType::Update(false), *surface_number, &self.props)?;
            }
        }

        let detail = json!({ "global_mesh_seed_data": 
            { 
                "global_mesh_seed_value": global_mesh_seed_value 
            },
            "is_action_id_should_be_increased": is_action_id_should_be_increased 
        });
        dispatch_custom_event(detail, &self.props.update_global_mesh_seed_event_name,
            &self.props.event_target)?;

        self.logging();

        Ok(())
    }
}
