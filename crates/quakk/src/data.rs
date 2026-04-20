use std::{
    any::Any,
    fmt::Debug,
    ops::{Add, Deref, Div, Mul, Sub},
};

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
        write!(f, "{:?}", self.inner)

        // if f.alternate() {
        //     write!(f, "{}({:?})", self.def.title, self.inner)
        // } else {
        // }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Number(f64);

impl DataTrait for Number {
    fn type_def(&self) -> DataTypeDef {
        DataTypeDef {
            title: "Number".to_string(),
            color: 55,
        }
    }
}

impl Deref for Number {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for Number {
    fn default() -> Self {
        Self(f64::default())
    }
}

impl From<Number> for Data {
    fn from(value: Number) -> Self {
        Data::new(value)
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Number(value)
    }
}

impl From<Number> for f64 {
    fn from(value: Number) -> Self {
        value.0
    }
}

impl From<Number> for usize {
    fn from(value: Number) -> Self {
        value.0 as usize
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        self.0.add(rhs.0).into()
    }
}

impl Sub for Number {
    type Output = Number;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0.sub(rhs.0).into()
    }
}

impl Mul for Number {
    type Output = Number;

    fn mul(self, rhs: Self) -> Self::Output {
        self.0.mul(rhs.0).into()
    }
}

impl Div for Number {
    type Output = Number;

    fn div(self, rhs: Self) -> Self::Output {
        self.0.div(rhs.0).into()
    }
}

#[derive(Debug, Clone)]
pub struct Text(String);

impl Default for Text {
    fn default() -> Self {
        Self(String::default())
    }
}

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
