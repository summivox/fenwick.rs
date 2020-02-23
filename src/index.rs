//! Iterators yielding index sequences for traversing Fenwick trees.
//!
//! - `down(i)` yields indices of nodes which can be summed together to obtain prefix sum up to `i`.
//! - `up(i, limit)` yields indices of nodes that should be updated when updating element `i`.
//!
//! Traditionally Fenwick trees are implemented using one-based arrays for both tree and value
//! arrays. While this simplifies the definition of index sequences, an offset is required for it
//! to work in languages (such as rust) that has zero-based array indexing. Alternatively, the
//! algorithms can be simplified to directly work with zero-based indices.
//!
//! This module implements both [zero-based](zero_based) and [one-based](one_based) index sequences.
//!
//! # Examples
//!
//! An ad-hoc 3D Fenwick tree over a 3D array may be implemented as follows:
//!
//! ```
//! use fenwick::index::zero_based::{down, up};
//! #
//! # // dummy zero-based 3D array interface
//! # const MAX: usize = 100;
//! # fn array3d_get(i: usize, j: usize, k: usize) -> i32 { 1 /* dummy */ }
//! # fn array3d_add_assign(i: usize, j: usize, k: usize, delta: i32) { /* dummy */ }
//!
//! fn update(i: usize, j: usize, k: usize, delta: i32) {
//!     for ii in up(i, MAX) {
//!         for jj in up(j, MAX) {
//!             for kk in up(k, MAX) {
//!                 array3d_add_assign(ii, jj, kk, delta);
//!             }
//!         }
//!     }
//! }
//!
//! fn prefix_sum(i: usize, j: usize, k: usize) -> i32 {
//!     let mut sum = 0i32;
//!     for ii in down(i) {
//!         for jj in down(j) {
//!             for kk in down(k) {
//!                 sum += array3d_get(ii, jj, kk);
//!             }
//!         }
//!     }
//!     sum
//! }
//! ```
//!

pub mod one_based {
    /// Creates an iterator that yields indices of nodes that make up the prefix sum up to `init`
    /// in a one-based Fenwick tree.
    ///
    /// Each output index `i` satisfies `1 <= i && i <= init` .
    ///
    /// # Panics
    ///
    /// Panics when `init` is zero (invalid for one-based indexing).
    ///
    /// # Examples
    ///
    /// See [module-level example](super).
    ///
    pub fn down(init: usize) -> impl Iterator<Item = usize> {
        assert!(1 <= init);
        std::iter::successors(Some(init), move |&i| {
            let next = next_down(i);
            if next > 0 {
                Some(next)
            } else {
                None
            }
        })
    }

    #[inline]
    fn next_down(i: usize) -> usize {
        i & i.wrapping_sub(1)
    }

    /// Creates an iterator that yields indices of nodes that need to be updated when updating an
    /// element in the original array in a one-based Fenwick tree with `limit_inclusive` elements.
    ///
    /// Each output index `i` satisfies `init <= i && i <= limit_inclusive` .
    ///
    /// # Panics
    ///
    /// Panics if the following assumption on input indices does not hold:
    /// `1 <= init && init <= limit_inclusive && limit_inclusive <= (usize::max_value() >> 1)` .
    ///
    /// Note that the upper bound on `limit_inclusive` is irrelevant in practice since it is the
    /// length of the backing array of the Fenwick tree and therefore limited by memory.
    ///
    /// # Examples
    ///
    /// See [module-level example](super).
    ///
    pub fn up(init: usize, limit_inclusive: usize) -> impl Iterator<Item = usize> {
        assert!(1 <= init);
        assert!(init <= limit_inclusive);
        assert!(limit_inclusive <= (usize::max_value() >> 1));
        std::iter::successors(Some(init), move |&i| {
            let next = next_up(i);
            if next <= limit_inclusive {
                Some(next)
            } else {
                None
            }
        })
    }

    #[inline]
    fn next_up(i: usize) -> usize {
        (i | i.wrapping_sub(1)) + 1
    }
}

pub mod zero_based {
    /// Creates an iterator that yields indices of nodes that make up the prefix sum up to `init`
    /// in a zero-based Fenwick tree.
    ///
    /// Each output index `i` satisfies `i <= init` .
    ///
    /// # Panics
    ///
    /// Panics when `i == usize::max_value()` .
    ///
    /// Note that this is irrelevant in practice since `i` is bound by the length of the backing
    /// array of the Fenwick tree and therefore limited by memory.
    ///
    /// # Examples
    ///
    /// See [module-level example](super).
    ///
    pub fn down(init: usize) -> impl Iterator<Item = usize> {
        assert_ne!(init, !0);
        std::iter::successors(Some(init), move |&i| {
            let next = next_down(i);
            if next != !0 { Some(next) } else { None }
        })
    }

    #[inline]
    fn next_down(i: usize) -> usize {
        (i & i.wrapping_add(1)).wrapping_sub(1)
    }

    /// Creates an iterator that yields indices of nodes that need to be updated when updating an
    /// element in the original array in a zero-based Fenwick tree with `limit_exclusive` elements.
    ///
    /// Each output index `i` satisfies `init <= i && i < limit_exclusive` .
    ///
    /// # Panics
    ///
    /// Panics if the following assumption on input indices does not hold:
    /// `init < limit_exclusive` .
    ///
    /// # Examples
    ///
    /// See [module-level example](super).
    ///
    pub fn up(init: usize, limit_exclusive: usize) -> impl Iterator<Item = usize> {
        assert!(init < limit_exclusive);
        std::iter::successors(Some(init), move |&i| {
            let next = next_up(i);
            if next < limit_exclusive { Some(next) } else { None }
        })
    }

    #[inline]
    fn next_up(i: usize) -> usize {
        i | i.wrapping_add(1)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn down_example() {
        let init_one = 0b1101110101011010000usize;
        let ans_one = vec![
            0b1101110101011010000usize,
            0b1101110101011000000usize,
            0b1101110101010000000usize,
            0b1101110101000000000usize,
            0b1101110100000000000usize,
            0b1101110000000000000usize,
            0b1101100000000000000usize,
            0b1101000000000000000usize,
            0b1100000000000000000usize,
            0b1000000000000000000usize,
        ];

        assert_eq!(
            super::one_based::down(init_one).collect::<Vec<usize>>(),
            ans_one
        );
        assert_eq!(
            super::zero_based::down(init_one - 1)
                .map(|x| x + 1)
                .collect::<Vec<usize>>(),
            ans_one
        );
    }

    #[test]
    fn up_example() {
        let init_one = 0b1101110101011010000usize;
        let limit = 0b100000000000000000000usize;
        let ans_one = vec![
            0b001101110101011010000usize,
            0b001101110101011100000usize,
            0b001101110101100000000usize,
            0b001101110110000000000usize,
            0b001101111000000000000usize,
            0b001110000000000000000usize,
            0b010000000000000000000usize,
            0b100000000000000000000usize,
        ];
        assert_eq!(
            super::one_based::up(init_one, limit).collect::<Vec<usize>>(),
            ans_one
        );
        assert_eq!(
            super::zero_based::up(init_one - 1, limit)
                .map(|x| x + 1)
                .collect::<Vec<usize>>(),
            ans_one
        );
    }
}
