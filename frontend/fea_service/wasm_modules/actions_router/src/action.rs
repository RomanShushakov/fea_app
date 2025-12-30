use wasm_bindgen::JsValue;
use serde_json::json;
use serde::Serialize;
use serde_wasm_bindgen::Serializer;

use crate::props::Props;
use crate::types::FEFloat;


#[derive(Clone, Debug)]
pub struct Coordinates
{
    pub x: FEFloat,
    pub y: FEFloat,
    pub z: FEFloat,
}


impl Coordinates
{
    pub fn create(x: FEFloat, y: FEFloat, z: FEFloat) -> Coordinates
    {
        Coordinates { x, y, z }
    }
}


#[derive(Debug, Clone)]
pub enum GeometryActionType
{
    // ( number, Coordinates, is_action_id_should_be_increased )
    AddPoint(u32, Coordinates, bool),

    // ( number, Coordinates, Coordinates, is_action_id_should_be_increased )
    UpdatePoint(u32, Coordinates, Coordinates, bool),

    // ( number, is_action_id_should_be_increased )
    DeletePoint(u32, bool),

    // ( number, is_action_id_should_be_increased )
    RestorePoint(u32, bool),

    // ( number, point_1_number, point_2_number, is_action_id_should_be_increased )
    AddLine(u32, u32, u32, bool),

    // ( number, old_point_1_number, old_point_2_number,
    // new_point_1_number, new_point_2_number, is_action_id_should_be_increased )
    UpdateLine(u32, u32, u32, u32, u32, bool),

    // ( number, is_action_id_should_be_increased )
    DeleteLine(u32, bool),

    // ( number, is_action_id_should_be_increased )
    RestoreLine(u32, bool),

    // ( number, point_1_number, point_2_number, point_3_number, point_4_number, is_action_id_should_be_increased )
    AddSurface(u32, u32, u32, u32, u32, bool),

    // ( number, old_point_1_number, old_point_2_number, old_point_3_number, old_point_4_number,
    // new_point_1_number, new_point_2_number, new_point_3_number, new_point_4_number, is_action_id_should_be_increased )
    UpdateSurface(u32, u32, u32, u32, u32, u32, u32, u32, u32, bool),

    // ( number, is_action_id_should_be_increased )
    RotateSurfaceVerticesClockwise(u32, bool),

    // ( number, is_action_id_should_be_increased )
    RotateSurfaceVerticesCounterClockwise(u32, bool),

    // ( number, is_action_id_should_be_increased )
    FlipSurfaceNormalAxis(u32, bool),

    // ( number, is_action_id_should_be_increased )
    DeleteSurface(u32, bool),

    // ( number, is_action_id_should_be_increased )
    RestoreSurface(u32, bool),
}


#[derive(Debug, Clone)]
pub enum PropertiesActionType
{
    // ( name, young_modulus, poisson_ratio, is_action_id_should_be_increased )
    AddMaterial(String, FEFloat, FEFloat, bool),

    // ( name, old_young_modulus, old_poisson_ratio,
    // new_young_modulus, new_poisson_ratio, is_action_id_should_be_increased )
    UpdateMaterial(String, FEFloat, FEFloat, FEFloat, FEFloat, bool),

    // ( name, is_action_id_should_be_increased )
    DeleteMaterial(String, bool),

    // ( name, is_action_id_should_be_increased )
    RestoreMaterial(String, bool),

    // ( name, area, area2, is_action_id_should_be_increased )
    AddTrussSection(String, FEFloat, Option<FEFloat>, bool),

    // ( name, old_area, old_area2, new_area, new_area2, is_action_id_should_be_increased )
    UpdateTrussSection(String, FEFloat, Option<FEFloat>, FEFloat, Option<FEFloat>, bool),

    // ( name, is_action_id_should_be_increased )
    DeleteTrussSection(String, bool),

    // ( name, is_action_id_should_be_increased )
    RestoreTrussSection(String, bool),

    // ( name, area, I11, I22, I12, It, shear_factor, is_action_id_should_be_increased )
    AddBeamSection(String, FEFloat, FEFloat, FEFloat, FEFloat, FEFloat, FEFloat, bool),

    // ( name, old_area, old_I11, old_I22, old_I12, old_It, old_shear_factor, new_area,
    // new_I11, new_I22, new_I12, new_It, new_shear_factor, is_action_id_should_be_increased )
    UpdateBeamSection(String, FEFloat, FEFloat, FEFloat, FEFloat, FEFloat,
        FEFloat, FEFloat, FEFloat, FEFloat, FEFloat, FEFloat, FEFloat, bool),

    // ( name, is_action_id_should_be_increased )
    DeleteBeamSection(String, bool),

    // ( name, is_action_id_should_be_increased )
    RestoreBeamSection(String, bool),

    // ( name, thickness, shear_factor, is_action_id_should_be_increased )
    AddPlateSection(String, FEFloat, FEFloat, bool),

    // ( name, old_thickness, old_shear_factor, new_thickness, new_shear_factor, is_action_id_should_be_increased )
    UpdatePlateSection(String, FEFloat, FEFloat, FEFloat, FEFloat, bool),

    // ( name, is_action_id_should_be_increased )
    DeletePlateSection(String, bool),

    // ( name, is_action_id_should_be_increased )
    RestorePlateSection(String, bool),

    // ( name, material_name, cross_section_name, cross_section_type,
    // is_action_id_should_be_increased )
    AddProperties(String, String, String, String, bool),

    // ( name, old_material_name, old_cross_section_name, old_cross_section_type,
    // new_material_name, new_cross_section_name, new_cross_section_type,
    // is_action_id_should_be_increased )
    UpdateProperties(String, String, String, String, String, String, String, bool),

    // ( name, is_action_id_should_be_increased )
    DeleteProperties(String, bool),

    // ( name, is_action_id_should_be_increased )
    RestoreProperties(String, bool),

    // ( name, old_line_numbers, new_line_numbers, is_action_id_should_be_increased )
    AssignPropertiesToLines(String, Vec<u32>, Vec<u32>, bool),

    // ( local_axis_1_direction, is_action_id_should_be_increased )
    AddBeamSectionLocalAxis1Direction(Vec<FEFloat>, bool),

    // ( local_axis_1_direction, is_action_id_should_be_increased )
    DeleteBeamSectionLocalAxis1Direction(Vec<FEFloat>, bool),

    // ( local_axis_1_direction, is_action_id_should_be_increased )
    RestoreBeamSectionLocalAxis1Direction(Vec<FEFloat>, bool),

    // ( local_axis_1_direction, old_line_numbers, new_line_numbers,
    // is_action_id_should_be_increased )
    AssignBeamSectionLocalAxis1Direction(Vec<FEFloat>, Vec<u32>, Vec<u32>, bool),

    // ( name, old_surface_numbers, new_surface_numbers, is_action_id_should_be_increased )
    AssignPropertiesToSurfaces(String, Vec<u32>, Vec<u32>, bool),
}


#[derive(Clone, Debug)]
pub struct ConcentratedLoad
{
    pub fx: FEFloat,
    pub fy: FEFloat,
    pub fz: FEFloat,
    pub mx: FEFloat,
    pub my: FEFloat,
    pub mz: FEFloat,
}


impl ConcentratedLoad
{
    pub fn create(fx: FEFloat, fy: FEFloat, fz: FEFloat, mx: FEFloat, my: FEFloat, mz: FEFloat) -> ConcentratedLoad
    {
        ConcentratedLoad { fx, fy, fz, mx, my, mz }
    }
}


#[derive(Clone, Debug)]
pub struct UniformlyDistributedLineLoad
{
    pub qx: FEFloat,
    pub qy: FEFloat,
    pub qz: FEFloat,
}


impl UniformlyDistributedLineLoad
{
    pub fn create(qx: FEFloat, qy: FEFloat, qz: FEFloat) -> UniformlyDistributedLineLoad
    {
        UniformlyDistributedLineLoad { qx, qy, qz }
    }
}


#[derive(Clone, Debug)]
pub struct UniformlyDistributedSurfaceLoad
{
    pub px: FEFloat,
    pub py: FEFloat,
    pub pz: FEFloat,
}


impl UniformlyDistributedSurfaceLoad
{
    pub fn create(px: FEFloat, py: FEFloat, pz: FEFloat) -> Self
    {
        UniformlyDistributedSurfaceLoad { px, py, pz }
    }
}


#[derive(Debug, Clone)]
pub enum LoadsActionType
{
    // ( point_number, ConcentratedLoad, is_action_id_should_be_increased )
    AddConcentratedLoad(u32, ConcentratedLoad, bool),

    // ( point_number, old ConcentratedLoad, new ConcentratedLoad, is_action_id_should_be_increased )
    UpdateConcentratedLoad(u32, ConcentratedLoad, ConcentratedLoad, bool),

    // ( point_number, is_action_id_should_be_increased )
    DeleteConcentratedLoad(u32, bool),

    // ( point_number, is_action_id_should_be_increased )
    RestoreConcentratedLoad(u32, bool),

    // ( line_number, UniformlyDistributedLineLoad, is_action_id_should_be_increased )
    AddUniformlyDistributedLineLoad(u32, UniformlyDistributedLineLoad, bool),

    // ( line_number, old UniformlyDistributedLineLoad, new UniformlyDistributedLineLoad, is_action_id_should_be_increased )
    UpdateUniformlyDistributedLineLoad(u32, UniformlyDistributedLineLoad, UniformlyDistributedLineLoad, bool),

    // ( line_number, is_action_id_should_be_increased )
    DeleteUniformlyDistributedLineLoad(u32, bool),

    // ( line_number, is_action_id_should_be_increased )
    RestoreUniformlyDistributedLineLoad(u32, bool),

    // ( surface_number, UniformlyDistributedSurfaceLoad, is_action_id_should_be_increased )
    AddUniformlyDistributedSurfaceLoad(u32, UniformlyDistributedSurfaceLoad, bool),

    // ( surface_number, old UniformlyDistributedSurfaceLoad, new UniformlyDistributedSurfaceLoad, is_action_id_should_be_increased )
    UpdateUniformlyDistributedSurfaceLoad(u32, UniformlyDistributedSurfaceLoad, UniformlyDistributedSurfaceLoad, bool),

    // ( surface_number, is_action_id_should_be_increased )
    DeleteUniformlyDistributedSurfaceLoad(u32, bool),

    // ( surface_number, is_action_id_should_be_increased )
    RestoreUniformlyDistributedSurfaceLoad(u32, bool),
}


#[derive(Clone, Debug)]
pub struct PointBoundaryCondition
{
    pub optional_ux: Option<FEFloat>,
    pub optional_uy: Option<FEFloat>,
    pub optional_uz: Option<FEFloat>,
    pub optional_rx: Option<FEFloat>,
    pub optional_ry: Option<FEFloat>,
    pub optional_rz: Option<FEFloat>,
}


#[derive(Debug, Clone)]
pub enum BoundaryConditionsActionType
{
    // ( point_number, BoundaryCondition, is_action_id_should_be_increased )
    AddPointBoundaryCondition(u32, PointBoundaryCondition, bool),

    // ( point_number, old BoundaryCondition, new BoundaryCondition, is_action_id_should_be_increased )
    UpdatePointBoundaryCondition(u32, PointBoundaryCondition, PointBoundaryCondition, bool),

    // ( point_number, is_action_id_should_be_increased )
    DeletePointBoundaryCondition(u32, bool),

    // ( point_number, is_action_id_should_be_increased )
    RestorePointBoundaryCondition(u32, bool),
}


impl PointBoundaryCondition
{
    pub fn create(optional_ux: Option<FEFloat>, optional_uy: Option<FEFloat>,
        optional_uz: Option<FEFloat>, optional_rx: Option<FEFloat>, optional_ry: Option<FEFloat>,
        optional_rz: Option<FEFloat>) -> PointBoundaryCondition
    {
        PointBoundaryCondition 
        { 
            optional_ux, optional_uy, optional_uz, 
            optional_rx, optional_ry, optional_rz, 
        }
    }
}


#[derive(Debug, Clone)]
pub enum MeshSeedActionType
{
    // ( old global mesh seed value, new global mesh seed value, is_action_id_should_be_increased )
    UpdateGlobalMeshSeed(u8, u8, bool),

    // ( lines mesh seed value, line_numbers, is_action_id_should_be_increased )
    UpdateLinesMeshSeed(u8, Vec<u32>, bool),

    // ( line_numbers, is_action_id_should_be_increased )
    UndoLinesMeshSeedUpdate(Vec<u32>, bool),

    // ( edge 1 and 3 mesh seed value, edge 2 and 4 mesh seed value, surface_numbers, is_action_id_should_be_increased )
    UpdateSurfacesMeshSeed(u8, u8, Vec<u32>, bool),

    // ( surface_numbers, is_action_id_should_be_increased )
    UndoSurfacesMeshSeedUpdate(Vec<u32>, bool),
}


#[derive(Debug, Clone)]
pub enum ActionType
{
    GeometryActionType(GeometryActionType),
    PropertiesActionType(PropertiesActionType),
    LoadsActionType(LoadsActionType),
    BoundaryConditionsActionType(BoundaryConditionsActionType),
    MeshSeedActionType(MeshSeedActionType),
}


impl From<GeometryActionType> for ActionType
{
    fn from(action_type: GeometryActionType) -> Self
    {
        ActionType::GeometryActionType(action_type)
    }
}


impl From<PropertiesActionType> for ActionType
{
    fn from(action_type: PropertiesActionType) -> Self
    {
        ActionType::PropertiesActionType(action_type)
    }
}


impl From<LoadsActionType> for ActionType
{
    fn from(action_type: LoadsActionType) -> Self
    {
        ActionType::LoadsActionType(action_type)
    }
}


impl From<BoundaryConditionsActionType> for ActionType
{
    fn from(action_type: BoundaryConditionsActionType) -> Self
    {
        ActionType::BoundaryConditionsActionType(action_type)
    }
}


impl From<MeshSeedActionType> for ActionType
{
    fn from(action_type: MeshSeedActionType) -> Self
    {
        ActionType::MeshSeedActionType(action_type)
    }
}


#[derive(Clone, Debug)]
pub struct Action
{
    action_id: u32,
    action_type: ActionType,
}


impl Action
{
    pub fn create(action_id: u32, action_type: ActionType) -> Action
    {
        Action { action_id, action_type }
    }


    pub fn is_action_id_same(&self, action_id: u32) -> bool
    {
        self.action_id == action_id
    }


    pub fn ref_action_id(&self) -> &u32
    {
        &self.action_id
    }


    pub fn ref_action_type(&self) -> &ActionType
    {
        &self.action_type
    }


    pub fn convert_to_message(&self, ref_props: &Props) -> Result<JsValue, JsValue>
    {
        let serializer = Serializer::json_compatible();
        match &self.action_type
        {
            ActionType::GeometryActionType(geometry_action) => 
            {
                match geometry_action
                {
                    GeometryActionType::AddPoint(number, coordinates, 
                        _is_action_id_should_be_increased) =>
                    {
                        let serialized_message = json!({ &ref_props.add_point_message_header: 
                            { 
                                "action_id": self.action_id, "number": number.to_string(), 
                                "x": coordinates.x.to_string(), "y": coordinates.y.to_string(), 
                                "z": coordinates.z.to_string(), 
                            } 
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    GeometryActionType::UpdatePoint(number, old_coordinates, new_coordinates,
                        _is_action_id_should_be_increased) =>
                    {
                        let serialized_message = json!({ &ref_props.update_point_message_header: 
                            { 
                                "action_id": self.action_id, "number": number.to_string(), 
                                "old_point_values": 
                                    { 
                                        "x":  old_coordinates.x, "y": old_coordinates.y, 
                                        "z": old_coordinates.z, 
                                    },
                                "new_point_values": 
                                    { 
                                        "x": new_coordinates.x.to_string(), "y": new_coordinates.y.to_string(), 
                                        "z": new_coordinates.z.to_string() 
                                    }
                            } 
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    GeometryActionType::DeletePoint(number, _is_action_id_should_be_increased) => 
                    {
                        let serialized_message = json!({ &ref_props.delete_point_message_header: 
                            { 
                                "action_id": self.action_id, "number": number,
                            } 
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    GeometryActionType::RestorePoint(_, _) => 
                    {
                        let error_message = "Actions router: Convert action to message: Restore point action is incorrect!";
                        Err(JsValue::from(error_message))
                    }, 
                    GeometryActionType::AddLine(number, point_1_number, point_2_number, 
                        _is_action_id_should_be_increased) =>
                    {
                        let serialized_message = json!({ &ref_props.add_line_message_header: 
                            { 
                                "action_id": self.action_id, "number": number.to_string(), 
                                "point_1_number": point_1_number.to_string(), 
                                "point_2_number": point_2_number.to_string(), 
                            }
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    GeometryActionType::UpdateLine(number, old_point_1_number, old_point_2_number,
                        new_point_1_number, new_point_2_number, _is_action_id_should_be_increased) => 
                    {
                        let serialized_message = json!({ &ref_props.update_line_message_header: 
                            { 
                                "action_id": self.action_id,
                                "number": number.to_string(), 
                                "old_line_values": 
                                { 
                                    "point_1_number":  old_point_1_number, 
                                    "point_2_number": old_point_2_number,
                                },
                                "new_line_values": 
                                { 
                                    "point_1_number":  new_point_1_number.to_string(), 
                                    "point_2_number": new_point_2_number.to_string(), 
                                },
                            }
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    GeometryActionType::DeleteLine(number, _is_action_id_should_be_increased) => 
                    {
                        let serialized_message = json!({ &ref_props.delete_line_message_header: 
                            { 
                                "action_id": self.action_id, "number": number.to_string(),
                            } 
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    GeometryActionType::RestoreLine(_, _) => 
                    {
                        let error_message = "Actions router: Convert action to message: Restore line action is incorrect!";
                        Err(JsValue::from(error_message))
                    },
                    GeometryActionType::AddSurface(number, point_1_number, point_2_number, 
                        point_3_number, point_4_number, _is_action_id_should_be_increased) =>
                    {
                        let serialized_message = json!({ &ref_props.add_surface_message_header: 
                            { 
                                "action_id": self.action_id, "number": number.to_string(), 
                                "point_1_number": point_1_number.to_string(), 
                                "point_2_number": point_2_number.to_string(), 
                                "point_3_number": point_3_number.to_string(), 
                                "point_4_number": point_4_number.to_string(), 
                            }
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    GeometryActionType::UpdateSurface(number, old_point_1_number, old_point_2_number,
                        old_point_3_number, old_point_4_number, new_point_1_number, new_point_2_number, 
                        new_point_3_number, new_point_4_number, _is_action_id_should_be_increased) => 
                    {
                        let serialized_message = json!({ &ref_props.update_surface_message_header: 
                            { 
                                "action_id": self.action_id,
                                "number": number.to_string(), 
                                "old_surface_values": 
                                { 
                                    "point_1_number":  old_point_1_number, 
                                    "point_2_number":  old_point_2_number,
                                    "point_3_number":  old_point_3_number,
                                    "point_4_number":  old_point_4_number,
                                },
                                "new_surface_values": 
                                { 
                                    "point_1_number":  new_point_1_number.to_string(), 
                                    "point_2_number":  new_point_2_number.to_string(), 
                                    "point_3_number":  new_point_3_number.to_string(), 
                                    "point_4_number":  new_point_4_number.to_string(), 
                                },
                            }
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    GeometryActionType::RotateSurfaceVerticesClockwise(
                        number, _is_action_id_should_be_increased) => 
                    {
                        let serialized_message = json!({ &ref_props.rotate_surface_vertices_clockwise_message_header: 
                            { 
                                "action_id": self.action_id, "number": number.to_string(),
                            } 
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    GeometryActionType::RotateSurfaceVerticesCounterClockwise(
                        number, _is_action_id_should_be_increased) => 
                    {
                        let serialized_message = json!({ 
                            &ref_props.rotate_surface_vertices_counter_clockwise_message_header: 
                            { 
                                "action_id": self.action_id, "number": number.to_string(),
                            } 
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    GeometryActionType::FlipSurfaceNormalAxis(
                        number, _is_action_id_should_be_increased) => 
                    {
                        let serialized_message = json!({ &ref_props.flip_surface_normal_axis_message_header: 
                            { 
                                "action_id": self.action_id, "number": number.to_string(),
                            } 
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    GeometryActionType::DeleteSurface(number, _is_action_id_should_be_increased) => 
                    {
                        let serialized_message = json!({ &ref_props.delete_surface_message_header: 
                            { 
                                "action_id": self.action_id, "number": number.to_string(),
                            } 
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    GeometryActionType::RestoreSurface(_, _) => 
                    {
                        let error_message = "Actions router: Convert action to message: Restore surface action is incorrect!";
                        Err(JsValue::from(error_message))
                    },
                }
            },
            ActionType::PropertiesActionType(properties_action) =>
            {
                match properties_action
                {
                    PropertiesActionType::AddMaterial(name, young_modulus, poisson_ratio, 
                        _is_action_id_should_be_increased) => 
                    {
                        let serialized_message = json!({ &ref_props.add_material_message_header: 
                            {
                                "action_id": self.action_id,
                                "name": name, 
                                "young_modulus": young_modulus.to_string(), 
                                "poisson_ratio": poisson_ratio.to_string(),
                            }
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    PropertiesActionType::UpdateMaterial(name, old_young_modulus, old_poisson_ratio,
                        new_young_modulus, new_poisson_ratio, _is_action_id_should_be_increased) =>
                    {
                        let serialized_message = json!({ &ref_props.update_material_message_header: 
                            {
                                "action_id": self.action_id,
                                "name": name, 
                                "old_material_values": 
                                    { 
                                        "young_modulus":  old_young_modulus,
                                        "poisson_ratio": old_poisson_ratio, 
                                    },
                                "new_material_values": 
                                    { 
                                        "young_modulus": new_young_modulus.to_string(),
                                        "poisson_ratio": new_poisson_ratio.to_string(), 
                                    }
                            }
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    PropertiesActionType::DeleteMaterial(name, _is_action_id_should_be_increased) =>
                    {
                        let serialized_message = json!({ &ref_props.delete_material_message_header: 
                            { 
                                "action_id": self.action_id, "name": name,
                            } 
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    PropertiesActionType::RestoreMaterial(_, _) => 
                    {
                        let error_message = "Actions router: Convert action to message: Restore material action is incorrect!";
                        Err(JsValue::from(error_message))
                    },
                    PropertiesActionType::AddTrussSection(name, area, area2, 
                        _is_action_id_should_be_increased) => 
                    {
                        let serialized_message = json!({ &ref_props.add_truss_section_message_header: 
                            {
                                "action_id": self.action_id,
                                "name": name, 
                                "area": area.to_string(), 
                                "area2": if let Some(area) = area2 { area.to_string() } else { "''".to_string() },
                            }
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    PropertiesActionType::UpdateTrussSection(name, old_area, old_area2, 
                        new_area, new_area2, _is_action_id_should_be_increased) => 
                    {
                        let serialized_message = json!({ &ref_props.update_truss_section_message_header: 
                            {
                                "action_id": self.action_id,
                                "name": name, 
                                "old_truss_section_values": 
                                    { 
                                        "area":  old_area,
                                        "area2": old_area2,
                                    },
                                "new_truss_section_values": 
                                    { 
                                        "area": new_area.to_string(),
                                        "area2": if let Some(area) = new_area2 { area.to_string() } else { "''".to_string() },
                                    }
                            }
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    PropertiesActionType::DeleteTrussSection(name, _is_action_id_should_be_increased) => 
                    {
                        let serialized_message = json!({ &ref_props.delete_truss_section_message_header: 
                            { 
                                "action_id": self.action_id, "name": name,
                            } 
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    PropertiesActionType::RestoreTrussSection(_, _) =>
                    {
                        let error_message = "Actions router: Convert action to message: Restore truss section action \
                            is incorrect!";
                        Err(JsValue::from(error_message))
                    },
                    PropertiesActionType::AddBeamSection(name, area, i11, i22, i12, it, 
                        shear_factor, _is_action_id_should_be_increased) => 
                    {
                        let serialized_message = json!({ &ref_props.add_beam_section_message_header: 
                            {
                                "action_id": self.action_id,
                                "name": name, 
                                "area": area.to_string(), 
                                "i11": i11.to_string(), 
                                "i22": i22.to_string(), 
                                "i12": i12.to_string(),
                                "it": it.to_string(), 
                                "shear_factor": shear_factor.to_string(),
                            }
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    PropertiesActionType::UpdateBeamSection(name, old_area, old_i11, old_i22, 
                        old_i12, old_it, old_shear_factor, new_area, new_i11, new_i22, 
                        new_i12, new_it, new_shear_factor, _is_action_id_should_be_increased) => 
                    {
                        let serialized_message = json!({ &ref_props.update_beam_section_message_header: 
                            {
                                "action_id": self.action_id,
                                "name": name, 
                                "old_beam_section_values": 
                                    { 
                                        "area": old_area,
                                        "i11": old_i11,
                                        "i22": old_i22,
                                        "i12": old_i12,
                                        "it": old_it,
                                        "shear_factor": old_shear_factor, 
                                    },
                                "new_beam_section_values": 
                                    { 
                                        "area": new_area.to_string(),
                                        "i11": new_i11.to_string(),
                                        "i22": new_i22.to_string(),
                                        "i12": new_i12.to_string(),
                                        "it": new_it.to_string(),
                                        "shear_factor": new_shear_factor.to_string(), 
                                    }
                            }
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    PropertiesActionType::DeleteBeamSection(name, _is_action_id_should_be_increased) => 
                    {
                        let serialized_message = json!({ &ref_props.delete_beam_section_message_header: 
                            { 
                                "action_id": self.action_id, "name": name,
                            } 
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    PropertiesActionType::RestoreBeamSection(_, _) =>
                    {
                        let error_message = "Actions router: Convert action to message: Restore beam section action \
                            is incorrect!";
                        Err(JsValue::from(error_message))
                    },
                    PropertiesActionType::AddPlateSection(name, thickness, shear_factor, 
                        _is_action_id_should_be_increased) => 
                    {
                        let serialized_message = json!({ &ref_props.add_plate_section_message_header: 
                            {
                                "action_id": self.action_id,
                                "name": name, 
                                "thickness": thickness.to_string(),  
                                "shear_factor": shear_factor.to_string(),
                            }
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    PropertiesActionType::UpdatePlateSection(name, old_thickness, old_shear_factor, 
                        new_thickness, new_shear_factor, _is_action_id_should_be_increased) => 
                    {
                        let serialized_message = json!({ &ref_props.update_plate_section_message_header: 
                            {
                                "action_id": self.action_id,
                                "name": name, 
                                "old_plate_section_values": 
                                    { 
                                        "thickness": old_thickness,
                                        "shear_factor": old_shear_factor, 
                                    },
                                "new_plate_section_values": 
                                    { 
                                        "thickness": new_thickness.to_string(),
                                        "shear_factor": new_shear_factor.to_string(), 
                                    }
                            }
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    PropertiesActionType::DeletePlateSection(name, _is_action_id_should_be_increased) => 
                    {
                        let serialized_message = json!({ &ref_props.delete_plate_section_message_header: 
                            { 
                                "action_id": self.action_id, "name": name,
                            } 
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    PropertiesActionType::RestorePlateSection(_, _) =>
                    {
                        let error_message = "Actions router: Convert action to message: Restore beam section action \
                            is incorrect!";
                        Err(JsValue::from(error_message))
                    },
                    PropertiesActionType::AddProperties(name, material_name, cross_section_name, 
                        cross_section_type, _is_action_id_should_be_increased) => 
                    {
                        let serialized_message = json!({ &ref_props.add_properties_message_header: 
                            {
                                "action_id": self.action_id,
                                "name": name, 
                                "material_name": material_name,
                                "cross_section_name": cross_section_name, 
                                "cross_section_type": cross_section_type, 
                            }
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    PropertiesActionType::UpdateProperties(name, old_material_name, old_cross_section_name, 
                        old_cross_section_type, new_material_name, new_cross_section_name, 
                        new_cross_section_type, _is_action_id_should_be_increased) => 
                    {
                        let serialized_message = json!({ &ref_props.update_properties_message_header: {
                            "action_id": self.action_id,
                            "name": name,
                            "old_properties_values": 
                                {
                                    "material_name": old_material_name,
                                    "cross_section_name": old_cross_section_name,
                                    "cross_section_type": old_cross_section_type,
                                },
                            "new_properties_values": 
                                {
                                    "material_name": new_material_name,
                                    "cross_section_name": new_cross_section_name,
                                    "cross_section_type": new_cross_section_type,
                                }
                            }
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    PropertiesActionType::DeleteProperties(name, _is_action_id_should_be_increased) => 
                    {
                        let serialized_message = json!({ &ref_props.delete_properties_message_header: 
                            { 
                                "action_id": self.action_id, 
                                "name": name,
                            }
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    PropertiesActionType::RestoreProperties(_, _) =>
                    {
                        let error_message = "Actions router: Convert action to message: Restore properties action \
                            is incorrect!";
                        Err(JsValue::from(error_message))
                    },
                    PropertiesActionType::AssignPropertiesToLines(name, old_line_numbers, 
                        new_line_numbers, _is_action_id_should_be_increased) => 
                    {
                        let serialized_message = json!({ &ref_props.assign_properties_to_lines_message_header: 
                            {
                                "action_id": self.action_id,
                                "name": name,
                                "old_assigned_properties_to_lines_values": 
                                    {
                                        "line_numbers": old_line_numbers,
                                    },
                                "new_assigned_properties_to_lines_values": 
                                    {
                                        "line_numbers": new_line_numbers,
                                    }
                            }
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    PropertiesActionType::AddBeamSectionLocalAxis1Direction(local_axis_1_direction, 
                        _is_action_id_should_be_increased) =>
                    {
                        let serialized_message = json!({ 
                            &ref_props.add_beam_section_local_axis_1_direction_message_header: 
                            { 
                                "action_id": self.action_id,
                                "local_axis_1_direction": local_axis_1_direction,
                            } 
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    PropertiesActionType::DeleteBeamSectionLocalAxis1Direction(local_axis_1_direction, 
                        _is_action_id_should_be_increased) => 
                    {
                        let serialized_message = json!({ 
                            &ref_props.delete_beam_section_local_axis_1_direction_message_header:
                            { 
                                "action_id": self.action_id,
                                "local_axis_1_direction": local_axis_1_direction,
                            } 
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    PropertiesActionType::RestoreBeamSectionLocalAxis1Direction(_, _) =>
                    {
                        let error_message = "Actions router: Convert action to message: Restore beam section local axis 1 \
                            direction action is incorrect!";
                        Err(JsValue::from(error_message))
                    },
                    PropertiesActionType::AssignBeamSectionLocalAxis1Direction(local_axis_1_direction, 
                        old_line_numbers, new_line_numbers, 
                        _is_action_id_should_be_increased) =>
                    {
                        let serialized_message = json!({ 
                            &ref_props.assign_beam_section_local_axis_1_direction_message_header: 
                            { 
                                "action_id": self.action_id,
                                "local_axis_1_direction": local_axis_1_direction,
                                "old_beam_section_orientation_values": 
                                    {
                                        "line_numbers": old_line_numbers,
                                    },
                                "new_beam_section_orientation_values": 
                                    {
                                        "line_numbers": new_line_numbers,
                                    },
                            } 
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    PropertiesActionType::AssignPropertiesToSurfaces(name, old_surface_numbers, 
                        new_surface_numbers, _is_action_id_should_be_increased) => 
                    {
                        let serialized_message = json!({ &ref_props.assign_properties_to_surfaces_message_header: 
                            {
                                "action_id": self.action_id,
                                "name": name,
                                "old_assigned_properties_to_surfaces_values": 
                                    {
                                        "surface_numbers": old_surface_numbers,
                                    },
                                "new_assigned_properties_to_surfaces_values": 
                                    {
                                        "surface_numbers": new_surface_numbers,
                                    }
                            }
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                }
            }
            ActionType::LoadsActionType(loads_action) =>
            {
                match loads_action
                {
                    LoadsActionType::AddConcentratedLoad(point_number, concentrated_load, 
                        _is_action_id_should_be_increased) =>
                    {
                        let ConcentratedLoad { fx, fy, fz, mx, my, mz } = 
                            concentrated_load;

                        let serialized_message = json!({ &ref_props.add_concentrated_load_message_header: 
                            {
                                "action_id": self.action_id,
                                "point_number": point_number.to_string(), 
                                "fx": fx.to_string(), 
                                "fy": fy.to_string(), 
                                "fz": fz.to_string(),
                                "mx": mx.to_string(), 
                                "my": my.to_string(), 
                                "mz": mz.to_string(),
                            }
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    LoadsActionType::UpdateConcentratedLoad(point_number, old_concentrated_load, 
                        new_concentrated_load, _is_action_id_should_be_increased) =>
                    {
                        let ConcentratedLoad { fx, fy, fz, mx, my, mz } = 
                            new_concentrated_load;

                        let serialized_message = json!({ &ref_props.update_concentrated_load_message_header: {
                            "action_id": self.action_id,
                            "point_number": point_number.to_string(), 
                            "old_concentrated_load_values": 
                                { 
                                    "fx": old_concentrated_load.fx, "fy": old_concentrated_load.fy,
                                    "fz": old_concentrated_load.fz, "mx": old_concentrated_load.mx,
                                    "my": old_concentrated_load.my, "mz": old_concentrated_load.mz,   
                                },
                            "new_concentrated_load_values": 
                                { 
                                    "fx": fx.to_string(), "fy": fy.to_string(),
                                    "fz": fz.to_string(), "mx": mx.to_string(),
                                    "my": my.to_string(), "mz": mz.to_string(),
                                }
                        } 
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    LoadsActionType::DeleteConcentratedLoad(point_number, _is_action_id_should_be_increased) => 
                    {
                        let serialized_message = json!({ &ref_props.delete_concentrated_load_message_header: 
                            { 
                                "action_id": self.action_id, 
                                "point_number": point_number, 
                            }
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    LoadsActionType::RestoreConcentratedLoad(_, _) =>
                    {
                        let error_message = "Actions router: Convert action to message: Restore concentrated load \
                            action is incorrect!";
                        Err(JsValue::from(error_message))
                    },
                    LoadsActionType::AddUniformlyDistributedLineLoad(line_number, uniformly_distributed_line_load, 
                        _is_action_id_should_be_increased) =>
                    {
                        let UniformlyDistributedLineLoad { qx, qy, qz } = 
                            uniformly_distributed_line_load;

                        let serialized_message = json!({ &ref_props.add_uniformly_distributed_line_load_message_header: 
                            {
                                "action_id": self.action_id,
                                "line_number": line_number.to_string(), 
                                "qx": qx.to_string(), 
                                "qy": qy.to_string(), 
                                "qz": qz.to_string(),
                            }
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    LoadsActionType::UpdateUniformlyDistributedLineLoad(line_number, 
                        old_uniformly_distributed_line_load, 
                        new_uniformly_distributed_line_load, 
                        _is_action_id_should_be_increased) =>
                    {
                        let UniformlyDistributedLineLoad { qx, qy, qz } = 
                            new_uniformly_distributed_line_load;

                        let serialized_message = json!({ &ref_props.update_uniformly_distributed_line_load_message_header: 
                            {
                                "action_id": self.action_id,
                                "line_number": line_number.to_string(), 
                                "old_uniformly_distributed_line_load_values": 
                                    { 
                                        "qx": old_uniformly_distributed_line_load.qx, 
                                        "qy": old_uniformly_distributed_line_load.qy,
                                        "qz": old_uniformly_distributed_line_load.qz, 
                                    },
                                "new_uniformly_distributed_line_load_values": 
                                    { 
                                        "qx": qx.to_string(), 
                                        "qy": qy.to_string(),
                                        "qz": qz.to_string(),
                                    }
                            } 
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    LoadsActionType::DeleteUniformlyDistributedLineLoad(line_number, _is_action_id_should_be_increased) => 
                    {
                        let serialized_message = json!({ &ref_props.delete_uniformly_distributed_line_load_message_header: 
                            { 
                                "action_id": self.action_id, 
                                "line_number": line_number, 
                            }
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    LoadsActionType::RestoreUniformlyDistributedLineLoad(_, _) =>
                    {
                        let error_message = "Actions router: Convert action to message: Restore uniformly \
                            distributed line load action is incorrect!";
                        Err(JsValue::from(error_message))
                    },
                    LoadsActionType::AddUniformlyDistributedSurfaceLoad(surface_number, 
                        uniformly_distributed_surface_load, 
                        _is_action_id_should_be_increased) =>
                    {
                        let UniformlyDistributedSurfaceLoad { px, py, pz } = 
                            uniformly_distributed_surface_load;

                        let serialized_message = json!({ &ref_props.add_uniformly_distributed_surface_load_message_header: 
                            {
                                "action_id": self.action_id,
                                "surface_number": surface_number.to_string(), 
                                "px": px.to_string(), 
                                "py": py.to_string(), 
                                "pz": pz.to_string(),
                            }
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    LoadsActionType::UpdateUniformlyDistributedSurfaceLoad(surface_number, 
                        old_uniformly_distributed_surface_load, 
                        new_uniformly_distributed_surface_load, 
                        _is_action_id_should_be_increased) =>
                    {
                        let UniformlyDistributedSurfaceLoad { px, py, pz } = 
                            new_uniformly_distributed_surface_load;

                        let serialized_message = json!({ &ref_props.update_uniformly_distributed_surface_load_message_header: 
                            {
                                "action_id": self.action_id,
                                "surface_number": surface_number.to_string(), 
                                "old_uniformly_distributed_surface_load_values": 
                                    { 
                                        "px": old_uniformly_distributed_surface_load.px, 
                                        "py": old_uniformly_distributed_surface_load.py,
                                        "pz": old_uniformly_distributed_surface_load.pz, 
                                    },
                                "new_uniformly_distributed_surface_load_values": 
                                    { 
                                        "px": px.to_string(), 
                                        "py": py.to_string(),
                                        "pz": pz.to_string(),
                                    }
                            } 
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    LoadsActionType::DeleteUniformlyDistributedSurfaceLoad(surface_number, 
                        _is_action_id_should_be_increased) => 
                    {
                        let serialized_message = json!({ &ref_props.delete_uniformly_distributed_surface_load_message_header: 
                            { 
                                "action_id": self.action_id, 
                                "surface_number": surface_number, 
                            }
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    LoadsActionType::RestoreUniformlyDistributedSurfaceLoad(_, _) =>
                    {
                        let error_message = "Actions router: Convert action to message: Restore uniformly \
                            distributed surface load action is incorrect!";
                        Err(JsValue::from(error_message))
                    },
                }
            },
            ActionType::BoundaryConditionsActionType(boundary_conditions_action) =>
            {
                match boundary_conditions_action
                {
                    BoundaryConditionsActionType::AddPointBoundaryCondition(point_number, boundary_condition, 
                        _is_action_id_should_be_increased) => 
                    {
                        let PointBoundaryCondition { optional_ux, optional_uy, optional_uz, 
                            optional_rx, optional_ry, optional_rz } = boundary_condition;

                        let serialized_message = json!({ &ref_props.add_point_boundary_condition_message_header: 
                            {
                                "action_id": self.action_id,
                                "point_number": point_number.to_string(), 
                                "optional_ux": if let Some(ux) = optional_ux { ux.to_string() } else { "''".to_string() },
                                "optional_uy": if let Some(uy) = optional_uy { uy.to_string() } else { "''".to_string() },
                                "optional_uz": if let Some(uz) = optional_uz { uz.to_string() } else { "''".to_string() },
                                "optional_rx": if let Some(rx) = optional_rx { rx.to_string() } else { "''".to_string() },
                                "optional_ry": if let Some(ry) = optional_ry { ry.to_string() } else { "''".to_string() },
                                "optional_rz": if let Some(rz) = optional_rz { rz.to_string() } else { "''".to_string() },
                            }
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    BoundaryConditionsActionType::UpdatePointBoundaryCondition(point_number, old_boundary_condition, 
                        new_boundary_condition, _is_action_id_should_be_increased) =>
                    {
                        let old_optional_ux = old_boundary_condition.optional_ux;
                        let old_optional_uy = old_boundary_condition.optional_uy;
                        let old_optional_uz = old_boundary_condition.optional_uz;
                        let old_optional_rx = old_boundary_condition.optional_rx;
                        let old_optional_ry = old_boundary_condition.optional_ry;
                        let old_optional_rz = old_boundary_condition.optional_rz;

                        let PointBoundaryCondition { optional_ux, optional_uy, optional_uz, 
                            optional_rx, optional_ry, optional_rz } = new_boundary_condition;
                        
                        let serialized_message = json!({ &ref_props.update_point_boundary_condition_message_header: 
                            {
                                "action_id": self.action_id,
                                "point_number": point_number.to_string(), 
                                "old_boundary_condition_values": 
                                    { 
                                        "optional_ux": old_optional_ux,
                                        "optional_uy": old_optional_uy,
                                        "optional_uz": old_optional_uz,
                                        "optional_rx": old_optional_rx,
                                        "optional_ry": old_optional_ry,
                                        "optional_rz": old_optional_rz,  
                                    },
                                "new_boundary_condition_values": 
                                    { 
                                        "optional_ux": if let Some(ux) = optional_ux { ux.to_string() } else { "''".to_string() },
                                        "optional_uy": if let Some(uy) = optional_uy { uy.to_string() } else { "''".to_string() },
                                        "optional_uz": if let Some(uz) = optional_uz { uz.to_string() } else { "''".to_string() },
                                        "optional_rx": if let Some(rx) = optional_rx { rx.to_string() } else { "''".to_string() },
                                        "optional_ry": if let Some(ry) = optional_ry { ry.to_string() } else { "''".to_string() },
                                        "optional_rz": if let Some(rz) = optional_rz { rz.to_string() } else { "''".to_string() },
                                    }
                            }
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    BoundaryConditionsActionType::DeletePointBoundaryCondition(point_number, 
                        _is_action_id_should_be_increased) => 
                    {
                        let serialized_message = json!({ &ref_props.delete_point_boundary_condition_message_header: 
                            { 
                                "action_id": self.action_id, 
                                "point_number": point_number, 
                            }
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    BoundaryConditionsActionType::RestorePointBoundaryCondition(_, _) => 
                    {
                        let error_message = "Actions router: Convert action to message: Restore point boundary \
                            condition action is incorrect!";
                        Err(JsValue::from(error_message))
                    },
                }
            },
            ActionType::MeshSeedActionType(mesh_seed_action) =>
            {
                match mesh_seed_action
                {
                    MeshSeedActionType::UpdateGlobalMeshSeed(old_global_mesh_seed_value, 
                        new_global_mesh_seed_value, _is_action_id_should_be_increased) =>
                    {
                        let serialized_message = json!({ &ref_props.update_global_mesh_seed_message_header: 
                            {
                                "action_id": self.action_id,
                                "old_global_mesh_seed_value": old_global_mesh_seed_value,
                                "new_global_mesh_seed_value": new_global_mesh_seed_value,
                            }
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    MeshSeedActionType::UpdateLinesMeshSeed(lines_mesh_seed_value,
                        line_numbers, _is_action_id_should_be_increased) =>
                    {
                        let serialized_message = json!({ &ref_props.update_lines_mesh_seed_message_header: 
                            {
                                "action_id": self.action_id,
                                "lines_mesh_seed_value": lines_mesh_seed_value,
                                "line_numbers": line_numbers,
                            }
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    MeshSeedActionType::UndoLinesMeshSeedUpdate(_, _) =>
                    {
                        let error_message = "Actions router: Convert action to message: Undo lines mesh seed \
                            action is incorrect!";
                        Err(JsValue::from(error_message))
                    },
                    MeshSeedActionType::UpdateSurfacesMeshSeed(edges_1_3_mesh_seed_value, 
                        edges_2_4_mesh_seed_value, surface_numbers, 
                        _is_action_id_should_be_increased) =>
                    {
                        let serialized_message = json!({ &ref_props.update_surfaces_mesh_seed_message_header: 
                            {
                                "action_id": self.action_id,
                                "surfaces_edges_1_3_mesh_seed_value": edges_1_3_mesh_seed_value,
                                "surfaces_edges_2_4_mesh_seed_value": edges_2_4_mesh_seed_value,
                                "surface_numbers": surface_numbers,
                            }
                        });
                        let error_message = "Actions router: Convert action to message: Action could not be converted!";
                        let message = serialized_message.serialize(&serializer)
                            .map_err(|_e| JsValue::from(error_message))?;
                        Ok(message)
                    },
                    MeshSeedActionType::UndoSurfacesMeshSeedUpdate(_, _) =>
                    {
                        let error_message = "Actions router: Convert action to message: Undo surfaces mesh seed \
                            action is incorrect!";
                        Err(JsValue::from(error_message))
                    }
                }
            }
        }
    }
}


#[derive(Debug)]
pub struct ActionInPool
{
    pub action: Action,
    pub add_to_active_actions: bool,
    pub is_redo: bool,
}


impl ActionInPool
{
    pub fn create(action: Action, add_to_active_actions: bool, is_redo: bool) -> Self
    {
        ActionInPool { action, add_to_active_actions, is_redo }
    }
}
