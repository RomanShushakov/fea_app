use wasm_bindgen::JsValue;
use std::f32::consts::PI;

use extended_matrix::{Vector3, Matrix, VectorTrait, BasicOperationsTrait, Position};


fn discretize_circle_on_plane(base_points_number: u32, radius: f32, abs_tol: f32) -> Vec<(f32, f32)>
{
    let d_angle = 2.0 * PI / base_points_number as f32;
    let local_coordinates = (0..base_points_number)
        .map(|point_number|
            {
                let angle = d_angle * point_number as f32;
                let local_x =
                    {
                        let value = radius * angle.cos();
                        if value.abs() < abs_tol { 0.0 } else { value }
                    };
                let local_y =
                    {
                        let value = radius * angle.sin();
                        if value.abs() < abs_tol { 0.0 } else { value }
                    };
                (local_x, local_y)
            })
        .collect::<Vec<(f32, f32)>>();
    local_coordinates
}


fn find_direction_vector_coordinates(coordinates: [f32; 3], rotation_matrix: &Matrix<f32>) 
    -> Result<[f32; 3], JsValue>
{
    let local_direction_vector = Vector3::create(&coordinates);
    let transformed_direction_vector = rotation_matrix.multiply(&local_direction_vector)
        .map_err(|e| JsValue::from(e))?;
    let direction_vector_x_coordinate = transformed_direction_vector
        .get_element_value(&Position(0, 0)).map_err(|e| JsValue::from(e))?;
    let direction_vector_y_coordinate = transformed_direction_vector
        .get_element_value(&Position(1, 0)).map_err(|e| JsValue::from(e))?;
    let direction_vector_z_coordinate = transformed_direction_vector
        .get_element_value(&Position(2, 0)).map_err(|e| JsValue::from(e))?;
    Ok([*direction_vector_x_coordinate, *direction_vector_y_coordinate, *direction_vector_z_coordinate])
}


fn extend_triangles_vertices_coordinates(vertex_coordinates: &[f32; 3], coordinates: [f32; 3], 
    rotation_matrix: &Matrix<f32>, triangles_vertices_coordinates: &mut Vec<f32>) 
    -> Result<(), JsValue>
{
    let direction_vector_coordinates_1 = 
        find_direction_vector_coordinates(coordinates, &rotation_matrix)?;
    triangles_vertices_coordinates.extend(&[
        vertex_coordinates[0] + direction_vector_coordinates_1[0],
        vertex_coordinates[1] + direction_vector_coordinates_1[1],
        vertex_coordinates[2] + direction_vector_coordinates_1[2]
    ]);
    Ok(())
}


pub fn monochrome_cone(
    vertex_coordinates: &[f32; 3], 
    base_center_point_coordinates: &[f32; 3], 
    height: f32, 
    radius: f32, 
    base_points_number: u32,  
    color: &[f32; 4], 
    abs_tol: f32,
    rel_tol: f32
)
    -> Result<(Vec<f32>, Vec<f32>), JsValue>
{
    if base_points_number < 2
    {
        return Err(JsValue::from("The base points number should be at least 2"));
    }

    let mut triangles_vertices_coordinates = Vec::new();
    let mut triangles_vertices_colors_values = Vec::new();

    let direction_vector = Vector3::create(&[
        base_center_point_coordinates[0] - vertex_coordinates[0],
        base_center_point_coordinates[1] - vertex_coordinates[1],
        base_center_point_coordinates[2] - vertex_coordinates[2],
    ]);
    let oriented_direction_vector = Vector3::create(&[
        direction_vector.norm().map_err(|e| JsValue::from(e))?, 0.0, 0.0
    ]);
    let rotation_matrix = oriented_direction_vector.rotation_matrix_to_align_with_vector(
        &direction_vector, rel_tol, abs_tol).map_err(|e| JsValue::from(e))?;

    let local_coordinates = discretize_circle_on_plane(base_points_number, radius, abs_tol);

    for i in 1..local_coordinates.len()
    {
        let coordinates_1 = [height, local_coordinates[i].1, local_coordinates[i].0];
        extend_triangles_vertices_coordinates(vertex_coordinates, coordinates_1, &rotation_matrix, 
            &mut triangles_vertices_coordinates)?;

        let coordinates_2 = [height, local_coordinates[i - 1].1, local_coordinates[i - 1].0];
        extend_triangles_vertices_coordinates(vertex_coordinates, coordinates_2, &rotation_matrix, 
            &mut triangles_vertices_coordinates)?;

        triangles_vertices_coordinates.extend(vertex_coordinates);

        for _ in 0..3
        {
            triangles_vertices_colors_values.extend(color);    
        }
    }

    let coordinates_1 = [height, local_coordinates[0].1, local_coordinates[0].0];
    extend_triangles_vertices_coordinates(vertex_coordinates, coordinates_1, &rotation_matrix, 
        &mut triangles_vertices_coordinates)?;

    let coordinates_2 = [height, local_coordinates[local_coordinates.len() - 1].1, 
        local_coordinates[local_coordinates.len() - 1].0];
    extend_triangles_vertices_coordinates(vertex_coordinates, coordinates_2, &rotation_matrix, 
        &mut triangles_vertices_coordinates)?;
    triangles_vertices_coordinates.extend(vertex_coordinates);

    for _ in 0..3
    {
        triangles_vertices_colors_values.extend(color);    
    }

    Ok((triangles_vertices_coordinates, triangles_vertices_colors_values))
}
