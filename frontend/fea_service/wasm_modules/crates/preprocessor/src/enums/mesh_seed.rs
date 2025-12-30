use serde::Serialize;


#[derive(Debug, Clone, Serialize)]
pub enum MeshSeed
{
    Global(u8),
    Line(u8),
    Surface(u8, u8)
}
