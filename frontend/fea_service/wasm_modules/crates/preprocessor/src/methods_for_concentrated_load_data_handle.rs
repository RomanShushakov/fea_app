use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

use crate::enums::{NotificationType, Status};
use crate::types::FEFloat;
use crate::structs::ConcentratedLoad;
use crate::traits::{StatusTrait, ServerNotificationTrait};
use crate::Preprocessor;


#[wasm_bindgen]
impl Preprocessor
{
    fn check_concentrated_load_on_point_absence(&self, point_number: u32) -> Result<(), JsValue>
    {
        if self.concentrated_loads.contains_key(&point_number)
        {
            let error_message = format!("Concentrated load applied at point with number {point_number} \
                already exists!");
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    fn check_concentrated_load_on_point_existence(&self, point_number: u32) -> Result<(), JsValue>
    {
        if !self.concentrated_loads.contains_key(&point_number)
        {
            let error_message = format!("Concentrated load applied at point with number {point_number} \
                does not exist!");
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    fn check_deleted_concentrated_load_existence(&mut self, action_id: u32, point_number: u32) 
        -> Result<(), JsValue>
    {
        if let Some(concentrated_load) = self.deleted_concentrated_loads.get_mut(&action_id)
        {
            match concentrated_load.get_status()
            {
                Status::Active | Status::Changed(_) =>
                {
                    let error_message = &format!("Incorrect status for concentrated load deleted by action {}!",
                        action_id);
                    Err(JsValue::from(error_message))
                },
                Status::Deleted(n) =>
                {
                    if n != point_number
                    {
                        let error_message = &format!("Incorrect point number for concentrated load deleted \
                            by action {}!", action_id);
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
            let error_message = &format!("No concentrated loads deleted by action {}!", action_id);
            Err(JsValue::from(error_message))
        }
    }


    pub(super) fn delete_concentrated_load_applied_at_point(&mut self, point_number: u32, action_id: u32)
        -> Result<(), JsValue>
    {
        if let Some(mut concentrated_load) = self.concentrated_loads.remove(&point_number)
        {
            concentrated_load.set_status(Status::Deleted(point_number));
            self.deleted_concentrated_loads.insert(action_id, concentrated_load.clone());
            concentrated_load.notify_server(NotificationType::Delete(false), point_number, &self.props)?; 
        }
        Ok(())
    }


    pub(super) fn restore_concentrated_load_applied_at_point(&mut self, point_number: u32, action_id: u32)
        -> Result<(), JsValue>
    {   
        if let Some(mut concentrated_load) = self.deleted_concentrated_loads.remove(&action_id)
        {
            match concentrated_load.get_status()
            {
                Status::Deleted(number) => 
                {
                    if number != point_number 
                    {
                        let error_message = &format!("Concentrated load does not applied at point {}!",
                            point_number);
                        return Err(JsValue::from(error_message));
                    }
                },
                _ => 
                {
                    let error_message = &format!("Incorrect status for concentrated load deleted by action {}!",
                        action_id);
                    return Err(JsValue::from(error_message));
                }
            }
            self.check_concentrated_load_on_point_absence(point_number)?;
            concentrated_load.set_status(Status::Active);
            self.concentrated_loads.insert(point_number, concentrated_load.clone());
            concentrated_load.notify_server(NotificationType::Add(false), point_number, &self.props)?;
        }
        Ok(())
    }


    pub fn add_concentrated_load(&mut self, action_id: u32, point_number: u32, fx: FEFloat, fy: FEFloat, 
        fz: FEFloat, mx: FEFloat, my: FEFloat, mz: FEFloat, is_action_id_should_be_increased: bool) 
        -> Result<(), JsValue>
    {
        self.check_point_number_existence(point_number)?;
        self.check_concentrated_load_on_point_absence(point_number)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let uid = self.generate_uid();
        let concentrated_load = ConcentratedLoad::create(fx, fy, fz, mx, my, mz, uid);

        self.concentrated_loads.insert(point_number, concentrated_load.clone());
        concentrated_load.notify_server(NotificationType::Add(is_action_id_should_be_increased), point_number, 
            &self.props)?;

        self.logging();

        Ok(())
    }


    pub fn update_concentrated_load(&mut self, action_id: u32, point_number: u32, fx: FEFloat, fy: FEFloat, 
        fz: FEFloat, mx: FEFloat, my: FEFloat, mz: FEFloat, is_action_id_should_be_increased: bool) 
        -> Result<(), JsValue>
    {
        self.check_point_number_existence(point_number)?;
        self.check_concentrated_load_on_point_existence(point_number)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let concentrated_load = self.concentrated_loads.get_mut(&point_number)
            .expect("Concentrated load is absent!");
        concentrated_load.set_load_components(fx, fy, fz, mx, my, mz);
        concentrated_load.notify_server(NotificationType::Update(is_action_id_should_be_increased), point_number, 
            &self.props)?;
        Ok(())
    }


    pub fn delete_concentrated_load(&mut self, action_id: u32, point_number: u32, 
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_point_number_existence(point_number)?;
        self.check_concentrated_load_on_point_existence(point_number)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let mut concentrated_load = self.concentrated_loads.remove(&point_number)
            .expect("Concentrated load is absent!");
        concentrated_load.set_status(Status::Deleted(point_number));

        self.deleted_concentrated_loads.insert(action_id, concentrated_load.clone());
    
        concentrated_load.notify_server(NotificationType::Delete(is_action_id_should_be_increased), point_number, 
            &self.props)?;

        self.logging();

        Ok(())
    }


    pub fn restore_concentrated_load(&mut self, action_id: u32, point_number: u32,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_deleted_concentrated_load_existence(action_id, point_number)?;
        self.check_concentrated_load_on_point_absence(point_number)?;

        let mut concentrated_load = self.deleted_concentrated_loads.remove(&action_id)
            .expect("Concentrated load is absent!");
        concentrated_load.set_status(Status::Active);

        self.concentrated_loads.insert(point_number, concentrated_load.clone());

        concentrated_load.notify_server(NotificationType::Add(is_action_id_should_be_increased), point_number, 
            &self.props)?;

        self.logging();

        Ok(())
    }
}
