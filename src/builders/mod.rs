// Copyright (c) 2019, Bayu Aldi Yansyah <bayualdiyansyah@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Vector builders
//!
//! # Overview
//!
//! There are 4 general mechanisms for creating vectors:
//!
//! 1. Conversion from other Rust primitive types: [`array`] and [`slice`].
//! 2. Using Gulali's vector builder routines (e.g., [`one_dim()`],
//!    [`range()`], etc.)
//! 3. Reading vectors from disk, either from standard or
//!    custom formats *(Not available yet)*
//! 4. Creating vectors from raw bytes through the use of
//!    strings or buffers *(Not available yet)*
//!
//! This section will not cover means of replicating, joining, or otherwise
//! expanding or mutating existing vectors. Those are covered in their
//! own sections.
//!
//! [`array`]: https://doc.rust-lang.org/std/primitive.array.html
//! [`slice`]: https://doc.rust-lang.org/std/slice/index.html
//!
//! # Converting Array and Slice to Vector
//! In general, numerical data arranged in an array-like structure
//! in Rust can be converted to vector through the use of the [`to_vec()`]
//! function. The most obvious examples are [`array`] and [`slice`].
//! See the documentation for [`to_vec()`] for details for its use.
//!
//! Examples:
//!
//! ```
//! let arr = [4, 4, 1, 6];
//! assert_eq!(arr.to_vec(), vec![4, 4, 1, 6]);
//!
//! let slice = &[2, 3, 1, 0];
//! assert_eq!(slice.to_vec(), vec![2, 3, 1, 0])
//! ```
//!
//! [`to_vec()`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.to_vec
//!
//! # Vector Builders
//! Gulali has built-in functions for creating vectors from scratch.
//!
//! [`one_dim()`] will create a one-dimensional vector with specified
//! shape and values. For example:
//!
//! ```
//! # use gulali::prelude::*;
//! // Generate a one-dimensional vector with shape [5]
//! // filled with zeros; f64 can be changed into any
//! // numeric data types.
//! let bias: Vec<f64> = Vec::one_dim()
//!     .with_shape([5])
//!     .zeros()
//!     .generate();
//!
//! assert_eq!(bias, [0.0, 0.0, 0.0, 0.0, 0.0]);
//! ```
//!
//! [`two_dim()`] will create a two-dimensional vector with specified
//! shape and values. For example:
//!
//! ```
//! # use gulali::prelude::*;
//! // Generate a two-dimensional vector with shape [2, 2]
//! // filled with ones; f64 can be changed into any
//! // numeric data types.
//! let matrix: Vec<Vec<f64>> = Vec::two_dim()
//!     .with_shape([2, 2])
//!     .ones()
//!     .generate();
//!
//! assert_eq!(matrix, [[1.0, 1.0], [1.0, 1.0]]);
//! ```
//!
//! [`three_dim()`] will create a three-dimensional vector with specified
//! shape and values. For example:
//!
//! ```
//! # use gulali::prelude::*;
//! // Generate a three-dimensional vector with shape [1, 1, 2]
//! // filled with 5.0; f64 can be changed into any
//! // numeric data types.
//! let test: Vec<Vec<Vec<f64>>> = Vec::three_dim()
//!     .with_shape([1, 1, 2])
//!     .full_of(5.0)
//!     .generate();
//!
//! assert_eq!(test, [[[5.0, 5.0]]]);
//! ```
//!
//! [`four_dim()`] will create a four-dimensional vector with specified
//! shape and values. For example:
//!
//! ```
//! # use gulali::prelude::*;
//! // Generate a four-dimensional vector with shape [1, 1, 1, 2]
//! // filled with ones; f64 can be changed into any
//! // numeric data types.
//! let test: Vec<Vec<Vec<Vec<f64>>>> = Vec::four_dim()
//!     .with_shape([1, 1, 1, 2])
//!     .ones()
//!     .generate();
//!
//! assert_eq!(test, [[[[1.0, 1.0]]]]);
//! ```
//!
//! [`one_dim()`]: trait.OneDimensional.html#tymethod.one_dim
//! [`two_dim()`]: trait.TwoDimensional.html#tymethod.two_dim
//! [`three_dim()`]: trait.ThreeDimensional.html#tymethod.three_dim
//! [`four_dim()`]: trait.FourDimensional.html#tymethod.four_dim
//!
//! [`range()`] will create vectors with regularly incrementing values.
//! For example:
//! ```
//! # use gulali::prelude::*;
//! let range1: Vec<i32> = Vec::range().stop_at(5).generate();
//! assert_eq!(range1, [0, 1, 2, 3, 4]);
//!
//! let range2: Vec<f64> = Vec::range()
//!     .start_at(1.0)
//!     .stop_at(3.0)
//!     .step_by(0.5)
//!     .generate();
//! assert_eq!(range2, [1.0, 1.5, 2.0, 2.5]);
//! ```
//!
//! [`range()`]: trait.Range.html#tymethod.range
//!
//! [`linspace()`] will create vectors with a specified number of elements,
//! and spaced equally between the specified beginning and end values.
//! For example:
//! ```
//! # use gulali::prelude::*;
//! // Generate linearly spaced vector within interval [2.0, 5.0]
//! let lin: Vec<f32> = Vec::linspace()
//!     .start_at(2.0)
//!     .stop_at(5.0)
//!     .with_size(10)
//!     .generate();
//!
//! assert_eq!(
//!     lin,
//!     [
//!         2.0, 2.33333333, 2.6666665, 2.9999998, 3.333333,
//!         3.6666663, 3.9999995, 4.333333, 4.6666665, 5.0
//!     ]
//! );
//! ```
//! The advantage of this vector builder function is that one
//! can guarantee the number of elements and the starting and
//! end point, which [`range()`] generally will not do for
//! arbitrary start, stop, and step values.
//!
//! [`linspace()`]: trait.Linspace.html#tymethod.linspace
//!
// TODO: Continue here https://docs.scipy.org/doc/numpy-1.16.1/user/basics.creation.html

mod four_dimensional;
mod linspace;
mod one_dimensional;
mod range;
mod three_dimensional;
mod two_dimensional;

pub use four_dimensional::*;
pub use linspace::*;
pub use one_dimensional::*;
pub use range::*;
pub use three_dimensional::*;
pub use two_dimensional::*;
