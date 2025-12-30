### Structs

struct **Point**
<br/>{
<br/>&emsp;**x**: f64,
<br/>&emsp;**y**: f64,
<br/>&emsp;**z**: f64,
<br/>&emsp;**uid**: u32,
<br/>&emsp;**status**: ***Status***<u32,>,
<br/>}

struct **Line**
<br/>{
<br/>&emsp;**point_1_number**: u32,
<br/>&emsp;**point_2_number**: u32,
<br/>&emsp;**uid**: u32,
<br/>&emsp;**status**: ***Status***<u32,>,
<br/>&emsp;**optional_property**: Option<**Property**>,
<br/>&emsp;**optional_local_axis_1_direction**: Option<**LocalAxis1Direction**>,
<br/>&emsp;**optional_transformed_local_axis_1_direction**: Option<**LocalAxis1Direction**>,
<br/>&emsp;**optional_mesh_seed**: Option<***MeshSeed***>,
<br/>&emsp;**optional_uniformly_distributed_line_load**: Option<***UniformlyDistributedLineLoad***>,
<br/>}

struct **Surface**
<br/>{
<br/>&emsp;**point_1_number**: u32,
<br/>&emsp;**point_2_number**: u32,
<br/>&emsp;**point_3_number**: u32,
<br/>&emsp;**point_4_number**: u32,
<br/>&emsp;**normal**: **Normal**,
<br/>&emsp;**uid**: u32,
<br/>&emsp;**status**: ***Status***<u32,>,
<br/>&emsp;**optional_property**: Option<**Property**>,
<br/>&emsp;**optional_mesh_seed**: Option<***MeshSeed***>,
<br/>&emsp;**optional_uniformly_distributed_surface_load**: Option<***UniformlyDistributedSurfaceLoad***>,
<br/>}

struct **Material**
<br/>{
<br/>&emsp;**young_modulus**: f64,
<br/>&emsp;**poisson_ratio**: f64,
<br/>&emsp;**status**: ***Status***<String,>,
<br/>}

struct **TrussSection**
<br/>{
<br/>&emsp;**area**: f64,
<br/>&emsp;**optional_area_2**: Option<f64,>,
<br/>&emsp;**status**: ***Status***<String,>,
<br/>}

struct **BeamSection**
<br/>{
<br/>&emsp;**area**: f64,
<br/>&emsp;**i11**: f64,
<br/>&emsp;**i22**: f64,
<br/>&emsp;**i12**: f64,
<br/>&emsp;**it**: f64,
<br/>&emsp;**shear_factor**: f64,
<br/>&emsp;**status**: ***Status***<String,>,
<br/>}

struct **PlateSection**
<br/>{
<br/>&emsp;**thickness**: f64,
<br/>&emsp;**shear_factor**: f64,
<br/>&emsp;**status**: ***Status***<String,>,
<br/>}

struct **Property**
<br/>{
<br/>&emsp;**material_name**: String,
<br/>&emsp;**cross_section**: ***CrossSection***,
<br/>&emsp;**status**: ***Status***<String,>,
<br/>}

struct **ExtendedProperty**
<br/>{
<br/>&emsp;**name**: String,
<br/>&emsp;**property**: **Property**,
<br/>}

struct **LocalAxis1Direction**(f64, f64, f64)

struct **Normal**(f64, f64, f64)

struct **PointBoundaryCondition**
<br/>{
<br/>&emsp;**optional_ux**: Option<f64,>,
<br/>&emsp;**optional_uy**: Option<f64,>,
<br/>&emsp;**optional_uz**: Option<f64,>,
<br/>&emsp;**optional_rx**: Option<f64,>,
<br/>&emsp;**optional_ry**: Option<f64,>,
<br/>&emsp;**optional_rz**: Option<f64,>,
<br/>&emsp;**uid**: u32,
<br/>&emsp;**status**: ***Status***<u32,>,
<br/>}

struct **LineBoundaryCondition**
<br/>{
<br/>&emsp;**optional_ux**: Option<f64,>,
<br/>&emsp;**optional_uy**: Option<f64,>,
<br/>&emsp;**optional_uz**: Option<f64,>,
<br/>&emsp;**optional_rx**: Option<f64,>,
<br/>&emsp;**optional_ry**: Option<f64,>,
<br/>&emsp;**optional_rz**: Option<f64,>,
<br/>&emsp;**uid**: u32,
<br/>&emsp;**status**: ***Status***<u32,>,
<br/>}

struct **SurfaceEdgeBoundaryCondition**
<br/>{
<br/>&emsp;**optional_ux**: Option<f64,>,
<br/>&emsp;**optional_uy**: Option<f64,>,
<br/>&emsp;**optional_uz**: Option<f64,>,
<br/>&emsp;**optional_rx**: Option<f64,>,
<br/>&emsp;**optional_ry**: Option<f64,>,
<br/>&emsp;**optional_rz**: Option<f64,>,
<br/>&emsp;**uid**: u32,
<br/>&emsp;**status**: ***Status***<u32,>,
<br/>}

struct **ConcentratedLoad**
<br/>{
<br/>&emsp;**fx**: f64,
<br/>&emsp;**fy**: f64,
<br/>&emsp;**fz**: f64,
<br/>&emsp;**mx**: f64,
<br/>&emsp;**my**: f64,
<br/>&emsp;**mz**: f64,
<br/>&emsp;**uid**: u32,
<br/>&emsp;**status**: ***Status***<u32,>,
<br/>}

struct **UniformlyDistributedLineLoad**(u32, f64, f64, f64)

struct **UniformlyDistributedSurfaceEdgeLoad**
<br/>{
<br/>&emsp;**qx**: f64,
<br/>&emsp;**qy**: f64,
<br/>&emsp;**qz**: f64,
<br/>&emsp;**uid**: u32,
<br/>&emsp;**status**: ***Status***<u32,>,
<br/>}

struct **UniformlyDistributedSurfaceLoad**(u32, f64, f64, f64)

### Enums

enum ***Status<K,>***
<br/>{
<br/>&emsp;Active,
<br/>&emsp;Changed(K),
<br/>&emsp;Deleted(K),
<br/>}

enum ***CrossSection***
<br/>{
<br/>&emsp;Truss(String),
<br/>&emsp;Beam(String),
<br/>&emsp;Plate(String),
<br/>}

enum ***MeshSeed***
<br/>{
<br/>&emsp;Global,
<br/>&emsp;Line(u8),
<br/>&emsp;Surface(u8, u8),
<br/>}

enum ***ParentKey***
<br/>{
<br/>&emsp;Point(u32),
<br/>&emsp;Material(String),
<br/>&emsp;CrossSection(***CrossSection***),
<br/>}

enum ***NotificationType***
<br/>{
<br/>&emsp;Add(bool),
<br/>&emsp;Update(bool),
<br/>&emsp;Delete(bool),
<br/>}

enum ***RelativeKey***
<br/>{
<br/>&emsp;Property(String),
<br/>&emsp;LocalAxis1Direction(**LocalAxis1Direction**),
<br/>}

### Traits

trait *StatusTrait*
<br/>{
<br/>&emsp;type Key,
<br/>&emsp;fn **get_mut_ref_status**(&mut self) -> &mut ***Status***<(Self::Key)>;
<br/>&emsp;fn **get_status**(&mut self) -> ***Status***<(Self::Key)>;
<br/>&emsp;fn **set_status**(&mut self, status: ***Status***<(Self::Key)>);
<br/>}

trait *ServerNotificationTrait*
<br/>{
<br/>&emsp;type Key,
<br/>&emsp;fn **get_event_detail**(&self, notification_type: ***NotificationType***, key: Self::Key) -> serde_json::Value;
<br/>&emsp;fn **get_event_name**(&self, notification_type: ***NotificationType***, props: &**Props**) -> String;
<br/>&emsp;fn **get_event_target**(&self, props: &**Props**) -> String;
<br/>&emsp;fn **notify_server**(&self, notification_type: ***NotificationType***, key: Self::Key, props: &**Props**)
<br/>&emsp;&emsp;-> Result<(), JsValue>;
<br/>}

trait *ChildTrait*
<br/>{
<br/>&emsp;fn **get_parent_keys**(&self) -> Vec<***ParentKey***>;
<br/>&emsp;fn **get_parent_index**(&self, parent_key: ***ParentKey***) -> Result<usize, JsValue>;
<br/>&emsp;fn **is_child_of_parent**(&self, parent_key: &***ParentKey***) -> bool;
<br/>}

trait *RelativeTrait*
<br/>{
<br/>&emsp;fn **get_relative_keys**(&self) -> Vec<Option<***RelativeKey***>>;
<br/>&emsp;fn **get_relative_index**(&self, relative_key: ***RelativeKey***) -> Result<usize, JsValue>;
<br/>&emsp;fn **is_relative_of**(&self, relative_key: &***RelativeKey***) -> bool;
<br/>&emsp;fn **set_relative_to_none**(&mut self, relative_key: &***RelativeKey***);
<br/>}

trait *PropertyTrait*
<br/>{
<br/>&emsp;fn **get_ref_property**(&self) -> &Option<**ExtendedProperty**>;
<br/>&emsp;fn **get_mut_ref_property**(&mut self) -> &mut Option<**ExtendedProperty**>;
<br/>&emsp;fn **set_propery**(&mut self, optional_property: Option<**ExtendedProperty**>);
<br/>&emsp;fn **is_property_name_same**(&self, property_name: String) -> bool;
<br/>&emsp;fn **is_property_assigned**(&self) -> bool;
<br/>&emsp;fn **get_optional_cross_section**(&self) -> Option<***CrossSection***>;
<br/>}

trait *MeshSeedTrait*
<br/>{
<br/>&emsp;fn **get_ref_mesh_seed**(&self) -> &Option<***MeshSeed***>;
<br/>&emsp;fn **get_mut_ref_mesh_seed**(&mut self) -> &mut Option<***MeshSeed***>;
<br/>&emsp;fn **is_mesh_seed_assigned**(&self) -> bool
<br/>&emsp;fn **is_mesh_seed_global**(&self) -> bool
<br/>&emsp;fn **set_mesh_seed**(&mut self, mesh_seed: Option<***MeshSeed***>)
<br/>}
