macro_rules! declare_less_equal_rule {
    ($ty: ty) => {
        $crate::paste::item! {
            /// A type that holds a value satisfying the `LessEqualRule`
            pub type [<LessEqual $ty:camel>]<const N: $ty> = $crate::Refined<[<LessEqualRule $ty:camel>]<N>>;

            /// Rule where the target value must be less than or equal to `N`
            pub type [<LessEqualRule $ty:camel>]<const N: $ty> = $crate::Or![$crate::rule::[<EqualRule $ty:camel>]<N>, $crate::rule::[<LessRule $ty:camel>]<N>];
        }
    };
    ($t: ty, $($ts: ty),+) => {
        declare_less_equal_rule!($t);
        declare_less_equal_rule!($($ts), +);
    };
}

declare_less_equal_rule!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

#[cfg(test)]
mod test {
    use crate::rule::LessEqualU8;

    #[test]
    fn test_less_equal_than_50u8_ok() {
        let less_equal_result = LessEqualU8::<50>::new(50);
        assert!(less_equal_result.is_ok());
    }

    #[test]
    fn test_less_equal_than_50u8_err() {
        let less_equal_result = LessEqualU8::<50>::new(51);
        assert!(less_equal_result.is_err());
    }
}
