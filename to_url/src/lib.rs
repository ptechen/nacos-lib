#[cfg(feature = "derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate to_url_derive;
#[cfg(feature = "derive")]
#[doc(hidden)]
pub use to_url_derive::*;

pub trait ToUrl {
    fn to_url(&self) -> String;
}