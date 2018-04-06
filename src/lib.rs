//! A [**Fenwick tree**][wiki] or **binary indexed tree**/**bit indexed tree** is a data structure
//! that supports the following two operations efficiently over an array of numbers `a[0..n]`:
//!
//! - Calculate a prefix sum: `a[0] + a[1] + ... + a[i]`
//! - Update one element: `a[i] += delta`
//!
//! With a naïve implementation, only one of the operations can be made to have constant time
//! complexity while the other one has to be linear. With Fenwick tree, both take only `O(log(N))`.
//!
//! [wiki]: https://en.wikipedia.org/wiki/Fenwick_tree
//!
//! # Get Started
//!
//! See example in module [array](array) which implements a generic 1D Fenwick tree.
//!
//! Multi-dimensional Fenwick trees can be easily implemented using the building blocks in module
//! [index](index) ond a multi-dimensional array (again of the same size/shape as the original).
//!
//! # How it works
//!
//! A Fenwick tree is implemented as an implicit data structure using an array `f` of the same
//! length as the original array `a`. Each node in the tree is represented as an element in `f`,
//! which stores the sum of a certain subset of elements in `a`. The operations can then be
//! implemented as follows:
//!
//! - Prefix sum is calculated by summing a subset of nodes in the implicit tree.
//! - Updating a element involves updating all nodes in the implicit tree that covers it.
//!
//! The algorithms for computing "which nodes adds up to a prefix" and "which nodes cover this
//! element" are based on the binary representation of input index and are exposed in module
//! [index](index).
//!
//! # References
//!
//! * [Original Paper](http://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.14.8917)
//! * [Tutorial on Topcoder](https://www.topcoder.com/community/data-science/data-science-tutorials/binary-indexed-trees/)
//!

#[cfg(test)]
extern crate rand;

pub mod index;

pub mod array;
// pub mod bit2d;
