//! Extra iterator adaptors which compute different distances between collections.
//!
//! To extend [`Iterator`] with methods in this crate, import the [`Distance`] trait:
//!
//! ```
//! use aabel_rs::distances::Distance;
//! ```
//!
//! Now, new methods like [`euclid`](Distance::euclid)
//! are available on all iterators:
//!
//! ```
//! use aabel_rs::distances::Distance;
//!
//! let it = [3., 4.].into_iter().euclid([0., 0.]);
//! assert_eq!(5., it)
//! ```
//!
//! This crate is inspired and works with [`Itertools`] crate.
//!
//! ## Rust Version
//!
//! This version of itertools requires Rust 1.32 or later.

pub(crate) mod cosine;
mod distance;
pub(crate) mod euclid;
pub(crate) mod hamming;
pub(crate) mod jaccard;
pub(crate) mod manhattan;

pub use cosine::cosine;
pub use distance::*;
pub use euclid::euclid;
pub use hamming::*;
pub use jaccard::jaccard;
pub use manhattan::manhattan;
