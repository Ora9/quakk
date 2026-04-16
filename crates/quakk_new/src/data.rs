use std::{
    any::{Any, type_name_of_val},
    fmt::Debug,
};

use anyhow::anyhow;

pub trait DataTrait: Any + Debug {}

// impl<T> DataTrait for T where T: Any + Debug {}

// #[derive(Debug)]
pub struct Data {
    inner: Box<dyn DataTrait>,
}

impl Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DataBox")
            .field("inner", &self.inner)
            .field("_type", &std::any::type_name_of_val(&self.inner))
            .finish()
    }
}

impl Data {
    pub fn new(value: impl DataTrait) -> Self {
        Data {
            inner: Box::new(value),
        }
    }

    pub fn into_number(self) -> Result<Number, anyhow::Error> {
        self.downcast::<Number>().ok_or(anyhow!("not a number"))
    }

    // pub fn into_f32(self) -> Result<f32, anyhow::Error> {
    //     self.downcast::<f32>().ok_or(anyhow!("not an f32"))
    // }

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

impl From<Number> for Data {
    fn from(value: Number) -> Self {
        Data::new(value)
    }
}

// impl From<u32> for Data {
//     fn from(value: u32) -> Self {
//         Data::new(value)
//     }
// }

pub type Number = f64;
impl DataTrait for Number {}
