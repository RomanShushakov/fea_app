### Structs

struct **FloatNumbers**(Vec<f32,>);

struct **Primitives**
<br/>{
<br/>&emsp;**points_coordinates**: **FloatNumbers**,
<br/>&emsp;**points_is_to_scale**: **FloatNumbers**,
<br/>&emsp;**points_reference_points**: **FloatNumbers**,
<br/>&emsp;**points_displacements**: **FloatNumbers**,
<br/>&emsp;**points_colors**: **FloatNumbers**,
<br/>&emsp;**lines_endpoints_coordinates**: **FloatNumbers**,
<br/>&emsp;**lines_endpoints_is_to_scale**: **FloatNumbers**,
<br/>&emsp;**lines_endpoints_reference_points**: **FloatNumbers**,
<br/>&emsp;**lines_endpoints_displacements**: **FloatNumbers**,
<br/>&emsp;**lines_endpoints_colors**: **FloatNumbers**,
<br/>&emsp;**triangles_vertices_coordinates**: **FloatNumbers**,
<br/>&emsp;**triangles_vertices_is_to_scale**: **FloatNumbers**,
<br/>&emsp;**triangles_vertices_reference_points**: **FloatNumbers**,
<br/>&emsp;**triangles_vertices_displacements**: **FloatNumbers**,
<br/>&emsp;**triangles_vertices_colors**: **FloatNumbers**,
<br/>}

struct **LinePrimitives**
<br/>{
<br/>&emsp;**basic_primitives**: **Primitives**,
<br/>&emsp;**optional_primitives_local_axis_1_direction**: Option<**Primitives**>,
<br/>&emsp;**optional_primitives_mesh_seed**: Option<**Primitives**>,
<br/>}

struct **SurfacePrimitives**
<br/>{
<br/>&emsp;**optional_primitives_for_selection**: Option<**Primitives**>,
<br/>&emsp;**optional_primitives_normal**: Option<**Primitives**>,
<br/>&emsp;**optional_primitives_edges_1_3**: Option<**Primitives**>,
<br/>&emsp;**optional_primitives_edges_2_4**: Option<**Primitives**>,
<br/>&emsp;**optional_primitives_mesh_seed_edges_1_3**: Option<**Primitives**>,
<br/>&emsp;**optional_primitives_mesh_seed_edges_2_4**: Option<**Primitives**>,
<br/>}

struct **BoundingBox**
<br/>{
<br/>&emsp;**optional_bounds**: Option<[f32; 6]>,
<br/>}

struct **Denotation**
<br/>{
<br/>&emsp;**notation**: String,
<br/>&emsp;**center**: [f32; 3],
<br/>&emsp;**color**: String,
<br/>}

struct **DenotationsData**
<br/>{
<br/>&emsp;**point_objects_denotations**: Vec<**Denotation**>,
<br/>&emsp;**line_objects_denotations**: Vec<**Denotation**>,
<br/>&emsp;**surface_objects_denotations**: Vec<**Denotation**>,
<br/>}

struct **Manipulation**
<br/>{
<br/>&emsp;**cursor_coord_x**: i32,
<br/>&emsp;**cursor_coord_y**: i32,
<br/>&emsp;**theta**: f32,
<br/>&emsp;**phi**: f32,
<br/>&emsp;**dx**: f32,
<br/>&emsp;**dy**: f32,
<br/>&emsp;**d_scale**: f32,
<br/>&emsp;**under_selection_box_colors**: Vec<u8,>,
<br/>&emsp;**selected_colors**: HashSet<[u8; 4]>,
<br/>&emsp;**selection_box_start_x**: Option<i32,>,
<br/>&emsp;**selection_box_start_y**: Option<i32,>,
<br/>&emsp;**is_point_visible**: bool,
<br/>&emsp;**is_line_visible**: bool,
<br/>&emsp;**is_surface_visible**: bool,
<br/>&emsp;**is_surface_edges_1_3_visible**: bool,
<br/>&emsp;**is_surface_edges_2_4_visible**: bool,
<br/>&emsp;**is_beam_section_orientation_visible**: bool,
<br/>&emsp;**is_surface_normal_visible**: bool,
<br/>&emsp;**is_geometry_visible**: bool,
<br/>&emsp;**is_load_visible**: bool,
<br/>&emsp;**is_boundary_condition_visible**: bool,
<br/>&emsp;**is_mesh_seed_visible**: bool,
<br/>&emsp;**is_mesh_visible**: bool,
<br/>}

struct **Point**
<br/>{
<br/>&emsp;**number**: u32,
<br/>&emsp;**primitives_for_selection**: **Primitives**,
<br/>&emsp;**primitives_visible**: **Primitives**,
<br/>&emsp;**primitives_visible_selected**: **Primitives**,
<br/>}

struct **Line**
<br/>{
<br/>&emsp;**number**: u32,
<br/>&emsp;**point_1_number**: u32,
<br/>&emsp;**point_2_number**: u32,
<br/>&emsp;**primitives_for_selection**: **Primitives**,
<br/>&emsp;**primitives_visible**: **Primitives**,
<br/>&emsp;**primitives_visible_selected**: **Primitives**,
<br/>&emsp;**optional_primitives_local_axis_1_direction**: Option<**Primitives**>,
<br/>&emsp;**optional_primitives_mesh_seed**: Option<**Primitives**>,
<br/>&emsp;**optional_uniformly_distributed_line_load**: Option<Rc<RefCell<**UniformlyDistributedLineLoad**>>>,
<br/>}

struct **Surface**
<br/>{
<br/>&emsp;**number**: u32,
<br/>&emsp;**point_1_number**: u32,
<br/>&emsp;**point_2_number**: u32,
<br/>&emsp;**point_3_number**: u32,
<br/>&emsp;**point_4_number**: u32,
<br/>&emsp;**primitives_for_selection**: **Primitives**,
<br/>&emsp;**primitives_visible_edges_1_3**: **Primitives**,
<br/>&emsp;**primitives_visible_selected_edges_1_3**: **Primitives**,
<br/>&emsp;**primitives_visible_edges_2_4**: **Primitives**,
<br/>&emsp;**primitives_visible_selected_edges_2_4**: **Primitives**,
<br/>&emsp;**primitives_normal**: **Primitives**,
<br/>&emsp;**optional_uniformly_distributed_surface_load**: Option<Rc<RefCell<**UniformlyDistributedSurfaceLoad**>>>,
<br/>}

struct **ConcentratedLoad**
<br/>{
<br/>&emsp;**point_number**: u32,
<br/>&emsp;**primitives_for_selection**: **Primitives**,
<br/>&emsp;**primitives_visible**: **Primitives**,
<br/>&emsp;**primitives_visible_selected**: **Primitives**,
<br/>}

struct **UniformlyDistributedLineLoad**
<br/>{
<br/>&emsp;**transformed_uid**: [u8; 4],
<br/>&emsp;**primitives_for_selection**: **Primitives**,
<br/>&emsp;**primitives_visible**: **Primitives**,
<br/>&emsp;**primitives_visible_selected**: **Primitives**,
<br/>}

struct **UniformlyDistributedSurfaceLoad**
<br/>{
<br/>&emsp;**transformed_uid**: [u8; 4],
<br/>&emsp;**primitives_for_selection**: **Primitives**,
<br/>&emsp;**primitives_visible**: **Primitives**,
<br/>&emsp;**primitives_visible_selected**: **Primitives**,
<br/>}

struct **PointBoundaryCondition**
<br/>{
<br/>&emsp;**point_number**: u32,
<br/>&emsp;**primitives_for_selection**: **Primitives**,
<br/>&emsp;**primitives_visible**: **Primitives**,
<br/>&emsp;**primitives_visible_selected**: **Primitives**,
<br/>}

struct **Scene**
<br/>{
<br/>&emsp;**scene_state**: ***SceneState***,
<br/>&emsp;**bounding_box**: **BoundingBox**,
<br/>&emsp;**points**: HashMap<[u8; 4], **Point**>,
<br/>&emsp;**selected_points**: HashMap<[u8; 4], **Point**>,
<br/>&emsp;**lines**: HashMap<[u8; 4], **Line**>,
<br/>&emsp;**selected_lines**: HashMap<[u8; 4], **Line**>,
<br/>&emsp;**surfaces**: HashMap<[u8; 4], **Surface**>,
<br/>&emsp;**selected_surfaces**: HashMap<[u8; 4], **Surface**>,
<br/>&emsp;**concentrated_loads**: HashMap<[u8; 4], **ConcentratedLoad**>,
<br/>&emsp;**selected_concentrated_loads**: HashMap<[u8; 4], **ConcentratedLoad**>,
<br/>&emsp;**uniformly_distributed_line_loads**: HashMap<[u8; 4], Rc<RefCell<**UniformlyDistributedLineLoad**>>>,
<br/>&emsp;**selected_uniformly_distributed_line_loads**: HashMap<[u8; 4], Rc<RefCell<**UniformlyDistributedLineLoad**>>>,
<br/>&emsp;**uniformly_distributed_surface_loads**: HashMap<[u8; 4], Rc<RefCell<**UniformlyDistributedSurfaceLoad**>>>,
<br/>&emsp;**selected_uniformly_distributed_surface_loads**: HashMap<[u8; 4], Rc<RefCell<**UniformlyDistributedSurfaceLoad**>>>,
<br/>&emsp;**point_boundary_conditions**: HashMap<[u8; 4], **PointBoundaryCondition**>,
<br/>&emsp;**selected_point_boundary_conditions**: HashMap<[u8; 4], **PointBoundaryCondition**>,
<br/>}

### Enums

enum ***PrimitiveType***
<br/>{
<br/>&emsp;Point,
<br/>&emsp;Line,
<br/>&emsp;Triangle,
<br/>}

enum ***SceneState***
<br/>{
<br/>&emsp;Preprocessor,
<br/>&emsp;Postprocessor,
<br/>}

enum ***PointObjectType***
<br/>{
<br/>&emsp;Point,
<br/>&emsp;Node,
<br/>}

enum ***LineObjectType***
<br/>{
<br/>&emsp;LineDefault,
<br/>&emsp;LineTruss,
<br/>&emsp;LineBeam,
<br/>&emsp;Element,
<br/>}

enum ***SurfaceObjectType***
<br/>{
<br/>&emsp;SurfaceDefault,
<br/>&emsp;SurfacePlate,
<br/>}

enum ***MeshSeedType***
<br/>{
<br/>&emsp;None,
<br/>&emsp;Global,
<br/>&emsp;Local,
<br/>}

### Traits

trait *BufferDataTrait*
<br/>{
<br/>&emsp;fn **convert**(&self) -> **js_sys::Object**;
<br/>&emsp;fn **binding_point**(&self) -> u32;
<br/>&emsp;fn **store**(&self, gl: &**web_sys::WebGlRenderingContext**, optional_ref_buffer: Option<&**web_sys::WebGlBuffer**>);
<br/>}

trait *DenotationTrait*
<br/>{
    <br/>&emsp;fn **get_notation**(&self) -> String;
    <br/>&emsp;fn **get_center**(&self) -> [f32; 3];
    <br/>&emsp;fn **get_color_str**(&self, is_selected: bool) -> String;
    <br/>&emsp;fn **get_denotation**(&self, is_selected: bool) -> **Denotation**;
<br/>}
