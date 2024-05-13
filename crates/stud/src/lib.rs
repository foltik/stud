pub mod prelude;

pub mod error;
pub mod sync;

#[cfg(feature = "bin")]
pub mod logger;
#[cfg(feature = "bin")]
pub mod rt;

#[cfg(feature = "bin")]
#[doc(hidden)]
pub mod bin;

#[cfg(feature = "bin")]
pub use stud_macros::main;
