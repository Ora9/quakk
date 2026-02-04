use std::{any::Any, fmt::Debug};

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

    // pub fn as_f32(&self) -> anyhow::Result<f32> {

    // }

    pub fn downcast_ref<T: DataType>(&self) -> Option<&T> {
        let inner = (&*self.inner) as &dyn Any;
        inner.downcast_ref::<T>()
    }
}

impl Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Data: {:?}", self.inner)
    }
}

// pub trait Value: Any {}

// type Number = f32;

// type Text = String;
