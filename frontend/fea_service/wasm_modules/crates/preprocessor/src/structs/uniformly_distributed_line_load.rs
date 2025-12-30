use serde::Serialize;

use crate::types::FEFloat;


#[derive(Debug, Clone, Serialize)]
pub struct UniformlyDistributedLineLoad(u32, FEFloat, FEFloat, FEFloat);


impl UniformlyDistributedLineLoad
{
    pub fn create(qx: FEFloat, qy: FEFloat, qz: FEFloat, uid: u32) -> Self
    {
        UniformlyDistributedLineLoad(uid, qx, qy, qz)
    }


    pub fn set_load_components(&mut self, qx: FEFloat, qy: FEFloat, qz: FEFloat)
    {
        self.1 = qx;
        self.2 = qy;
        self.3 = qz;
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
