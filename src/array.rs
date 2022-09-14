//! Operations on an 1D Fenwick tree stored in a zero-based slice.
//!
//! # Examples
//!
//! ```
//! use fenwick::array::{update, prefix_sum};
//!
//! let fw = &mut [0i32; 10]; // backing array of Fenwick tree (NOT original array!)
//! assert_eq!(prefix_sum(fw, 0), 0);
//! assert_eq!(prefix_sum(fw, 9), 0);
//! update(fw, 0, 3); // original array: [3, 0, 0, 0, 0, 0, 0, 0, 0, 0]
//! assert_eq!(prefix_sum(fw, 0), 3);
//! assert_eq!(prefix_sum(fw, 9), 3);
//! update(fw, 5, 9); // original array: [3, 0, 0, 0, 0, 9, 0, 0, 0, 0]
//! assert_eq!(prefix_sum(fw, 4), 3);
//! assert_eq!(prefix_sum(fw, 5), 12);
//! assert_eq!(prefix_sum(fw, 6), 12);
//! update(fw, 4, -5); // original array: [3, 0, 0, 0, -5, 9, 0, 0, 0, 0]
//! assert_eq!(prefix_sum(fw, 4), -2);
//! assert_eq!(prefix_sum(fw, 5), 7);
//! update(fw, 0, -2); // original array: [1, 0, 0, 0, -5, 9, 0, 0, 0, 0]
//! assert_eq!(prefix_sum(fw, 4), -4);
//! assert_eq!(prefix_sum(fw, 5), 5);
//! ```
//!

use core::ops::AddAssign;

use crate::index::zero_based::{down as seq_dn, up as seq_up};

/// Updates one element in the Fenwick tree stored in a borrowed slice (zero-based).
///
/// Conceptually performs `a[i] += delta` on the original array `a`.
///
/// # Panics
///
/// Panics if `fenwick[i]` is out of bound.
///
/// # Examples
///
/// See [module-level example](self).
///
pub fn update<T>(fenwick: &mut [T], i: usize, delta: T)
where
    T: AddAssign + Copy + Default
{
    for ii in seq_up(i, fenwick.len()) {
        fenwick[ii] += delta;
    }
}

/// Calculates the prefix sum up to and including `i` in the Fenwick tree stored in a borrowed slice
/// (zero-based).
///
/// Conceptually calculates `a[0] + ... + a[i]` on the original array `a`.
///
/// # Panics
///
/// Panics if `fenwick[i]` is out of bound.
///
/// # Examples
///
/// See [module-level example](self).
///
pub fn prefix_sum<T>(fenwick: &[T], i: usize) -> T
where
    T: AddAssign + Copy + Default
{
    let mut sum = T::default();
    for ii in seq_dn(i) {
        sum += fenwick[ii];
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;

    use itertools::Itertools;
    use rand::prelude::*;

    #[test]
    fn randoms() {
        let mut rng = thread_rng();
        for len in 0..256 {
            random_one(&mut rng, len);
        }
    }

    fn random_one<TRng: Rng>(rng: &mut TRng, len: usize) {
        let dist = rand::distributions::Uniform::new_inclusive(-100, 100);
        let data = rng.sample_iter(dist).take(len).collect_vec();
        let psum = data.iter().scan(0, |s, x| {
            *s += x;
            Some(*s)
        }).collect_vec();

        let mut fenwick = std::vec![0i32; len];

        let mut ops = data.iter().enumerate().collect_vec();
        ops.shuffle(rng);
        for (i, x) in ops {
            update(&mut fenwick, i, *x);
        }

        for (i, s) in psum.iter().enumerate() {
            assert_eq!(prefix_sum(&fenwick, i), *s);
        }
    }
}
