use crate::enums::GLMode;
use crate::structs::{Primitives, FloatNumbers};
use crate::traits::{DenotationTrait, SelectedObjectTrait};
use crate::functions::convert_vec_to_array;
use crate::Props;


#[derive(Debug, Clone)]
pub struct Point
{
    number: u32,
    primitives_for_selection: Primitives,
    primitives_visible: Primitives,
    primitives_visible_selected: Primitives,
}


impl Point
{
    fn get_primitives_for_selection(transformed_uid: [u8; 4], x: f32, y: f32, z: f32) -> Primitives
    {
        let mut primitives_for_selection = Primitives::create();
        primitives_for_selection.extend_points_coordinates(&[x, y, z]);
        primitives_for_selection.extend_points_is_to_scale(&[1.0]);
        primitives_for_selection.extend_points_reference_points(&[x, y, z]);
        primitives_for_selection.extend_points_displacements(&[0.0, 0.0, 0.0]);
        let color_for_selection = transformed_uid.map(|v| v as f32 / 255.0);
        primitives_for_selection.extend_points_colors(&color_for_selection);
        primitives_for_selection
    }


    pub fn create(
        transformed_uid: [u8; 4], 
        number: u32, 
        x: f32, 
        y: f32, 
        z: f32, 
        props: &Props
    ) 
        -> Self
    {
        let primitives_for_selection = Point::get_primitives_for_selection(transformed_uid, x, y, z);

        let mut primitives_visible = primitives_for_selection.clone();
        let mut points_colors = FloatNumbers::create();
        points_colors.extend(&props.point_color);
        primitives_visible.set_points_colors(points_colors);
        let mut primitives_visible_selected = primitives_for_selection.clone();
        
        let mut selected_points_colors = FloatNumbers::create();
        selected_points_colors.extend(&props.drawn_object_selected_color);
        primitives_visible_selected.set_points_colors(selected_points_colors);
        Point { number, primitives_for_selection, primitives_visible, primitives_visible_selected }
    }


    pub fn get_primitives(&self, gl_mode: &GLMode, is_selected: bool) -> Primitives
    {
        match gl_mode
        {
            GLMode::Selection => self.primitives_for_selection.clone(),
            GLMode::Visible => 
            {
                if is_selected
                {
                    self.primitives_visible_selected.clone()
                }
                else
                {
                    self.primitives_visible.clone()
                }
            }
        }
    }


    pub fn update_coordinates(&mut self, x: f32, y: f32, z: f32)
    {
        let mut points_coordinates = FloatNumbers::create();
        points_coordinates.extend(&[x, y, z]);
        self.primitives_for_selection.set_points_coordinates(points_coordinates.clone());
        self.primitives_visible.set_points_coordinates(points_coordinates.clone());
        self.primitives_visible_selected.set_points_coordinates(points_coordinates);
    }


    pub fn get_number(&self) -> u32
    {
        self.number
    }


    pub fn get_coordinates(&self) -> [f32; 3]
    {
        let coordinates = convert_vec_to_array(
            self.primitives_for_selection.get_ref_points_coordinates().to_vec());
        coordinates
    }
}


impl DenotationTrait for Point
{
    fn get_notation(&self) -> String 
    {
        self.number.to_string()
    }


    fn get_center(&self) -> [f32; 3] 
    {
        let center = convert_vec_to_array(self.primitives_for_selection.get_ref_points_coordinates().to_vec());
        center
    }


    fn get_color_str(&self, is_selected: bool) -> String 
    {
        let color_array_data = 
            if is_selected
            {
                self.primitives_visible_selected.get_ref_points_colors()
            }
            else
            {
                self.primitives_visible.get_ref_points_colors()
            };
        let color_array: [f32; 4] = convert_vec_to_array(color_array_data.to_vec());
        let color_str = format!(
            "rgb({}, {}, {})", color_array[0] * 255.0, color_array[1] * 255.0, color_array[2] * 255.0,
        );
        color_str
    }
}


impl SelectedObjectTrait for Point {}
