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

// NOTE: example above used in `README`

use std::ops::AddAssign;

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
    use std::ops::AddAssign;
    use rand::{thread_rng, Rng, distributions::{Distribution, Range}};

    fn partial_sum_scanner<T>(s: &mut T, x: &T) -> Option<T>
    where
        T: AddAssign + Copy,
    {
        *s += *x;
        Some(*s)
    }

    #[test]
    fn randoms() {
        let mut rng = thread_rng();
        for len in 0..130usize {
            random_one(&mut rng, len);
        }
    }
    fn random_one<TRng: Rng>(rng: &mut TRng, len: usize) {
        let mut data = vec![0i32; len];
        let range = Range::new_inclusive(-50, 50);
        for x in data.iter_mut() {
            *x = range.sample(rng);
        }
        let psum: Vec<i32> = data.iter().scan(0i32, partial_sum_scanner).collect();
        let mut fenwick = vec![0i32; data.len()];
        {
            for (i, x) in data.iter().enumerate() {
                super::update(&mut fenwick, i, *x);
            }
        }
        for (i, s) in psum.iter().enumerate() {
            assert_eq!(super::prefix_sum(&fenwick, i), *s);
        }
    }
}
