use crate::enums::Status;


pub trait StatusTrait
{
    type Key;
    fn get_mut_ref_status(&mut self) -> &mut Status<Self::Key>;
    fn get_status(&mut self) -> Status<Self::Key>
        where Self::Key: Clone
    {

        let ref_mut_status = self.get_mut_ref_status();
        let status = ref_mut_status.clone();
        status.clone()
    }
    fn set_status(&mut self, status: Status<Self::Key>)
    {
        let ref_mut_status = self.get_mut_ref_status();
        *ref_mut_status = status;
    }
}
