use cargo_snippet::snippet;

#[snippet("_gf_ops")]
trait GFOps<T> {
    fn add(self, other: Self) -> Self;
    fn add_assign(&mut self, other: Self);
    fn sub(self, other: Self) -> Self;
    fn sub_assign(&mut self, other: Self);
    fn mul(self, other: Self) -> Self;
    fn mul_assign(&mut self, other: Self);
    fn inv(self) -> Self;
    fn div(self, other: Self) -> Self;
    fn div_assign(&mut self, other: Self);
}

#[snippet("_gf_trait", include = "_gf_ops")]
trait GFn<T>: GFOps<T> + From<T> + Into<T> {
    fn modulo() -> T;
}

#[snippet("_gf_impl", include = "_gf_trait")]
impl<T, GF: GFn<T> + From<T> + Into<T> + Default> GFOps<T> for GF
where
    T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + PartialEq
        + PartialOrd
        + From<i32>
        + Copy
        + std::ops::Rem<Output = T>,
{
    fn add(mut self, other: Self) -> Self {
        self.add_assign(other);
        self
    }
    fn add_assign(&mut self, other: Self) {
        *self = Self::from(
            (Self::into(std::mem::take(self)) + Self::into(other)) % Self::modulo(),
        );
    }
    fn sub(mut self, other: Self) -> Self {
        self.sub_assign(other);
        self
    }
    fn sub_assign(&mut self, other: Self) {
        *self = Self::from(
            (Self::into(std::mem::take(self)) + Self::modulo() - Self::into(other))
                % Self::modulo(),
        );
    }
    fn mul(mut self, other: Self) -> Self {
        self.mul_assign(other);
        self
    }
    fn mul_assign(&mut self, other: Self) {
        *self = Self::from(
            (Self::into(std::mem::take(self)) * Self::into(other)) % Self::modulo(),
        );
    }
    fn inv(self) -> Self {
        let mut a = Self::into(self);
        let mut b = Self::modulo();
        let mut u = 1.into();
        let mut v = 0.into();
        while b != 0.into() {
            let t = a / b;
            a = a - t * b;
            std::mem::swap(&mut a, &mut b);
            u = u - t * v;
            std::mem::swap(&mut u, &mut v);
        }
        u = u % Self::modulo();
        if u < 0.into() {
            u = u + Self::modulo();
        }
        Self::from(u)
    }
    fn div(mut self, other: Self) -> Self {
        self.div_assign(other);
        self
    }
    fn div_assign(&mut self, other: Self) {
        self.mul_assign(other.inv())
    }
}

#[snippet("gf_macro", include = "_gf_impl")]
macro_rules! GF {
    ($t: ident, $e: expr, $b: ty) => {
        #[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
        struct $t {
            value: $b,
        }
        impl GFn<$b> for $t {
            fn modulo() -> $b {
                $e
            }
        }
        impl std::ops::Add for $t {
            type Output = Self;
            fn add(self, other: Self) -> Self {
                GFOps::<$b>::add(self, other)
            }
        }
        impl std::ops::AddAssign for $t {
            fn add_assign(&mut self, other: Self) {
                GFOps::<$b>::add_assign(self, other)
            }
        }
        impl std::ops::Sub for $t {
            type Output = Self;
            fn sub(self, other: Self) -> Self {
                GFOps::<$b>::sub(self, other)
            }
        }
        impl std::ops::SubAssign for $t {
            fn sub_assign(&mut self, other: Self) {
                GFOps::<$b>::sub_assign(self, other)
            }
        }
        impl std::ops::Mul for $t {
            type Output = Self;
            fn mul(self, other: Self) -> Self {
                GFOps::<$b>::mul(self, other)
            }
        }
        impl std::ops::MulAssign for $t {
            fn mul_assign(&mut self, other: Self) {
                GFOps::<$b>::mul_assign(self, other)
            }
        }
        impl std::ops::Div for $t {
            type Output = Self;
            fn div(self, other: Self) -> Self {
                GFOps::<$b>::div(self, other)
            }
        }
        impl std::ops::DivAssign for $t {
            fn div_assign(&mut self, other: Self) {
                GFOps::<$b>::div_assign(self, other)
            }
        }
        impl From<$b> for $t {
            fn from(value: $b) -> $t {
                $t { value }
            }
        }
        impl Into<$b> for $t {
            fn into(self) -> $b {
                self.value
            }
        }
        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.value)
            }
        }
    };
}

#[snippet("gf", include = "gf_macro")]
GF!(GFp, 1000000007, i64);

#[test]
fn test_gf() {
    let a: GFp = 423343.into();
    let b = 74324.into();
    let c = 13231.into();
    let d = 8432455.into();
    //let r: GFp = (a * b + c) / d;
    assert_eq!((a * b + c) / d, 79639022.into());
    assert_eq!(b - a, 999650988.into());
}
