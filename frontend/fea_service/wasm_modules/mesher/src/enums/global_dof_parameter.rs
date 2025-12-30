pub enum GlobalDOFParameter
{
    X,
    Y,
    Z,
    ThX,
    ThY,
    ThZ,
}


impl GlobalDOFParameter
{
    pub fn as_string(&self) -> String
    {
        match self
        {
            GlobalDOFParameter::X => "x".to_string(),
            GlobalDOFParameter::Y => "y".to_string(),
            GlobalDOFParameter::Z => "z".to_string(),
            GlobalDOFParameter::ThX => "th_x".to_string(),
            GlobalDOFParameter::ThY => "th_y".to_string(),
            GlobalDOFParameter::ThZ => "th_z".to_string(),
        }
    }
}
