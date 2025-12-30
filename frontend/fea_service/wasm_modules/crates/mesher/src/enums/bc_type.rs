pub enum BCType
{
    Force,
    Displacment,
}


impl BCType
{
    pub fn as_string(&self) -> String
    {
        match self
        {
            BCType::Displacment => "displacement".to_string(),
            BCType::Force => "force".to_string(),
        }
    }
}
