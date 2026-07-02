use stano_common::id_type;

id_type!(BookId, uuid_v7);

#[derive(Clone, Debug)]
pub struct Book {
    pub id: BookId,
    pub title: String,
    pub author: String,
}

impl Book {
    pub fn new(title: String, author: String) -> Self {
        Self {
            id: BookId::new(),
            title,
            author,
        }
    }
}
