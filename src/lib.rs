/// Iterator extension trait for summing numbers with overflow checking.
pub trait CheckedSum<T> {
    /// Adds an iterator of numbers checking for overflow, returns `None` if overflow occurred.
    fn checked_sum(self) -> Option<T>;
}

impl<T, I> CheckedSum<T> for I
where
    T: CheckedAdd + Default,
    I: Iterator<Item=T>
{
    fn checked_sum(mut self) -> Option<T> {
        self.try_fold(T::default(), |acc, value| {
            acc.checked_add(&value)
        })
    }
}

/// Numeric type supporting overflow-checked addition.
pub trait CheckedAdd: Sized {
    /// Adds two numbers checking for overflow, returns `None` if overflow occurred.
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