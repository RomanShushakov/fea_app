use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use serde_json::json;
use finite_element_method::
{
    is_points_of_quadrilateral_on_the_same_line, is_points_of_quadrilateral_on_the_same_plane, 
    convex_hull_on_four_points_on_plane
};

use crate::traits::{ChildTrait, StatusTrait, ServerNotificationTrait, PropertyTrait, MeshSeedTrait};
use crate::enums::{NotificationType, Status, ParentKey, MeshSeed};
use crate::types::FEFloat;
use crate::structs::{Surface as NewSurface, ExtendedProperty, Normal, UniformlyDistributedSurfaceLoad};
use crate::Preprocessor;

use crate::functions::
{
    dispatch_custom_event, find_surface_normal_components, move_objects_to_dst,
};


#[wasm_bindgen]
impl Preprocessor
{
    fn check_surface_number_absence(&self, number: u32) -> Result<(), JsValue>
    {
        if self.surfaces.contains_key(&number)
        {
            let error_message = format!("Surface with number {number} already exists!");
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    pub(super) fn check_surface_number_existence(&self, number: u32) -> Result<(), JsValue>
    {
        if !self.surfaces.contains_key(&number)
        {
            let error_message = format!("Surface with number {number} does not exist!");
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }

    
    fn check_surface_through_points_absence(&self, point_1_number: u32, point_2_number: u32, 
        point_3_number: u32, point_4_number: u32) -> Result<(), JsValue>
    {
        if self.surfaces.values().position(|surface| 
            surface.are_points_same(point_1_number, point_2_number, point_3_number, point_4_number)).is_some()
        {
            let error_message = &format!("Surface through points {:?}, {:?}, {:?}, {:?} already exists!", 
                point_1_number, point_2_number, point_3_number, point_4_number);
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    fn check_surface_diagonals_through_surface_edges_absence(&self, point_1_number: u32, point_2_number: u32, 
        point_3_number: u32, point_4_number: u32) -> Result<(), JsValue>
    {
        if self.surfaces.values().position(|surface| 
            surface.are_diagonal_1_3_points_same(point_1_number, point_2_number) ||
            surface.are_diagonal_1_3_points_same(point_2_number, point_3_number) ||
            surface.are_diagonal_1_3_points_same(point_3_number, point_4_number) ||
            surface.are_diagonal_1_3_points_same(point_4_number, point_1_number) ||
            surface.are_diagonal_2_4_points_same(point_1_number, point_2_number) ||
            surface.are_diagonal_2_4_points_same(point_2_number, point_3_number) ||
            surface.are_diagonal_2_4_points_same(point_3_number, point_4_number) ||
            surface.are_diagonal_2_4_points_same(point_4_number, point_1_number) ||
            surface.are_edge_1_points_same(point_1_number, point_3_number) ||
            surface.are_edge_1_points_same(point_2_number, point_4_number) ||
            surface.are_edge_2_points_same(point_1_number, point_3_number) ||
            surface.are_edge_2_points_same(point_2_number, point_4_number) ||
            surface.are_edge_3_points_same(point_1_number, point_3_number) ||
            surface.are_edge_3_points_same(point_2_number, point_4_number) ||
            surface.are_edge_4_points_same(point_1_number, point_3_number) ||
            surface.are_edge_4_points_same(point_2_number, point_4_number)).is_some()
        {
            let error_message = &format!("Surface diagonal points {:?}, {:?}, or diagonal points {:?}, {:?} 
                lie on surface edge!", point_1_number, point_3_number, point_2_number, point_4_number);
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    fn check_deleted_surface_existence(&mut self, action_id: u32, number: u32) -> Result<(), JsValue>
    {
        if let Some(surfaces) = self.deleted_surfaces.get_mut(&action_id)
        {
            if surfaces.len() != 1
            {
                let error_message = &format!("Incorrect number of surfaces deleted by action {}!",
                    action_id);
                return Err(JsValue::from(error_message));
            }

            match surfaces[0].get_status()
            {
                Status::Active | Status::Changed(_) =>
                {
                    let error_message = &format!("Incorrect status for surface deleted by action {}!",
                        action_id);
                    Err(JsValue::from(error_message))
                },
                Status::Deleted(n) =>
                {
                    if n != number
                    {
                        let error_message = &format!("Incorrect number for surface deleted by action {}!",
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
            let error_message = &format!("No surfaces deleted by action {}!", action_id);
            Err(JsValue::from(error_message))
        }
    }


    fn check_points_of_quadrilateral_not_on_the_same_line(&self, point_1_number: u32, point_2_number: u32,
        point_3_number: u32, point_4_number: u32) -> Result<(), JsValue>
    {
        let points_coordinates = self.get_points_coordinates(
            &[point_1_number, point_2_number, point_3_number, point_4_number]);

        if is_points_of_quadrilateral_on_the_same_line(&points_coordinates[0], &points_coordinates[1], 
            &points_coordinates[2], &points_coordinates[3])
        {
            let error_message = "Points should not lie on the same line!";
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    fn check_points_of_quadrilateral_on_the_same_plane(&self, point_1_number: u32, point_2_number: u32,
        point_3_number: u32, point_4_number: u32) -> Result<(), JsValue>
    {
        let points_coordinates = self.get_points_coordinates(
            &[point_1_number, point_2_number, point_3_number, point_4_number]);

        if !is_points_of_quadrilateral_on_the_same_plane(&points_coordinates[0], &points_coordinates[1], 
            &points_coordinates[2], &points_coordinates[3], self.props.abs_tol)
        {
            let error_message = "Points should lie on the same plane!";
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    fn check_quadrilateral_convexity(&self, point_1_number: u32, point_2_number: u32, point_3_number: u32, 
        point_4_number: u32) -> Result<Vec<u32>, JsValue>
    {
        let points_coordinates = self.get_points_coordinates(
            &[point_1_number, point_2_number, point_3_number, point_4_number]);

        let sorted_point_numbers = convex_hull_on_four_points_on_plane(
                &[point_1_number, point_2_number, point_3_number, point_4_number],
                &[&points_coordinates[0], &points_coordinates[1], &points_coordinates[2], &points_coordinates[3]],
                self.props.rel_tol,
                self.props.abs_tol
            ).map_err(JsValue::from)?;

        if sorted_point_numbers.len() != 4
        {
            let error_message = "Polygon should be convex!";
            return Err(JsValue::from(error_message));
        }

        Ok(sorted_point_numbers)
    }


    pub(super) fn get_numbers_of_surfaces_passing_through_point(&self, point_number: u32) -> Vec<u32>
    {
        
        self.surfaces.iter()
            .filter(|(_, surface)| 
                surface.is_child_of_parent(&ParentKey::Point(point_number)))
            .map(|(surface_number, _)| *surface_number)
            .collect::<Vec<u32>>()
    }


    fn get_surface_vertices_expected_coordinates(&self, surface: &NewSurface, point_number: u32, 
        x: FEFloat, y: FEFloat, z: FEFloat) -> Result<Vec<[FEFloat; 3]>, JsValue>
    {
        let point_numbers = surface.get_point_numbers();
        let updated_point_index = surface.get_parent_index(ParentKey::Point(point_number))?;
        let mut points_coordinates = self.get_points_coordinates(
            &[point_numbers[0], point_numbers[1], point_numbers[2], point_numbers[3]]);
        points_coordinates[updated_point_index] = [x, y, z];
        Ok(points_coordinates)
    }


    fn get_non_planar_surfaces_passing_through_point(&self, point_number: u32, x: FEFloat, y: FEFloat, z: FEFloat) 
        -> Result<Vec<u32>, JsValue>
    {
        let surface_numbers = self.get_numbers_of_surfaces_passing_through_point(point_number);
        let mut non_planar_surface_numbers = Vec::new();

        for surface_number in surface_numbers.iter()
        {
            let surface = self.surfaces.get(surface_number).expect("Surface is absent!");
            let points_coordinates = self.get_surface_vertices_expected_coordinates(surface,
                point_number, x, y, z)?;

            if is_points_of_quadrilateral_on_the_same_line(&points_coordinates[0], 
                &points_coordinates[1], &points_coordinates[2], &points_coordinates[3]) || 
                !is_points_of_quadrilateral_on_the_same_plane(&points_coordinates[0], 
                    &points_coordinates[1], &points_coordinates[2], &points_coordinates[3],
                    self.props.abs_tol)
            {
                non_planar_surface_numbers.push(*surface_number);
            }   
        }
        Ok(non_planar_surface_numbers)
    }


    fn get_concave_planar_surfaces_passing_through_point(&self, point_number: u32, x: FEFloat, y: FEFloat, z: FEFloat)
        -> Result<Vec<u32>, JsValue>
    {
        let surface_numbers = self.get_numbers_of_surfaces_passing_through_point(point_number);
        let non_planar_surface_numbers = self.get_non_planar_surfaces_passing_through_point(point_number, 
            x, y, z)?;
        let mut concave_surface_numbers = Vec::new();

        for surface_number in surface_numbers.iter().filter(|n| !non_planar_surface_numbers.contains(n))
        {
            let surface = self.surfaces.get(surface_number).expect("Surface is absent!");
            let points_coordinates = self.get_surface_vertices_expected_coordinates(surface,
                point_number, x, y, z)?;

            let point_numbers = surface.get_point_numbers();
            match convex_hull_on_four_points_on_plane(
                &point_numbers,
                &[&points_coordinates[0], &points_coordinates[1], &points_coordinates[2], &points_coordinates[3]],
                self.props.rel_tol, self.props.abs_tol)
            {
                Ok(sorted_point_numbers) => 
                {
                    // if sorted_point_numbers.len() != 4 || sorted_point_numbers != point_numbers
                    if sorted_point_numbers.len() != 4
                    {
                        concave_surface_numbers.push(*surface_number);
                    }
                }
                Err(_) => concave_surface_numbers.push(*surface_number),
            }
        }
        Ok(concave_surface_numbers)

    }


    pub(super) fn delete_surfaces_passing_through_point(&mut self, point_number: u32, action_id: u32)
        -> Result<(), JsValue>
    {
        let surface_numbers = self.get_numbers_of_surfaces_passing_through_point(point_number);
        if !surface_numbers.is_empty()
        {
            let mut surfaces = Vec::new();
            for surface_number in surface_numbers.into_iter()
            {
                let mut surface = self.surfaces.remove(&surface_number).expect("Surface is absent!");
                surface.set_status(Status::Deleted(surface_number));
                surfaces.push(surface);
            }
            move_objects_to_dst(surfaces.clone(), &mut self.deleted_surfaces, action_id);
            for mut surface in surfaces.into_iter()
            {
                let surface_number = match surface.get_status()
                    {
                        Status::Deleted(number) => number,
                        _ => unreachable!(),
                    };
                surface.notify_server(NotificationType::Delete(false), surface_number, &self.props)?;
            }
        }
        Ok(())
    }


    pub(super) fn restore_surfaces_passing_through_point(&mut self, point_number: u32, action_id: u32)
        -> Result<(), JsValue>
    {   
        if let Some(surfaces) = self.deleted_surfaces.remove(&action_id)
        {
            for mut surface in surfaces.into_iter()
            {
                if !surface.is_child_of_parent(&ParentKey::Point(point_number))
                {
                    let error_message = &format!("Surface does not pass through the point {}!",
                        point_number);
                    return Err(JsValue::from(error_message));
                }
                let surface_number = match surface.get_status()
                    {
                        Status::Deleted(number) => Ok(number),
                        _ => 
                        {
                            let error_message = &format!("Incorrect status for surface deleted by action {}!",
                                action_id);
                            Err(JsValue::from(error_message))
                        }
                    }?;
                self.check_surface_number_absence(surface_number)?;
                surface.set_status(Status::Active);
                self.surfaces.insert(surface_number, surface.clone());
                surface.notify_server(NotificationType::Add(false), surface_number, &self.props)?;
            }
        }
        Ok(())
    }


    // pub(super) fn check_surface_edge_through_points_absence(&self, point_1_number: u32, point_2_number: u32) 
    //     -> Result<(), JsValue>
    // {
    //     if self.surfaces.values().position(|surface| 
    //         surface.are_edge_1_points_same(point_1_number, point_2_number) ||
    //         surface.are_edge_2_points_same(point_1_number, point_2_number) ||
    //         surface.are_edge_3_points_same(point_1_number, point_2_number) ||
    //         surface.are_edge_4_points_same(point_1_number, point_2_number)).is_some()
    //     {
    //         let error_message = &format!("Surface edge through points {:?}, {:?} already exists!", 
    //             point_1_number, point_2_number);
    //         return Err(JsValue::from(error_message));
    //     }
    //     Ok(())
    // }


    pub(super) fn check_surface_diagonals_through_points_absence(&self, point_1_number: u32, 
        point_2_number: u32) -> Result<(), JsValue>
    {
        if self.surfaces.values().position(|surface| 
            surface.are_diagonal_1_3_points_same(point_1_number, point_2_number) ||
            surface.are_diagonal_2_4_points_same(point_1_number, point_2_number)).is_some()
        {
            let error_message = &format!("Surface diagonal through points {:?}, {:?} already exists!", 
                point_1_number, point_2_number);
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    pub(super) fn delete_non_planar_surfaces_passing_through_point(&mut self, point_number: u32, x: FEFloat, 
        y: FEFloat, z: FEFloat, action_id: u32) -> Result<(), JsValue>
    {
        let surface_numbers = self.get_non_planar_surfaces_passing_through_point(point_number, x, y, z)?;
        if !surface_numbers.is_empty()
        {
            let mut surfaces = Vec::new();
            for surface_number in surface_numbers.into_iter()
            {
                let mut surface = self.surfaces.remove(&surface_number).expect("Surface is absent!");
                surface.set_status(Status::Deleted(surface_number));
                surfaces.push(surface);
            }
            move_objects_to_dst(surfaces.clone(), &mut self.deleted_surfaces, action_id);
            for mut surface in surfaces.into_iter()
            {
                let surface_number = match surface.get_status()
                    {
                        Status::Deleted(number) => number,
                        _ => unreachable!(),
                    };
                surface.notify_server(NotificationType::Delete(false), surface_number, &self.props)?;
            }
        }
        Ok(())
    }


    pub(super) fn delete_concave_planar_surfaces_passing_through_point(&mut self, point_number: u32, x: FEFloat, 
        y: FEFloat, z: FEFloat, action_id: u32) -> Result<(), JsValue>
    {
        let surface_numbers = self.get_concave_planar_surfaces_passing_through_point(point_number, x, y, z)?;
        if !surface_numbers.is_empty() 
        {
            let mut surfaces = Vec::new();
            for surface_number in surface_numbers.into_iter()
            {
                let mut surface = self.surfaces.remove(&surface_number).expect("Surface is absent!");
                surface.set_status(Status::Deleted(surface_number));
                surfaces.push(surface);
            }
            move_objects_to_dst(surfaces.clone(), &mut self.deleted_surfaces, action_id);
            for mut surface in surfaces.into_iter()
            {
                let surface_number = match surface.get_status()
                    {
                        Status::Deleted(number) => number,
                        _ => unreachable!(),
                    };
                surface.notify_server(NotificationType::Delete(false), surface_number, &self.props)?;
            }
        }
        Ok(())
    }


    pub fn add_surface(&mut self, action_id: u32, number: u32, point_1_number: u32, point_2_number: u32,
        point_3_number: u32, point_4_number: u32, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_surface_number_absence(number)?;
        self.check_point_numbers_existence(&[point_1_number, point_2_number, point_3_number, 
            point_4_number])?;
        Self::check_point_numbers_unique(&[point_1_number, point_2_number, point_3_number, 
            point_4_number])?;
        self.check_surface_through_points_absence(point_1_number, point_2_number, point_3_number, point_4_number)?;
        self.check_points_of_quadrilateral_not_on_the_same_line(point_1_number, point_2_number, point_3_number, 
            point_4_number)?;
        self.check_points_of_quadrilateral_on_the_same_plane(point_1_number, point_2_number, point_3_number, 
            point_4_number)?;
        let sorted_point_numbers = self.check_quadrilateral_convexity(point_1_number, 
            point_2_number, point_3_number, point_4_number)?;
        self.check_surface_diagonals_through_surface_edges_absence(sorted_point_numbers[0], 
            sorted_point_numbers[1], sorted_point_numbers[2], 
            sorted_point_numbers[3])?;
        // self.check_line_at_surface_edges_absence(sorted_point_numbers[0], 
        //     sorted_point_numbers[1], sorted_point_numbers[2], 
        //     sorted_point_numbers[3])?;
        self.check_line_through_surface_diagonals_absence(sorted_point_numbers[0], 
            sorted_point_numbers[1], sorted_point_numbers[2], 
            sorted_point_numbers[3])?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let uid = self.generate_uid();
        let points_coordinates = self.get_points_coordinates(&sorted_point_numbers[..3]);
        let surface_normal_components = find_surface_normal_components(&points_coordinates)?;
        let normal = Normal::create(&surface_normal_components)?;
        let surface = NewSurface::create(sorted_point_numbers[0], 
            sorted_point_numbers[1], sorted_point_numbers[2], 
            sorted_point_numbers[3], normal, uid);

        self.surfaces.insert(number, surface.clone());
        surface.notify_server(NotificationType::Add(is_action_id_should_be_increased), number, &self.props)?;

        self.logging();

        Ok(())
    }


    pub fn update_surface(&mut self, action_id: u32, number: u32, point_1_number: u32, point_2_number: u32,
        point_3_number: u32, point_4_number: u32, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_surface_number_existence(number)?;
        self.check_point_numbers_existence(&[point_1_number, point_2_number, point_3_number, 
            point_4_number])?;
        Self::check_point_numbers_unique(&[point_1_number, point_2_number, point_3_number, 
            point_4_number])?;
        self.check_surface_through_points_absence(point_1_number, point_2_number, point_3_number, point_4_number)?;
        self.check_points_of_quadrilateral_not_on_the_same_line(point_1_number, point_2_number, point_3_number, 
            point_4_number)?;
        self.check_points_of_quadrilateral_on_the_same_plane(point_1_number, point_2_number, point_3_number, 
            point_4_number)?;
        let sorted_point_numbers = self.check_quadrilateral_convexity(point_1_number, 
            point_2_number, point_3_number, point_4_number)?;
        self.check_surface_diagonals_through_surface_edges_absence(sorted_point_numbers[0], 
            sorted_point_numbers[1], sorted_point_numbers[2], 
            sorted_point_numbers[3])?;
        // self.check_line_at_surface_edges_absence(sorted_point_numbers[0], 
        //     sorted_point_numbers[1], sorted_point_numbers[2], 
        //     sorted_point_numbers[3])?;
        self.check_line_through_surface_diagonals_absence(sorted_point_numbers[0], 
            sorted_point_numbers[1], sorted_point_numbers[2], 
            sorted_point_numbers[3])?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let points_coordinates = self.get_points_coordinates(&sorted_point_numbers[..3]);
        let surface_normal_components = find_surface_normal_components(&points_coordinates)?;
        let normal = Normal::create(&surface_normal_components)?;
        
        let surface = self.surfaces.get_mut(&number).expect("Surface is absent!");
        surface.set_point_numbers(sorted_point_numbers[0], sorted_point_numbers[1], 
            sorted_point_numbers[2], sorted_point_numbers[3]);
        surface.set_normal(normal);
        surface.notify_server(NotificationType::Update(is_action_id_should_be_increased), number, &self.props)?;

        self.logging();

        Ok(())
    }


    pub fn rotate_surface_vertices_clockwise(&mut self, action_id: u32, number: u32, 
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_surface_number_existence(number)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let surface = self.surfaces.get_mut(&number).expect("Surface is absent!");
        surface.rotate_vertices_clockwise();
        surface.notify_server(NotificationType::Update(is_action_id_should_be_increased), number, &self.props)?;

        self.logging();

        Ok(())
    }


    pub fn rotate_surface_vertices_counter_clockwise(&mut self, action_id: u32, number: u32, 
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_surface_number_existence(number)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let surface = self.surfaces.get_mut(&number).expect("Surface is absent!");
        surface.rotate_vertices_counter_clockwise();
        surface.notify_server(NotificationType::Update(is_action_id_should_be_increased), number, &self.props)?;

        self.logging();

        Ok(())
    }


    pub fn flip_surface_normal_axis(&mut self, action_id: u32, number: u32, 
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_surface_number_existence(number)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let surface = self.surfaces.get_mut(&number).expect("Surface is absent!");
        surface.flip_normal_axis();
        surface.notify_server(NotificationType::Update(is_action_id_should_be_increased), number, &self.props)?;

        self.logging();

        Ok(())
    }


    pub fn delete_surface(&mut self, action_id: u32, number: u32, is_action_id_should_be_increased: bool) 
        -> Result<(), JsValue>
    {
        self.check_surface_number_existence(number)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let mut surface = self.surfaces.remove(&number).expect("Surface is absent!");
        surface.set_status(Status::Deleted(number));

        self.deleted_surfaces.insert(action_id, vec![surface.clone()]);
    
        surface.notify_server(NotificationType::Delete(is_action_id_should_be_increased), number, &self.props)?;

        self.logging();

        Ok(())
    }


    pub fn restore_surface(&mut self, action_id: u32, number: u32, is_action_id_should_be_increased: bool) 
        -> Result<(), JsValue>
    {
        self.check_deleted_surface_existence(action_id, number)?;
        self.check_surface_number_absence(number)?;

        let mut surface = self.deleted_surfaces.remove(&action_id).expect("Surface is absent!").remove(0);
        surface.set_status(Status::Active);

        self.surfaces.insert(number, surface.clone());

        surface.notify_server(NotificationType::Add(is_action_id_should_be_increased), number, &self.props)?;

        self.logging();

        Ok(())
    }


    fn check_surface_without_other_property(&self, number: u32, property_name: &str) -> Result<(), JsValue>
    {
        if let Some(surface) = self.surfaces.get(&number)
        {
            if surface.is_property_assigned()
                && !surface.is_property_name_same(property_name.to_string()) 
                {
                    let error_message = format!("Another property has been already assigned to surface with \
                        number {number}!");
                    return Err(JsValue::from(error_message));
                }
            Ok(())
        }
        else
        {
            let error_message = format!("Surface with number {number} does not exist!");
            Err(JsValue::from(error_message))
        }
    }


    pub fn assign_properties_to_surfaces(&mut self, action_id: u32, name: &str, surface_numbers: &[u32], 
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_property_to_surfaces_applicability(name)?;
        for number in surface_numbers.iter()
        {
            self.check_surface_number_existence(*number)?;
            self.check_surface_without_other_property(*number, name)?;
        }
        
        self.clear_all_deleted_objects_by_action_id(action_id);

        for (number, surface) in self.surfaces.iter_mut()
        {
            if surface.is_property_name_same(name.to_string())
            {
                surface.set_propery(None);
                surface.set_mesh_seed(None);
                surface.notify_server(NotificationType::Update(false), *number, &self.props)?;
            }
        }

        let property = self.new_properties.get(name).cloned().expect("Property is absent!");
        let extended_property = ExtendedProperty::create(name.to_string(), property); 

        for number in surface_numbers.iter()
        {
            let surface = self.surfaces.get_mut(number).expect("Surface is absent!");
            surface.set_propery(Some(extended_property.clone()));
            surface.set_mesh_seed(Some(MeshSeed::Global(self.global_mesh_seed_value)));
            surface.notify_server(NotificationType::Update(false), *number, &self.props)?;
        }

        if is_action_id_should_be_increased
        {
            let detail = json!(null);
            dispatch_custom_event(detail, &self.props.increase_action_id_event_name, 
                &self.props.event_target)?;
        }

        self.logging();

        Ok(())
    }


    fn check_mesh_seed_can_be_assigned(&self, number: u32) -> Result<(), JsValue>
    {
        if let Some(surface) = self.surfaces.get(&number)
        {
            if surface.is_property_assigned()
            {
                Ok(())
            }
            else
            {
                let error_message = format!("Mesh seed could not be set for surface {number}!");
                Err(JsValue::from(error_message))
            }
        }
        else
        {
            let error_message = format!("Surface with number {number} does not exist!");
            Err(JsValue::from(error_message))
        }
    }


    pub fn update_surfaces_mesh_seed(&mut self, action_id: u32, surfaces_edges_1_3_mesh_seed_value: u8,
        surfaces_edges_2_4_mesh_seed_value: u8, surface_numbers: &[u32], is_action_id_should_be_increased: bool) 
        -> Result<(), JsValue>
    {
        for number in surface_numbers.iter()
        {
            self.check_mesh_seed_can_be_assigned(*number)?;
        }

        self.clear_all_deleted_objects_by_action_id(action_id);

        let mut changed_surfaces = Vec::new();
        for number in surface_numbers.iter()
        {
            let surface = self.surfaces.get_mut(number).expect("Surface is absent!");
            let mut changed_surface = surface.clone();
            changed_surface.set_status(Status::Changed(*number));
            changed_surfaces.push(changed_surface);
            surface.set_mesh_seed(Some(MeshSeed::Surface(surfaces_edges_1_3_mesh_seed_value, 
                surfaces_edges_2_4_mesh_seed_value)));
            surface.notify_server(NotificationType::Update(false), *number, &self.props)?;
        }

        move_objects_to_dst(changed_surfaces, &mut self.changed_surfaces, action_id);

        if is_action_id_should_be_increased
        {
            let detail = json!(null);
            dispatch_custom_event(detail, &self.props.increase_action_id_event_name, 
                &self.props.event_target)?;
        }

        self.logging();

        Ok(())
    }


    pub fn undo_surfaces_mesh_seed_update(&mut self, action_id: u32, surface_numbers: &[u32],
        _is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        if let Some(surfaces) = self.changed_surfaces.remove(&action_id)
        {
            for mut surface in surfaces.into_iter()
            {
                match surface.get_status()
                {
                    Status::Active | Status::Deleted(_) =>
                    {
                        let error_message = &format!("Incorrect status for surface changed by action {}!",
                            action_id);
                        return Err(JsValue::from(error_message));
                    },
                    Status::Changed(n) =>
                    {
                        if !surface_numbers.contains(&n)
                        {
                            let error_message = &format!("Incorrect property for surface changed by action {}!",
                                action_id);
                            return Err(JsValue::from(error_message));
                        }
                        if !surface.is_mesh_seed_assigned()
                        {
                            let error_message = &format!("Incorrect mesh seed for surface \
                                changed by action {action_id}!");
                            return Err(JsValue::from(error_message));
                        }
                        surface.set_status(Status::Active);
                        self.surfaces.insert(n, surface.clone());
                        surface.notify_server(NotificationType::Update(false), n, &self.props)?;
                    }
                }
            }
        }
        else
        {
            let error_message = &format!("There are no surfaces chanded by action id {action_id:?}!");
            return Err(JsValue::from(error_message));
        }

        self.logging();

        Ok(())
    }


    fn check_uniformly_distributed_surface_load_on_surface_absence(&self, surface_number: u32) -> Result<(), JsValue>
    {
        self.check_surface_number_existence(surface_number)?;
        let surface = self.surfaces.get(&surface_number).expect("Surface is absent!");
        if surface.is_uniformly_distributed_surface_load_assigned()
        {
            let error_message = format!("Uniformly distributed surface load applied at surface with number {} \
                already exists!", surface_number);
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    fn check_uniformly_distributed_surface_load_on_surface_existence(&self, surface_number: u32) 
        -> Result<(), JsValue>
    {
        self.check_surface_number_existence(surface_number)?;
        let surface = self.surfaces.get(&surface_number).expect("Surface is absent!");
        if !surface.is_uniformly_distributed_surface_load_assigned()
        {
            let error_message = format!("Uniformly distributed surface load applied at surface with number {} \
                does not exist!", surface_number);
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    fn check_deleted_uniformly_distributed_surface_load_on_surface_existence(&mut self, action_id: u32, 
        surface_number: u32) -> Result<(), JsValue>
    {
        if let Some(surfaces) = self.changed_surfaces.get_mut(&action_id)
        {
            if surfaces.len() != 1
            {
                let error_message = &format!("Incorrect number of surfaces changed by action {}!",
                    action_id);
                return Err(JsValue::from(error_message));
            }

            let changed_surface = &mut surfaces[0];
            match changed_surface.get_status()
            {
                Status::Active | Status::Deleted(_) =>
                {
                    let error_message = &format!("Incorrect status for surface changed by action {}!",
                        action_id);
                    Err(JsValue::from(error_message))
                },
                Status::Changed(n) =>
                {
                    if n != surface_number
                    {
                        let error_message = &format!("Incorrect number for surface changed by action {}!",
                            action_id);
                        Err(JsValue::from(error_message))
                    }
                    else
                    {
                        if !changed_surface.is_uniformly_distributed_surface_load_assigned()
                        {
                            let error_message = &format!("No uniformly distributed surface loads deleted \
                                by action {}!", action_id);
                            return Err(JsValue::from(error_message));
                        }
                        Ok(())
                    }
                }
            }
        }
        else
        {
            let error_message = &format!("No surfaces deleted by action {}!", action_id);
            Err(JsValue::from(error_message))
        }
    }


    fn check_uniformly_distributed_surface_load_can_be_added(&self, number: u32) -> Result<(), JsValue>
    {
        if let Some(surface) = self.surfaces.get(&number)
        {
            if surface.is_property_assigned()
            {
                Ok(())
            }
            else
            {
                let error_message = format!("Uniformly distributed surface load could not be added to \
                    surface with number {number}!");
                Err(JsValue::from(error_message))
            }
        }
        else
        {
            let error_message = format!("Surface with number {number} does not exist!");
            Err(JsValue::from(error_message))
        }
    }


    pub fn add_uniformly_distributed_surface_load(&mut self, action_id: u32, surface_number: u32, px: FEFloat, 
        py: FEFloat, pz: FEFloat, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_uniformly_distributed_surface_load_on_surface_absence(surface_number)?;
        self.check_uniformly_distributed_surface_load_can_be_added(surface_number)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let uid = self.generate_uid();
        let uniformly_distributed_surface_load = 
            UniformlyDistributedSurfaceLoad::create(px, py, pz, uid);

        let surface = self.surfaces.get_mut(&surface_number).expect("Surface is absent!");
        surface.set_uniformly_distributed_surface_load(Some(uniformly_distributed_surface_load));

        surface.notify_server(NotificationType::Update(is_action_id_should_be_increased), surface_number, &self.props)?;

        self.logging();

        Ok(())
    }


    pub fn update_uniformly_distributed_surface_load(&mut self, action_id: u32, surface_number: u32, px: FEFloat, 
        py: FEFloat, pz: FEFloat, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_uniformly_distributed_surface_load_on_surface_existence(surface_number)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let surface = self.surfaces.get_mut(&surface_number).expect("Surface is absent!");
        let uniformly_distributed_surface_load = 
            surface.get_mut_ref_optional_uniformly_distributed_surface_load()
                .as_mut()
                .expect("Uniformly distributed surface load is absent!");
        uniformly_distributed_surface_load.set_load_components(px, py, pz);

        surface.notify_server(NotificationType::Update(is_action_id_should_be_increased), surface_number, &self.props)?;

        self.logging();

        Ok(())
    }


    pub fn delete_uniformly_distributed_surface_load(&mut self, action_id: u32, surface_number: u32,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_uniformly_distributed_surface_load_on_surface_existence(surface_number)?;

        self.clear_all_deleted_objects_by_action_id(action_id);

        let surface = self.surfaces.get_mut(&surface_number).expect("Surface is absent!");
        let mut changed_surface = surface.clone();
        changed_surface.set_status(Status::Changed(surface_number));
        surface.set_uniformly_distributed_surface_load(None);

        move_objects_to_dst(vec![changed_surface], &mut self.changed_surfaces, action_id);

        surface.notify_server(NotificationType::Update(is_action_id_should_be_increased), surface_number, &self.props)?;

        self.logging();

        Ok(())
    }


    pub fn restore_uniformly_distributed_surface_load(&mut self, action_id: u32, surface_number: u32,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.check_deleted_uniformly_distributed_surface_load_on_surface_existence(action_id, surface_number)?;
        self.check_uniformly_distributed_surface_load_on_surface_absence(surface_number)?;

        let mut surface = self.changed_surfaces.remove(&action_id).expect("Surface is absent!").remove(0);
        surface.set_status(Status::Active);

        self.surfaces.insert(surface_number, surface.clone());

        surface.notify_server(NotificationType::Update(is_action_id_should_be_increased), surface_number, &self.props)?;

        self.logging();

        Ok(())
    }
}
