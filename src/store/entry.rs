use uuid::Uuid;

pub struct Entry {
    pub id: Uuid,
    pub key: String,
    pub value: Vec<u8>,
}

impl Entry {
    pub fn new(key: String, value: Vec<u8>) -> Self {
        Self {
            id: Uuid::new_v4(),
            key,
            value,
        }
    }
}
