use crate::types::FEFloat;


#[derive(Clone)]
pub struct Props
{
    pub rel_tol: FEFloat,
    pub abs_tol: FEFloat,
    pub event_target: String,

    pub add_point_event_name: String,
    pub update_point_event_name: String,
    pub delete_point_event_name: String,

    pub add_line_event_name: String,
    pub update_line_event_name: String,
    pub delete_line_event_name: String,

    pub add_surface_event_name: String,
    pub update_surface_event_name: String,
    pub delete_surface_event_name: String,

    pub add_material_event_name: String,
    pub update_material_event_name: String,
    pub delete_material_event_name: String,

    pub add_truss_section_event_name: String,
    pub update_truss_section_event_name: String,
    pub delete_truss_section_event_name: String,

    pub add_beam_section_event_name: String,
    pub update_beam_section_event_name: String,
    pub delete_beam_section_event_name: String,

    pub add_plate_section_event_name: String,
    pub update_plate_section_event_name: String,
    pub delete_plate_section_event_name: String,

    pub add_properties_event_name: String,
    pub update_properties_event_name: String,
    pub delete_properties_event_name: String,

    pub increase_action_id_event_name: String,

    pub add_beam_local_axis_1_direction_event_name: String,
    pub delete_beam_local_axis_1_direction_event_name: String,
    pub update_beam_section_orientation_data_event_name: String,

    pub local_axis_1_direction_input_info_message_header: String,

    pub add_concentrated_load_event_name: String,
    pub update_concentrated_load_event_name: String,
    pub delete_concentrated_load_event_name: String,

    pub add_point_boundary_condition_event_name: String,
    pub update_point_boundary_condition_event_name: String,
    pub delete_point_boundary_condition_event_name: String,

    pub update_global_mesh_seed_event_name: String,
}


impl Props
{
    pub(super) fn create(
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

        update_global_mesh_seed_event_name: String,
    ) 
        -> Self
    {
        Props 
        { 
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
        }
    }
}
