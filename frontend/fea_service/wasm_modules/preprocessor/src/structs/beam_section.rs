use wasm_bindgen::JsValue;
use serde_json::{Value, json};

use crate::props::Props;
use crate::traits::{StatusTrait, ServerNotificationTrait};
use crate::types::FEFloat;
use crate::enums::{Status, NotificationType};
use crate::functions::check_value_positive;


#[derive(Debug, Clone)]
pub struct BeamSection
{
    area: FEFloat,
    i11: FEFloat,
    i22: FEFloat,
    i12: FEFloat,
    it: FEFloat,
    shear_factor: FEFloat,
    status: Status<String>,
}


impl BeamSection
{
    pub fn create(area: FEFloat, i11: FEFloat, i22: FEFloat, i12: FEFloat, it: FEFloat, shear_factor: FEFloat) 
        -> Result<Self, JsValue>
    {
        check_value_positive("Area", area)?;
        check_value_positive("I11", i11)?;
        check_value_positive("I22", i22)?;
        check_value_positive("It", it)?;
        check_value_positive("Shear factor", shear_factor)?;

        Ok(BeamSection { area, i11, i22, i12, it, shear_factor, status: Status::Active })
    }


    pub fn is_data_same(&self, area: FEFloat, i11: FEFloat, i22: FEFloat, i12: FEFloat, it: FEFloat, 
        shear_factor: FEFloat) -> bool
    {
        self.area == area && self.i11 == i11 && self.i22 == i22 && self.i12 == i12 && self.it == it && 
        self.shear_factor == shear_factor
    }


    pub fn get_data(&self) -> [FEFloat; 6]
    {
        [self.area, self.i11, self.i22, self.i12, self.it, self.shear_factor]
    }


    pub fn set_data(&mut self, area: FEFloat, i11: FEFloat, i22: FEFloat, i12: FEFloat, it: FEFloat, 
        shear_factor: FEFloat) -> Result<(), JsValue>
    {
        check_value_positive("Area", area)?;
        check_value_positive("I11", i11)?;
        check_value_positive("I22", i22)?;
        check_value_positive("It", it)?;
        check_value_positive("Shear factor", shear_factor)?;

        self.area = area;
        self.i11 = i11;
        self.i22 = i22;
        self.i12 = i12;
        self.it = it;
        self.shear_factor = shear_factor;

        Ok(())
    }
}



impl StatusTrait for BeamSection
{
    type Key = String;
    fn get_mut_ref_status(&mut self) -> &mut Status<Self::Key> 
    {
       &mut self.status 
    }
}


impl ServerNotificationTrait for BeamSection
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
                    "beam_section_data": 
                    { 
                        "name": key, 
                        "area": self.area, 
                        "i11": self.i11, 
                        "i22": self.i22, 
                        "i12": self.i12, 
                        "it": self.it,
                        "shear_factor": self.shear_factor 
                    },
                    "is_action_id_should_be_increased": is_action_id_should_be_increased 
                })
            },
            NotificationType::Delete(is_action_id_should_be_increased) =>
            {
                json!({ 
                    "beam_section_data": { "name": key },
                    "is_action_id_should_be_increased": is_action_id_should_be_increased 
                })
            }
        }
    }


    fn get_event_name(&self, notification_type: &NotificationType, props: &Props) -> String 
    {
        match notification_type
        {
            NotificationType::Add(_) => props.add_beam_section_event_name.clone(),
            NotificationType::Update(_) => props.update_beam_section_event_name.clone(),
            NotificationType::Delete(_) => props.delete_beam_section_event_name.clone(),
        }
    }


    fn get_event_target(&self, props: &Props) -> String 
    {
        props.event_target.clone()
    }
}
