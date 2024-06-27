pub trait CopyFromId<ID> {
    fn copy_from_id(&self, id: ID) -> Self;
}