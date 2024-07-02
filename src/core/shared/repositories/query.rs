pub struct Paged<T> {
    pub data: Vec<T>,
    pub meta: InfoPaged
}

pub struct InfoPaged {
    pub total_pages: usize,
    pub number: usize,
    pub size: usize,
}

pub struct Query {
    pub pagination: PaginationDef
}

pub struct PaginationDef {
    pub page_number: usize,
    pub page_size: usize
}