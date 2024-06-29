pub trait CanGetId<REF> {
    fn id(&self) -> REF;
}