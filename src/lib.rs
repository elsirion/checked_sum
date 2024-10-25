//! Utility crate for summing up iterators in a safe way. The [`CheckedSum`]
//! trait is implemented for any iterator of items implementing [`CheckedAdd`],
//! which in turn is implemented for all integer primitives but can also be
//! implemented for other types like newtypes wrapping integers.
//!
//! ```
//! use checked_sum::CheckedSum;
//!
//! // If the sum fits into the type, it is returned
//! let numbers = vec![1u8, 2, 3, 4, 5];
//! assert_eq!(numbers.into_iter().checked_sum(), Some(15),);
//!
//! // If the sum overflows, `None` is returned
//! let numbers = vec![255u8, 1];
//! assert_eq!(numbers.into_iter().checked_sum(), None,);
//! ```

/// Iterator extension trait for summing numbers with overflow checking.
pub trait CheckedSum<T> {
    /// Adds an iterator of numbers checking for overflow, returns `None` if
    /// overflow occurred.
    fn checked_sum(self) -> Option<T>;
}

impl<T, I> CheckedSum<T> for I
where
    T: CheckedAdd + Default,
    I: Iterator<Item = T>,
{
    fn checked_sum(mut self) -> Option<T> {
        self.try_fold(T::default(), |acc, value| acc.checked_add(&value))
    }
}

/// Numeric type supporting overflow-checked addition.
///
/// The trait is implemented for all primitive types and can be implemented
/// manually for custom newtypes. Please note that [`CheckedSum`] also requires
/// the item type to implement `Default`.
///
/// ```
/// use checked_sum::{CheckedAdd, CheckedSum};
///
/// #[derive(Debug, PartialEq, Default)]
/// struct MyInt(u32);
///
/// impl CheckedAdd for MyInt {
///     fn checked_add(&self, other: &Self) -> Option<Self> {
///         self.0.checked_add(other.0).map(MyInt)
///     }
/// }
///
/// let numbers = vec![MyInt(1), MyInt(2), MyInt(3)];
/// assert_eq!(numbers.into_iter().checked_sum(), Some(MyInt(6)));
/// ```
pub trait CheckedAdd: Sized {
    /// Adds two numbers checking for overflow, returns `None` if overflow
    /// occurred.
    fn checked_add(&self, other: &Self) -> Option<Self>;
}

macro_rules! impl_checked_add {
    ($($t:ty),*) => {
        $(
            impl CheckedAdd for $t {
                fn checked_add(&self, other: &Self) -> Option<Self> {
                    <$t>::checked_add(*self, *other)
                }
            }
        )*
    };
}

impl_checked_add!(u8, u16, u32, u64, u128, usize);
impl_checked_add!(i8, i16, i32, i64, i128, isize);

#[cfg(test)]
mod tests {
    use crate::CheckedSum;

    #[test]
    fn test_checked_sum_some() {
        let values: Vec<u64> = vec![];
        let maybe_sum = values.into_iter().checked_sum();
        assert_eq!(maybe_sum, Some(0));

        let values = vec![1u8, 2, 3, 4, 5];
        let maybe_sum = values.into_iter().checked_sum();
        assert_eq!(maybe_sum, Some(15));
    }

    #[test]
    fn test_checked_sum_none() {
        {
            let values = vec![255u8, 1];
            let maybe_sum = values.into_iter().checked_sum();
            assert_eq!(maybe_sum, None);
        }
        {
            let values = vec![1u8; 256];
            let maybe_sum = values.into_iter().checked_sum();
            assert_eq!(maybe_sum, None);
        }
        {
            let values = vec![u8::MAX, u8::MAX, u8::MAX];
            let maybe_sum = values.into_iter().checked_sum();
            assert_eq!(maybe_sum, None);
        }
    }
}
