use crate::{ObjectId, Transform};

pub trait InspectableList {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn item_mut(&mut self, index: usize) -> Option<FieldValue<'_>>;
    fn add(&mut self);
    fn remove(&mut self, index: usize);
}

pub trait AsFieldValue {
    fn as_field_value(&mut self) -> FieldValue<'_>;
}

pub enum FieldValue<'a> {
    F32(&'a mut f32),
    I32(&'a mut i32),
    Bool(&'a mut bool),
    ObjectReference(&'a mut Option<ObjectId>),
    Vector2(&'a mut [f32; 2]),
    Vector3(&'a mut [f32; 3]),
    Slider {
        current: &'a mut f32,
        range: &'a mut [f32; 2],
    }, //it's just an array that holds start and end along with the current value
    Text(&'a mut String),
    Enum {
        selected: &'a mut usize,
        options: &'static [&'static str],
    },
    List(&'a mut dyn InspectableList),
}

pub struct Field<'a> {
    pub name: &'static str,
    pub value: FieldValue<'a>,
}

impl AsFieldValue for f32 {
    fn as_field_value(&mut self) -> FieldValue<'_> {
        FieldValue::F32(self)
    }
}

impl AsFieldValue for i32 {
    fn as_field_value(&mut self) -> FieldValue<'_> {
        FieldValue::I32(self)
    }
}

impl AsFieldValue for bool {
    fn as_field_value(&mut self) -> FieldValue<'_> {
        FieldValue::Bool(self)
    }
}

impl AsFieldValue for String {
    fn as_field_value(&mut self) -> FieldValue<'_> {
        FieldValue::Text(self)
    }
}

impl<T: AsFieldValue + Default> InspectableList for Vec<T> {
    fn len(&self) -> usize {
        Vec::len(self)
    }

    fn item_mut(&mut self, index: usize) -> Option<FieldValue<'_>> {
        self.get_mut(index).map(|item| item.as_field_value())
    }

    fn add(&mut self) {
        self.push(T::default());
    }

    fn remove(&mut self, index: usize) {
        if index < self.len() {
            self.remove(index);
        }
    }
}

pub trait Inspectable {
    fn type_name(&self) -> &'static str;
    fn fields(&mut self) -> Vec<Field<'_>>;
}

impl Inspectable for Transform {
    fn type_name(&self) -> &'static str {
        "Transform"
    }

    fn fields(&mut self) -> Vec<Field<'_>> {
        vec![
            Field {
                name: "x_cells",
                value: FieldValue::F32(&mut self.x_cells),
            },
            Field {
                name: "y_cells",
                value: FieldValue::F32(&mut self.y_cells),
            },
            Field {
                name: "angle_rad",
                value: FieldValue::F32(&mut self.angle_rad),
            },
        ]
    }
}
