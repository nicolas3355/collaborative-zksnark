#![no_std]

#![deny(
    warnings,
    unused,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
#![forbid(unsafe_code)]

//! This library implements a twisted Edwards curve whose base field is the scalar field of the curve CP6.
//! This allows defining cryptographic primitives that use elliptic curves over the scalar field of the latter curve.
//! This curve was generated as part of the paper [\[BCGMMW20, “Zexe”\]](https://eprint.iacr.org/2018/962).
//!
//! Curve information:
//! * Base field: q = 258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458177
//! * Scalar field: r = 32333053251621136751331591711861691692049189094364332567435817881934511297123972799646723302813083835942624121493
//! * Valuation(q - 1, 2) = 46
//! * Valuation(r - 1, 2) = 2
//! * Curve equation: ax^2 + y^2 =1 + dx^2y^2, where
//!    * a = -1
//!    * d = 79743

#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "r1cs")]
pub mod constraints;
mod curves;
mod fields;

pub use curves::*;
pub use fields::*;
