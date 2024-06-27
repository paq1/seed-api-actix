pub trait CopyFromId {
    fn copy_from_id(&self, id: String) -> Self;
}