use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

use crate::enums::{NotificationType, Status};
use crate::types::FEFloat;
use crate::structs::PointBoundaryCondition;
use crate::traits::{StatusTrait, ServerNotificationTrait};
use crate::Preprocessor;


#[wasm_bindgen]
impl Preprocessor
{
    fn check_point_boundary_condition_on_point_absence(&self, point_number: u32) -> Result<(), JsValue>
    {
        if self.point_boundary_conditions.contains_key(&point_number)
        {
            let error_message = format!("Point boundary condition applied at point with number {point_number} \
                already exists!");
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    fn check_point_boundary_condition_on_point_existence(&self, point_number: u32) -> Result<(), JsValue>
    {
        if !self.point_boundary_conditions.contains_key(&point_number)
        {
            let error_message = format!("Point boundary condition applied at point with number {point_number} \
                does not exist!");
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    fn check_deleted_point_boundary_condition_existence(&mut self, action_id: u32, point_number: u32) 
        -> Result<(), JsValue>
    {
        if let Some(point_boundary_condition) = self.deleted_point_boundary_conditions
            .get_mut(&action_id)
        {
            match point_boundary_condition.get_status()
            {
                Status::Active | Status::Changed(_) =>
                {
                    let error_message = &format!("Incorrect status for point boundary condition deleted by \
                        action {}!", action_id);
                    return Err(JsValue::from(error_message));
                },
                Status::Deleted(n) =>
                {
                    if n != point_number
                    {
                        let error_message = &format!("Incorrect point number for point boundary condition \
                            deleted by action {}!", action_id);
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
            let error_message = &format!("No point boundary conditions deleted by action {}!", action_id);
            return Err(JsValue::from(error_message));
        }
    }


    pub(super) fn delete_point_boundary_condition_applied_at_point(&mut self, point_number: u32, action_id: u32)
        -> Result<(), JsValue>
    {
        if let Some(mut point_boundary_condition) = 
            self.point_boundary_conditions.remove(&point_number)
        {
            point_boundary_condition.set_status(Status::Deleted(point_number));
            self.deleted_point_boundary_conditions.insert(action_id, point_boundary_condition.clone());
            point_boundary_condition.notify_server(NotificationType::Delete(false), point_number, &self.props)?; 
        }
        Ok(())
    }


    pub(super) fn restore_point_boundary_condition_applied_at_point(&mut self, point_number: u32, action_id: u32)
        -> Result<(), JsValue>
    {   
        if let Some(mut point_boundary_condition) = 
            self.deleted_point_boundary_conditions.remove(&action_id)
        {
            match point_boundary_condition.get_status()
            {
                Status::Deleted(number) => 
                {
                    if number != point_number 
                    {
                        let error_message = &format!("Point boundary condition does not applied at point {}!",
                            point_number);
                        return Err(JsValue::from(error_message));
                    }
                },
                _ => 
                {
                    let error_message = &format!("Incorrect status for point boundary condition deleted by \
                        action {}!", action_id);
                    return Err(JsValue::from(error_message));
                }
            }
            self.check_point_boundary_condition_on_point_absence(point_number)?;
            point_boundary_condition.set_status(Status::Active);
            self.point_boundary_conditions.insert(point_number, point_boundary_condition.clone());
            point_boundary_condition.notify_server(NotificationType::Add(false), point_number, &self.props)?;
        }
        Ok(())
    }


    pub fn add_point_boundary_condition(&mut self, action_id: u32, point_number: u32,
        optional_ux: Option<FEFloat>, optional_uy: Option<FEFloat>, optional_uz: Option<FEFloat>,
        optional_rx: Option<FEFloat>, optional_ry: Option<FEFloat>, optional_rz: Option<FEFloat>,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_point_number_existence(point_number)?;
        self.check_point_boundary_condition_on_point_absence(point_number)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let uid = self.generate_uid();
        let point_boundary_condition = PointBoundaryCondition::create(optional_ux, optional_uy,
            optional_uz, optional_rx, optional_ry, optional_rz, uid);

        self.point_boundary_conditions.insert(point_number, point_boundary_condition.clone());
        point_boundary_condition.notify_server(NotificationType::Add(is_action_id_should_be_increased), point_number, 
            &self.props)?;

        self.logging();

        Ok(())
    }


    pub fn update_point_boundary_condition(&mut self, action_id: u32, point_number: u32,
        optional_ux: Option<FEFloat>, optional_uy: Option<FEFloat>, optional_uz: Option<FEFloat>,
        optional_rx: Option<FEFloat>, optional_ry: Option<FEFloat>, optional_rz: Option<FEFloat>,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_point_number_existence(point_number)?;
        self.check_point_boundary_condition_on_point_existence(point_number)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let point_boundary_condition = 
            self.point_boundary_conditions.get_mut(&point_number).expect("Point boundary condition is absent!");
        point_boundary_condition.set_displacement_components(optional_ux, optional_uy, optional_uz, optional_rx,
            optional_ry, optional_rz);
        point_boundary_condition.notify_server(NotificationType::Update(is_action_id_should_be_increased), point_number, 
            &self.props)?;
            
        self.logging();

        Ok(())
    }


    pub fn delete_point_boundary_condition(&mut self, action_id: u32, point_number: u32,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_point_number_existence(point_number)?;
        self.check_point_boundary_condition_on_point_existence(point_number)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let mut point_boundary_condition = self.point_boundary_conditions.remove(&point_number)
            .expect("Point boundary condition is absent!");
        point_boundary_condition.set_status(Status::Deleted(point_number));

        self.deleted_point_boundary_conditions.insert(action_id, point_boundary_condition.clone());
    
        point_boundary_condition.notify_server(NotificationType::Delete(is_action_id_should_be_increased), point_number, 
            &self.props)?;

        self.logging();

        Ok(())
    }


    pub fn restore_point_boundary_condition(&mut self, action_id: u32, point_number: u32,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_deleted_point_boundary_condition_existence(action_id, point_number)?;
        self.check_point_boundary_condition_on_point_absence(point_number)?;

        let mut point_boundary_condition = self.deleted_point_boundary_conditions.remove(&action_id)
            .expect("Point boundary condition is absent!");
        point_boundary_condition.set_status(Status::Active);

        self.point_boundary_conditions.insert(point_number, point_boundary_condition.clone());

        point_boundary_condition.notify_server(NotificationType::Add(is_action_id_should_be_increased), point_number, 
            &self.props)?;

        self.logging();

        Ok(())
    }
}
