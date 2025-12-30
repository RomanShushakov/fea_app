use crate::structs::ExtremeGlobalAnalysisValues;
use crate::props::Props;


#[derive(Debug)]
pub struct NodeData
{
    pub uid: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub ux: f32,
    pub uy: f32,
    pub uz: f32,
    pub optional_u_result_coeff: Option<[f32; 5]>,
    pub optional_fx_coeff: Option<[f32; 5]>,
    pub optional_fy_coeff: Option<[f32; 5]>,
    pub optional_fz_coeff: Option<[f32; 5]>,
    pub optional_mx_coeff: Option<[f32; 5]>,
    pub optional_my_coeff: Option<[f32; 5]>,
    pub optional_mz_coeff: Option<[f32; 5]>,
}


impl NodeData
{
    pub fn init(uid: u32, x: f32, y: f32, z: f32) -> Self
    {
        NodeData 
        { 
            uid,
            x,
            y,
            z,
            ux: 0.0, 
            uy: 0.0, 
            uz: 0.0,
            optional_u_result_coeff: None,
            optional_fx_coeff: None, 
            optional_fy_coeff: None, 
            optional_fz_coeff: None, 
            optional_mx_coeff: None, 
            optional_my_coeff: None, 
            optional_mz_coeff: None,
        }
    }


    pub fn update(
        &mut self,
        ux: f32,
        uy: f32,
        uz: f32,
        fx: f32,
        fy: f32,
        fz: f32,
        mx: f32,
        my: f32,
        mz: f32,
        extreme_global_displacements_data: &ExtremeGlobalAnalysisValues,
        extreme_global_loads_data: &ExtremeGlobalAnalysisValues,
        props: &Props,
    )
    {
        self.ux = ux;
        self.uy = uy;
        self.uz = uz;

        let u_result = (ux.powi(2) + uy.powi(2) + uz.powi(2)).sqrt();
        self.optional_u_result_coeff = extreme_global_displacements_data.get_u_f_result_coeff(
            u_result, props,
        );

        self.optional_fx_coeff = extreme_global_loads_data.get_u_f_coeff(fx, props);
        self.optional_fy_coeff = extreme_global_loads_data.get_u_f_coeff(fy, props);
        self.optional_fz_coeff = extreme_global_loads_data.get_u_f_coeff(fz, props);
        self.optional_mx_coeff = extreme_global_loads_data.get_r_m_coeff(mx, props);
        self.optional_my_coeff = extreme_global_loads_data.get_r_m_coeff(my, props);
        self.optional_mz_coeff = extreme_global_loads_data.get_r_m_coeff(mz, props);
    }
}
