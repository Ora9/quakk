use std::{any::Any, fmt::Debug};

use anyhow::anyhow;

#[derive(Debug)]
pub struct DataTypeDef {
    pub title: String,
    pub color: u32,
}

pub trait DataTrait: Any + Debug {
    fn type_def(&self) -> DataTypeDef;
}

pub struct Data {
    inner: Box<dyn DataTrait>,
    def: DataTypeDef,
}

impl Data {
    pub fn new(value: impl DataTrait) -> Self {
        Data {
            def: value.type_def(),

            inner: Box::new(value),
        }
    }

    pub fn into_number(self) -> Result<Number, anyhow::Error> {
        self.downcast::<Number>().ok_or(anyhow!("not a number"))
    }

    pub fn downcast<T: DataTrait>(self) -> Option<T> {
        (self.inner as Box<dyn Any>)
            .downcast::<T>()
            .ok()
            .map(|data| *data)
    }

    pub fn downcast_ref<T: DataTrait>(&self) -> Option<&T> {
        ((&*self.inner) as &dyn Any).downcast_ref::<T>()
    }
}

impl Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{}({:?})", self.def.title, self.inner)
        } else {
            write!(f, "{:?}", self.inner)
        }
    }
}

pub type Number = f64;

impl From<Number> for Data {
    fn from(value: Number) -> Self {
        Data::new(value)
    }
}

impl DataTrait for Number {
    fn type_def(&self) -> DataTypeDef {
        DataTypeDef {
            title: "Number".to_string(),
            color: 55,
        }
    }
}
