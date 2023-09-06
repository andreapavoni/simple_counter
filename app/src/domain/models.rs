#[derive(Clone)]
pub struct Counter {
    pub id: String,
    pub name: String,
    pub value: i32,
}

impl Counter {
    pub fn new(name: String, value: i32) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            value,
        }
    }
}

impl std::fmt::Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Counter {{ id: {}, name: {}, value: {} }}", self.id, self.name, self.value)
    }
}
