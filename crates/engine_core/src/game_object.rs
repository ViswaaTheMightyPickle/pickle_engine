use crate::{ObjectId, Transform, inspector::Inspectable};

pub struct GameObject {
    pub id: ObjectId,
    pub name: String,
    pub components: Vec<Box<dyn Inspectable>>,
}

impl GameObject {
    pub fn new(id: ObjectId, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            components: vec![Box::new(Transform::default())],
        }
    }
}
