use crate::structs::LocalAxis1Direction;


#[derive(Debug, PartialEq, Clone)]
pub enum RelativeKey
{
    Property(String),
    LocalAxis1Direction(LocalAxis1Direction),
}
