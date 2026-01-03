#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "nightly", feature(f16, f128))]
#![no_std]

/// Cast from one type to another.
///
/// Mirrors [`From`] but with casting semantics. `CastFrom<T> for U` means
/// "U can be created from T via cast".
///
/// # Example
///
/// ```
/// use casting::CastFrom;
///
/// let x: u8 = 42;
/// let y = u16::cast_from(x);
/// assert_eq!(y, 42u16);
/// ```
pub trait CastFrom<T> {
    /// Casts `value` from type `T` to `Self`.
    ///
    /// This method performs a numeric conversion using the `as` keyword.
    fn cast_from(value: T) -> Self;
}

impl<T> CastFrom<T> for T {
    fn cast_from(value: T) -> Self {
        value
    }
}

/// Cast into another type.
///
/// Mirrors [`Into`] but with casting semantics. Automatically implemented for
/// all types that implement [`CastFrom`].
///
/// **Do not implement this trait directly.** Implement [`CastFrom`] instead.
///
/// # Example
///
/// ```
/// use casting::CastInto;
///
/// let x: u8 = 42;
/// let y: u16 = x.cast_into();
/// assert_eq!(y, 42u16);
/// ```
pub trait CastInto<T> {
    /// Casts `self` into type `T`.
    ///
    /// This method performs a numeric conversion using the `as` keyword.
    fn cast_into(self) -> T;
}

impl<T, U: CastFrom<T>> CastInto<U> for T {
    fn cast_into(self) -> U {
        U::cast_from(self)
    }
}

macro_rules! impl_cast {
    (@inner ($from:ty) => ($into:ty)) => { #[cfg(feature = "nightly")] impl_cast! { @inner $from => $into } };
    (@inner ($from:ty) => $into:ty)   => { #[cfg(feature = "nightly")] impl_cast! { @inner $from => $into } };
    (@inner $from:ty => ($into:ty))   => { #[cfg(feature = "nightly")] impl_cast! { @inner $from => $into } };

    (@inner $from:ty => $into:ty) => {
        impl CastFrom<$from> for $into {
            #[inline(always)]
            fn cast_from(value: $from) -> Self {
                value as $into
            }
        }
    };

    // Entry point
    ($($from:tt => $($into:tt),+;)+ $(,)?) => {
        $(
            $(
                impl_cast! { @inner $from => $into }
            )+
        )+
    };
}

impl_cast! {
    bool   =>  u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize;
    char   =>  u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize;
    u8     =>      u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, (f16), f32, f64, (f128), char;
    u16    =>  u8,      u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, (f16), f32, f64, (f128);
    u32    =>  u8, u16,      u64, u128, usize, i8, i16, i32, i64, i128, isize, (f16), f32, f64, (f128);
    u64    =>  u8, u16, u32,      u128, usize, i8, i16, i32, i64, i128, isize, (f16), f32, f64, (f128);
    u128   =>  u8, u16, u32, u64,       usize, i8, i16, i32, i64, i128, isize, (f16), f32, f64, (f128);
    usize  =>  u8, u16, u32, u64, u128,        i8, i16, i32, i64, i128, isize, (f16), f32, f64, (f128);
    i8     =>  u8, u16, u32, u64, u128, usize,     i16, i32, i64, i128, isize, (f16), f32, f64, (f128);
    i16    =>  u8, u16, u32, u64, u128, usize, i8,      i32, i64, i128, isize, (f16), f32, f64, (f128);
    i32    =>  u8, u16, u32, u64, u128, usize, i8, i16,      i64, i128, isize, (f16), f32, f64, (f128);
    i64    =>  u8, u16, u32, u64, u128, usize, i8, i16, i32,      i128, isize, (f16), f32, f64, (f128);
    i128   =>  u8, u16, u32, u64, u128, usize, i8, i16, i32, i64,       isize, (f16), f32, f64, (f128);
    isize  =>  u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128,        (f16), f32, f64, (f128);
    (f16)  =>  u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize,        f32, f64, (f128);
    f32    =>  u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, (f16),      f64, (f128);
    f64    =>  u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, (f16), f32,      (f128);
    (f128) =>  u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, (f16), f32, f64;
}
