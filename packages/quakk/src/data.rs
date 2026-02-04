use std::{any::Any, fmt::Debug};

use anyhow::anyhow;

pub trait DataType: Any + Debug {}

impl<T> DataType for T where T: Any + Debug {}

pub struct Data {
    inner: Box<dyn DataType>,
}

impl Data {
    pub fn new(value: impl DataType) -> Self {
        Data {
            inner: Box::new(value),
        }
    }

    pub fn into_f32(self) -> Result<f32, anyhow::Error> {
        self.downcast::<f32>().ok_or(anyhow!("not an f32"))
    }

    pub fn into_string(self) -> Result<String, anyhow::Error> {
        self.downcast::<String>().ok_or(anyhow!("not an f32"))
    }

    pub fn downcast<T: DataType>(self) -> Option<T> {
        (self.inner as Box<dyn Any>)
            .downcast::<T>()
            .ok()
            .map(|data| *data)
    }

    pub fn downcast_ref<T: DataType>(&self) -> Option<&T> {
        ((&*self.inner) as &dyn Any).downcast_ref::<T>()
    }
}

impl Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Data: {:?}", self.inner)
    }
}
