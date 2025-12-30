#[derive(Clone)]
pub struct Props
{
    pub add_point_message_header: String,
    pub update_point_message_header: String,
    pub delete_point_message_header: String,
    pub restore_point_message_header: String,

    pub add_line_message_header: String,
    pub update_line_message_header: String,
    pub delete_line_message_header: String,
    pub restore_line_message_header: String,

    pub add_surface_message_header: String,
    pub update_surface_message_header: String,
    pub rotate_surface_vertices_clockwise_message_header: String,
    pub rotate_surface_vertices_counter_clockwise_message_header: String,
    pub flip_surface_normal_axis_message_header: String,
    pub delete_surface_message_header: String,
    pub restore_surface_message_header: String,

    pub add_material_message_header: String,
    pub update_material_message_header: String,
    pub delete_material_message_header: String,
    pub restore_material_message_header: String,

    pub add_truss_section_message_header: String,
    pub update_truss_section_message_header: String,
    pub delete_truss_section_message_header: String,
    pub restore_truss_section_message_header: String,

    pub add_beam_section_message_header: String,
    pub update_beam_section_message_header: String,
    pub delete_beam_section_message_header: String,
    pub restore_beam_section_message_header: String,

    pub add_plate_section_message_header: String,
    pub update_plate_section_message_header: String,
    pub delete_plate_section_message_header: String,
    pub restore_plate_section_message_header: String,

    pub add_properties_message_header: String,
    pub update_properties_message_header: String,
    pub delete_properties_message_header: String,
    pub restore_properties_message_header: String,

    pub assign_properties_to_lines_message_header: String,

    pub add_beam_section_local_axis_1_direction_message_header: String,
    pub delete_beam_section_local_axis_1_direction_message_header: String,
    pub assign_beam_section_local_axis_1_direction_message_header: String,
    pub restore_beam_section_local_axis_1_direction_message_header: String,

    pub assign_properties_to_surfaces_message_header: String,

    pub add_concentrated_load_message_header: String,
    pub update_concentrated_load_message_header: String,
    pub delete_concentrated_load_message_header: String,
    pub restore_concentrated_load_message_header: String,

    pub add_uniformly_distributed_line_load_message_header: String,
    pub update_uniformly_distributed_line_load_message_header: String,
    pub delete_uniformly_distributed_line_load_message_header: String,
    pub restore_uniformly_distributed_line_load_message_header: String,

    pub add_uniformly_distributed_surface_load_message_header: String,
    pub update_uniformly_distributed_surface_load_message_header: String,
    pub delete_uniformly_distributed_surface_load_message_header: String,
    pub restore_uniformly_distributed_surface_load_message_header: String,

    pub add_point_boundary_condition_message_header: String,
    pub update_point_boundary_condition_message_header: String,
    pub delete_point_boundary_condition_message_header: String,
    pub restore_point_boundary_condition_message_header: String,

    pub update_global_mesh_seed_message_header: String,

    pub update_lines_mesh_seed_message_header: String,
    pub undo_lines_mesh_seed_update_message_header: String,

    pub update_surfaces_mesh_seed_message_header: String,
    pub undo_surfaces_mesh_seed_update_message_header: String,
    
    pub undo_message_header: String,
    pub redo_message_header: String,

    pub local_axis_1_direction_input_info_message_header: String,

    pub max_point_number: u32,
    pub max_line_number: u32,
    pub max_surface_number: u32,
}


impl Props
{
    pub(crate) fn create(
        add_point_message_header: String,
        update_point_message_header: String,
        delete_point_message_header: String,
        restore_point_message_header: String,

        add_line_message_header: String,
        update_line_message_header: String,
        delete_line_message_header: String,
        restore_line_message_header: String,

        add_surface_message_header: String,
        update_surface_message_header: String,
        rotate_surface_vertices_clockwise_message_header: String,
        rotate_surface_vertices_counter_clockwise_message_header: String,
        flip_surface_normal_axis_message_header: String,
        delete_surface_message_header: String,
        restore_surface_message_header: String,

        add_material_message_header: String,
        update_material_message_header: String,
        delete_material_message_header: String,
        restore_material_message_header: String,

        add_truss_section_message_header: String,
        update_truss_section_message_header: String,
        delete_truss_section_message_header: String,
        restore_truss_section_message_header: String,

        add_beam_section_message_header: String,
        update_beam_section_message_header: String,
        delete_beam_section_message_header: String,
        restore_beam_section_message_header: String,

        add_plate_section_message_header: String,
        update_plate_section_message_header: String,
        delete_plate_section_message_header: String,
        restore_plate_section_message_header: String,

        add_properties_message_header: String,
        update_properties_message_header: String,
        delete_properties_message_header: String,
        restore_properties_message_header: String,

        assign_properties_to_lines_message_header: String,

        add_beam_section_local_axis_1_direction_message_header: String,
        delete_beam_section_local_axis_1_direction_message_header: String,
        assign_beam_section_local_axis_1_direction_message_header: String,
        restore_beam_section_local_axis_1_direction_message_header: String,

        assign_properties_to_surfaces_message_header: String,

        add_concentrated_load_message_header: String,
        update_concentrated_load_message_header: String,
        delete_concentrated_load_message_header: String,
        restore_concentrated_load_message_header: String,

        add_uniformly_distributed_line_load_message_header: String,
        update_uniformly_distributed_line_load_message_header: String,
        delete_uniformly_distributed_line_load_message_header: String,
        restore_uniformly_distributed_line_load_message_header: String,

        add_uniformly_distributed_surface_load_message_header: String,
        update_uniformly_distributed_surface_load_message_header: String,
        delete_uniformly_distributed_surface_load_message_header: String,
        restore_uniformly_distributed_surface_load_message_header: String,

        add_point_boundary_condition_message_header: String,
        update_point_boundary_condition_message_header: String,
        delete_point_boundary_condition_message_header: String,
        restore_point_boundary_condition_message_header: String,

        update_global_mesh_seed_message_header: String,

        update_lines_mesh_seed_message_header: String,
        undo_lines_mesh_seed_update_message_header: String,

        update_surfaces_mesh_seed_message_header: String,
        undo_surfaces_mesh_seed_update_message_header: String,

        undo_message_header: String,
        redo_message_header: String,

        local_axis_1_direction_input_info_message_header: String,

        max_point_number: u32,
        max_line_number: u32,
        max_surface_number: u32,
    ) 
        -> Self
    {
        Props 
        { 
            add_point_message_header,
            update_point_message_header,
            delete_point_message_header,
            restore_point_message_header,

            add_line_message_header,
            update_line_message_header,
            delete_line_message_header,
            restore_line_message_header,

            add_surface_message_header,
            update_surface_message_header,
            rotate_surface_vertices_clockwise_message_header,
            rotate_surface_vertices_counter_clockwise_message_header,
            flip_surface_normal_axis_message_header,
            delete_surface_message_header,
            restore_surface_message_header,

            add_material_message_header,
            update_material_message_header,
            delete_material_message_header,
            restore_material_message_header,

            add_truss_section_message_header,
            update_truss_section_message_header,
            delete_truss_section_message_header,
            restore_truss_section_message_header,

            add_beam_section_message_header,
            update_beam_section_message_header,
            delete_beam_section_message_header,
            restore_beam_section_message_header,

            add_plate_section_message_header,
            update_plate_section_message_header,
            delete_plate_section_message_header,
            restore_plate_section_message_header,

            add_properties_message_header,
            update_properties_message_header,
            delete_properties_message_header,
            restore_properties_message_header,

            assign_properties_to_lines_message_header,

            add_beam_section_local_axis_1_direction_message_header,
            delete_beam_section_local_axis_1_direction_message_header,
            assign_beam_section_local_axis_1_direction_message_header,
            restore_beam_section_local_axis_1_direction_message_header,

            assign_properties_to_surfaces_message_header,

            add_concentrated_load_message_header,
            update_concentrated_load_message_header,
            delete_concentrated_load_message_header,
            restore_concentrated_load_message_header,

            add_uniformly_distributed_line_load_message_header,
            update_uniformly_distributed_line_load_message_header,
            delete_uniformly_distributed_line_load_message_header,
            restore_uniformly_distributed_line_load_message_header,

            add_uniformly_distributed_surface_load_message_header,
            update_uniformly_distributed_surface_load_message_header,
            delete_uniformly_distributed_surface_load_message_header,
            restore_uniformly_distributed_surface_load_message_header,

            add_point_boundary_condition_message_header,
            update_point_boundary_condition_message_header,
            delete_point_boundary_condition_message_header,
            restore_point_boundary_condition_message_header,

            update_global_mesh_seed_message_header,

            update_lines_mesh_seed_message_header,
            undo_lines_mesh_seed_update_message_header,

            update_surfaces_mesh_seed_message_header,
            undo_surfaces_mesh_seed_update_message_header,

            undo_message_header,
            redo_message_header,

            local_axis_1_direction_input_info_message_header,

            max_point_number,
            max_line_number,
            max_surface_number,
        }
    }
}
