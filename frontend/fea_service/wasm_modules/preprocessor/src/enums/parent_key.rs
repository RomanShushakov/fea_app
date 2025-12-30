use crate::enums::CrossSection;


#[derive(Debug, PartialEq, Clone)]
pub enum ParentKey
{
    Point(u32),
    Material(String),
    CrossSection(CrossSection),
}
