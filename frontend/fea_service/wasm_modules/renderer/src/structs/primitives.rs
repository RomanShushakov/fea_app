use std::ops::AddAssign;

use crate::structs::FloatNumbers;


#[derive(Clone, Debug)]
pub struct Primitives
{
    points_coordinates: FloatNumbers,
    points_is_to_scale: FloatNumbers,
    points_reference_points: FloatNumbers,
    points_displacements: FloatNumbers,
    points_colors: FloatNumbers,
    lines_endpoints_coordinates: FloatNumbers,
    lines_endpoints_is_to_scale: FloatNumbers,
    lines_endpoints_reference_points: FloatNumbers,
    lines_endpoints_displacements: FloatNumbers,
    lines_endpoints_colors: FloatNumbers,
    triangles_vertices_coordinates: FloatNumbers,
    triangles_vertices_is_to_scale: FloatNumbers,
    triangles_vertices_reference_points: FloatNumbers,
    triangles_vertices_displacements: FloatNumbers,
    triangles_vertices_colors: FloatNumbers,
}


impl Primitives
{
    pub fn create() -> Self
    {
        Primitives 
        { 
            points_coordinates: FloatNumbers::create(), 
            points_is_to_scale: FloatNumbers::create(),
            points_reference_points: FloatNumbers::create(),
            points_displacements: FloatNumbers::create(),
            points_colors: FloatNumbers::create(), 
            lines_endpoints_coordinates: FloatNumbers::create(), 
            lines_endpoints_is_to_scale: FloatNumbers::create(),
            lines_endpoints_reference_points: FloatNumbers::create(),
            lines_endpoints_displacements: FloatNumbers::create(),
            lines_endpoints_colors: FloatNumbers::create(), 
            triangles_vertices_coordinates: FloatNumbers::create(), 
            triangles_vertices_is_to_scale: FloatNumbers::create(),
            triangles_vertices_reference_points: FloatNumbers::create(),
            triangles_vertices_displacements: FloatNumbers::create(),
            triangles_vertices_colors: FloatNumbers::create(), 
        }
    }


    pub fn extend_points_coordinates(&mut self, point_coordinates: &[f32])
    {
        self.points_coordinates.extend(point_coordinates);
    }


    pub fn extend_points_is_to_scale(&mut self, point_is_to_scale: &[f32])
    {
        self.points_is_to_scale.extend(point_is_to_scale);
    }


    pub fn extend_points_reference_points(&mut self, point_reference_point: &[f32])
    {
        self.points_reference_points.extend(point_reference_point);
    }


    pub fn extend_points_displacements(&mut self, point_displacement: &[f32])
    {
        self.points_displacements.extend(point_displacement);
    }


    pub fn extend_points_colors(&mut self, point_color: &[f32])
    {
        self.points_colors.extend(point_color);
    }


    pub fn extend_lines_endpoints_coordinates(&mut self, endpoint_coordinates: &[f32])
    {
        self.lines_endpoints_coordinates.extend(endpoint_coordinates);
    }


    pub fn extend_lines_endpoints_is_to_scale(&mut self, endpoint_is_to_scale: &[f32])
    {
        self.lines_endpoints_is_to_scale.extend(endpoint_is_to_scale);
    }


    pub fn extend_lines_endpoints_reference_points(&mut self, endpoint_reference_point: &[f32])
    {
        self.lines_endpoints_reference_points.extend(endpoint_reference_point);
    }


    pub fn extend_lines_endpoints_displacements(&mut self, endpoint_displacement: &[f32])
    {
        self.lines_endpoints_displacements.extend(endpoint_displacement);
    }


    pub fn extend_lines_endpoints_colors(&mut self, endpoint_color: &[f32])
    {
        self.lines_endpoints_colors.extend(endpoint_color);
    }


    pub fn extend_triangles_vertices_coordinates(&mut self, vertex_coordinates: &[f32])
    {
        self.triangles_vertices_coordinates.extend(vertex_coordinates);
    }


    pub fn extend_triangles_vertices_is_to_scale(&mut self, vertex_is_to_scale: &[f32])
    {
        self.triangles_vertices_is_to_scale.extend(vertex_is_to_scale);
    }


    pub fn extend_triangles_vertices_reference_points(&mut self, vertex_reference_point: &[f32])
    {
        self.triangles_vertices_reference_points.extend(vertex_reference_point);
    }


    pub fn extend_triangles_vertices_displacements(&mut self, vertex_displacement: &[f32])
    {
        self.triangles_vertices_displacements.extend(vertex_displacement);
    }


    pub fn extend_triangles_vertices_colors(&mut self, vertex_color: &[f32])
    {
        self.triangles_vertices_colors.extend(vertex_color);
    }


    pub fn get_ref_points_coordinates(&self) -> &FloatNumbers
    {
        &self.points_coordinates
    }


    pub fn set_points_coordinates(&mut self, points_coordinates: FloatNumbers)
    {
        self.points_coordinates = points_coordinates;
    }


    pub fn get_ref_points_is_to_scale(&self) -> &FloatNumbers
    {
        &self.points_is_to_scale
    }


    pub fn set_points_is_to_scale(&mut self, points_is_to_scale: FloatNumbers)
    {
        self.points_is_to_scale = points_is_to_scale;
    }


    pub fn get_ref_points_reference_points(&self) -> &FloatNumbers
    {
        &self.points_reference_points
    }


    pub fn set_points_reference_points(&mut self, reference_points: FloatNumbers)
    {
        self.points_reference_points = reference_points;
    }


    pub fn get_ref_points_displacements(&self) -> &FloatNumbers
    {
        &self.points_displacements
    }


    pub fn set_points_displacements(&mut self, points_displacements: FloatNumbers)
    {
        self.points_displacements = points_displacements;
    }


    pub fn get_ref_points_colors(&self) -> &FloatNumbers
    {
        &self.points_colors
    }


    pub fn set_points_colors(&mut self, points_colors: FloatNumbers)
    {
        self.points_colors = points_colors;
    }


    pub fn get_ref_lines_endpoints_coordinates(&self) -> &FloatNumbers
    {
        &self.lines_endpoints_coordinates
    }


    pub fn set_lines_endpoints_coordinates(&mut self, lines_endpoints_coordinates: FloatNumbers)
    {
        self.lines_endpoints_coordinates = lines_endpoints_coordinates;
    }


    pub fn get_ref_lines_endpoints_is_to_scale(&self) -> &FloatNumbers
    {
        &self.lines_endpoints_is_to_scale
    }


    pub fn get_ref_lines_endpoints_reference_points(&self) -> &FloatNumbers
    {
        &self.lines_endpoints_reference_points
    }


    pub fn set_lines_endpoints_reference_points(&mut self, lines_endpoints_reference_points: FloatNumbers)
    {
        self.lines_endpoints_reference_points = lines_endpoints_reference_points;
    }


    pub fn get_ref_lines_endpoints_displacements(&self) -> &FloatNumbers
    {
        &self.lines_endpoints_displacements
    }


    pub fn set_lines_endpoints_displacements(&mut self, endpoints_displacements: FloatNumbers)
    {
        self.lines_endpoints_displacements = endpoints_displacements;
    }


    pub fn get_ref_lines_endpoints_colors(&self) -> &FloatNumbers
    {
        &self.lines_endpoints_colors
    }


    pub fn set_lines_endpoints_colors(&mut self, lines_endpoints_colors: FloatNumbers)
    {
        self.lines_endpoints_colors = lines_endpoints_colors;
    }


    pub fn get_ref_triangles_vertices_coordinates(&self) -> &FloatNumbers
    {
        &self.triangles_vertices_coordinates
    }


    pub fn set_triangles_vertices_coordinates(&mut self, triangles_vertices_coordinates: FloatNumbers)
    {
        self.triangles_vertices_coordinates = triangles_vertices_coordinates;
    }


    pub fn get_ref_triangles_vertices_is_to_scale(&self) -> &FloatNumbers
    {
        &self.triangles_vertices_is_to_scale
    }


    pub fn get_ref_triangles_vertices_reference_points(&self) -> &FloatNumbers
    {
        &self.triangles_vertices_reference_points
    }


    pub fn set_triangles_vertices_reference_points(&mut self, triangles_vertices_reference_points: FloatNumbers)
    {
        self.triangles_vertices_reference_points = triangles_vertices_reference_points;
    }


    pub fn get_ref_triangles_vertices_displacements(&self) -> &FloatNumbers
    {
        &self.triangles_vertices_displacements
    }


    pub fn set_triangles_vertices_displacements(&mut self, triangles_vertices_displacements: FloatNumbers)
    {
        self.triangles_vertices_displacements = triangles_vertices_displacements;
    }


    pub fn get_ref_triangles_vertices_colors(&self) -> &FloatNumbers
    {
        &self.triangles_vertices_colors
    }


    pub fn set_triangles_vertices_colors(&mut self, triangles_vertices_colors: FloatNumbers)
    {
        self.triangles_vertices_colors = triangles_vertices_colors;
    }


    pub fn get_points_count(&self) -> i32
    {
        (self.points_coordinates.len() / 3) as i32
    }


    pub fn get_lines_count(&self) -> i32
    {
        (self.lines_endpoints_coordinates.len() / 3) as i32
    }


    pub fn get_triangles_count(&self) -> i32
    {
        (self.triangles_vertices_coordinates.len() / 3) as i32
    }
}


impl AddAssign for Primitives
{
    fn add_assign(&mut self, other: Self) 
    {
        self.extend_points_coordinates(&other.get_ref_points_coordinates().to_vec());
        self.extend_points_is_to_scale(&other.get_ref_points_is_to_scale().to_vec());
        self.extend_points_reference_points(&other.get_ref_points_reference_points().to_vec());
        self.extend_points_displacements(&other.get_ref_points_displacements().to_vec());
        self.extend_points_colors(&other.get_ref_points_colors().to_vec());
        self.extend_lines_endpoints_coordinates(
            &other.get_ref_lines_endpoints_coordinates().to_vec());
        self.extend_lines_endpoints_is_to_scale(
            &other.get_ref_lines_endpoints_is_to_scale().to_vec());
        self.extend_lines_endpoints_reference_points(
            &other.get_ref_lines_endpoints_reference_points().to_vec());
        self.extend_lines_endpoints_displacements(
            &other.get_ref_lines_endpoints_displacements().to_vec());
        self.extend_lines_endpoints_colors(&other.get_ref_lines_endpoints_colors().to_vec());
        self.extend_triangles_vertices_coordinates(
            &other.get_ref_triangles_vertices_coordinates().to_vec());
        self.extend_triangles_vertices_is_to_scale(
            &other.get_ref_triangles_vertices_is_to_scale().to_vec());
        self.extend_triangles_vertices_reference_points(
            &other.get_ref_triangles_vertices_reference_points().to_vec());
        self.extend_triangles_vertices_displacements(
            &other.get_ref_triangles_vertices_displacements().to_vec());
        self.extend_triangles_vertices_colors(&other.get_ref_triangles_vertices_colors().to_vec());
    }
}
