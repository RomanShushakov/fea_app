use serde::Serialize;


#[derive(Debug, Clone, Copy, Serialize)]
pub enum Status<T>
{
    Active,
    Changed(T),
    Deleted(T),
}
