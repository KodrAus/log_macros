use std::fmt;

use log::kv;

pub trait Capture<T: ?Sized> {
    fn capture(&self) -> kv::Value;
}

pub type WithDefault = FromDisplay;

pub enum FromDisplay {}

impl<T> Capture<FromDisplay> for T
where
    T: fmt::Display + 'static,
{
    default fn capture(&self) -> kv::Value {
        kv::Value::from_display(self)
    }
}

pub enum AsDisplay {}

impl<T> Capture<AsDisplay> for T
where
    T: fmt::Display,
{
    default fn capture(&self) -> kv::Value {
        kv::Value::from(self as &dyn fmt::Display)
    }
}

pub enum FromDebug {}

impl<T> Capture<FromDebug> for T
where
    T: fmt::Debug + 'static,
{
    default fn capture(&self) -> kv::Value {
        kv::Value::from_debug(self)
    }
}

pub enum AsDebug {}

impl<T> Capture<AsDebug> for T
where
    T: fmt::Debug,
{
    default fn capture(&self) -> kv::Value {
        kv::Value::from(self as &dyn fmt::Debug)
    }
}

impl<T: ?Sized> Capture<T> for str {
    fn capture(&self) -> kv::Value {
        kv::Value::from(self)
    }
}
