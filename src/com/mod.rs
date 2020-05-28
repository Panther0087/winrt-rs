//! All COM related functionality

mod interface;
mod ptr;
mod raw_ptr;
mod ref_count;
pub(crate) mod unknown;

pub use interface::ComInterface;
pub use ptr::ComPtr;
pub use raw_ptr::{NonNullRawComPtr, RawComPtr};
pub use ref_count::RefCount;
pub use unknown::IUnknown;
