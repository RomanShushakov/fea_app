use serde::Serialize;

use crate::types::FEFloat;


#[derive(Debug, Clone, Serialize)]
pub struct UniformlyDistributedSurfaceLoad(u32, FEFloat, FEFloat, FEFloat);


impl UniformlyDistributedSurfaceLoad
{
    pub fn create(px: FEFloat, py: FEFloat, pz: FEFloat, uid: u32) -> Self
    {
        UniformlyDistributedSurfaceLoad(uid, px, py, pz)
    }


    pub fn set_load_components(&mut self, px: FEFloat, py: FEFloat, pz: FEFloat)
    {
        self.1 = px;
        self.2 = py;
        self.3 = pz;
    }


    pub fn get_load_components(&self) -> [FEFloat; 3]
    {
        [self.1, self.2, self.3]
    }


    pub fn get_uid(&self) -> u32
    {
        self.0
    }
}
