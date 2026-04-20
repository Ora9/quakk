use std::{any::Any, fmt::Debug, ops::Deref};

use anyhow::{Context, anyhow};

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
        self.downcast::<Number>().context("not a number")
    }

    pub fn into_text(self) -> Result<Text, anyhow::Error> {
        self.downcast::<Text>().context("not a text")
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

impl DataTrait for Number {
    fn type_def(&self) -> DataTypeDef {
        DataTypeDef {
            title: "Number".to_string(),
            color: 55,
        }
    }
}

impl From<Number> for Data {
    fn from(value: Number) -> Self {
        Data::new(value)
    }
}

#[derive(Debug)]
pub struct Text(String);

impl DataTrait for Text {
    fn type_def(&self) -> DataTypeDef {
        DataTypeDef {
            title: "Text".to_string(),
            color: 64,
        }
    }
}

impl Deref for Text {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for Text {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Text {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<Text> for Data {
    fn from(value: Text) -> Self {
        Data::new(value)
    }
}
