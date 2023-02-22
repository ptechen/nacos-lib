#[cfg(feature = "to_url_derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate to_url_derive;
#[cfg(feature = "to_url_derive")]
#[doc(hidden)]
pub use to_url_derive::*;

pub trait ToUrl {
    fn to_url(&self) -> String;
}