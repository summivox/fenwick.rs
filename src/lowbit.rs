use num_traits;

pub fn lowbit<T>(x: T) -> T
where
    T: num_traits::PrimInt + num_traits::Unsigned,
{
    x & (!x + T::one())
}
