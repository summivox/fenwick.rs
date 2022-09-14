A [**Fenwick tree**][wiki] or **binary indexed tree**/**bit indexed tree** is a data structure
that supports the following two operations efficiently over an array of numbers `a[0..n]`:

- Calculate a prefix sum: `a[0] + a[1] + ... + a[i]`
- Update one element: `a[i] += delta`

With a naïve implementation, only one of the operations can be made to have constant time
complexity while the other one has to be linear. With Fenwick tree, both take only `O(log(N))`.

This crate is `no_std` and has no (non-dev) dependencies.

[wiki]: https://en.wikipedia.org/wiki/Fenwick_tree

# Examples

Use the `array` module for operations on a 1D Fenwick tree:

```rust
use fenwick::array::{update, prefix_sum};

let fw = &mut [0i32; 10]; // backing array of Fenwick tree (NOT original array!)
assert_eq!(prefix_sum(fw, 0), 0);
assert_eq!(prefix_sum(fw, 9), 0);
update(fw, 0, 3); // original array: [3, 0, 0, 0, 0, 0, 0, 0, 0, 0]
assert_eq!(prefix_sum(fw, 0), 3);
assert_eq!(prefix_sum(fw, 9), 3);
update(fw, 5, 9); // original array: [3, 0, 0, 0, 0, 9, 0, 0, 0, 0]
assert_eq!(prefix_sum(fw, 4), 3);
assert_eq!(prefix_sum(fw, 5), 12);
assert_eq!(prefix_sum(fw, 6), 12);
update(fw, 4, -5); // original array: [3, 0, 0, 0, -5, 9, 0, 0, 0, 0]
assert_eq!(prefix_sum(fw, 4), -2);
assert_eq!(prefix_sum(fw, 5), 7);
update(fw, 0, -2); // original array: [1, 0, 0, 0, -5, 9, 0, 0, 0, 0]
assert_eq!(prefix_sum(fw, 4), -4);
assert_eq!(prefix_sum(fw, 5), 5);
```

Use the `index` module to implement multidimensional Fenwick trees:

```rust
use fenwick::index::zero_based::{down, up};
const MAX: usize = 1000;

fn update(i: usize, j: usize, k: usize, delta: i32) {
    for ii in up(i, MAX) {
        for jj in up(j, MAX) {
            for kk in up(k, MAX) {
                /* increment 3D array at [ii, jj, kk] by delta */
            }
        }
    }
}

fn prefix_sum(i: usize, j: usize, k: usize) -> i32 {
    let mut sum = 0i32;
    for ii in down(i) {
        for jj in down(j) {
            for kk in down(k) {
                /* increment sum by 3D array at [ii, jj, kk] */
            }
        }
    }
    sum
}
```

# References

* [Original Paper](http://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.14.8917)
* [Tutorial on Topcoder](https://www.topcoder.com/community/data-science/data-science-tutorials/binary-indexed-trees/)

