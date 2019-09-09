//! # Licensing
//! This Source Code is subject to the terms of the Mozilla Public License
//! version 2.0 (the "License"). You can obtain a copy of the License at
//! http://mozilla.org/MPL/2.0/.

/// Asserts that two expressions are approximately equal to each other.
///
/// You can optionally add an optional diff value. If you don't supply
///  a diff value as an argument, Tolerance::tolerance() is the default used.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate appro_eq;
/// # fn main() {
/// assert_appro_eq!(1f64, 1.5f64, 0.6f64); // does not panic
/// assert_appro_eq!(0f64, 1e-12f64); // does not panic
/// assert_appro_eq!(vec![0f64, 1.0, 0.0], vec![1e-12f64, 1.0, -1e-13f64]); // does not panic
/// # }
/// ```
/// ```should_panic
/// # #[macro_use] extern crate appro_eq;
/// # fn main() {
/// assert_appro_eq!(1f64, 2f64); // panics
/// # }
/// ```
#[macro_export]
#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
macro_rules! assert_appro_eq {
    ($a:expr, $b:expr) => {{
        assert!(
            $crate::AbsApproEq::abs_appro_eq(&$a, &$b),
            "assertion failed: `(left == right)` (left: `{:?}` , right: `{:?}`)",
            $a,
            $b
        );
    }};
    ($a:expr, $b:expr, $eps:expr) => {{
        assert!(
            $crate::AbsApproEqWithTol::abs_appro_eq_with_tol(&$a, &$b, &$eps),
            "assertion failed: `(left == right)` (left: `{:?}` , right: `{:?}`, eps: `{:?}`)",
            $a,
            $b,
            $eps
        );
    }};
}

/// Asserts that the absolute error of the two expressions is small enough.
///
/// You can optionally add an optional diff value. If you don't supply
///  a diff value as an argument, Tolerance::tolerance() is the default used.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate appro_eq;
/// # fn main() {
/// assert_appro_eq_abs!(1f64, 1.5f64, 0.6f64); // does not panic
/// assert_appro_eq_abs!(0f64, 1e-12f64); // does not panic
/// assert_appro_eq_abs!(vec![0f64, 1.0, 0.0], vec![1e-12f64, 1.0, -1e-13f64]); // does not panic
/// # }
/// ```
/// ```should_panic
/// # #[macro_use] extern crate appro_eq;
/// # fn main() {
/// assert_appro_eq_abs!(1f64, 2f64); // panics
/// # }
/// ```
#[macro_export]
#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
macro_rules! assert_appro_eq_abs {
    ($a:expr, $b:expr) => {{
        assert!(
            $crate::AbsApproEq::abs_appro_eq(&$a, &$b),
            "assertion failed: `(left == right)` (left: `{:?}` , right: `{:?}`)",
            $a,
            $b
        );
    }};
    ($a:expr, $b:expr, $eps:expr) => {{
        assert!(
            $crate::AbsApproEqWithTol::abs_appro_eq_with_tol(&$a, &$b, &$eps),
            "assertion failed: `(left == right)` (left: `{:?}` , right: `{:?}`, eps: `{:?}`)",
            $a,
            $b,
            $eps
        );
    }};
}

/// Asserts that the relative error of the two expressions is small enough.
///
/// You can optionally add an optional diff value. If you don't supply
///  a diff value as an argument, Tolerance::tolerance() is the default used.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate appro_eq;
/// # fn main() {
/// assert_appro_eq_rel!(1f64, 1.5f64, 0.6f64); // does not panic
/// assert_appro_eq_rel!(1f64, 1.0 + 1e-12f64); // does not panic
/// assert_appro_eq_rel!(vec![1f64, 1.0, 1.0], vec![1.0 + 1e-12f64, 1.0, 1.0 - 1e-13f64]); // does not panic
/// # }
/// ```
/// ```should_panic
/// # #[macro_use] extern crate appro_eq;
/// # fn main() {
/// assert_appro_eq_rel!(1f64, 2f64); // panics
/// # }
/// ```
#[macro_export]
#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
macro_rules! assert_appro_eq_rel {
    ($a:expr, $b:expr) => {{
        assert!(
            $crate::RelApproEq::rel_appro_eq(&$a, &$b),
            "assertion failed: `(left == right)` (left: `{:?}` , right: `{:?}`)",
            $a,
            $b
        );
    }};
    ($a:expr, $b:expr, $eps:expr) => {{
        assert!(
            $crate::RelApproEqWithTol::rel_appro_eq_with_tol(&$a, &$b, &$eps),
            "assertion failed: `(left == right)` (left: `{:?}` , right: `{:?}`, eps: `{:?}`)",
            $a,
            $b,
            $eps
        );
    }};
}

/// Asserts that two expressions are approximately equal to each other.
///
/// You can optionally add an optional diff value. If you don't supply
///  a diff value as an argument, Tolerance::tolerance() is the default used.
///
/// Unlike assert_appro_eq!, debug_assert_appro_eq! statements are only enabled in non optimized builds by default.
/// An optimized build will omit all debug_assert_appro_eq! statements unless -C debug-assertions is passed to the compiler.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate appro_eq;
/// # fn main() {
/// debug_assert_appro_eq!(1f64, 1.5f64, 0.6f64); // does not panic
/// debug_assert_appro_eq!(0f64, 1e-12f64); // does not panic
/// debug_assert_appro_eq!(vec![0f64, 1.0, 0.0], vec![1e-12f64, 1.0, -1e-13f64]); // does not panic
/// # }
/// ```
/// ```should_panic
/// # #[macro_use] extern crate appro_eq;
/// # fn main() {
/// debug_assert_appro_eq!(1f64, 2f64); // panics
/// # }
/// ```
#[macro_export]
#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
macro_rules! debug_assert_appro_eq {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { assert_appro_eq!($($arg)*); })
}

/// Asserts that the absolute error of the two expressions is small enough.
///
/// You can optionally add an optional diff value. If you don't supply
///  a diff value as an argument, Tolerance::tolerance() is the default used.
///
/// Unlike assert_appro_eq_abs!, debug_assert_appro_eq_abs! statements are only enabled in non optimized builds by default.
/// An optimized build will omit all debug_assert_appro_eq_abs! statements unless -C debug-assertions is passed to the compiler.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate appro_eq;
/// # fn main() {
/// debug_assert_appro_eq_abs!(1f64, 1.5f64, 0.6f64); // does not panic
/// debug_assert_appro_eq_abs!(0f64, 1e-12f64); // does not panic
/// debug_assert_appro_eq_abs!(vec![0f64, 1.0, 0.0], vec![1e-12f64, 1.0, -1e-13f64]); // does not panic
/// # }
/// ```
/// ```should_panic
/// # #[macro_use] extern crate appro_eq;
/// # fn main() {
/// debug_assert_appro_eq_abs!(1f64, 2f64); // panics
/// # }
/// ```
#[macro_export]
#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
macro_rules! debug_assert_appro_eq_abs {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { assert_appro_eq_abs!($($arg)*); })
}

/// Asserts that the relative error of the two expressions is small enough.
///
/// You can optionally add an optional diff value. If you don't supply
///  a diff value as an argument, Tolerance::tolerance() is the default used.
///
/// Unlike assert_appro_eq_rel!, debug_assert_appro_eq_rel! statements are only enabled in non optimized builds by default.
/// An optimized build will omit all debug_assert_appro_eq_rel! statements unless -C debug-assertions is passed to the compiler.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate appro_eq;
/// # fn main() {
/// debug_assert_appro_eq_rel!(1f64, 1.5f64, 0.6f64); // does not panic
/// debug_assert_appro_eq_rel!(1f64, 1.0 + 1e-12f64); // does not panic
/// debug_assert_appro_eq_rel!(vec![1f64, 1.0, 1.0], vec![1.0 + 1e-12f64, 1.0, 1.0 - 1e-13f64]); // does not panic
/// # }
/// ```
/// ```should_panic
/// # #[macro_use] extern crate appro_eq;
/// # fn main() {
/// debug_assert_appro_eq_rel!(1f64, 2f64); // panics
/// # }
/// ```
#[macro_export]
#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
macro_rules! debug_assert_appro_eq_rel {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { assert_appro_eq_rel!($($arg)*); })
}
