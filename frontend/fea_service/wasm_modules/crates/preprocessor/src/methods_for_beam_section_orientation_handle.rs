use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

use crate::enums::{NotificationType, RelativeKey};
use crate::types::FEFloat;
use crate::structs::LocalAxis1Direction;
use crate::traits::ServerNotificationTrait;
use crate::Preprocessor;
use crate::functions::
{
    move_objects_with_relative_keys_to_changed, move_objects_with_relative_keys_to_active,
};


#[wasm_bindgen]
impl Preprocessor
{
    fn check_beam_section_local_axis_1_direction_absence(&self, local_axis_1_direction: &LocalAxis1Direction) 
        -> Result<(), JsValue>
    {
        if self.beam_sections_local_axis_1_directions.iter().position(|direction| 
            direction == local_axis_1_direction).is_some()
        {
            let error_message = format!("{} Beam section local axis 1 direction {local_axis_1_direction:?} \
                already exists!", &self.props.local_axis_1_direction_input_info_message_header);
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    pub(super) fn check_beam_section_local_axis_1_direction_existence(&self, 
        local_axis_1_direction: &LocalAxis1Direction) -> Result<(), JsValue>
    {
        if self.beam_sections_local_axis_1_directions.iter().position(|direction| 
            direction == local_axis_1_direction).is_none()
        {
            let error_message = format!("{} Beam section local axis 1 direction {local_axis_1_direction:?} \
                does not exist!", &self.props.local_axis_1_direction_input_info_message_header);
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    fn check_deleted_beam_section_local_axis_1_direction_existence(&mut self, action_id: u32, 
        local_axis_1_direction: &LocalAxis1Direction) -> Result<(), JsValue>
    {
        if let Some(loc_axis_1_direction) = 
            self.deleted_beam_sections_local_axis_1_directions.get_mut(&action_id)
        {
            if loc_axis_1_direction != local_axis_1_direction
            {
                let error_message = &format!("Incorrect beam section local axis 1 direction \
                    deleted by action {}!", action_id);
                Err(JsValue::from(error_message))
            }
            else
            {
                Ok(())
            }
        }
        else
        {
            let error_message = &format!("No beam sections local axis 1 directions deleted by action {}!", 
                action_id);
            Err(JsValue::from(error_message))
        }
    }


    pub fn add_beam_section_local_axis_1_direction(&mut self, action_id: u32, components: &[FEFloat], 
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        let local_axis_1_direction = LocalAxis1Direction::create(components, &self.props)?;
        self.check_beam_section_local_axis_1_direction_absence(&local_axis_1_direction)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        self.beam_sections_local_axis_1_directions.push(local_axis_1_direction.clone());
        local_axis_1_direction.notify_server(NotificationType::Add(is_action_id_should_be_increased), 0, 
            &self.props)?;

        self.logging();

        Ok(())
    }


    pub fn delete_beam_section_local_axis_1_direction(&mut self, action_id: u32, components: &[FEFloat], 
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        let local_axis_1_direction = LocalAxis1Direction::create(components, &self.props)?;
        self.check_beam_section_local_axis_1_direction_existence(&local_axis_1_direction)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        move_objects_with_relative_keys_to_changed(&mut self.lines, &mut self.changed_lines,
            &[RelativeKey::LocalAxis1Direction(local_axis_1_direction.clone())], action_id, 
            &self.props)?;

        let position = self.beam_sections_local_axis_1_directions
            .iter()
            .position(|direction| *direction == local_axis_1_direction)
            .expect("Beam section local axis 1 direction is absent!");

        let _ = self.beam_sections_local_axis_1_directions.remove(position);

        self.deleted_beam_sections_local_axis_1_directions.insert(action_id, local_axis_1_direction.clone());
    
        local_axis_1_direction.notify_server(NotificationType::Delete(is_action_id_should_be_increased), 0, 
            &self.props)?;

        self.logging();

        Ok(())
    }


    pub fn restore_beam_section_local_axis_1_direction(&mut self, action_id: u32, components: &[FEFloat], 
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        let local_axis_1_direction = LocalAxis1Direction::create(components, &self.props)?;
        self.check_deleted_beam_section_local_axis_1_direction_existence(action_id, &local_axis_1_direction)?;
        self.check_beam_section_local_axis_1_direction_absence(&local_axis_1_direction)?;

        let _ = self.deleted_beam_sections_local_axis_1_directions.remove(&action_id)
            .expect("Beam section local axis 1 direction is absent!");

        self.beam_sections_local_axis_1_directions.push(local_axis_1_direction.clone());

        local_axis_1_direction.notify_server(NotificationType::Add(is_action_id_should_be_increased), 0, 
            &self.props)?;

        move_objects_with_relative_keys_to_active(&mut self.changed_lines, &mut self.lines,
            &[RelativeKey::LocalAxis1Direction(local_axis_1_direction)], action_id, &self.props)?;

        self.logging();

        Ok(())
    }
}
