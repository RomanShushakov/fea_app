pub struct BoundingBox
{
    pub optional_bounds: Option<[f32; 6]>,  // [x_min, y_min, z_min, x_max, y_max, z_max]
}


impl BoundingBox
{
    pub fn create() -> Self
    {
        BoundingBox { optional_bounds: None }
    }


    pub fn expand_bounds(&mut self, coordinates: &[f32; 3])
    {
        if let Some(bounds) = self.optional_bounds.as_mut()
        {
            for i in 0..3
            {
                if coordinates[i] < bounds[i] 
                {
                    bounds[i] = coordinates[i];
                }
                if coordinates[i] > bounds[i + 3]
                {
                    bounds[i + 3] = coordinates[i];
                }
            }
        }
        else
        {
            let bounds = [
                coordinates[0], coordinates[1], coordinates[2],
                coordinates[0], coordinates[1], coordinates[2],
            ];
            self.optional_bounds = Some(bounds);
        }
    }


    pub fn get_center(&self) -> [f32; 3]
    {
        if let Some(bounds) = self.optional_bounds
        {
            [(bounds[0] + bounds[3]) / 2.0, (bounds[1] + bounds[4]) / 2.0, (bounds[2] + bounds[5]) / 2.0]
        }
        else
        {
            [0.0, 0.0, 0.0]
        }
    }


    pub fn get_scale(&self) -> f32
    {
        if let Some(bounds) = self.optional_bounds
        {
            let diagonal = f32::sqrt(
                (bounds[3] - bounds[0]).powi(2) + 
                (bounds[4] - bounds[1]).powi(2) + 
                (bounds[5] - bounds[2]).powi(2)
            );
            if diagonal == 0.0
            {
                1.0
            }
            else
            {
                2.0 / diagonal
            }
        }
        else 
        {
            1.0    
        }
    }


    pub fn reset(&mut self)
    {
        self.optional_bounds = None;
    }
}
