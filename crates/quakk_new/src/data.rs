use std::{
    any::{Any, type_name_of_val},
    fmt::Debug,
};

use anyhow::anyhow;

pub trait Data: Any + Debug {}

impl<T> Data for T where T: Any + Debug {}

// #[derive(Debug)]
pub struct DataBox {
    inner: Box<dyn Data>,
}

impl Debug for DataBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DataBox")
            .field("inner", &self.inner)
            .field("_type", &std::any::type_name_of_val(&self.inner))
            .finish()
    }
}

impl DataBox {
    pub fn new(value: impl Data) -> Self {
        DataBox {
            inner: Box::new(value),
        }
    }

    pub fn into_f32(self) -> Result<f32, anyhow::Error> {
        self.downcast::<f32>().ok_or(anyhow!("not an f32"))
    }

    pub fn downcast<T: Data>(self) -> Option<T> {
        (self.inner as Box<dyn Any>)
            .downcast::<T>()
            .ok()
            .map(|data| *data)
    }

    pub fn downcast_ref<T: Data>(&self) -> Option<&T> {
        ((&*self.inner) as &dyn Any).downcast_ref::<T>()
    }
}

impl From<f32> for DataBox {
    fn from(value: f32) -> Self {
        DataBox::new(value)
    }
}

impl From<u32> for DataBox {
    fn from(value: u32) -> Self {
        DataBox::new(value)
    }
}
