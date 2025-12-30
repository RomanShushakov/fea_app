use extended_matrix::SquareMatrix;


pub trait RotationMatrixTrait
{
    fn get_ref_optional_rotation_matrix(&self) -> &Option<SquareMatrix<f32>>;

    fn get_mut_ref_optional_rotation_matrix(&mut self) -> &mut Option<SquareMatrix<f32>>;

    fn set_optional_rotation_matrix(&mut self, rotation_matrix_elements: [f32; 9])
    {
        *self.get_mut_ref_optional_rotation_matrix() = 
            Some(SquareMatrix::create(3, &rotation_matrix_elements));
    }
}
