//! Functions implementing a 1D Fenwick tree stored in a borrowed array/slice (zero-based).
//!
//! See
//!
//! # Examples
//!
//! ```rust
//! use fenwick::array::{update, prefix_sum};
//!
//! let fw = &mut [0i32; 10];
//! assert_eq!(prefix_sum(fw, 0), 0);
//! assert_eq!(prefix_sum(fw, 9), 0);
//! update(fw, 0, 3);
//! assert_eq!(prefix_sum(fw, 0), 3);
//! assert_eq!(prefix_sum(fw, 9), 3);
//! update(fw, 5, 9);
//! assert_eq!(prefix_sum(fw, 4), 3);
//! assert_eq!(prefix_sum(fw, 5), 12);
//! assert_eq!(prefix_sum(fw, 6), 12);
//! update(fw, 4, -5);
//! assert_eq!(prefix_sum(fw, 4), -2);
//! assert_eq!(prefix_sum(fw, 5), 7);
//! ```
//!

use std::ops::{AddAssign, Index};

use index::zero_based::{down as seq_dn, up as seq_up};

/// Updates one element in the Fenwick tree stored in a borrowed slice (zero-based).
///
/// Conceptually performs `a[i] += delta` on the original array `a`.
///
/// # Examples
///
/// See [module-level example](self).
///
/// # Panics
///
/// Out-of-bound access if input index `i` is out of bound.
///
pub fn update<TValue, TArray>(fenwick: &mut TArray, i: usize, delta: TValue)
where
    TValue: AddAssign + Copy + Default,
    TArray: AsMut<[TValue]> + ?Sized
{
    let a = fenwick.as_mut();
    for ii in seq_up(i, a.len()) {
        a[ii] += delta;
    }
}

/// Calculates the prefix sum up to and including `i` in the Fenwick tree stored in a borrowed slice
/// (zero-based).
///
/// Conceptually calculates `a[0] + ... + a[i]` on the original array `a`.
///
/// # Examples
///
/// See [module-level example](self).
///
/// # Panics
///
/// Out-of-bound access if input index `i` is out of bound.
///
pub fn prefix_sum<TValue, TArray>(fenwick: &TArray, i: usize) -> TValue
where
    TValue: AddAssign + Copy + Default,
    TArray: Index<usize, Output=TValue> + ?Sized,
{
    let mut ret = TValue::default();
    for ii in seq_dn(i) {
        ret += fenwick[ii];
    }
    ret
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
