use cargo_snippet::snippet;
use std::usize;

#[snippet("_wrapping_add_signed_impl_fn")]
macro_rules! wrapping_add_signed_impl_fn {
    ($add_t:ty, $fn_name:ident) => {
        fn $fn_name(&self, x: $add_t) -> Self {
            if x >= 0 {
                self.wrapping_add(x as Self)
            } else {
                self.wrapping_sub((-x) as Self)
            }
        }
    }
}

#[snippet("_wrapping_add_signed_impl", include = "_wrapping_add_signed_impl_fn")]
macro_rules! wrapping_add_signed_impl {
    ($self_t:ty) => {
        impl WrappingAddSigned for $self_t {
            wrapping_add_signed_impl_fn!(i32, wrapping_add_i32);
            wrapping_add_signed_impl_fn!(i64, wrapping_add_i64);
        }
    }
}

#[snippet("_wrapping_add_signed_macro", include = "_wrapping_add_signed_impl")]
macro_rules! wrapping_add_signed {
    () => {
        trait WrappingAddSigned {
            fn wrapping_add_i32(&self, x: i32) -> Self;
            fn wrapping_add_i64(&self, x: i64) -> Self;
        }
        wrapping_add_signed_impl!(usize);
        wrapping_add_signed_impl!(u32);
        wrapping_add_signed_impl!(u64);
    }
}

#[snippet("wrapping_add_signed", include = "_wrapping_add_signed_macro")]
wrapping_add_signed!();

#[test]
fn test_wrapping_add_i32() {
    assert_eq!((0 as usize).wrapping_add_i32(-1), usize::MAX);
    assert_eq!(usize::MAX.wrapping_add_i32(1), 0);
}

#[test]
fn test_wrapping_add_i64() {
    assert_eq!((0 as usize).wrapping_add_i64(-1), usize::MAX);
    assert_eq!(usize::MAX.wrapping_add_i64(1 as i64), 0);
}
