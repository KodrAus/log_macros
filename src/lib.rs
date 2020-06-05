#![feature(min_specialization)]

#[doc(hidden)]
pub mod __private;

#[doc(inline)]
pub use __private::__PrivateLogCapture as Capture;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt;

    #[test]
    fn capture_default() {
        struct SomeType;

        impl fmt::Display for SomeType {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "some type")
            }
        }

        // Capture an arbitrary `Display`
        let _ = SomeType.capture_with_default();

        // Capture a structured number
        assert_eq!(Some(42u64), 42u64.capture_with_default().to_u64());

        // Capture a borrowed string
        let v: &str = &String::from("a string");
        assert_eq!(Some("a string"), v.capture_with_default().to_borrowed_str());
    }
}
