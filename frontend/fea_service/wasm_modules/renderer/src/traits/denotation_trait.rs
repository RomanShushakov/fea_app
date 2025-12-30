use crate::structs::Denotation;


pub trait DenotationTrait
{
    fn get_notation(&self) -> String;
    fn get_center(&self) -> [f32; 3];
    fn get_color_str(&self, is_selected: bool) -> String;
    fn get_denotation(&self, is_selected: bool) -> Denotation
    {
        let notation = self.get_notation();
        let center = self.get_center();
        let color = self.get_color_str(is_selected);
        Denotation { notation, center, color }
    }
}
