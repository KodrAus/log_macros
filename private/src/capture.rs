use std::fmt;

use log::kv;

pub trait Capture<T: ?Sized> {
    fn capture(&self) -> kv::Value;
}

pub type WithDefault = FromDisplay;

extern {
    pub type FromDisplay;
    pub type AsDisplay;
    pub type FromDebug;
    pub type AsDebug;
}

impl<T> Capture<FromDisplay> for T
where
    T: fmt::Display + 'static,
{
    default fn capture(&self) -> kv::Value {
        kv::Value::from_display(self)
    }
}

impl<T> Capture<AsDisplay> for T
where
    T: fmt::Display,
{
    default fn capture(&self) -> kv::Value {
        kv::Value::from(self as &dyn fmt::Display)
    }
}

impl<T> Capture<FromDebug> for T
where
    T: fmt::Debug + 'static,
{
    default fn capture(&self) -> kv::Value {
        kv::Value::from_debug(self)
    }
}

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

/**
An API to the specialized `Capture` trait for consuming in a macro.

This trait serves a few purposes in the private macro API:

- It supports auto-ref so that something like a `u64` or `&str` can be
captured using the same `x.method()` syntax.
- It uses `Self` bounds on each method, and is unconditionally implemented
so that when a bound isn't satisfied we get a more accurate type error.
- It uses clumsily uglified names that are unlikely to clash in non-hygeinic
contexts.
*/
pub trait __PrivateLogCapture {
    fn __private_log_capture_with_default(&self) -> kv::Value where Self: Capture<WithDefault> {
        Capture::capture(self)
    }

    fn __private_log_capture_from_display(&self) -> kv::Value where Self: Capture<FromDisplay> {
        Capture::capture(self)
    }

    fn __private_log_capture_from_debug(&self) -> kv::Value where Self: Capture<FromDebug> {
        Capture::capture(self)
    }

    fn __private_log_capture_as_display(&self) -> kv::Value where Self: Capture<AsDisplay> {
        Capture::capture(self)
    }

    fn __private_log_capture_as_debug(&self) -> kv::Value where Self: Capture<AsDebug> {
        Capture::capture(self)
    }
}

impl<T: ?Sized> __PrivateLogCapture for T { }
