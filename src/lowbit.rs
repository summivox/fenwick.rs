use num_traits::{PrimInt, Unsigned, WrappingAdd};

/// Returns the least-significant set bit in unsigned integer `x` (i.e. keeping only the rightmost
/// `1` in the binary representation of `x`). If `x` is zero, returns zero.
///
/// This is equivalent to `1 << x.trailing_zeros()` for non-zero `x`.
///
/// # Examples
///
/// ```rust
/// use fenwick::lowbit;
/// assert_eq!(lowbit(0u64), 0u64);
/// assert_eq!(lowbit(0x80000000u32), 0x80000000u32);
/// assert_eq!(lowbit(1u8), 1u8);
/// assert_eq!(lowbit(0b1010111010000u32), 0b10000u32);
/// ```
///
#[inline]
pub fn lowbit<T>(x: T) -> T
where
    T: PrimInt + Unsigned + WrappingAdd,
{
    x & ((!x).wrapping_add(&T::one()))
}

#[cfg(test)]
mod tests {
    use super::lowbit;

    #[test]
    fn lowbit_definition_exhaustive() {
        for zero_based in 0..(u16::max_value() as u32 + 1) {
            let one_based = zero_based + 1;
            assert_eq!(lowbit(one_based), 1u32 << one_based.trailing_zeros());
        }
    }
}
