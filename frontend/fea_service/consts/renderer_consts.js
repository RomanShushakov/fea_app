export const INIT_CURSOR_COORD_X = -1;
export const INIT_CURSOR_COORD_Y = -1;
export const INIT_THETA = 0.0;
export const INIT_PHI = 0.0;
export const INIT_DX = 0.0;
export const INIT_DY = 0.0;
export const INIT_D_SCALE = 0.0;
export const INIT_IS_POINT_VISIBLE = true;
export const INIT_IS_LINE_VISIBLE = true;
export const INIT_IS_SURFACE_VISIBLE = true;
export const INIT_IS_SURFACE_EDGES_1_3_VISIBLE = true;
export const INIT_IS_SURFACE_EDGES_2_4_VISIBLE = true;
export const INIT_IS_BEAM_SECTION_ORIENTATION_VISIBLE = true;
export const INIT_IS_SURFACE_NORMAL_VISIBLE = true;
export const INIT_IS_LOAD_VISIBLE = true;
export const INIT_IS_BOUNDARY_CONDITION_VISIBLE = true;
export const INIT_IS_MESH_SEED_VISIBLE = true;

export const INIT_IS_NODE_VISIBLE = true;
export const INIT_IS_TRUSS_ELEMENT_VISIBLE = true;
export const INIT_IS_BEAM_ELEMENT_VISIBLE = true;
export const INIT_IS_PLATE_ELEMENT_VISIBLE = true;
export const INIT_IS_LOCAL_AXES_VISIBLE = false;

export const CS_ORIGIN = [0.0, 0.0, 0.0]; 
export const CS_AXIS_X = [1.0, 0.0, 0.0]; 
export const CS_AXIS_X_COLOR = [0.3843, 0.1490, 0.1607, 1.0];
export const CS_AXIS_Y = [0.0, 1.0, 0.0]; 
export const CS_AXIS_Y_COLOR = [0.1372, 0.3019, 0.1764, 1.0]; 
export const CS_AXIS_Z = [0.0, 0.0, 1.0];
export const CS_AXIS_Z_COLOR = [0.4549, 0.4588, 0.9019, 1.0];

export const CS_AXES_CAPS_HEIGHT = 0.15; // arrow length
export const CS_AXES_CAPS_WIDTH = 0.075; // half of arrow width
export const CS_AXES_CAPS_BASE_POINTS_NUMBER = 12; // the number of points in cone circular base

export const CS_AXES_SCALE = 0.1;
export const CS_AXES_X_SHIFT = 0.85; // shift of the cs in the x-direction
export const CS_AXES_Y_SHIFT = 0.85; // shift of the cs in the y-direction
export const CS_AXES_Z_SHIFT = -1.5; // shift of the cs in the z-direction
export const AXIS_X_DENOTATION_SHIFT_X = 0.1;
export const AXIS_X_DENOTATION_SHIFT_Y = -0.05;
export const AXIS_Y_DENOTATION_SHIFT_X = -0.05;
export const AXIS_Y_DENOTATION_SHIFT_Y = 0.1;
export const AXIS_Z_DENOTATION_SHIFT_X = -0.05;
export const AXIS_Z_DENOTATION_SHIFT_Y = -0.05;
export const AXIS_Z_DENOTATION_SHIFT_Z = 0.1;

export const CANVAS_AXES_DENOTATION_COLOR = "rgb(211, 211, 211)"; // LightGrey

export const HINT_SHIFT_X = 0.05;
export const ROTATION_HINT_SHIFT_Y = 0.85;
export const ZOOM_HINT_SHIFT_Y = 0.9;
export const PAN_HINT_SHIFT_Y = 0.95;
export const HINTS_COLOR = "rgb(211, 211, 211)"; // LightGrey

export const DRAWN_OBJECT_SELECTED_COLOR = [1.0, 0.0, 0.0, 1.0]; // red

export const NODE_COLOR = [1.0, 1.0, 0.0, 1.0]; // yellow
export const POINT_COLOR = [0.26, 0.81, 0.20, 1.0]; // apple
export const DRAWN_POINT_OBJECT_DENOTATION_SHIFT = 0.02;

export const LINE_DEFAULT_COLOR = [0.6, 0.196, 0.8, 1.0]; // DarkOrchid
export const LINE_TRUSS_PROPS_COLOR = [0.4, 0.803, 0.666, 1.0]; // MediumAquamarine
export const LINE_BEAM_PROPS_COLOR = [1.0, 0.894, 0.709, 1.0]; // Moccasin
export const ELEMENT_COLOR = [0.0, 0.0, 1.0, 0.33]; // cyan
export const DRAWN_LINE_OBJECTS_DENOTATION_SHIFT = 0.01;

export const SURFACE_DEFAULT_COLOR = [0.737, 0.56, 0.56, 1.0]; // RosyBrown
export const SURFACE_PLATE_PROPS_COLOR = [0.627, 0.321, 0.176, 1.0]; // Sienna

export const SELECTION_RECTANGLE_STROKE_COLOR = "rgb(211, 211, 211)"; // LightGrey
export const SELECTION_RECTANGLE_FILL_COLOR = "rgba(47, 79, 79, 0.5)"; // DarkSlateGrey

export const SELECTED_OBJECTS_EVENT_NAME = "selected_objects";

export const BEAM_SECTION_ORIENTATION_LINE_LENGTH = 0.05; // line length
export const BEAM_SECTION_ORIENTATION_CAP_HEIGHT = 0.01; // arrow length
export const BEAM_SECTION_ORIENTATION_CAP_WIDTH = 0.005; // half of arrow width
export const BEAM_SECTION_ORIENTATION_CAP_BASE_POINTS_NUMBER = 6; // the number of points in cone circular base
export const BEAM_SECTION_ORIENTATION_COLOR = [1.0, 0.843, 0.0, 1.0]; // Gold

export const SURFACE_NORMAL_LINE_LENGTH = 0.05; // line length
export const SURFACE_NORMAL_CAP_HEIGHT = 0.01; // arrow length
export const SURFACE_NORMAL_CAP_WIDTH = 0.005; // half of arrow width
export const SURFACE_NORMAL_CAP_BASE_POINTS_NUMBER = 6; // the number of points in cone circular base
export const SURFACE_NORMAL_COLOR = [0.901, 0.901, 0.98, 1.0]; // Lavender

export const CONCENTRATED_LOAD_LINE_LENGTH = 0.04; // line length
export const CONCENTRATED_LOAD_CAP_HEIGHT = 0.0085; // arrow length
export const CONCENTRATED_LOAD_CAP_WIDTH = 0.00325; // half of arrow width
export const CONCENTRATED_LOAD_CAP_BASE_POINTS_NUMBER = 6; // the number of points in cone circular base
export const CONCENTRATED_LOAD_COLOR = [0.117, 0.564, 1.0, 1.0]; // DodgerBlue

export const UNIFORMLY_DISTRIBUTED_LINE_LOAD_LINE_LENGTH = 0.04; // line length
export const UNIFORMLY_DISTRIBUTED_LINE_LOAD_CAP_HEIGHT = 0.0085; // arrow length
export const UNIFORMLY_DISTRIBUTED_LINE_LOAD_CAP_WIDTH = 0.00325; // half of arrow width
export const UNIFORMLY_DISTRIBUTED_LINE_LOAD_CAP_BASE_POINTS_NUMBER = 6; // the number of points in cone circular base
export const UNIFORMLY_DISTRIBUTED_LINE_LOAD_COLOR = [0.529, 0.807, 0.921, 1.0]; // SkyBlue
export const NUMBER_OF_UNIFORMLY_DISTRIBUTED_LINE_LOAD_ARROWS = 4;

export const UNIFORMLY_DISTRIBUTED_SURFACE_LOAD_LINE_LENGTH = 0.04; // line length
export const UNIFORMLY_DISTRIBUTED_SURFACE_LOAD_CAP_HEIGHT = 0.0085; // arrow length
export const UNIFORMLY_DISTRIBUTED_SURFACE_LOAD_CAP_WIDTH = 0.00325; // half of arrow width
export const UNIFORMLY_DISTRIBUTED_SURFACE_LOAD_CAP_BASE_POINTS_NUMBER = 6; // the number of points in cone circular base
export const UNIFORMLY_DISTRIBUTED_SURFACE_LOAD_COLOR = [0.274, 0.509, 0.705, 1.0]; // SteelBlue
export const NUMBER_OF_UNIFORMLY_DISTRIBUTED_SURFACE_LOAD_ARROWS = 4;

export const POINT_BOUNDARY_CONDITION_CAP_HEIGHT = 0.0085; // arrow length
export const POINT_BOUNDARY_CONDITION_CAP_WIDTH = 0.00325; // half of arrow width
export const POINT_BOUNDARY_CONDITION_CAP_BASE_POINTS_NUMBER = 6; // the number of points in cone circular base
export const POINT_BOUNDARY_CONDITION_COLOR = [1.0, 0.549, 0.0, 1.0]; // DarkOrange

export const GLOBAL_MESH_SEED_COLOR = [0.498, 1.0, 0.831, 1.0]; // Aquamarine
export const LOCAL_MESH_SEED_COLOR = [1.0, 1.0, 0.0, 1.0]; // yellow

export const SYMBOLS_MIN_LINE_LENGTH = 0.025;
export const SYMBOLS_MIN_CAP_HEIGHT = 0.005;
export const SYMBOLS_MIN_CAP_WIDTH = 0.0025;
export const SYMBOLS_MAX_LINE_LENGTH = 0.1;
export const SYMBOLS_MAX_CAP_HEIGHT = 0.02;
export const SYMBOLS_MAX_CAP_WIDTH = 0.01;
export const SYMBOLS_CAP_BASE_POINTS_NUMBER = 6;

export const COLOR_BAR_SHIFT_X = 0.05;
export const COLOR_BAR_Y_BOTTOM = 0.45;
export const COLOR_BAR_Y_TOP = 0.2;
export const COLOR_BAR_WIDTH = 0.015;
export const COLOR_BAR_MIN_COLOR = [0.0, 1.0, 1.0, 1.0]; // Blue
export const COLOR_BAR_MAX_COLOR = [1.0, 0.0, 0.0, 1.0]; // Red

export const COLOR_BAR_CAPTION_SHIFT_X = 0.05;
export const COLOR_BAR_CAPTION_HEADER_SHIFT_Y = 0.1;
export const COLOR_BAR_CAPTION_COMPONENT_SHIFT_Y = 0.15;
export const COLOR_BAR_CAPTION_EXTREME_VALUE_SHIFT_X = 0.07;

export const LOCAL_AXIS_LINE_LENGTH = 0.05; // line length
export const LOCAL_AXIS_CAP_HEIGHT = 0.01; // arrow length
export const LOCAL_AXIS_CAP_WIDTH = 0.005; // half of arrow width
export const LOCAL_AXIS_CAP_BASE_POINTS_NUMBER = 6; // the number of points in cone circular base
export const LOCAL_AXIS_R_COLOR = [0.3843, 0.1490, 0.1607, 1.0];
export const LOCAL_AXIS_S_COLOR = [0.1372, 0.3019, 0.1764, 1.0]; 
export const LOCAL_AXIS_T_COLOR = [0.4549, 0.4588, 0.9019, 1.0];

export const PLATE_SCALE_FOR_LOAD_ARROWS = 0.8;
