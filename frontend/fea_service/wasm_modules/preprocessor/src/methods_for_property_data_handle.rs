use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

use crate::enums::{NotificationType, Status, CrossSection, ParentKey, RelativeKey};
use crate::structs::Property as NewProperty;
use crate::traits::{StatusTrait, ServerNotificationTrait, ChildTrait};
use crate::functions::
{
    get_objects_numbers_with_relative_keys, move_objects_with_relative_keys_to_changed,
    move_objects_with_relative_keys_to_active, move_objects_to_dst,
};
use crate::Preprocessor;


#[wasm_bindgen]
impl Preprocessor
{
    fn check_property_name_absence(&self, name: &str) -> Result<(), JsValue>
    {
        if self.new_properties.contains_key(name)
        {
            let error_message = format!("Property with name {name} already exists!");
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    pub(super) fn check_property_name_existence(&self, name: &str) -> Result<(), JsValue>
    {
        if !self.new_properties.contains_key(name)
        {
            let error_message = format!("Property with name {name} does not exist!");
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    pub(super) fn check_property_to_lines_applicability(&self, name: &str) -> Result<(), JsValue>
    {
        self.check_property_name_existence(name)?;
        if self.new_properties.get(name).expect("Property is absent!").is_plate_cross_section()
        {
            let error_message = format!("Property with name {name} could not be applied to line!");
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    pub(super) fn check_property_to_surfaces_applicability(&self, name: &str) -> Result<(), JsValue>
    {
        self.check_property_name_existence(name)?;
        if !self.new_properties.get(name).expect("Property is absent!").is_plate_cross_section()
        {
            let error_message = format!("Property with name {name} could not be applied to surface!");
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    fn check_cross_section_existence(&self, cross_section: &CrossSection) -> Result<(), JsValue>
    {
        match cross_section
        {
            CrossSection::Truss(name) => self.check_truss_section_name_existence(name),
            CrossSection::Beam(name) => self.check_beam_section_name_existence(name),
            CrossSection::Plate(name) => self.check_plate_section_name_existence(name),
        }
    }


    fn check_property_data_absence(&self, material_name: String, cross_section: CrossSection) -> Result<(), JsValue>
    {
        if self.new_properties.values().position(|property| 
            property.is_data_same(material_name.clone(), cross_section.clone())).is_some()
        {
            let error_message = &format!("Property with material name {:?}, and cross section {:?} 
                already exists!", material_name, cross_section);
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    fn check_deleted_property_existence(&mut self, action_id: u32, name: &str) -> Result<(), JsValue>
    {
        if let Some(properties) = self.deleted_new_properties.get_mut(&action_id)
        {
            if properties.len() != 1
            {
                let error_message = &format!("Incorrect number of properties deleted by action {action_id}!");
                return Err(JsValue::from(error_message));
            }

            match properties[0].get_status()
            {
                Status::Active | Status::Changed(_) =>
                {
                    let error_message = &format!("Incorrect status for property deleted by action {action_id}!");
                    return Err(JsValue::from(error_message));
                },
                Status::Deleted(n) =>
                {
                    if n != name
                    {
                        let error_message = &format!("Incorrect name for property deleted by action {action_id}!");
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
            let error_message = &format!("No properties deleted by action {action_id}!");
            return Err(JsValue::from(error_message));
        }
    }

    
    pub fn add_properties(&mut self, action_id: u32, name: &str, material_name: &str, cross_section_name: &str, 
        cross_section_type: &str, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_material_name_existence(material_name)?;
        let cross_section = CrossSection::create(cross_section_name, cross_section_type)?;
        self.check_cross_section_existence(&cross_section)?;

        self.check_property_name_absence(name)?;
        self.check_property_data_absence(material_name.to_string(), cross_section.clone())?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let property = NewProperty::create(material_name.to_string(), cross_section);

        self.new_properties.insert(name.to_string(), property.clone());
        property.notify_server(NotificationType::Add(is_action_id_should_be_increased), name.to_string(),
            &self.props)?;

        self.logging();

        Ok(())
    }


    pub fn update_properties(&mut self, action_id: u32, name: &str, material_name: &str, cross_section_name: &str, 
        cross_section_type: &str, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_material_name_existence(material_name)?;
        let cross_section = CrossSection::create(cross_section_name, cross_section_type)?;
        self.check_cross_section_existence(&cross_section)?;

        self.check_property_name_existence(name)?;
        self.check_property_data_absence(material_name.to_string(), cross_section.clone())?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let property = self.new_properties.get_mut(name).expect("Property is absent!");
        property.set_data(material_name.to_string(), cross_section.clone());
        property.notify_server(NotificationType::Update(is_action_id_should_be_increased), name.to_string(), 
            &self.props)?;

        let relative_key = RelativeKey::Property(name.to_string());
        let line_numbers = get_objects_numbers_with_relative_keys(&self.lines,
            &[relative_key.clone()])
            .iter().map(|(line_number, _)| *line_number).collect::<Vec<u32>>();

        if is_action_id_should_be_increased
        {
            if cross_section.is_truss()
            {
                self.move_lines_with_local_axis_1_direction_to_changed(&line_numbers, action_id);
                self.assign_properties_to_lines(action_id, name, &line_numbers, false)?;
                move_objects_with_relative_keys_to_changed(&mut self.surfaces, &mut self.changed_surfaces, 
                    &[relative_key.clone()], action_id, &self.props)?;
            }
            if cross_section.is_beam()
            {
                self.assign_properties_to_lines(action_id, name, &line_numbers, false)?;
                move_objects_with_relative_keys_to_changed(&mut self.surfaces, &mut self.changed_surfaces, 
                    &[relative_key.clone()], action_id, &self.props)?;
            }
            if cross_section.is_plate()
            {
                move_objects_with_relative_keys_to_changed(&mut self.lines, &mut self.changed_lines, 
                    &[relative_key.clone()], action_id, &self.props)?;
            }
        }
        else
        {
            if cross_section.is_truss()
            {
                move_objects_with_relative_keys_to_active(&mut self.changed_lines, &mut self.lines,
                    &[relative_key.clone()], action_id, &self.props)?;
            }
            if cross_section.is_beam()
            {
                if line_numbers.is_empty()
                {
                    move_objects_with_relative_keys_to_active(&mut self.changed_lines, &mut self.lines,
                        &[relative_key.clone()], action_id, &self.props)?;
                }
                else
                {
                    self.move_lines_with_local_axis_1_direction_to_active(&line_numbers, action_id)?;
                    self.assign_properties_to_lines(action_id, name, &line_numbers, false)?;
                }
            }
            if cross_section.is_plate()
            {
                move_objects_with_relative_keys_to_active(&mut self.changed_surfaces, &mut self.surfaces,
                    &[relative_key], action_id, &self.props)?;
            }
        }

        self.logging();

        Ok(())
    }


    pub fn delete_properties(&mut self, action_id: u32, name: &str, is_action_id_should_be_increased: bool) 
        -> Result<(), JsValue>
    {
        self.check_property_name_existence(name)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        move_objects_with_relative_keys_to_changed(&mut self.lines, &mut self.changed_lines, 
            &[RelativeKey::Property(name.to_string())], action_id, &self.props)?;
        move_objects_with_relative_keys_to_changed(&mut self.surfaces, &mut self.changed_surfaces, 
            &[RelativeKey::Property(name.to_string())], action_id, &self.props)?;

        let mut property = self.new_properties.remove(name).expect("Property is absent!");
        property.set_status(Status::Deleted(name.to_string()));

        self.deleted_new_properties.insert(action_id, vec![property.clone()]);
    
        property.notify_server(NotificationType::Delete(is_action_id_should_be_increased), name.to_string(),
            &self.props)?;

        self.logging();

        Ok(())
    }


    pub fn restore_properties(&mut self, action_id: u32, name: &str, is_action_id_should_be_increased: bool) 
        -> Result<(), JsValue>
    {
        self.check_deleted_property_existence(action_id, name)?;
        self.check_property_name_absence(name)?;

        let mut property = self.deleted_new_properties.remove(&action_id).expect("Property is absent!")
            .remove(0);
        property.set_status(Status::Active);

        self.new_properties.insert(name.to_string(), property.clone());

        property.notify_server(NotificationType::Add(is_action_id_should_be_increased), name.to_string(), 
            &self.props)?;

        move_objects_with_relative_keys_to_active(&mut self.changed_lines, &mut self.lines,
            &[RelativeKey::Property(name.to_string())], action_id, &self.props)?;
        move_objects_with_relative_keys_to_active(&mut self.changed_surfaces, &mut self.surfaces,
            &[RelativeKey::Property(name.to_string())], action_id, &self.props)?;

        self.logging();

        Ok(())
    }


    fn get_properties_names_with_parent_key(&self, parent_key: &ParentKey) -> Vec<String>
    {
        let properties_names = self.new_properties.iter()
            .filter(|(_, property)| property.is_child_of_parent(parent_key))
            .map(|(property_name, _)| property_name.to_string())
            .collect::<Vec<String>>();
        properties_names
    }


    pub(super) fn delete_properties_with_parent_key(&mut self, parent_key: ParentKey, action_id: u32)
        -> Result<(), JsValue>
    {
        let properties_names = self.get_properties_names_with_parent_key(&parent_key);
        if !properties_names.is_empty()
        {
            let relative_keys = properties_names.iter()
                .map(|property_name| RelativeKey::Property(property_name.to_string()))
                .collect::<Vec<RelativeKey>>();
            move_objects_with_relative_keys_to_changed(&mut self.lines, &mut self.changed_lines, 
                &relative_keys, action_id, &self.props)?;
            
            move_objects_with_relative_keys_to_changed(&mut self.surfaces, &mut self.changed_surfaces, 
                &relative_keys, action_id, &self.props)?;

            let mut properties = Vec::new();
            for property_name in properties_names.into_iter()
            {
                let mut property = self.new_properties.remove(&property_name).expect("Property is absent!");
                property.set_status(Status::Deleted(property_name));
                properties.push(property);
            }
            move_objects_to_dst(properties.clone(), &mut self.deleted_new_properties, action_id);
            for mut property in properties.into_iter()
            {
                let property_name = match property.get_status()
                    {
                        Status::Deleted(name) => name,
                        _ => unreachable!(),
                    };
                property.notify_server(NotificationType::Delete(false), property_name, &self.props)?;
            }
        }
        Ok(())
    }


    pub(super) fn restore_properties_with_parent_key(&mut self, parent_key: ParentKey, action_id: u32)
        -> Result<(), JsValue>
    {   
        if let Some(properties) = self.deleted_new_properties.remove(&action_id)
        {
            let mut properties_names = Vec::new();

            for mut property in properties.into_iter()
            {
                if !property.is_child_of_parent(&parent_key)
                {
                    let error_message = &format!("There are no property with parent key {parent_key:?}!");
                    return Err(JsValue::from(error_message));
                }
                let property_name = match property.get_status()
                    {
                        Status::Deleted(name) => Ok(name),
                        _ => 
                        {
                            let error_message = &format!("Incorrect status for property deleted by action {}!",
                                action_id);
                            Err(JsValue::from(error_message))
                        }
                    }?;
                self.check_property_name_absence(&property_name)?;
                property.set_status(Status::Active);
                self.new_properties.insert(property_name.clone(), property.clone());
                property.notify_server(NotificationType::Add(false), property_name.clone(), &self.props)?;

                properties_names.push(property_name);
            }

            let relative_keys = properties_names.iter()
                .map(|property_name| RelativeKey::Property(property_name.to_string()))
                .collect::<Vec<RelativeKey>>();
            move_objects_with_relative_keys_to_active(&mut self.changed_lines, &mut self.lines,
                &relative_keys, action_id, &self.props)?;

            move_objects_with_relative_keys_to_active(&mut self.changed_surfaces, &mut self.surfaces,
                &relative_keys, action_id, &self.props)?;
        }
        Ok(())
    }
}
