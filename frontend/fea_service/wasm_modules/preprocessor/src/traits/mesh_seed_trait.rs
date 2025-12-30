use crate::enums::MeshSeed;


pub trait MeshSeedTrait
{
    fn get_ref_mesh_seed(&self) -> &Option<MeshSeed>;
    fn get_mut_ref_mesh_seed(&mut self) -> &mut Option<MeshSeed>;
    fn is_mesh_seed_assigned(&self) -> bool
    {
        self.get_ref_mesh_seed().is_some()
    }


    fn is_mesh_seed_global(&self) -> bool
    {
        if let Some(mesh_seed) = self.get_ref_mesh_seed().as_ref()
        {
            match mesh_seed
            {
                MeshSeed::Global(_) => return true,
                _ => return false,
            }
        }
        false
    }


    fn set_mesh_seed(&mut self, mesh_seed: Option<MeshSeed>)
    {
        *self.get_mut_ref_mesh_seed() = mesh_seed;
    }
}
