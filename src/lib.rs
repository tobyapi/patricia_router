mod result;
mod tree;

#[doc(hidden)]
mod node;

#[doc(hidden)]
mod utils;

pub use crate::result::Result;
pub use crate::tree::Tree;

pub type Router<T> = crate::tree::Tree<T>;
