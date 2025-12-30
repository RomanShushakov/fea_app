use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

use crate::enums::{NotificationType, Status, ParentKey, CrossSection};
use crate::types::FEFloat;
use crate::structs::TrussSection as NewTrussSection;
use crate::traits::{StatusTrait, ServerNotificationTrait};
use crate::Preprocessor;


#[wasm_bindgen]
impl Preprocessor
{
    fn check_truss_section_name_absence(&self, name: &str) -> Result<(), JsValue>
    {
        if self.truss_sections.contains_key(name)
        {
            let error_message = format!("Truss section with name {name} already exists!");
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    pub(super) fn check_truss_section_name_existence(&self, name: &str) -> Result<(), JsValue>
    {
        if !self.truss_sections.contains_key(name)
        {
            let error_message = format!("Truss section with name {name} does not exist!");
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    fn check_truss_section_data_absence(&self, area: FEFloat, optional_area2: Option<FEFloat>) -> Result<(), JsValue>
    {
        if self.truss_sections.values().position(|truss_section| 
            truss_section.is_data_same(area, optional_area2)).is_some()
        {
            let error_message = &format!("Truss section with area {:?}, and area2 {:?} 
                already exists!", area, optional_area2);
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    fn check_deleted_truss_section_existence(&mut self, action_id: u32, name: &str) -> Result<(), JsValue>
    {
        if let Some(truss_section) = self.deleted_truss_sections.get_mut(&action_id)
        {
            match truss_section.get_status()
            {
                Status::Active | Status::Changed(_) =>
                {
                    let error_message = &format!("Incorrect status for truss section deleted by action {}!",
                        action_id);
                    return Err(JsValue::from(error_message));
                },
                Status::Deleted(n) =>
                {
                    if n != name
                    {
                        let error_message = &format!("Incorrect name for truss section deleted by action {}!",
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
            let error_message = &format!("No truss sections deleted by action {}!", action_id);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn add_truss_section(&mut self, action_id: u32, name: &str, area: FEFloat, optional_area2: Option<FEFloat>, 
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_truss_section_name_absence(name)?;
        self.check_truss_section_data_absence(area, optional_area2)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let truss_section = NewTrussSection::create(area, optional_area2)?;

        self.truss_sections.insert(name.to_string(), truss_section.clone());
        truss_section.notify_server(NotificationType::Add(is_action_id_should_be_increased), name.to_string(),
            &self.props)?;

        self.logging();

        Ok(())
    }


    pub fn update_truss_section(&mut self, action_id: u32, name: &str, area: FEFloat,
        optional_area2: Option<FEFloat>, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_truss_section_name_existence(name)?;
        self.check_truss_section_data_absence(area, optional_area2)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let truss_section = self.truss_sections.get_mut(name).expect("Truss section is absent!");
        truss_section.set_data(area, optional_area2)?;
        truss_section.notify_server(NotificationType::Update(is_action_id_should_be_increased), name.to_string(), 
            &self.props)?;

        self.logging();

        Ok(())
    }


    pub fn delete_truss_section(&mut self, action_id: u32, name: &str, is_action_id_should_be_increased: bool) 
        -> Result<(), JsValue>
    {
        self.check_truss_section_name_existence(name)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let cross_section = CrossSection::create_truss(name);
        let parent_key = ParentKey::CrossSection(cross_section);
        self.delete_properties_with_parent_key(parent_key, action_id)?;

        let mut truss_section = self.truss_sections.remove(name).expect("Truss section is absent!");
        truss_section.set_status(Status::Deleted(name.to_string()));

        self.deleted_truss_sections.insert(action_id, truss_section.clone());
    
        truss_section.notify_server(NotificationType::Delete(is_action_id_should_be_increased), name.to_string(), 
            &self.props)?;

        self.logging();

        Ok(())
    }


    pub fn restore_truss_section(&mut self, action_id: u32, name: &str, is_action_id_should_be_increased: bool) 
        -> Result<(), JsValue>
    {
        self.check_deleted_truss_section_existence(action_id, name)?;
        self.check_truss_section_name_absence(name)?;

        let mut truss_section = self.deleted_truss_sections.remove(&action_id)
            .expect("Truss section is absent!");
        truss_section.set_status(Status::Active);

        self.truss_sections.insert(name.to_string(), truss_section.clone());

        truss_section.notify_server(NotificationType::Add(is_action_id_should_be_increased), name.to_string(), 
            &self.props)?;

        let cross_section = CrossSection::create_truss(name);
        let parent_key = ParentKey::CrossSection(cross_section);
        self.restore_properties_with_parent_key(parent_key, action_id)?;

        self.logging();

        Ok(())
    }
}
