use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

use crate::enums::{NotificationType, Status, ParentKey};
use crate::types::FEFloat;
use crate::structs::Material as NewMaterial;
use crate::traits::{StatusTrait, ServerNotificationTrait};
use crate::Preprocessor;


#[wasm_bindgen]
impl Preprocessor
{
    fn check_material_name_absence(&self, name: &str) -> Result<(), JsValue>
    {
        if self.materials.contains_key(name)
        {
            let error_message = format!("Material with name {name} already exists!");
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    pub(super) fn check_material_name_existence(&self, name: &str) -> Result<(), JsValue>
    {
        if !self.materials.contains_key(name)
        {
            let error_message = format!("Material with name {name} does not exist!");
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    fn check_material_data_absence(&self, young_modulus: FEFloat, poisson_ratio: FEFloat) -> Result<(), JsValue>
    {
        if self.materials.values().position(|material| 
            material.is_data_same(young_modulus, poisson_ratio)).is_some()
        {
            let error_message = &format!("Material with Young's modulus {:?}, and Poisson's ratio {:?} 
                already exists!", young_modulus, poisson_ratio);
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    fn check_deleted_material_existence(&mut self, action_id: u32, name: &str) -> Result<(), JsValue>
    {
        if let Some(material) = self.deleted_materials.get_mut(&action_id)
        {
            match material.get_status()
            {
                Status::Active | Status::Changed(_) =>
                {
                    let error_message = &format!("Incorrect status for material deleted by action {}!",
                        action_id);
                    Err(JsValue::from(error_message))
                },
                Status::Deleted(n) =>
                {
                    if n != name
                    {
                        let error_message = &format!("Incorrect name for material deleted by action {}!",
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
            let error_message = &format!("No materials deleted by action {}!", action_id);
            Err(JsValue::from(error_message))
        }
    }


    pub fn add_material(&mut self, action_id: u32, name: &str, young_modulus: FEFloat, poisson_ratio: FEFloat, 
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_material_name_absence(name)?;
        self.check_material_data_absence(young_modulus, poisson_ratio)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let material = NewMaterial::create(young_modulus, poisson_ratio)?;

        self.materials.insert(name.to_string(), material.clone());
        material.notify_server(NotificationType::Add(is_action_id_should_be_increased), name.to_string(),
            &self.props)?;

        self.logging();

        Ok(())
    }


    pub fn update_material(&mut self, action_id: u32, name: &str, young_modulus: FEFloat, poisson_ratio: FEFloat, 
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_material_name_existence(name)?;
        self.check_material_data_absence(young_modulus, poisson_ratio)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let material = self.materials.get_mut(name).expect("Material is absent!");
        material.set_data(young_modulus, poisson_ratio)?;
        material.notify_server(NotificationType::Update(is_action_id_should_be_increased), name.to_string(), 
            &self.props)?;

        self.logging();

        Ok(())
    }


    pub fn delete_material(&mut self, action_id: u32, name: &str, is_action_id_should_be_increased: bool) 
        -> Result<(), JsValue>
    {
        self.check_material_name_existence(name)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let parent_key = ParentKey::Material(name.to_string());
        self.delete_properties_with_parent_key(parent_key, action_id)?;

        let mut material = self.materials.remove(name).expect("Material is absent!");
        material.set_status(Status::Deleted(name.to_string()));

        self.deleted_materials.insert(action_id, material.clone());
    
        material.notify_server(NotificationType::Delete(is_action_id_should_be_increased), name.to_string(),
            &self.props)?;

        self.logging();

        Ok(())
    }


    pub fn restore_material(&mut self, action_id: u32, name: &str, is_action_id_should_be_increased: bool) 
        -> Result<(), JsValue>
    {
        self.check_deleted_material_existence(action_id, name)?;
        self.check_material_name_absence(name)?;

        let mut material = self.deleted_materials.remove(&action_id).expect("Material is absent!");
        material.set_status(Status::Active);

        self.materials.insert(name.to_string(), material.clone());

        material.notify_server(NotificationType::Add(is_action_id_should_be_increased), name.to_string(), 
            &self.props)?;

        let parent_key = ParentKey::Material(name.to_string());
        self.restore_properties_with_parent_key(parent_key, action_id)?;

        self.logging();

        Ok(())
    }
}
