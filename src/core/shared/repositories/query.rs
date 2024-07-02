pub struct Paged<T> {
    pub data: Vec<T>,
    pub meta: PaginationDef
}

pub struct Query {
    pub pagination: PaginationDef
}

pub struct PaginationDef {
    pub page_number: usize,
    pub page_size: usize
}