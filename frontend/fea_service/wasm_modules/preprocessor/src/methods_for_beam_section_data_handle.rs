use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

use crate::enums::{NotificationType, Status, CrossSection, ParentKey};
use crate::types::FEFloat;
use crate::structs::BeamSection as NewBeamSection;
use crate::traits::{StatusTrait, ServerNotificationTrait};
use crate::Preprocessor;


#[wasm_bindgen]
impl Preprocessor
{
    fn check_beam_section_name_absence(&self, name: &str) -> Result<(), JsValue>
    {
        if self.beam_sections.contains_key(name)
        {
            let error_message = format!("Beam section with name {name} already exists!");
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    pub(super) fn check_beam_section_name_existence(&self, name: &str) -> Result<(), JsValue>
    {
        if !self.beam_sections.contains_key(name)
        {
            let error_message = format!("Beam section with name {name} does not exist!");
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    fn check_beam_section_data_absence(&self, area: FEFloat, i11: FEFloat, i22: FEFloat, i12: FEFloat, it: FEFloat,
        shear_factor: FEFloat) -> Result<(), JsValue>
    {
        if self.beam_sections.values().position(|beam_section| 
            beam_section.is_data_same(area, i11, i22, i12, it, shear_factor)).is_some()
        {
            let error_message = &format!("Beam section with area {area}, I11 {i11}, I22 {i22}, I12 {i12},
                It {it} and Shear factor {shear_factor} already exists!");
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    fn check_deleted_beam_section_existence(&mut self, action_id: u32, name: &str) -> Result<(), JsValue>
    {
        if let Some(beam_section) = self.deleted_beam_sections.get_mut(&action_id)
        {
            match beam_section.get_status()
            {
                Status::Active | Status::Changed(_) =>
                {
                    let error_message = &format!("Incorrect status for beam section deleted by action {}!",
                        action_id);
                    return Err(JsValue::from(error_message));
                },
                Status::Deleted(n) =>
                {
                    if n != name
                    {
                        let error_message = &format!("Incorrect name for beam section deleted by action {}!",
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
            let error_message = &format!("No beam sections deleted by action {}!", action_id);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn add_beam_section(&mut self, action_id: u32, name: &str, area: FEFloat,
        i11: FEFloat, i22: FEFloat, i12: FEFloat, it: FEFloat, shear_factor: FEFloat,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_beam_section_name_absence(name)?;
        self.check_beam_section_data_absence(area, i11, i22, i12, it, shear_factor)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let beam_section = NewBeamSection::create(area, i11, i22, i12, it, shear_factor)?;

        self.beam_sections.insert(name.to_string(), beam_section.clone());
        beam_section.notify_server(NotificationType::Add(is_action_id_should_be_increased), name.to_string(),
            &self.props)?;

        self.logging();

        Ok(())
    }


    pub fn update_beam_section(&mut self, action_id: u32, name: &str, area: FEFloat,
        i11: FEFloat, i22: FEFloat, i12: FEFloat, it: FEFloat, shear_factor: FEFloat,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_beam_section_name_existence(name)?;
        self.check_beam_section_data_absence(area, i11, i22, i12, it, shear_factor)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let beam_section = self.beam_sections.get_mut(name).expect("Beam section is absent!");
        beam_section.set_data(area, i11, i22, i12, it, shear_factor)?;
        beam_section.notify_server(NotificationType::Update(is_action_id_should_be_increased), name.to_string(), 
            &self.props)?;

        self.logging();

        Ok(())
    }


    pub fn delete_beam_section(&mut self, action_id: u32, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_beam_section_name_existence(name)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let cross_section = CrossSection::create_beam(name);
        let parent_key = ParentKey::CrossSection(cross_section);
        self.delete_properties_with_parent_key(parent_key, action_id)?;

        let mut beam_section = self.beam_sections.remove(name).expect("Beam section is absent!");
        beam_section.set_status(Status::Deleted(name.to_string()));

        self.deleted_beam_sections.insert(action_id, beam_section.clone());
    
        beam_section.notify_server(NotificationType::Delete(is_action_id_should_be_increased), name.to_string(), 
            &self.props)?;

        self.logging();

        Ok(())
    }


    pub fn restore_beam_section(&mut self, action_id: u32, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_deleted_beam_section_existence(action_id, name)?;
        self.check_beam_section_name_absence(name)?;

        let mut beam_section = self.deleted_beam_sections.remove(&action_id)
            .expect("Beam section is absent!");
        beam_section.set_status(Status::Active);

        self.beam_sections.insert(name.to_string(), beam_section.clone());

        beam_section.notify_server(NotificationType::Add(is_action_id_should_be_increased), name.to_string(), 
            &self.props)?;

        let cross_section = CrossSection::create_beam(name);
        let parent_key = ParentKey::CrossSection(cross_section);
        self.restore_properties_with_parent_key(parent_key, action_id)?;

        self.logging();

        Ok(())
    }
}
