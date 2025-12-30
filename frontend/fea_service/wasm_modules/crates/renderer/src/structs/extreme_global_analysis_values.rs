use crate::functions::get_value_coeff;
use crate::props::Props;


#[derive(Clone, Copy, Debug)]
pub struct ExtremeGlobalAnalysisValues
{
    optional_extreme_u_f: Option<[f32; 2]>,
    optional_extreme_r_m: Option<[f32; 2]>,
    optional_extreme_u_f_result: Option<[f32; 2]>,
    optional_extreme_r_m_result: Option<[f32; 2]>,
}


impl ExtremeGlobalAnalysisValues
{
    pub fn create(
        extreme_u_f_x: [f32; 2],
        extreme_u_f_y: [f32; 2],
        extreme_u_f_z: [f32; 2],
        extreme_r_m_x: [f32; 2],
        extreme_r_m_y: [f32; 2],
        extreme_r_m_z: [f32; 2],
        extreme_u_f_result: [f32; 2],
        extreme_r_m_result: [f32; 2],
    )
        -> Self
    {
        let min_u_f = extreme_u_f_x[0].min(extreme_u_f_y[0]).min(extreme_u_f_z[0]);
        let max_u_f = extreme_u_f_x[1].max(extreme_u_f_y[1]).max(extreme_u_f_z[1]);
        let optional_extreme_u_f = 
            if [min_u_f, max_u_f] != [0.0, 0.0] 
            {
                Some([min_u_f, max_u_f]) 
            } 
            else
            { 
                None
            };

        let min_r_m = extreme_r_m_x[0].min(extreme_r_m_y[0]).min(extreme_r_m_z[0]);
        let max_r_m = extreme_r_m_x[1].max(extreme_r_m_y[1]).max(extreme_r_m_z[1]);
        let optional_extreme_r_m = 
            if [min_r_m, max_r_m] != [0.0, 0.0] 
            {
                Some([min_r_m, max_r_m]) 
            } 
            else
            { 
                None
            };

        let optional_extreme_u_f_result = 
            if extreme_u_f_result != [0.0, 0.0] 
            {
                Some(extreme_u_f_result) 
            } 
            else
            { 
                None
            };

        let optional_extreme_r_m_result = 
            if extreme_r_m_result != [0.0, 0.0] 
            {
                Some(extreme_r_m_result) 
            } 
            else
            { 
                None
            };

        ExtremeGlobalAnalysisValues 
        { 
            optional_extreme_u_f,
            optional_extreme_r_m,
            optional_extreme_u_f_result,
            optional_extreme_r_m_result,
        }
    }


    pub fn get_u_f_coeff(&self, u_f: f32, props: &Props) -> Option<[f32; 5]>
    {
        get_value_coeff(&self.optional_extreme_u_f, u_f, props)
    }


    pub fn get_r_m_coeff(&self, r_m: f32, props: &Props) -> Option<[f32; 5]>
    {
        get_value_coeff(&self.optional_extreme_r_m, r_m, props)
    }


    pub fn get_u_f_result_coeff(&self, u_f_result: f32, props: &Props) -> Option<[f32; 5]>
    {
        get_value_coeff(&self.optional_extreme_u_f_result, u_f_result, props)
    }

    pub fn get_r_m_result_coeff(&self, r_m_result: f32, props: &Props) -> Option<[f32; 5]>
    {
        get_value_coeff(&self.optional_extreme_r_m_result, r_m_result, props)
    }


    pub fn get_optional_extreme_u_f_result(&self) -> Option<[f32; 2]>
    {
        self.optional_extreme_u_f_result
    }


    pub fn get_optional_extreme_u_f(&self) -> Option<[f32; 2]>
    {
        self.optional_extreme_u_f
    }


    pub fn get_optional_extreme_r_m(&self) -> Option<[f32; 2]>
    {
        self.optional_extreme_r_m
    }
}
