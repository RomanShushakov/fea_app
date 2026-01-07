use enums::CrossSection;
use wasm_bindgen::prelude::*;
use serde_json::json;
use std::collections::HashMap;
use serde::Serialize;
use serde_wasm_bindgen::Serializer;

mod props;
use props::Props;

mod types;
use types::FEFloat;

mod structs;
use structs::
{
    Point as NewPoint, Line as NewLine, Surface as NewSurface, Material as NewMaterial, TrussSection as NewTrussSection,
    BeamSection as NewBeamSection, PlateSection, Property as NewProperty, LocalAxis1Direction, ConcentratedLoad,
    PointBoundaryCondition,
};

mod traits;

mod enums;
use enums::MeshSeed as NewMeshSeed;

mod functions;
use functions::{log, clear_deleted_objects_by_action_id};

mod methods_for_point_data_handle;
mod methods_for_line_data_handle;
mod methods_for_surface_data_handle;
mod methods_for_material_data_handle;
mod methods_for_truss_section_data_handle;
mod methods_for_beam_section_data_handle;
mod methods_for_plate_section_data_handle;
mod methods_for_property_data_handle;
mod methods_for_beam_section_orientation_handle;
mod methods_for_concentrated_load_data_handle;
mod methods_for_point_boundary_conditions_data_handle;
mod methods_for_mesh_seed_data_handle;


#[wasm_bindgen]
pub struct Preprocessor
{
    props: Props,

    points: HashMap<u32, NewPoint>,
    deleted_points: HashMap<u32, NewPoint>,

    lines: HashMap<u32, NewLine>,
    changed_lines: HashMap<u32, Vec<NewLine>>,
    deleted_lines: HashMap<u32, Vec<NewLine>>,

    surfaces: HashMap<u32, NewSurface>,
    deleted_surfaces: HashMap<u32, Vec<NewSurface>>,
    changed_surfaces: HashMap<u32, Vec<NewSurface>>,

    materials: HashMap<String, NewMaterial>,
    deleted_materials: HashMap<u32, NewMaterial>,

    truss_sections: HashMap<String, NewTrussSection>,
    deleted_truss_sections: HashMap<u32, NewTrussSection>,

    beam_sections: HashMap<String, NewBeamSection>,
    deleted_beam_sections: HashMap<u32, NewBeamSection>,

    plate_sections: HashMap<String, PlateSection>,
    deleted_plate_sections: HashMap<u32, PlateSection>,

    new_properties: HashMap<String, NewProperty>,
    deleted_new_properties: HashMap<u32, Vec<NewProperty>>,

    beam_sections_local_axis_1_directions: Vec<LocalAxis1Direction>,
    deleted_beam_sections_local_axis_1_directions: HashMap<u32, LocalAxis1Direction>,

    concentrated_loads: HashMap<u32, ConcentratedLoad>,
    deleted_concentrated_loads: HashMap<u32, ConcentratedLoad>,

    point_boundary_conditions: HashMap<u32, PointBoundaryCondition>,
    deleted_point_boundary_conditions: HashMap<u32, PointBoundaryCondition>,

    global_mesh_seed_value: u8,
}


#[wasm_bindgen]
impl Preprocessor
{
    pub fn create(
        rel_tol: FEFloat,
        abs_tol: FEFloat, 
        event_target: String,

        add_point_event_name: String,
        update_point_event_name: String,
        delete_point_event_name: String,

        add_line_event_name: String,
        update_line_event_name: String,
        delete_line_event_name: String,

        add_surface_event_name: String,
        update_surface_event_name: String,
        delete_surface_event_name: String,

        add_material_event_name: String,
        update_material_event_name: String,
        delete_material_event_name: String,

        add_truss_section_event_name: String,
        update_truss_section_event_name: String,
        delete_truss_section_event_name: String,

        add_beam_section_event_name: String,
        update_beam_section_event_name: String,
        delete_beam_section_event_name: String,

        add_plate_section_event_name: String,
        update_plate_section_event_name: String,
        delete_plate_section_event_name: String,

        add_properties_event_name: String,
        update_properties_event_name: String,
        delete_properties_event_name: String,

        increase_action_id_event_name: String,

        add_beam_local_axis_1_direction_event_name: String,
        delete_beam_local_axis_1_direction_event_name: String,
        update_beam_section_orientation_data_event_name: String,

        local_axis_1_direction_input_info_message_header: String,

        add_concentrated_load_event_name: String,
        update_concentrated_load_event_name: String,
        delete_concentrated_load_event_name: String,

        add_point_boundary_condition_event_name: String,
        update_point_boundary_condition_event_name: String,
        delete_point_boundary_condition_event_name: String,

        init_global_mesh_seed_value: u8,
        update_global_mesh_seed_event_name: String,
    ) 
        -> Self
    {
        let props = Props::create(
            rel_tol,
            abs_tol, 
            event_target,

            add_point_event_name,
            update_point_event_name,
            delete_point_event_name,

            add_line_event_name,
            update_line_event_name,
            delete_line_event_name,

            add_surface_event_name,
            update_surface_event_name,
            delete_surface_event_name,

            add_material_event_name,
            update_material_event_name,
            delete_material_event_name,

            add_truss_section_event_name,
            update_truss_section_event_name,
            delete_truss_section_event_name,

            add_beam_section_event_name,
            update_beam_section_event_name,
            delete_beam_section_event_name,

            add_plate_section_event_name,
            update_plate_section_event_name,
            delete_plate_section_event_name,

            add_properties_event_name,
            update_properties_event_name,
            delete_properties_event_name,

            increase_action_id_event_name,

            add_beam_local_axis_1_direction_event_name,
            delete_beam_local_axis_1_direction_event_name,
            update_beam_section_orientation_data_event_name,

            local_axis_1_direction_input_info_message_header,

            add_concentrated_load_event_name,
            update_concentrated_load_event_name,
            delete_concentrated_load_event_name,

            add_point_boundary_condition_event_name,
            update_point_boundary_condition_event_name,
            delete_point_boundary_condition_event_name,

            update_global_mesh_seed_event_name,
        );
        let points = HashMap::new();
        let deleted_points = HashMap::new();

        let lines = HashMap::new();
        let changed_lines = HashMap::new();
        let deleted_lines = HashMap::new();

        let surfaces = HashMap::new();
        let changed_surfaces = HashMap::new();
        let deleted_surfaces = HashMap::new();
        
        let materials = HashMap::new();
        let deleted_materials = HashMap::new();

        let truss_sections = HashMap::new();
        let deleted_truss_sections = HashMap::new();

        let beam_sections = HashMap::new();
        let deleted_beam_sections = HashMap::new();

        let plate_sections = HashMap::new();
        let deleted_plate_sections = HashMap::new();

        let new_properties = HashMap::new();
        let deleted_new_properties = HashMap::new();

        let beam_sections_local_axis_1_directions = Vec::new();
        let deleted_beam_sections_local_axis_1_directions = HashMap::new();

        let concentrated_loads = HashMap::new();
        let deleted_concentrated_loads = HashMap::new();

        let point_boundary_conditions = HashMap::new();
        let deleted_point_boundary_conditions = HashMap::new();
        Preprocessor 
        { 
            props, 

            points, 
            deleted_points, 

            lines, changed_lines, deleted_lines,

            surfaces, changed_surfaces, deleted_surfaces,

            materials, deleted_materials,

            truss_sections, deleted_truss_sections,

            beam_sections, deleted_beam_sections,

            plate_sections, deleted_plate_sections,

            new_properties, deleted_new_properties,

            beam_sections_local_axis_1_directions, deleted_beam_sections_local_axis_1_directions,

            concentrated_loads, deleted_concentrated_loads,

            point_boundary_conditions, deleted_point_boundary_conditions,

            global_mesh_seed_value: init_global_mesh_seed_value,
        }
    }


    pub fn reset(&mut self) 
    {
        self.points = HashMap::new();
        self.deleted_points = HashMap::new();
        self.lines = HashMap::new();
        self.changed_lines = HashMap::new();
        self.deleted_lines = HashMap::new();
        self.surfaces = HashMap::new();
        self.deleted_surfaces = HashMap::new();
        self.changed_surfaces = HashMap::new();
        self.materials = HashMap::new();
        self.deleted_materials = HashMap::new();
        self.truss_sections = HashMap::new();
        self.deleted_truss_sections = HashMap::new();
        self.beam_sections = HashMap::new();
        self.deleted_beam_sections = HashMap::new();
        self.plate_sections = HashMap::new();
        self.deleted_plate_sections = HashMap::new();
        self.new_properties = HashMap::new();
        self.deleted_new_properties = HashMap::new();
        self.beam_sections_local_axis_1_directions = Vec::new();
        self.deleted_beam_sections_local_axis_1_directions = HashMap::new();
        self.concentrated_loads = HashMap::new();
        self.deleted_concentrated_loads = HashMap::new();
        self.point_boundary_conditions = HashMap::new();
        self.deleted_point_boundary_conditions = HashMap::new();
    }


    pub(crate) fn clear_all_deleted_objects_by_action_id(&mut self, action_id: u32)
    {
        clear_deleted_objects_by_action_id(&mut self.deleted_points, action_id);
        clear_deleted_objects_by_action_id(&mut self.deleted_lines, action_id);
        clear_deleted_objects_by_action_id(&mut self.deleted_surfaces, action_id);
        clear_deleted_objects_by_action_id(&mut self.deleted_materials, action_id);
        clear_deleted_objects_by_action_id(&mut self.deleted_truss_sections, action_id);
        clear_deleted_objects_by_action_id(&mut self.deleted_beam_sections, action_id);
        clear_deleted_objects_by_action_id(&mut self.deleted_plate_sections, action_id);
        clear_deleted_objects_by_action_id(&mut self.deleted_new_properties, action_id);
        clear_deleted_objects_by_action_id(&mut self.deleted_beam_sections_local_axis_1_directions, action_id);
        clear_deleted_objects_by_action_id(&mut self.deleted_concentrated_loads, action_id);
        clear_deleted_objects_by_action_id(&mut self.deleted_point_boundary_conditions, action_id);
    }


    fn extract_trusses_and_beams_for_mesher(
        &self, 
        points: &mut HashMap<u32, [FEFloat; 3]>,
        trusses: &mut HashMap<
            u32, 
            (
                u32, 
                u32, 
                FEFloat, 
                FEFloat, 
                Option<FEFloat>,
            )
        >,
        beams: &mut HashMap<
            u32, 
            (
                u32, 
                u32, 
                FEFloat, 
                FEFloat, 
                FEFloat, 
                FEFloat, 
                FEFloat, 
                FEFloat, 
                FEFloat, 
                FEFloat, 
                FEFloat, 
                FEFloat, 
                FEFloat, 
                u8,
            )
        >,
        uniformly_distributed_line_loads: &mut HashMap<u32, [FEFloat; 3]>
    ) 
        -> Result<(), JsValue>
    {
        let error_message_header = "Preprocessor: Extract trusses and beams action";

        for (line_number, line) in self.lines.iter()
        {
            if let Some(extended_property) = line.get_ref_optional_property()
            {
                let property_name = extended_property.get_name();
                if let Some(property) = self.new_properties.get(&property_name) 
                {
                    let (material_name, cross_section) = property.get_data();
                    if let Some(material) = self.materials.get(&material_name)
                    {
                        match cross_section
                        {
                            CrossSection::Truss(name) => 
                            {
                                if let Some(truss_section) = self.truss_sections.get(&name)
                                {
                                    let point_numbers = line.get_point_numbers();
                                    if let [Some(point_1), Some(point_2)] = [
                                        self.points.get(&point_numbers[0]), self.points.get(&point_numbers[1])]
                                    {
                                        if line.is_local_axis_1_direction_assigned()
                                        {
                                            let error_message = &format!("{}: Local axis 1 direction could \
                                                not be assigned to line {}!", error_message_header, line_number);
                                            return Err(JsValue::from(error_message));
                                        }
                                        if line.is_uniformly_distributed_line_load_assigned()
                                        {
                                            let error_message = &format!("{}: Uniformly distributed line load \
                                                not be applied to line {}!", error_message_header, line_number);
                                            return Err(JsValue::from(error_message));
                                        }
                                        let point_1_coordinates = point_1.get_coordinates();
                                        points.insert(point_numbers[0], point_1_coordinates);
                                        let point_2_coordinates = point_2.get_coordinates();
                                        points.insert(point_numbers[1], point_2_coordinates);
                                        let [young_modulus, _poisson_ratio] = material.get_data();
                                        let (area, area_2) = truss_section.get_data();
                                        trusses.insert(*line_number, (point_numbers[0], point_numbers[1], 
                                            young_modulus, area, area_2));
                                    }
                                    else
                                    {
                                        let error_message = &format!("{}: Some point from {:?} does not exist!",
                                            error_message_header, point_numbers);
                                        return Err(JsValue::from(error_message));
                                    }
                                }
                                else
                                {
                                    let error_message = &format!("{}: {} Truss section does not exist!",
                                        error_message_header, name);
                                    return Err(JsValue::from(error_message));
                                }

                            },
                            CrossSection::Beam(name) => 
                            {
                                if let Some(beam_section) = self.beam_sections.get(&name)
                                {
                                    let point_numbers = line.get_point_numbers();
                                    if let [Some(point_1), Some(point_2)] = [
                                        self.points.get(&point_numbers[0]), self.points.get(&point_numbers[1])]
                                    {
                                        if let Some(transformed_local_axis_1_direction) = 
                                            line.get_ref_optional_transformed_local_axis_1_direction()
                                        {
                                            if let Some(mesh_seed) = line.get_ref_optional_mesh_seed()
                                            {
                                                match mesh_seed
                                                {
                                                    NewMeshSeed::Global(mesh_seed_value) | 
                                                    NewMeshSeed::Line(mesh_seed_value) =>
                                                    {
                                                        if let Some(uniformly_distributed_line_load) = 
                                                            line.get_ref_optional_uniformly_distributed_line_load()
                                                        {
                                                            let uniformly_distributed_line_load_components = 
                                                                uniformly_distributed_line_load.get_load_components();
                                                            uniformly_distributed_line_loads.insert(*line_number, 
                                                                uniformly_distributed_line_load_components);
                                                        }
                                                        let point_1_coordinates = point_1.get_coordinates();
                                                        points.insert(point_numbers[0], point_1_coordinates);
                                                        let point_2_coordinates = point_2.get_coordinates();
                                                        points.insert(point_numbers[1], point_2_coordinates);
                                                        let [young_modulus, poisson_ratio] = material.get_data();
                                                        let [area, i11, i22, i12, it, shear_factor] = 
                                                            beam_section.get_data();
                                                        let transformed_local_axis_1_direction_components = 
                                                            transformed_local_axis_1_direction.get_components();

                                                        beams.insert(*line_number, 
                                                            (
                                                                point_numbers[0], 
                                                                point_numbers[1], 
                                                                young_modulus,
                                                                poisson_ratio,
                                                                area,
                                                                i11,
                                                                i22,
                                                                i12,
                                                                it,
                                                                shear_factor,
                                                                transformed_local_axis_1_direction_components[0],
                                                                transformed_local_axis_1_direction_components[1],
                                                                transformed_local_axis_1_direction_components[2],
                                                                *mesh_seed_value,
                                                            )
                                                        );
                                                    },
                                                    NewMeshSeed::Surface(_, _) =>
                                                    {
                                                        let error_message = &format!("{}: Surface mesh seed \
                                                            should be assigned to line {}!", error_message_header, 
                                                            line_number);
                                                        return Err(JsValue::from(error_message));
                                                    }
                                                }
                                            }
                                            else
                                            {
                                                let error_message = &format!("{}: Mesh seed should \
                                                    be assigned to line {}!", error_message_header, line_number);
                                                return Err(JsValue::from(error_message));
                                            }
                                        }
                                        else
                                        {
                                            let error_message = &format!("{}: Local axis 1 direction should \
                                                be assigned to line {}!", error_message_header, line_number);
                                            return Err(JsValue::from(error_message));
                                        }
                                    }
                                    else
                                    {
                                        let error_message = &format!("{}: Some point from {:?} does not exist!",
                                            error_message_header, point_numbers);
                                        return Err(JsValue::from(error_message));
                                    }
                                }
                                else
                                {
                                    let error_message = &format!("{}: {} Truss section does not exist!",
                                        error_message_header, name);
                                    return Err(JsValue::from(error_message));
                                }
                            },
                            CrossSection::Plate(name) =>
                            {
                                let error_message = &format!("{}: {} {} Incorrect cross section applied \
                                    to line!", error_message_header, name, line_number);
                                return Err(JsValue::from(error_message));
                            }
                        }
                    }
                    else 
                    {
                        let error_message = &format!("{}: {} Material does not exist!",
                            error_message_header, material_name);
                        return Err(JsValue::from(error_message));
                    }
                }
                else
                {
                    let error_message = &format!("{}: {} Property does not exist!",
                        error_message_header, property_name);
                    return Err(JsValue::from(error_message));
                }
            }
        }
        Ok(())
    }


    fn extract_plates_for_mesher(
        &self, 
        points: &mut HashMap<u32, [FEFloat; 3]>,
        plates: &mut HashMap<u32,
            (
                u32,
                u32,
                u32,
                u32,
                FEFloat, 
                FEFloat, 
                FEFloat, 
                FEFloat, 
                u8,
                u8,
            )
        >,
        uniformly_distributed_surface_loads: &mut HashMap<u32, [FEFloat; 3]>
    ) 
        -> Result<(), JsValue>
    {
        let error_message_header = "Preprocessor: Extract plates action";

        for (surface_number, surface) in self.surfaces.iter()
        {
            if let Some(extended_property) = surface.get_ref_optional_property()
            {
                let property_name = extended_property.get_name();
                if let Some(property) = self.new_properties.get(&property_name) 
                {
                    let (material_name, cross_section) = property.get_data();
                    if let Some(material) = self.materials.get(&material_name)
                    {
                        match cross_section
                        {
                            CrossSection::Plate(name) => 
                            {
                                if let Some(plate_section) = self.plate_sections.get(&name)
                                {
                                    let point_numbers = surface.get_point_numbers();
                                    if let [Some(point_1), Some(point_2), Some(point_3), 
                                        Some(point_4)] = [self.points.get(&point_numbers[0]), 
                                        self.points.get(&point_numbers[1]), self.points.get(&point_numbers[2]),
                                        self.points.get(&point_numbers[3])]
                                    {
                                        if let Some(mesh_seed) = surface.get_ref_optional_mesh_seed()
                                        {
                                            let (edges_1_3_mesh_seed_value, edges_2_4_mesh_seed_value) = 
                                                match mesh_seed
                                                {
                                                    NewMeshSeed::Global(mesh_seed_value) =>
                                                    {
                                                        Ok((mesh_seed_value, mesh_seed_value))
                                                    } 
                                                    NewMeshSeed::Line(_) =>
                                                    {
                                                        let error_message = &format!("{}: Line mesh seed \
                                                            should be assigned to surface {}!", error_message_header, 
                                                            surface_number);
                                                        Err(JsValue::from(error_message))
                                                    },
                                                    NewMeshSeed::Surface(edges_1_3_value, edges_2_4_value) =>
                                                    {
                                                        Ok((edges_1_3_value, edges_2_4_value))
                                                    }
                                                }?;
                                            if let Some(uniformly_distributed_surface_load) = 
                                                surface.get_ref_optional_uniformly_distributed_surface_load()
                                            {
                                                let uniformly_distributed_surface_load_components = 
                                                    uniformly_distributed_surface_load.get_load_components();
                                                uniformly_distributed_surface_loads.insert(*surface_number, 
                                                    uniformly_distributed_surface_load_components);
                                            }
                                            let point_1_coordinates = point_1.get_coordinates();
                                            points.insert(point_numbers[0], point_1_coordinates);
                                            let point_2_coordinates = point_2.get_coordinates();
                                            points.insert(point_numbers[1], point_2_coordinates);
                                            let point_3_coordinates = point_3.get_coordinates();
                                            points.insert(point_numbers[2], point_3_coordinates);
                                            let point_4_coordinates = point_4.get_coordinates();
                                            points.insert(point_numbers[3], point_4_coordinates);
                                            let [young_modulus, poisson_ratio] = material.get_data();
                                            let [thickness, shear_factor] = plate_section.get_data();
                                            plates.insert(*surface_number, 
                                                (
                                                    point_numbers[0], 
                                                    point_numbers[1], 
                                                    point_numbers[2], 
                                                    point_numbers[3], 
                                                    young_modulus,
                                                    poisson_ratio,
                                                    thickness,
                                                    shear_factor,
                                                    *edges_1_3_mesh_seed_value,
                                                    *edges_2_4_mesh_seed_value,
                                                )
                                            );
                                        }
                                        else
                                        {
                                            let error_message = &format!("{}: Mesh seed should \
                                                be assigned to surface {}!", error_message_header, surface_number);
                                            return Err(JsValue::from(error_message));
                                        }
                                    }
                                    else
                                    {
                                        let error_message = &format!("{}: Some point from {:?} does not exist!",
                                            error_message_header, point_numbers);
                                        return Err(JsValue::from(error_message));
                                    }
                                }
                                else
                                {
                                    let error_message = &format!("{}: {} Truss section does not exist!",
                                        error_message_header, name);
                                    return Err(JsValue::from(error_message));
                                }
                            },
                            CrossSection::Truss(name) | CrossSection::Beam(name) =>
                            {
                                let error_message = &format!("{}: {} {} Incorrect cross section applied \
                                    to surface!", error_message_header, name, surface_number);
                                return Err(JsValue::from(error_message));
                            }
                        }
                    }
                    else 
                    {
                        let error_message = &format!("{}: {} Material does not exist!",
                            error_message_header, material_name);
                        return Err(JsValue::from(error_message));
                    }
                }
                else
                {
                    let error_message = &format!("{}: {} Property does not exist!",
                        error_message_header, property_name);
                    return Err(JsValue::from(error_message));
                }
            }
        }
        Ok(())
    }


    fn extract_concentrated_loads_for_mesher(&self, points: &HashMap<u32, [FEFloat; 3]>,
        concentrated_loads: &mut HashMap<u32, [FEFloat; 6]>) -> Result<(), JsValue>
    {
        for (point_number, concentrated_load) in self.concentrated_loads.iter()
        {
            if points.get(point_number).is_none()
            {
                let error_message = &format!("Preprocessor: Extract concentrated loads action: \
                    Concentrated load could not be applied to point {point_number} of structure which does not contain \
                    any properties!");
                return Err(JsValue::from(error_message));
            }
            let load_components = concentrated_load.get_load_components();
            concentrated_loads.insert(*point_number, load_components);
        }
        Ok(())
    }


    fn extract_points_boundary_conditions_for_mesher(&self, points: &HashMap<u32, [FEFloat; 3]>,
        point_boundary_conditions: &mut HashMap<u32, [Option<FEFloat>; 6]>) -> Result<(), JsValue>
    {
        for (point_number, point_boundary_condition) in self.point_boundary_conditions.iter()
        {
            if points.get(point_number).is_none()
            {
                let error_message = &format!("Preprocessor: Extract point boundary conditions action: \
                    Boundary condition could not be applied to point {point_number} of structure which does not contain \
                    any properties!");
                return Err(JsValue::from(error_message));
            }
            let displacement_components = 
                point_boundary_condition.get_displacement_components();
            point_boundary_conditions.insert(*point_number, displacement_components);
        }
        Ok(())
    }


    pub fn extract_data_for_mesher(&self) -> Result<JsValue, JsValue>
    {
        let mut points = HashMap::new();
        let mut trusses = HashMap::new();
        let mut beams = HashMap::new();
        let mut uniformly_distributed_line_loads = HashMap::new();
        let mut plates = HashMap::new();
        let mut uniformly_distributed_surface_loads = HashMap::new();
        let mut concentrated_loads = HashMap::new();
        let mut point_boundary_conditions = HashMap::new();

        self.extract_trusses_and_beams_for_mesher(&mut points, &mut trusses, &mut beams, 
            &mut uniformly_distributed_line_loads)?;
        self.extract_plates_for_mesher(&mut points, &mut plates, &mut uniformly_distributed_surface_loads)?;
        self.extract_concentrated_loads_for_mesher(&points, &mut concentrated_loads)?;
        self.extract_points_boundary_conditions_for_mesher(&points, &mut point_boundary_conditions)?;

        let extracted_data = json!({ 
            "points": points,
            "trusses": trusses,
            "beams": beams,
            "uniformly_distributed_line_loads": uniformly_distributed_line_loads,
            "plates": plates,
            "uniformly_distributed_surface_loads": uniformly_distributed_surface_loads,
            "concentrated_loads": concentrated_loads,
            "point_boundary_conditions": point_boundary_conditions,
        });
        let serializer = Serializer::json_compatible();
        let composed_extracted_data =
            extracted_data.serialize(&serializer)
                .or(Err(JsValue::from("Preprocessor: Extract data for mesher: Data could not be \
                    composed for extraction!")))?;
        Ok(composed_extracted_data)
    }


    fn get_existed_uids(&self) -> Vec<u32>
    {
        let mut existed_uids = Vec::new();
        for point in self.points.values()
        {
            let uid = point.get_uid();
            existed_uids.push(uid);
        }
        for point in self.deleted_points.values()
        {
            let uid = point.get_uid();
            existed_uids.push(uid);
        }
        for line in self.lines.values()
        {
            let uid = line.get_uid();
            existed_uids.push(uid);
            if let Some(uniformly_distributed_line_load) = 
                line.get_ref_optional_uniformly_distributed_line_load()
            {
                let uid = uniformly_distributed_line_load.get_uid();
                existed_uids.push(uid);
            }
        }
        for lines in self.deleted_lines.values()
        {
            for line in lines.iter()
            {
                let uid = line.get_uid();
                existed_uids.push(uid);
                if let Some(uniformly_distributed_line_load) = 
                    line.get_ref_optional_uniformly_distributed_line_load()
                {
                    let uid = uniformly_distributed_line_load.get_uid();
                    existed_uids.push(uid);
                }
            }
        }
        for surface in self.surfaces.values()
        {
            let uid = surface.get_uid();
            existed_uids.push(uid);
            if let Some(uniformly_distributed_surface_load) = 
                surface.get_ref_optional_uniformly_distributed_surface_load()
            {
                let uid = uniformly_distributed_surface_load.get_uid();
                existed_uids.push(uid);
            }
        }
        for surfaces in self.deleted_surfaces.values()
        {
            for surface in surfaces.iter()
            {
                let uid = surface.get_uid();
                existed_uids.push(uid);
                if let Some(uniformly_distributed_surface_load) = 
                    surface.get_ref_optional_uniformly_distributed_surface_load()
                {
                    let uid = uniformly_distributed_surface_load.get_uid();
                    existed_uids.push(uid);
                }
            }
        }
        for concenctrated_load in self.concentrated_loads.values()
        {
            let uid = concenctrated_load.get_uid();
            existed_uids.push(uid);
        }
        for concentrated_load in self.deleted_concentrated_loads.values()
        {
            let uid = concentrated_load.get_uid();
            existed_uids.push(uid);
        }
        for point_boundary_condition in self.point_boundary_conditions.values()
        {
            let uid = point_boundary_condition.get_uid();
            existed_uids.push(uid);
        }
        for point_boundary_condition in self.deleted_point_boundary_conditions.values()
        {
            let uid = point_boundary_condition.get_uid();
            existed_uids.push(uid);
        }
        existed_uids
    }


    pub(crate) fn generate_uid(&self) -> u32
    {
        let existed_uids = self.get_existed_uids();
        
        {
            let mut current_uid = rand::random::<u32>();
            while existed_uids.contains(&current_uid)
            {
                current_uid = rand::random::<u32>();
            }
            current_uid
        }
    }


    pub(crate) fn logging(&self)
    {
        log(&format!("Geometry: \n
            Points: {:?}, \n
            Deleted points: {:?}, \n
            Lines: {:?}, \n
            Changed lines: {:?}, \n
            Deleted lines: {:?}, \n
            Surfaces: {:?}, \n
            Changed surfaces: {:?}, \n
            Deleted surfaces: {:?}, \n
            Materials: {:?}, \n
            Deleted materials: {:?}, \n
            Truss sections: {:?}, \n
            Deleted truss sections: {:?}, \n
            Beam sections: {:?}, \n
            Deleted beam sections: {:?}, \n
            Plate sections: {:?}, \n
            Deleted plate sections: {:?}, \n
            Properties: {:?}, \n
            Deleted properties: {:?}, \n
            Beam sections local axis 1 directions: {:?}, \n
            Deleted beam sections local axis 1 directions: {:?}, \n
            Concentrated loads: {:?}, \n
            Deleted concentrated loads: {:?}, \n
            Point boundary conditions: {:?}, \n
            Deleted point boundary conditions: {:?}, \n
            Global mesh seed value: {}, \n",

            self.points,
            self.deleted_points,
            self.lines,
            self.changed_lines,
            self.deleted_lines,
            self.surfaces,
            self.changed_surfaces,
            self.deleted_surfaces,
            self.materials,
            self.deleted_materials,
            self.truss_sections,
            self.deleted_truss_sections,
            self.beam_sections,
            self.deleted_beam_sections,
            self.plate_sections,
            self.deleted_plate_sections,
            self.new_properties,
            self.deleted_new_properties,
            self.beam_sections_local_axis_1_directions,
            self.deleted_beam_sections_local_axis_1_directions,
            self.concentrated_loads,
            self.deleted_concentrated_loads,
            self.point_boundary_conditions,
            self.deleted_point_boundary_conditions,
            self.global_mesh_seed_value)
        );
    }
}
