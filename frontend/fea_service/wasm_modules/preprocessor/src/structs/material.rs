use wasm_bindgen::JsValue;
use serde_json::{Value, json};

use crate::props::Props;
use crate::types::FEFloat;
use crate::enums::{Status, NotificationType};
use crate::functions::check_value_positive;
use crate::traits::{StatusTrait, ServerNotificationTrait};


#[derive(Debug, Clone)]
pub struct Material
{
    young_modulus: FEFloat,
    poisson_ratio: FEFloat,
    status: Status<String>,
}


impl Material
{
    pub fn create(young_modulus: FEFloat, poisson_ratio: FEFloat) -> Result<Self, JsValue>
    {
        check_value_positive("Young's modulus", young_modulus)?;
        check_value_positive("Poisson's ratio", poisson_ratio)?;

        Ok(Material { young_modulus, poisson_ratio, status: Status::Active })
    }


    pub fn is_data_same(&self, young_modulus: FEFloat, poisson_ratio: FEFloat) -> bool
    {
        self.young_modulus == young_modulus && self.poisson_ratio == poisson_ratio
    }


    pub fn get_data(&self) -> [FEFloat; 2]
    {
        [self.young_modulus, self.poisson_ratio]
    }


    pub fn set_data(&mut self, young_modulus: FEFloat, poisson_ratio: FEFloat) -> Result<(), JsValue>
    {
        check_value_positive("Young's modulus", young_modulus)?;
        check_value_positive("Poisson's ratio", poisson_ratio)?;

        self.young_modulus = young_modulus;
        self.poisson_ratio = poisson_ratio;

        Ok(())
    }
}


impl StatusTrait for Material
{
    type Key = String;
    fn get_mut_ref_status(&mut self) -> &mut Status<Self::Key> 
    {
        &mut self.status
    }
}


impl ServerNotificationTrait for Material
{
    type Key = String;
    fn get_event_detail(&self, notification_type: &NotificationType, key: Self::Key) -> Value 
    {
        match notification_type
        {
            NotificationType::Add(is_action_id_should_be_increased) | 
            NotificationType::Update(is_action_id_should_be_increased) =>
            {
                json!({ 
                    "material_data":
                    { 
                        "name": key, "young_modulus": self.young_modulus, 
                        "poisson_ratio": self.poisson_ratio 
                    },
                    "is_action_id_should_be_increased": is_action_id_should_be_increased 
                })
            },
            NotificationType::Delete(is_action_id_should_be_increased) =>
            {
                json!({ 
                    "material_data": { "name": key },
                    "is_action_id_should_be_increased": is_action_id_should_be_increased 
                })
            }
        }
    }


    fn get_event_name(&self, notification_type: &NotificationType, props: &Props) -> String 
    {
        match notification_type
        {
            NotificationType::Add(_) => props.add_material_event_name.clone(),
            NotificationType::Update(_) => props.update_material_event_name.clone(),
            NotificationType::Delete(_) => props.delete_material_event_name.clone(),
        }
    }


    fn get_event_target(&self, props: &Props) -> String 
    {
        props.event_target.clone()
    }
}
