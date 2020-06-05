mod capture;

use log::kv;
use capture::*;

/**
Capture a type as a `Value` using one of a number of strategies.
*/
pub trait __PrivateLogCapture {
    /**
    Capture a type with the default strategy.

    This will attempt to retain the structure of the input type.
    */
    fn capture_with_default(&self) -> kv::Value where Self: Capture<WithDefault> {
        Capture::capture(self)
    }

    /**
    Capture a type that implements `Display`.

    This will attempt to retain the structure of the input type.
    */
    fn capture_from_display(&self) -> kv::Value where Self: Capture<FromDisplay> {
        Capture::capture(self)
    }

    /**
    Capture a type that implements `Debug`.

    This will attempt to retain the structure of the input type.
    */
    fn capture_from_debug(&self) -> kv::Value where Self: Capture<FromDebug> {
        Capture::capture(self)
    }

    /**
    Capture a type that implements `Display`.
    */
    fn capture_as_display(&self) -> kv::Value where Self: Capture<AsDisplay> {
        Capture::capture(self)
    }

    /**
    Capture a type that implements `Debug`.
    */
    fn capture_as_debug(&self) -> kv::Value where Self: Capture<AsDebug> {
        Capture::capture(self)
    }

    #[doc(hidden)]
    fn __private_log_capture_with_default(&self) -> kv::Value where Self: Capture<WithDefault> {
        Capture::capture(self)
    }

    #[doc(hidden)]
    fn __private_log_capture_from_display(&self) -> kv::Value where Self: Capture<FromDisplay> {
        Capture::capture(self)
    }

    #[doc(hidden)]
    fn __private_log_capture_from_debug(&self) -> kv::Value where Self: Capture<FromDebug> {
        Capture::capture(self)
    }

    #[doc(hidden)]
    fn __private_log_capture_as_display(&self) -> kv::Value where Self: Capture<AsDisplay> {
        Capture::capture(self)
    }

    #[doc(hidden)]
    fn __private_log_capture_as_debug(&self) -> kv::Value where Self: Capture<AsDebug> {
        Capture::capture(self)
    }
}

impl<T: ?Sized> __PrivateLogCapture for T { }
