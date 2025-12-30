use crate::functions::get_value_coeff;
use crate::props::Props;

#[derive(Clone, Copy, Debug)]
pub struct ExtremeElementsAnalysisValues
{
    optional_extreme_f_r: Option<[f32; 2]>,
    optional_extreme_f_s: Option<[f32; 2]>,
    optional_extreme_f_t: Option<[f32; 2]>,
    optional_extreme_m_r: Option<[f32; 2]>,
    optional_extreme_m_s: Option<[f32; 2]>,
    optional_extreme_m_t: Option<[f32; 2]>,
    optional_extreme_mem_f_r: Option<[f32; 2]>,
    optional_extreme_mem_f_s: Option<[f32; 2]>,
    optional_extreme_mem_f_r_s: Option<[f32; 2]>,
    optional_extreme_bend_m_r: Option<[f32; 2]>,
    optional_extreme_bend_m_s: Option<[f32; 2]>,
    optional_extreme_bend_m_r_s: Option<[f32; 2]>,
    optional_extreme_shear_f_r_t: Option<[f32; 2]>,
    optional_extreme_shear_f_s_t: Option<[f32; 2]>,
}


impl ExtremeElementsAnalysisValues
{
    pub fn init() -> Self
    {
        ExtremeElementsAnalysisValues 
        { 
            optional_extreme_f_r: None, 
            optional_extreme_f_s: None, 
            optional_extreme_f_t: None, 
            optional_extreme_m_r: None, 
            optional_extreme_m_s: None, 
            optional_extreme_m_t: None, 
            optional_extreme_mem_f_r: None, 
            optional_extreme_mem_f_s: None, 
            optional_extreme_mem_f_r_s: None, 
            optional_extreme_bend_m_r: None, 
            optional_extreme_bend_m_s: None, 
            optional_extreme_bend_m_r_s: None,
            optional_extreme_shear_f_r_t: None, 
            optional_extreme_shear_f_s_t: None, 
        }
    }


    pub fn update_f_r_loads(&mut self, extreme_f_r: [f32; 2])
    {
        if extreme_f_r != [0.0, 0.0] 
        {
            self.optional_extreme_f_r = Some(extreme_f_r);
        }
    }


    pub fn update_f_s_loads(&mut self, extreme_f_s: [f32; 2])
    {
        if extreme_f_s != [0.0, 0.0] 
        {
            self.optional_extreme_f_s = Some(extreme_f_s);
        }
    }


    pub fn update_f_t_loads(&mut self, extreme_f_t: [f32; 2])
    {
        if extreme_f_t != [0.0, 0.0] 
        {
            self.optional_extreme_f_t = Some(extreme_f_t);
        }
    }


    pub fn update_m_r_loads(&mut self, extreme_m_r: [f32; 2])
    {
        if extreme_m_r != [0.0, 0.0] 
        {
            self.optional_extreme_m_r = Some(extreme_m_r);
        }
    }


    pub fn update_m_s_loads(&mut self, extreme_m_s: [f32; 2])
    {
        if extreme_m_s != [0.0, 0.0] 
        {
            self.optional_extreme_m_s = Some(extreme_m_s);
        }
    }


    pub fn update_m_t_loads(&mut self, extreme_m_t: [f32; 2])
    {
        if extreme_m_t != [0.0, 0.0] 
        {
            self.optional_extreme_m_t = Some(extreme_m_t);
        }
    }


    pub fn update_mem_f_r_loads(&mut self, extreme_mem_f_r: [f32; 2])
    {
        if extreme_mem_f_r != [0.0, 0.0] 
        {
            self.optional_extreme_mem_f_r = Some(extreme_mem_f_r);
        }
    }


    pub fn update_mem_f_s_loads(&mut self, extreme_mem_f_s: [f32; 2])
    {
        if extreme_mem_f_s != [0.0, 0.0] 
        {
            self.optional_extreme_mem_f_s = Some(extreme_mem_f_s);
        }
    }


    pub fn update_mem_f_r_s_loads(&mut self, extreme_mem_f_r_s: [f32; 2])
    {
        if extreme_mem_f_r_s != [0.0, 0.0] 
        {
            self.optional_extreme_mem_f_r_s = Some(extreme_mem_f_r_s);
        }
    }


    pub fn update_bend_m_r_loads(&mut self, extreme_bend_m_r: [f32; 2])
    {
        if extreme_bend_m_r != [0.0, 0.0] 
        {
            self.optional_extreme_bend_m_r = Some(extreme_bend_m_r);
        }
    }


    pub fn update_bend_m_s_loads(&mut self, extreme_bend_m_s: [f32; 2])
    {
        if extreme_bend_m_s != [0.0, 0.0] 
        {
            self.optional_extreme_bend_m_s = Some(extreme_bend_m_s);
        }
    }


    pub fn update_bend_m_r_s_loads(&mut self, extreme_bend_m_r_s: [f32; 2])
    {
        if extreme_bend_m_r_s != [0.0, 0.0] 
        {
            self.optional_extreme_bend_m_r_s = Some(extreme_bend_m_r_s);
        }
    }


    pub fn update_shear_f_r_t_loads(&mut self, extreme_shear_f_r_t: [f32; 2])
    {
        if extreme_shear_f_r_t != [0.0, 0.0] 
        {
            self.optional_extreme_shear_f_r_t = Some(extreme_shear_f_r_t);
        }
    }


    pub fn update_shear_f_s_t_loads(&mut self, extreme_shear_f_s_t: [f32; 2])
    {
        if extreme_shear_f_s_t != [0.0, 0.0] 
        {
            self.optional_extreme_shear_f_s_t = Some(extreme_shear_f_s_t);
        }
    }


    pub fn get_f_r_coeff(&self, f_r: f32, props: &Props) -> Option<[f32; 5]>
    {
        get_value_coeff(&self.optional_extreme_f_r, f_r, props)
    }


    pub fn get_f_s_coeff(&self, f_s: f32, props: &Props) -> Option<[f32; 5]>
    {
        get_value_coeff(&self.optional_extreme_f_s, f_s, props)
    }


    pub fn get_f_t_coeff(&self, f_t: f32, props: &Props) -> Option<[f32; 5]>
    {
        get_value_coeff(&self.optional_extreme_f_t, f_t, props)
    }


    pub fn get_m_r_coeff(&self, m_r: f32, props: &Props) -> Option<[f32; 5]>
    {
        get_value_coeff(&self.optional_extreme_m_r, m_r, props)
    }


    pub fn get_m_s_coeff(&self, m_s: f32, props: &Props) -> Option<[f32; 5]>
    {
        get_value_coeff(&self.optional_extreme_m_s, m_s, props)
    }


    pub fn get_m_t_coeff(&self, m_t: f32, props: &Props) -> Option<[f32; 5]>
    {
        get_value_coeff(&self.optional_extreme_m_t, m_t, props)
    }


    pub fn get_mem_f_r_coeff(&self, mem_f_r: f32, props: &Props) -> Option<[f32; 5]>
    {
        get_value_coeff(&self.optional_extreme_mem_f_r, mem_f_r, props)
    }


    pub fn get_mem_f_s_coeff(&self, mem_f_s: f32, props: &Props) -> Option<[f32; 5]>
    {
        get_value_coeff(&self.optional_extreme_mem_f_s, mem_f_s, props)
    }


    pub fn get_mem_f_r_s_coeff(&self, mem_f_r_s: f32, props: &Props) -> Option<[f32; 5]>
    {
        get_value_coeff(&self.optional_extreme_mem_f_r_s, mem_f_r_s, props)
    }


    pub fn get_bend_m_r_coeff(&self, bend_m_r: f32, props: &Props) -> Option<[f32; 5]>
    {
        get_value_coeff(&self.optional_extreme_bend_m_r, bend_m_r, props)
    }


    pub fn get_bend_m_s_coeff(&self, bend_m_s: f32, props: &Props) -> Option<[f32; 5]>
    {
        get_value_coeff(&self.optional_extreme_bend_m_s, bend_m_s, props)
    }


    pub fn get_bend_m_r_s_coeff(&self, bend_m_r_s: f32, props: &Props) -> Option<[f32; 5]>
    {
        get_value_coeff(&self.optional_extreme_bend_m_r_s, bend_m_r_s, props)
    }


    pub fn get_shear_f_r_t_coeff(&self, shear_f_r_t: f32, props: &Props) -> Option<[f32; 5]>
    {
        get_value_coeff(&self.optional_extreme_shear_f_r_t, shear_f_r_t, props)
    }


    pub fn get_shear_f_s_t_coeff(&self, shear_f_s_t: f32, props: &Props) -> Option<[f32; 5]>
    {
        get_value_coeff(&self.optional_extreme_shear_f_s_t, shear_f_s_t, props)
    }


    pub fn get_optional_extreme_f_r(&self) -> Option<[f32; 2]>
    {
        self.optional_extreme_f_r
    }


    pub fn get_optional_extreme_f_s(&self) -> Option<[f32; 2]>
    {
        self.optional_extreme_f_s
    }


    pub fn get_optional_extreme_f_t(&self) -> Option<[f32; 2]>
    {
        self.optional_extreme_f_t
    }


    pub fn get_optional_extreme_m_r(&self) -> Option<[f32; 2]>
    {
        self.optional_extreme_m_r
    }


    pub fn get_optional_extreme_m_s(&self) -> Option<[f32; 2]>
    {
        self.optional_extreme_m_s
    }


    pub fn get_optional_extreme_m_t(&self) -> Option<[f32; 2]>
    {
        self.optional_extreme_m_t
    }


    pub fn get_optional_extreme_mem_f_r(&self) -> Option<[f32; 2]>
    {
        self.optional_extreme_mem_f_r
    }


    pub fn get_optional_extreme_mem_f_s(&self) -> Option<[f32; 2]>
    {
        self.optional_extreme_mem_f_s
    }


    pub fn get_optional_extreme_mem_f_r_s(&self) -> Option<[f32; 2]>
    {
        self.optional_extreme_mem_f_r_s
    }


    pub fn get_optional_extreme_bend_m_r(&self) -> Option<[f32; 2]>
    {
        self.optional_extreme_bend_m_r
    }


    pub fn get_optional_extreme_bend_m_s(&self) -> Option<[f32; 2]>
    {
        self.optional_extreme_bend_m_s
    }


    pub fn get_optional_extreme_bend_m_r_s(&self) -> Option<[f32; 2]>
    {
        self.optional_extreme_bend_m_r_s
    }


    pub fn get_optional_extreme_shear_f_r_t(&self) -> Option<[f32; 2]>
    {
        self.optional_extreme_shear_f_r_t
    }


    pub fn get_optional_extreme_shear_f_s_t(&self) -> Option<[f32; 2]>
    {
        self.optional_extreme_shear_f_s_t
    }
}
