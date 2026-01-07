use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::JsValue;
use serde_json::json;

use crate::enums::
{
    SceneState, GLMode, LineObjectType, SurfaceObjectType, MeshSeedType,
};
use crate::structs::
{
    BoundingBox, Primitives, Point, DenotationsData, Manipulation, Line, Surface, ConcentratedLoad,
    UniformlyDistributedLineLoad, UniformlyDistributedSurfaceLoad, PointBoundaryCondition, Node, NodeData,
    TrussElement, TrussElementData, BeamElement, BeamElementData, PlateElementData, PlateElement,
};
use crate::Props;
use crate::functions::
{
    transform_u32_to_array_of_u8, dispatch_custom_event, move_selected_objects_into_regular, 
    move_rc_selected_objects_into_rc_regular, move_regular_object_into_selected_objects, 
    move_rc_regular_object_into_rc_selected_objects,
};
use crate::traits::{DenotationTrait, RotationMatrixTrait, LineElementDataTrait, QuadElementDataTrait};


pub struct Scene
{
    scene_state: SceneState,
    bounding_box: BoundingBox,

    pub(super) points: HashMap<[u8; 4], Point>,
    pub(super) selected_points: HashMap<[u8; 4], Point>,
    pub(super) lines: HashMap<[u8; 4], Line>,
    pub(super) selected_lines: HashMap<[u8; 4], Line>,
    pub(super) surfaces: HashMap<[u8; 4], Surface>,
    pub(super) selected_surfaces: HashMap<[u8; 4], Surface>,
    pub(super) concentrated_loads: HashMap<[u8; 4], ConcentratedLoad>,
    pub(super) selected_concentrated_loads: HashMap<[u8; 4], ConcentratedLoad>,
    pub(super) uniformly_distributed_line_loads: HashMap<[u8; 4], Rc<RefCell<UniformlyDistributedLineLoad>>>,
    pub(super) selected_uniformly_distributed_line_loads: HashMap<[u8; 4], Rc<RefCell<UniformlyDistributedLineLoad>>>,
    pub(super) uniformly_distributed_surface_loads: HashMap<[u8; 4], Rc<RefCell<UniformlyDistributedSurfaceLoad>>>,
    pub(super) selected_uniformly_distributed_surface_loads: HashMap<[u8; 4], Rc<RefCell<UniformlyDistributedSurfaceLoad>>>,
    pub(super) point_boundary_conditions: HashMap<[u8; 4], PointBoundaryCondition>,
    pub(super) selected_point_boundary_conditions: HashMap<[u8; 4], PointBoundaryCondition>,

    pub(super) nodes: HashMap<[u8; 4], Node>,
    pub(super) selected_nodes: HashMap<[u8; 4], Node>,
    pub(super) truss_elements: HashMap<[u8; 4], TrussElement>,
    pub(super) selected_truss_elements: HashMap<[u8; 4], TrussElement>,
    pub(super) beam_elements: HashMap<[u8; 4], BeamElement>,
    pub(super) selected_beam_elements: HashMap<[u8; 4], BeamElement>,
    pub(super) plate_elements: HashMap<[u8; 4], PlateElement>,
    pub(super) selected_plate_elements: HashMap<[u8; 4], PlateElement>,
}


impl Scene
{
    pub fn create() -> Self
    {
        Scene 
        { 
            scene_state: SceneState::Preprocessor, 
            bounding_box: BoundingBox::create(),

            points: HashMap::new(),
            selected_points: HashMap::new(), 
            lines: HashMap::new(), 
            selected_lines: HashMap::new(),
            surfaces: HashMap::new(), 
            selected_surfaces: HashMap::new(), 
            concentrated_loads: HashMap::new(),
            selected_concentrated_loads: HashMap::new(), 
            uniformly_distributed_line_loads: HashMap::new(),
            selected_uniformly_distributed_line_loads: HashMap::new(), 
            uniformly_distributed_surface_loads: HashMap::new(), 
            selected_uniformly_distributed_surface_loads: HashMap::new(), 
            point_boundary_conditions: HashMap::new(),
            selected_point_boundary_conditions: HashMap::new(),

            nodes: HashMap::new(),
            selected_nodes: HashMap::new(), 
            truss_elements: HashMap::new(),
            selected_truss_elements: HashMap::new(),
            beam_elements: HashMap::new(),
            selected_beam_elements: HashMap::new(),
            plate_elements: HashMap::new(),
            selected_plate_elements: HashMap::new(),
        }
    }


    fn reset_bounds(&mut self)
    {
        self.bounding_box.reset();
    }


    fn redefine_bounds(&mut self)
    {
        self.reset_bounds();
        for point in self.points.values()
        {
            let coordinates = point.get_center();
            self.bounding_box.expand_bounds(&coordinates);
        }
        for selected_point in self.selected_points.values()
        {
            let coordinates = selected_point.get_center();
            self.bounding_box.expand_bounds(&coordinates);
        }
    }


    pub fn get_scale(&self) -> f32
    {
        self.bounding_box.get_scale()
    }


    pub fn get_center(&self) -> [f32; 3]
    {
        self.bounding_box.get_center()
    }


    pub fn get_scene_state(&self) -> SceneState
    {
        self.scene_state
    }


    pub fn get_mut_ref_scene_state(&mut self) -> &mut SceneState
    {
        &mut self.scene_state
    }


    pub fn set_scene_state(&mut self, scene_state: SceneState)
    {
        self.scene_state = scene_state;
    }


    pub fn add_point(&mut self, number: u32, uid: u32, x: f32, y: f32, z: f32, props: &Props) -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor => 
            {
                self.bounding_box.expand_bounds(&[x, y, z]);
                let transformed_uid = transform_u32_to_array_of_u8(uid);
                let point = Point::create(transformed_uid, number, x, y, z, props);
                self.points.insert(transformed_uid, point);
                Ok(())
            },
            SceneState::Postprocessor(_) => 
            {
                Err(JsValue::from("Could not add point to postprocessor!"))
            }
        }
    }


    pub fn update_point(&mut self, number: u32, uid: u32, x: f32, y: f32, z: f32, props: &Props) -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor => 
            {
                let transformed_uid = transform_u32_to_array_of_u8(uid);
                let optional_point = 
                    if let Some(point) = self.points.get_mut(&transformed_uid) 
                    {
                        Some(point) 
                    }
                    else { self.selected_points.get_mut(&transformed_uid) };

                if let Some(point) = optional_point
                {
                    point.update_coordinates(x, y, z);
                    self.redefine_bounds();
                    for line in self.lines.values_mut()
                    {
                        line.update_endpoint_coordinates(number, [x, y, z], props);
                    }
                    for line in self.selected_lines.values_mut()
                    {
                        line.update_endpoint_coordinates(number, [x, y, z], props);
                    }
                    
                    for surface in self.surfaces.values_mut()
                    {
                        surface.update_vertex_point_coordinates(number, [x, y, z], 
                            props);
                    }
                    for surface in self.selected_surfaces.values_mut()
                    {
                        surface.update_vertex_point_coordinates(number, [x, y, z], 
                            props);
                    }

                    for concentrated_load in self.concentrated_loads.values_mut()
                    {
                        concentrated_load.update_point_coordinates(number, [x, y, z]);
                    }
                    for concentrated_load in self.selected_concentrated_loads.values_mut()
                    {
                        concentrated_load.update_point_coordinates(number, [x, y, z]);
                    }

                    for point_boundary_condition in 
                        self.point_boundary_conditions.values_mut()
                    {
                        point_boundary_condition.update_point_coordinates(number, 
                            [x, y, z]);
                    }
                    for point_boundary_condition in 
                        self.selected_point_boundary_conditions.values_mut()
                    {
                        point_boundary_condition.update_point_coordinates(number, 
                            [x, y, z]);
                    }
                }
                else
                {
                    let error_message = format!("Point with number {number} does not exist!");
                    return Err(JsValue::from(error_message));
                }
                Ok(())
            },
            SceneState::Postprocessor(_) => 
            {
                Err(JsValue::from("Could not update point in postprocessor!"))
            }
        }
    }


    pub fn delete_point(&mut self, number: u32, uid: u32) -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor => 
            {
                let transformed_uid = transform_u32_to_array_of_u8(uid);

                if self.points.remove(&transformed_uid).is_some() || 
                    self.selected_points.remove(&transformed_uid).is_some()
                {
                    self.redefine_bounds();
                }
                else
                {
                    let error_message = format!("Point with number {number} does not exist!");
                    return Err(JsValue::from(error_message));
                }
                Ok(())
            },
            SceneState::Postprocessor(_) => 
            {
                Err(JsValue::from("Could not delete point from postprocessor!"))
            }
        }
    }


    fn get_point_coordinates(&self, number: u32) -> Result<[f32; 3], JsValue>
    {
        if let Some(point) = self.points.values().find(|p| p.get_number() == number)
        {
            let coordinates = point.get_coordinates();
            return Ok(coordinates);
        }
        if let Some(point) = self.selected_points.values().find(|p| p.get_number() == number)
        {
            let coordinates = point.get_coordinates();
            return Ok(coordinates);
        }
        Err(JsValue::from(&format!("Point with number {number} does not exist!")))
    }


    pub fn add_line(
        &mut self,
        number: u32,
        uid: u32,
        point_1_number: u32,
        point_2_number: u32, 
        optional_transformed_local_axis_1_direction: Option<Box<[f32]>>,
        line_object_type: LineObjectType, 
        line_mesh_seed_value: Option<u8>,
        mesh_seed_type: MeshSeedType, 
        optional_uniformly_distributed_line_load_uid: Option<u32>, 
        optional_uniformly_distributed_line_load_components: Option<Box<[f32]>>,
        props: &Props,
    ) 
        -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor => 
            {
                let transformed_uid = transform_u32_to_array_of_u8(uid);
                let point_1_coordinates = self.get_point_coordinates(point_1_number)?;
                let point_2_coordinates = self.get_point_coordinates(point_2_number)?;
                let mut line = Line::create(transformed_uid, number, 
                    point_1_number, point_2_number, 
                    point_1_coordinates, point_2_coordinates, line_object_type, 
                    None, None, props);
                line.update_line_local_axis_1_direction(optional_transformed_local_axis_1_direction, props)?;
                line.update_line_mesh_seed(line_mesh_seed_value, mesh_seed_type, props);
                if let (Some(uniformly_distributed_line_load_uid), 
                    Some(uniformly_distributed_line_load_components)) = 
                    (optional_uniformly_distributed_line_load_uid, optional_uniformly_distributed_line_load_components)
                {
                    let transformed_uniformly_distributed_line_load_uid = 
                        transform_u32_to_array_of_u8(uniformly_distributed_line_load_uid);
                    let [qx, qy, qz] = [uniformly_distributed_line_load_components[0],
                        uniformly_distributed_line_load_components[1], uniformly_distributed_line_load_components[2]];
                    line.update_uniformly_distributed_line_load(
                        transformed_uniformly_distributed_line_load_uid, point_1_coordinates, 
                        point_2_coordinates, qx, qy, qz, props)?;

                    if let Some(uniformly_distributed_line_load) = 
                        line.get_ref_uniformly_distributed_line_load()
                    {
                        self.uniformly_distributed_line_loads.insert(transformed_uniformly_distributed_line_load_uid, 
                            uniformly_distributed_line_load);
                    }
                }
                self.lines.insert(transformed_uid, line);
                Ok(())
            },
            SceneState::Postprocessor(_) => 
            {
                Err(JsValue::from("Could not add line to postprocessor!"))
            }
        }
    }


    pub fn update_line(
        &mut self,
        number: u32,
        uid: u32,
        point_1_number: u32,
        point_2_number: u32,
        optional_transformed_local_axis_1_direction: Option<Box<[f32]>>,
        line_object_type: LineObjectType, 
        line_mesh_seed_value: Option<u8>,
        mesh_seed_type: MeshSeedType, 
        optional_uniformly_distributed_line_load_uid: Option<u32>, 
        optional_uniformly_distributed_line_load_components: Option<Box<[f32]>>,
        props: &Props,
    ) 
        -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor => 
            {
                let transformed_uid = transform_u32_to_array_of_u8(uid);
                let point_1_coordinates = self.get_point_coordinates(point_1_number)?;
                let point_2_coordinates = self.get_point_coordinates(point_2_number)?;
                let optional_line = 
                    if let Some(line) = self.lines.get_mut(&transformed_uid)
                    {
                        Some(line)
                    }   
                    else { self.selected_lines.get_mut(&transformed_uid) };
                    
                if let Some(line) = optional_line
                {
                    line.update_endpoints(point_1_number, point_2_number, 
                        point_1_coordinates, point_2_coordinates);
                    line.update_line_object_type(line_object_type, props);
                    line.update_line_local_axis_1_direction(optional_transformed_local_axis_1_direction, props)?;
                    line.update_line_mesh_seed(line_mesh_seed_value, mesh_seed_type, props);
                    if let (Some(uniformly_distributed_line_load_uid), 
                        Some(uniformly_distributed_line_load_components)) = 
                        (optional_uniformly_distributed_line_load_uid, optional_uniformly_distributed_line_load_components)
                    {
                        let transformed_uniformly_distributed_line_load_uid = 
                            transform_u32_to_array_of_u8(uniformly_distributed_line_load_uid);
                        let [qx, qy, qz] = [uniformly_distributed_line_load_components[0],
                            uniformly_distributed_line_load_components[1], uniformly_distributed_line_load_components[2]];
                        line.update_uniformly_distributed_line_load(
                            transformed_uniformly_distributed_line_load_uid, point_1_coordinates, 
                            point_2_coordinates, qx, qy, qz, props)?;

                        if let Some(uniformly_distributed_line_load) = 
                            line.get_ref_uniformly_distributed_line_load()
                        {
                            self.uniformly_distributed_line_loads.insert(
                                transformed_uniformly_distributed_line_load_uid, 
                                Rc::clone(&uniformly_distributed_line_load));
                            if self.selected_uniformly_distributed_line_loads.contains_key(
                                &transformed_uniformly_distributed_line_load_uid)
                            {
                                self.selected_uniformly_distributed_line_loads.insert(
                                    transformed_uniformly_distributed_line_load_uid, 
                                    uniformly_distributed_line_load);
                            }
                        }
                    }
                    else 
                    if let Some(transformed_uid) = 
                        line.get_optional_uniformly_distributed_line_load_transformed_uid()
                    {
                        let _ = self.uniformly_distributed_line_loads.remove(&transformed_uid);
                        let _ = self.selected_uniformly_distributed_line_loads.remove(&transformed_uid);
                    }
                }
                else
                {
                    let error_message = format!("Line with number {number} does not exist!");
                    return Err(JsValue::from(error_message));
                }
                Ok(())
            },
            SceneState::Postprocessor(_) => 
            {
                Err(JsValue::from("Could not update line in postprocessor!"))
            }
        }
    }


    pub fn delete_line(&mut self, number: u32, uid: u32) -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor => 
            {
                let transformed_uid = transform_u32_to_array_of_u8(uid);

                let optional_line = 
                    {
                        if let Some(line) = self.lines.remove(&transformed_uid)
                        {
                            Some(line)
                        }
                        else { self.selected_lines.remove(&transformed_uid) }
                    };

                if let Some(line) = optional_line
                {
                    if let Some(uniformly_distributed_line_load_transformed_uid) = 
                        line.get_optional_uniformly_distributed_line_load_transformed_uid()
                    {
                        let _ = self.uniformly_distributed_line_loads.remove(
                            &uniformly_distributed_line_load_transformed_uid,
                        );
                        let _ = self.selected_uniformly_distributed_line_loads.remove(
                            &uniformly_distributed_line_load_transformed_uid,
                        );
                    }
                    Ok(())
                }
                else
                {
                    let error_message = format!("Line with number {number} does not exist!");
                    Err(JsValue::from(error_message))
                }
            },
            SceneState::Postprocessor(_) => 
            {
                Err(JsValue::from("Could not delete line from postprocessor!"))
            }
        }
    }


    pub fn add_surface(
        &mut self, number: u32,
        uid: u32,
        point_1_number: u32,
        point_2_number: u32, 
        point_3_number: u32, 
        point_4_number: u32,
        normal: &[f32],
        surface_object_type: SurfaceObjectType, 
        surface_mesh_seed_values: Option<Box<[u8]>>,
        mesh_seed_type: MeshSeedType,
        optional_uniformly_distributed_surface_load_uid: Option<u32>, 
        optional_uniformly_distributed_surface_load_components: Option<Box<[f32]>>,
        props: &Props,
    )
        -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor => 
            {
                let transformed_uid = transform_u32_to_array_of_u8(uid);
                let point_1_coordinates = self.get_point_coordinates(point_1_number)?;
                let point_2_coordinates = self.get_point_coordinates(point_2_number)?;
                let point_3_coordinates = self.get_point_coordinates(point_3_number)?;
                let point_4_coordinates = self.get_point_coordinates(point_4_number)?;
                let mut surface = Surface::create(transformed_uid, number, 
                    point_1_number, point_2_number, point_3_number, point_4_number, 
                    point_1_coordinates, point_2_coordinates, point_3_coordinates, point_4_coordinates,
                    surface_object_type, 
                    None, None, 
                    None, None,
                    props);
                surface.update_surface_normal(normal, props)?;
                surface.update_surface_mesh_seed(surface_mesh_seed_values, mesh_seed_type, props);
                if let (Some(uniformly_distributed_surface_load_uid), 
                    Some(uniformly_distributed_surface_load_components)) = 
                    (optional_uniformly_distributed_surface_load_uid, optional_uniformly_distributed_surface_load_components)
                {
                    let transformed_uniformly_distributed_surface_load_uid = 
                        transform_u32_to_array_of_u8(uniformly_distributed_surface_load_uid);
                    let [px, py, pz] = [uniformly_distributed_surface_load_components[0],
                        uniformly_distributed_surface_load_components[1], 
                        uniformly_distributed_surface_load_components[2]];
                    surface.update_uniformly_distributed_surface_load(
                        transformed_uniformly_distributed_surface_load_uid, point_1_coordinates, 
                        point_2_coordinates, point_3_coordinates, point_4_coordinates, px, py, pz, props)?;

                    if let Some(uniformly_distributed_surface_load) = 
                        surface.get_ref_uniformly_distributed_surface_load()
                    {
                        self.uniformly_distributed_surface_loads.insert(
                            transformed_uniformly_distributed_surface_load_uid, 
                            uniformly_distributed_surface_load);
                    }
                }
                self.surfaces.insert(transformed_uid, surface);
                Ok(())
            },
            SceneState::Postprocessor(_) => 
            {
                Err(JsValue::from("Could not add surface to postprocessor!"))
            }
        }
    }


    pub fn update_surface(
        &mut self,
        number: u32,
        uid: u32,
        point_1_number: u32,
        point_2_number: u32, 
        point_3_number: u32,
        point_4_number: u32,
        normal: &[f32],
        surface_object_type: SurfaceObjectType, 
        surface_mesh_seed_values: Option<Box<[u8]>>,
        mesh_seed_type: MeshSeedType, 
        optional_uniformly_distributed_surface_load_uid: Option<u32>, 
        optional_uniformly_distributed_surface_load_components: Option<Box<[f32]>>,
        props: &Props,
    )
        -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor => 
            {
                let transformed_uid = transform_u32_to_array_of_u8(uid);
                let point_1_coordinates = self.get_point_coordinates(point_1_number)?;
                let point_2_coordinates = self.get_point_coordinates(point_2_number)?;
                let point_3_coordinates = self.get_point_coordinates(point_3_number)?;
                let point_4_coordinates = self.get_point_coordinates(point_4_number)?;
                let optional_surface = 
                    if let Some(surface) = self.surfaces.get_mut(&transformed_uid)
                    {
                        Some(surface)
                    }   
                    else { self.selected_surfaces.get_mut(&transformed_uid) };
                    
                if let Some(surface) = optional_surface
                {
                    surface.update_vertices(point_1_number, point_2_number, point_3_number, point_4_number, 
                        point_1_coordinates, point_2_coordinates, point_3_coordinates, point_4_coordinates);
                    surface.update_surface_object_type(surface_object_type, props);
                    surface.update_surface_normal(normal, props)?;
                    surface.update_surface_mesh_seed(surface_mesh_seed_values, mesh_seed_type, props);
                    if let (Some(uniformly_distributed_surface_load_uid), 
                        Some(uniformly_distributed_surface_load_components)) = 
                        (optional_uniformly_distributed_surface_load_uid, 
                            optional_uniformly_distributed_surface_load_components)
                    {
                        let transformed_uniformly_distributed_surface_load_uid = 
                            transform_u32_to_array_of_u8(uniformly_distributed_surface_load_uid);
                        let [px, py, pz] = [uniformly_distributed_surface_load_components[0],
                            uniformly_distributed_surface_load_components[1], 
                            uniformly_distributed_surface_load_components[2]];
                        surface.update_uniformly_distributed_surface_load(
                            transformed_uniformly_distributed_surface_load_uid, point_1_coordinates, 
                            point_2_coordinates, point_3_coordinates, point_4_coordinates, px, py, pz, props)?;

                        if let Some(uniformly_distributed_surface_load) = 
                            surface.get_ref_uniformly_distributed_surface_load()
                        {
                            self.uniformly_distributed_surface_loads.insert(
                                transformed_uniformly_distributed_surface_load_uid, 
                                Rc::clone(&uniformly_distributed_surface_load));
                            if self.selected_uniformly_distributed_surface_loads.contains_key(
                                &transformed_uniformly_distributed_surface_load_uid)
                            {
                                self.selected_uniformly_distributed_surface_loads.insert(
                                    transformed_uniformly_distributed_surface_load_uid, 
                                    uniformly_distributed_surface_load);
                            }
                        }
                    }
                    else 
                    if let Some(transformed_uid) = 
                        surface.get_optional_uniformly_distributed_surface_load_transformed_uid()
                    {
                        let _ = self.uniformly_distributed_surface_loads.remove(&transformed_uid);
                        let _ = self.selected_uniformly_distributed_surface_loads.remove(&transformed_uid);
                    }
                }
                else
                {
                    let error_message = format!("Surface with number {number} does not exist!");
                    return Err(JsValue::from(error_message));
                }
                Ok(())
            },
            SceneState::Postprocessor(_) => 
            {
                Err(JsValue::from("Could not update surface in postprocessor!"))
            }
        }
    }


    pub fn delete_surface(&mut self, number: u32, uid: u32) -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor => 
            {
                let transformed_uid = transform_u32_to_array_of_u8(uid);

                let optional_surface = 
                    {
                        if let Some(surface) = self.surfaces.remove(&transformed_uid)
                        {
                            Some(surface)
                        }
                        else { self.selected_surfaces.remove(&transformed_uid) }
                    };

                if let Some(surface) = optional_surface
                {
                    if let Some(uniformly_distributed_surface_load_transformed_uid) = 
                        surface.get_optional_uniformly_distributed_surface_load_transformed_uid()
                    {
                        let _ = self.uniformly_distributed_surface_loads.remove(
                            &uniformly_distributed_surface_load_transformed_uid);
                        let _ = self.selected_uniformly_distributed_surface_loads.remove(
                            &uniformly_distributed_surface_load_transformed_uid);
                    }
                    Ok(())
                }
                else
                {
                    let error_message = format!("Surface with number {number} does not exist!");
                    Err(JsValue::from(error_message))
                }
            },
            SceneState::Postprocessor(_) => 
            {
                Err(JsValue::from("Could not delete surface from postprocessor!"))
            }
        }
    }


    pub fn add_concentrated_load(
        &mut self,
        point_number: u32,
        uid: u32,
        fx: f32,
        fy: f32,
        fz: f32,
        mx: f32,
        my: f32, 
        mz: f32,
        props: &Props,
    )
        -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor => 
            {
                let transformed_uid = transform_u32_to_array_of_u8(uid);
                let point_coordinates = self.get_point_coordinates(point_number)?;
                let concentrated_load = ConcentratedLoad::create(transformed_uid, point_number,
                    point_coordinates, fx, fy, fz, mx, my, mz, props)?;
                self.concentrated_loads.insert(transformed_uid, concentrated_load);
                Ok(())
            },
            SceneState::Postprocessor(_) => 
            {
                Err(JsValue::from("Could not add concentrated load to postprocessor!"))
            }
        }
    }


    pub fn update_concentrated_load(
        &mut self,
        point_number: u32,
        uid: u32,
        fx: f32,
        fy: f32,
        fz: f32,
        mx: f32,
        my: f32, 
        mz: f32,
        props: &Props,
    )
        -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor => 
            {
                let transformed_uid = transform_u32_to_array_of_u8(uid);
                let optional_concentrated_load = 
                    if let Some(concentrated_load) = 
                        self.concentrated_loads.get_mut(&transformed_uid)
                    {
                        Some(concentrated_load)
                    }   
                    else { self.selected_concentrated_loads.get_mut(&transformed_uid) };
                    
                if let Some(concentrated_load) = optional_concentrated_load
                {
                    concentrated_load.update_load_components(transformed_uid, fx, fy, fz, mx, my, mz, props)?;
                }
                else
                {
                    let error_message = format!("Concentrated load applied at point with number {point_number} \
                        does not exist!");
                    return Err(JsValue::from(error_message));
                }
                Ok(())
            },
            SceneState::Postprocessor(_) => 
            {
                Err(JsValue::from("Could not update concentrated load in postprocessor!"))
            }
        }
    }


    pub fn delete_concentrated_load(&mut self, point_number: u32, uid: u32) -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor => 
            {
                let transformed_uid = transform_u32_to_array_of_u8(uid);

                if self.concentrated_loads.remove(&transformed_uid).is_some() || 
                    self.selected_concentrated_loads.remove(&transformed_uid).is_some()
                {
                    Ok(())
                }
                else
                {
                    let error_message = format!("Concentrated load applied at point with number {point_number} \
                        does not exist!");
                    Err(JsValue::from(error_message))
                }
            },
            SceneState::Postprocessor(_) => 
            {
                Err(JsValue::from("Could not delete concentrated load from postprocessor!"))
            }
        }
    }


    pub fn add_point_boundary_condition(
        &mut self,
        point_number: u32,
        uid: u32,
        optional_ux: Option<f32>, 
        optional_uy: Option<f32>,
        optional_uz: Option<f32>,
        optional_rx: Option<f32>,
        optional_ry: Option<f32>,
        optional_rz: Option<f32>,
        props: &Props,
    )
        -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor => 
            {
                let transformed_uid = transform_u32_to_array_of_u8(uid);
                let point_coordinates = self.get_point_coordinates(point_number)?;
                let point_boundary_condition = PointBoundaryCondition::create(transformed_uid, 
                    point_number, point_coordinates, optional_ux, optional_uy, optional_uz, optional_rx, optional_ry,
                    optional_rz, props)?;
                self.point_boundary_conditions.insert(transformed_uid, point_boundary_condition);
                Ok(())
            },
            SceneState::Postprocessor(_) => 
            {
                Err(JsValue::from("Could not add point_boundary_condition to postprocessor!"))
            }
        }
    }


    pub fn update_point_boundary_condition(
        &mut self,
        point_number: u32,
        uid: u32,
        optional_ux: Option<f32>, 
        optional_uy: Option<f32>,
        optional_uz: Option<f32>,
        optional_rx: Option<f32>,
        optional_ry: Option<f32>,
        optional_rz: Option<f32>,
        props: &Props,
    )
        -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor => 
            {
                let transformed_uid = transform_u32_to_array_of_u8(uid);
                let optional_point_boundary_condition = 
                    if let Some(point_boundary_condition) = 
                        self.point_boundary_conditions.get_mut(&transformed_uid)
                    {
                        Some(point_boundary_condition)
                    }   
                    else { self.selected_point_boundary_conditions.get_mut(&transformed_uid) };
                    
                if let Some(point_boundary_condition) = optional_point_boundary_condition
                {
                    point_boundary_condition.update_displacement_components(transformed_uid, optional_ux, optional_uy,
                        optional_uz, optional_rx, optional_ry, optional_rz, props)?;
                }
                else
                {
                    let error_message = format!("Point boundary condition applied at point with number \
                        {point_number} does not exist!");
                    return Err(JsValue::from(error_message));
                }
                Ok(())
            },
            SceneState::Postprocessor(_) => 
            {
                Err(JsValue::from("Could not update point boundary condition in postprocessor!"))
            }
        }
    }


    pub fn delete_point_boundary_condition(&mut self, point_number: u32, uid: u32) -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor => 
            {
                let transformed_uid = transform_u32_to_array_of_u8(uid);

                if self.point_boundary_conditions.remove(&transformed_uid).is_some() || 
                    self.selected_point_boundary_conditions.remove(&transformed_uid).is_some()
                {
                    Ok(())
                }
                else
                {
                    let error_message = format!("Point boundary condition applied at point with number \
                        {point_number} does not exist!");
                    Err(JsValue::from(error_message))
                }
            },
            SceneState::Postprocessor(_) => 
            {
                Err(JsValue::from("Could not delete point boundary condition from postprocessor!"))
            }
        }
    }


    pub fn add_nodes(&mut self, converted_nodes_data: HashMap<String, NodeData>, props: &Props) -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor => Err(JsValue::from("Could not add nodes to preprocessor!")),
            SceneState::Postprocessor(_) => 
            {
                self.reset_bounds();
                self.reset_postprocessor_data();
                for node_data in converted_nodes_data.into_values()
                {
                    self.bounding_box.expand_bounds(&[node_data.x, node_data.y, node_data.z]);
                    let transformed_uid = transform_u32_to_array_of_u8(node_data.uid);
                    let node = Node::create(
                        transformed_uid,
                        node_data.x,
                        node_data.y,
                        node_data.z,
                        &[node_data.ux, node_data.uy, node_data.uz],
                        node_data.optional_u_result_coeff,
                        node_data.optional_fx_coeff,
                        node_data.optional_fy_coeff,
                        node_data.optional_fz_coeff,
                        node_data.optional_mx_coeff,
                        node_data.optional_my_coeff,
                        node_data.optional_mz_coeff,
                        props,
                    )?;
                    self.nodes.insert(transformed_uid, node);
                }
                Ok(())
            }
        }
    }


    pub fn add_truss_elements(
        &mut self, converted_truss_elements_data: HashMap<u32, TrussElementData>, props: &Props
    ) 
        -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor => Err(JsValue::from("Could not add truss elements to preprocessor!")),
            SceneState::Postprocessor(_) => 
            {
                for truss_element_data in converted_truss_elements_data.into_values()
                {
                    let transformed_uid = transform_u32_to_array_of_u8(truss_element_data.get_uid());
                    let truss_element = TrussElement::create(
                        transformed_uid,
                        truss_element_data.get_node_1_coordinates(),
                        truss_element_data.get_node_2_coordinates(),
                        &truss_element_data.get_node_1_displacement(),
                        &truss_element_data.get_node_2_displacement(),
                        truss_element_data.get_optional_node_1_u_result_coeff(),
                        truss_element_data.get_optional_node_2_u_result_coeff(),
                        truss_element_data.get_ref_optional_rotation_matrix(),
                        truss_element_data.get_optional_force_r_coeff(),
                        props,
                    )?;
                    self.truss_elements.insert(transformed_uid, truss_element);
                }
                Ok(())
            }
        }
    }


    pub fn add_beam_elements(
        &mut self, converted_beam_elements_data: HashMap<u32, BeamElementData>, props: &Props
    ) 
        -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor => Err(JsValue::from("Could not add beam elements to preprocessor!")),
            SceneState::Postprocessor(_) => 
            {
                for beam_element_data in converted_beam_elements_data.into_values()
                {
                    let transformed_uid = transform_u32_to_array_of_u8(beam_element_data.get_uid());
                    let beam_element = BeamElement::create(
                        transformed_uid,
                        beam_element_data.get_node_1_coordinates(),
                        beam_element_data.get_node_2_coordinates(),
                        &beam_element_data.get_node_1_displacement(),
                        &beam_element_data.get_node_2_displacement(),
                        beam_element_data.get_optional_node_1_u_result_coeff(),
                        beam_element_data.get_optional_node_2_u_result_coeff(),
                        beam_element_data.get_ref_optional_rotation_matrix(),
                        beam_element_data.get_optional_force_r_coeff(),
                        beam_element_data.get_optional_force_s_coeff(),
                        beam_element_data.get_optional_force_t_coeff(),
                        beam_element_data.get_optional_moment_r_coeff(),
                        beam_element_data.get_optional_moment_s_node_1_coeff(),
                        beam_element_data.get_optional_moment_s_coeff(),
                        beam_element_data.get_optional_moment_s_node_2_coeff(),
                        beam_element_data.get_optional_moment_t_node_1_coeff(),
                        beam_element_data.get_optional_moment_t_coeff(),
                        beam_element_data.get_optional_moment_t_node_2_coeff(),
                        props,
                    )?;
                    self.beam_elements.insert(transformed_uid, beam_element);
                }
                Ok(())
            }
        }
    }


    pub fn add_plate_elements(
        &mut self, converted_plate_elements_data: HashMap<u32, PlateElementData>, props: &Props
    ) 
        -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor => Err(JsValue::from("Could not add plate elements to preprocessor!")),
            SceneState::Postprocessor(_) => 
            {
                for plate_element_data in converted_plate_elements_data.into_values()
                {
                    let transformed_uid = transform_u32_to_array_of_u8(plate_element_data.get_uid());
                    let plate_element = PlateElement::create(
                        transformed_uid,
                        plate_element_data.get_node_1_coordinates(),
                        plate_element_data.get_node_2_coordinates(),
                        plate_element_data.get_node_3_coordinates(),
                        plate_element_data.get_node_4_coordinates(),
                        &plate_element_data.get_node_1_displacement(),
                        &plate_element_data.get_node_2_displacement(),
                        &plate_element_data.get_node_3_displacement(),
                        &plate_element_data.get_node_4_displacement(),
                        plate_element_data.get_optional_node_1_u_result_coeff(),
                        plate_element_data.get_optional_node_2_u_result_coeff(),
                        plate_element_data.get_optional_node_3_u_result_coeff(),
                        plate_element_data.get_optional_node_4_u_result_coeff(),
                        plate_element_data.get_ref_optional_rotation_matrix(),
                        plate_element_data.get_optional_mem_force_r_coeff(),
                        plate_element_data.get_optional_mem_force_s_coeff(),
                        plate_element_data.get_optional_mem_force_r_s_coeff(),
                        plate_element_data.get_optional_bend_moment_r_coeff(),
                        plate_element_data.get_optional_bend_moment_s_coeff(),
                        plate_element_data.get_optional_bend_moment_r_s_coeff(),
                        plate_element_data.get_optional_shear_force_r_t_coeff(),
                        plate_element_data.get_optional_shear_force_s_t_coeff(),
                        props,
                    )?;
                    self.plate_elements.insert(transformed_uid, plate_element);
                }
                Ok(())
            }
        }
    }


    pub fn get_primitives(&self, gl_mode: GLMode, manipulation: &Manipulation) -> Primitives
    {
        match self.scene_state
        {
            SceneState::Preprocessor => self.get_preprocessor_primitives(gl_mode, manipulation),
            SceneState::Postprocessor((optional_result_plot, _, _, _)) => 
            {
                self.get_postprocessor_primitives(gl_mode, manipulation, optional_result_plot)
            },
        }
    }


    pub fn get_denotations_data(&self, manipulation: &Manipulation) -> DenotationsData
    {
        match self.scene_state
        {
            SceneState::Preprocessor => self.get_preprocessor_objects_data_for_denotation(manipulation),
            SceneState::Postprocessor(_) => self.get_postprocessor_objects_data_for_denotation(manipulation),
        }
    }


    fn move_objects_into_regular(&mut self, selected_colors: &HashSet<[u8; 4]>)
    {
        move_selected_objects_into_regular(
            selected_colors, &mut self.selected_points, &mut self.points,
        );
        move_selected_objects_into_regular(
            selected_colors, &mut self.selected_lines, &mut self.lines,
        );
        move_selected_objects_into_regular(
            selected_colors, &mut self.selected_surfaces, &mut self.surfaces,
        );
        move_selected_objects_into_regular(
            selected_colors, 
            &mut self.selected_concentrated_loads, 
            &mut self.concentrated_loads,
        );
        move_rc_selected_objects_into_rc_regular(
            selected_colors, 
            &mut self.selected_uniformly_distributed_line_loads, 
            &mut self.uniformly_distributed_line_loads,
        );
        move_rc_selected_objects_into_rc_regular(
            selected_colors, 
            &mut self.selected_uniformly_distributed_surface_loads, 
            &mut self.uniformly_distributed_surface_loads,
        );
        move_selected_objects_into_regular(
            selected_colors, 
            &mut self.selected_point_boundary_conditions, 
            &mut self.point_boundary_conditions,
        );
        move_selected_objects_into_regular(
            selected_colors, 
            &mut self.selected_nodes, 
            &mut self.nodes,
        );
        move_selected_objects_into_regular(
            selected_colors, 
            &mut self.selected_truss_elements, 
            &mut self.truss_elements,
        );
        move_selected_objects_into_regular(
            selected_colors, 
            &mut self.selected_beam_elements, 
            &mut self.beam_elements,
        );
        move_selected_objects_into_regular(
            selected_colors, 
            &mut self.selected_plate_elements, 
            &mut self.plate_elements,
        );
    }


    pub fn select_objects(
        &mut self,
        selected_colors: &HashSet<[u8; 4]>,
        drop_selection: &js_sys::Function, 
        is_server_should_be_notified: bool,
        props: &Props,
    )
        -> Result<(), JsValue>
    {
        self.move_objects_into_regular(selected_colors);

        let mut selected_objects = Vec::new();
        for key in selected_colors.iter()
        {
            move_regular_object_into_selected_objects(
                key, 
                &mut selected_objects, 
                &mut self.points, 
                &mut self.selected_points,
            );
            move_regular_object_into_selected_objects(
                key, 
                &mut selected_objects, 
                &mut self.lines, 
                &mut self.selected_lines,
            );
            move_regular_object_into_selected_objects(
                key, 
                &mut selected_objects, 
                &mut self.surfaces, 
                &mut self.selected_surfaces,
            );
            move_regular_object_into_selected_objects(
                key, 
                &mut selected_objects, 
                &mut self.concentrated_loads, 
                &mut self.selected_concentrated_loads,
            );
            move_rc_regular_object_into_rc_selected_objects(
                key, 
                &mut selected_objects, 
                &mut self.uniformly_distributed_line_loads, 
                &mut self.selected_uniformly_distributed_line_loads,
            );
            move_rc_regular_object_into_rc_selected_objects(
                key, 
                &mut selected_objects, 
                &mut self.uniformly_distributed_surface_loads, 
                &mut self.selected_uniformly_distributed_surface_loads,
            );
            move_regular_object_into_selected_objects(
                key, 
                &mut selected_objects, 
                &mut self.point_boundary_conditions, 
                &mut self.selected_point_boundary_conditions,
            );
            move_regular_object_into_selected_objects(
                key, 
                &mut selected_objects, 
                &mut self.nodes, 
                &mut self.selected_nodes,
            );
            move_regular_object_into_selected_objects(
                key, 
                &mut selected_objects, 
                &mut self.truss_elements, 
                &mut self.selected_truss_elements,
            );
            move_regular_object_into_selected_objects(
                key, 
                &mut selected_objects, 
                &mut self.beam_elements, 
                &mut self.selected_beam_elements,
            );
            move_regular_object_into_selected_objects(
                key, 
                &mut selected_objects, 
                &mut self.plate_elements, 
                &mut self.selected_plate_elements,
            );
        }

        if selected_objects.is_empty()
        {
            let this = JsValue::null();
            let _ = drop_selection.call0(&this);
        }
        else
        if is_server_should_be_notified
        {
            let detail = json!({ "selected_objects": selected_objects });
            dispatch_custom_event(detail, &props.selected_object_event_name,
                &props.event_target)?;
        }
        Ok(())
    }   


    pub fn reset(&mut self) 
    {
        self.bounding_box = BoundingBox::create();

        self.points = HashMap::new();
        self.selected_points = HashMap::new();
        self.lines = HashMap::new();
        self.selected_lines = HashMap::new();
        self.surfaces = HashMap::new();
        self.selected_surfaces = HashMap::new();
        self.concentrated_loads = HashMap::new();
        self.selected_concentrated_loads = HashMap::new();
        self.uniformly_distributed_line_loads = HashMap::new();
        self.selected_uniformly_distributed_line_loads = HashMap::new();
        self.uniformly_distributed_surface_loads = HashMap::new();
        self.selected_uniformly_distributed_surface_loads = HashMap::new();
        self.point_boundary_conditions = HashMap::new();
        self.selected_point_boundary_conditions = HashMap::new();

        self.nodes = HashMap::new();
        self.selected_nodes = HashMap::new();
        self.truss_elements = HashMap::new();
        self.selected_truss_elements = HashMap::new();
        self.beam_elements = HashMap::new();
        self.selected_beam_elements = HashMap::new();
        self.plate_elements = HashMap::new();
        self.selected_plate_elements = HashMap::new();
    }


    fn reset_postprocessor_data(&mut self)
    {
        self.nodes = HashMap::new();
        self.selected_nodes = HashMap::new();
        self.truss_elements = HashMap::new();
        self.selected_truss_elements = HashMap::new();
        self.beam_elements = HashMap::new();
        self.selected_beam_elements = HashMap::new();
        self.plate_elements = HashMap::new();
        self.selected_plate_elements = HashMap::new();
    }
}
