use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

use crate::enums::{NotificationType, Status, CrossSection, ParentKey};
use crate::types::FEFloat;
use crate::structs::PlateSection;
use crate::traits::{StatusTrait, ServerNotificationTrait};
use crate::Preprocessor;


#[wasm_bindgen]
impl Preprocessor
{
    fn check_plate_section_name_absence(&self, name: &str) -> Result<(), JsValue>
    {
        if self.plate_sections.contains_key(name)
        {
            let error_message = format!("Plate section with name {name} already exists!");
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    pub(super) fn check_plate_section_name_existence(&self, name: &str) -> Result<(), JsValue>
    {
        if !self.plate_sections.contains_key(name)
        {
            let error_message = format!("Plate section with name {name} does not exist!");
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    fn check_plate_section_data_absence(&self, thickness: FEFloat, shear_factor: FEFloat) -> Result<(), JsValue>
    {
        if self.plate_sections.values().position(|plate_section| 
            plate_section.is_data_same(thickness, shear_factor)).is_some()
        {
            let error_message = &format!("Plate section with thickness {thickness} and 
                Shear factor {shear_factor} already exists!");
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    fn check_deleted_plate_section_existence(&mut self, action_id: u32, name: &str) -> Result<(), JsValue>
    {
        if let Some(plate_section) = self.deleted_plate_sections.get_mut(&action_id)
        {
            match plate_section.get_status()
            {
                Status::Active | Status::Changed(_) =>
                {
                    let error_message = &format!("Incorrect status for plate section deleted by action {}!",
                        action_id);
                    return Err(JsValue::from(error_message));
                },
                Status::Deleted(n) =>
                {
                    if n != name
                    {
                        let error_message = &format!("Incorrect name for plate section deleted by action {}!",
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
            let error_message = &format!("No plate sections deleted by action {}!", action_id);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn add_plate_section(&mut self, action_id: u32, name: &str, thickness: FEFloat, shear_factor: FEFloat,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_plate_section_name_absence(name)?;
        self.check_plate_section_data_absence(thickness, shear_factor)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let plate_section = PlateSection::create(thickness, shear_factor)?;

        self.plate_sections.insert(name.to_string(), plate_section.clone());
        plate_section.notify_server(NotificationType::Add(is_action_id_should_be_increased), name.to_string(),
            &self.props)?;

        self.logging();

        Ok(())
    }


    pub fn update_plate_section(&mut self, action_id: u32, name: &str, thickness: FEFloat, shear_factor: FEFloat,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_plate_section_name_existence(name)?;
        self.check_plate_section_data_absence(thickness, shear_factor)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let plate_section = self.plate_sections.get_mut(name).expect("Plate section is absent!");
        plate_section.set_data(thickness, shear_factor)?;
        plate_section.notify_server(NotificationType::Update(is_action_id_should_be_increased), name.to_string(), 
            &self.props)?;

        self.logging();

        Ok(())
    }


    pub fn delete_plate_section(&mut self, action_id: u32, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_plate_section_name_existence(name)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let cross_section = CrossSection::create_plate(name);
        let parent_key = ParentKey::CrossSection(cross_section);
        self.delete_properties_with_parent_key(parent_key, action_id)?;

        let mut plate_section = self.plate_sections.remove(name).expect("Plate section is absent!");
        plate_section.set_status(Status::Deleted(name.to_string()));

        self.deleted_plate_sections.insert(action_id, plate_section.clone());
    
        plate_section.notify_server(NotificationType::Delete(is_action_id_should_be_increased), name.to_string(), 
            &self.props)?;

        self.logging();

        Ok(())
    }


    pub fn restore_plate_section(&mut self, action_id: u32, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_deleted_plate_section_existence(action_id, name)?;
        self.check_plate_section_name_absence(name)?;

        let mut plate_section = self.deleted_plate_sections.remove(&action_id)
            .expect("Plate section is absent!");
        plate_section.set_status(Status::Active);

        self.plate_sections.insert(name.to_string(), plate_section.clone());

        plate_section.notify_server(NotificationType::Add(is_action_id_should_be_increased), name.to_string(), 
            &self.props)?;

        let cross_section = CrossSection::create_plate(name);
        let parent_key = ParentKey::CrossSection(cross_section);
        self.restore_properties_with_parent_key(parent_key, action_id)?;

        self.logging();

        Ok(())
    }
}
