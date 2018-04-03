use std::ops::AddAssign;

use index_iter::zero_based::{down as seq_dn, up as seq_up};

pub fn update<T>(fenwick: &mut [T], i: usize, delta: T)
where
    T: AddAssign + Copy + Default,
{
    for ii in seq_up(i, fenwick.len()) {
        fenwick[ii] += delta;
    }
}

pub fn partial_sum<T>(fenwick: &[T], i: usize) -> T
where
    T: AddAssign + Copy + Default,
{
    let mut ret = T::default();
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
        for (i, x) in data.iter().enumerate() {
            super::update(&mut fenwick[..], i, *x);
        }
        for (i, s) in psum.iter().enumerate() {
            assert_eq!(super::partial_sum(&fenwick[..], i), *s);
        }
    }
}
