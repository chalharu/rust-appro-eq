#![cfg_attr(feature = "i128", feature(i128_type))]

#[cfg(feature = "num-complex")]
extern crate num_complex;

#[cfg(feature = "ndarray")]
extern crate ndarray;

#[cfg(feature = "num-rational")]
extern crate num_rational;

#[macro_use]
extern crate appro_eq;

#[cfg(feature = "num-complex")]
use num_complex::Complex;

#[cfg(feature = "num-rational")]
use num_rational::Rational;

#[cfg(feature = "ndarray")]
use ndarray::{ArrayD, IxDyn, arr1, arr2, arr3};

use std::rc::Rc;
use std::sync::Arc;
use std::cell::{Cell, RefCell};
use std::time::{Duration, Instant, SystemTime};

macro_rules! panic_test_rel {
    ($name:ident, $($arg:tt)*) => (
        mod $name {
            #[allow(unused_imports)]
            use super::*;

            #[test]
            #[should_panic]
            fn normal() {
                assert_appro_eq_rel!($($arg)*);
            }

            #[test]
            #[cfg_attr(debug_assertions, should_panic)]
            fn debug() {
                debug_assert_appro_eq_rel!($($arg)*);
            }
        }
    )
}

macro_rules! panic_test_abs {
    ($name:ident, $($arg:tt)*) => (
        mod $name {
            #[allow(unused_imports)]
            use super::*;

            #[test]
            #[should_panic]
            fn normal() {
                assert_appro_eq_abs!($($arg)*);
            }

            #[test]
            #[cfg_attr(debug_assertions, should_panic)]
            fn debug() {
                debug_assert_appro_eq_abs!($($arg)*);
            }
        }
    )
}

macro_rules! panic_test_none {
    ($name:ident, $($arg:tt)*) => (
        mod $name {
            #[allow(unused_imports)]
            use super::*;

            #[test]
            #[should_panic]
            fn normal() {
                assert_appro_eq!($($arg)*);
            }

            #[test]
            #[cfg_attr(debug_assertions, should_panic)]
            fn debug() {
                debug_assert_appro_eq!($($arg)*);
            }
        }
    )
}

macro_rules! panic_test_all {
    ($name:ident, $($arg:tt)*) => (
        mod $name {
            #[allow(unused_imports)]
            use super::*;
            panic_test_none!(none, $($arg)*);
            panic_test_abs!(abs, $($arg)*);
            panic_test_rel!(rel, $($arg)*);
        }
    )
}

macro_rules! ok_test_rel {
    ($name:ident, $($arg:tt)*) => (
        mod $name {
            #[allow(unused_imports)]
            use super::*;

            #[test]
            fn normal() {
                assert_appro_eq_rel!($($arg)*);
            }

            #[test]
            fn debug() {
                debug_assert_appro_eq_rel!($($arg)*);
            }
        }
    )
}

macro_rules! ok_test_abs {
    ($name:ident, $($arg:tt)*) => (
        mod $name {
            #[allow(unused_imports)]
            use super::*;

            #[test]
            fn normal() {
                assert_appro_eq_abs!($($arg)*);
            }

            #[test]
            fn debug() {
                debug_assert_appro_eq_abs!($($arg)*);
            }
        }
    )
}

macro_rules! ok_test_none {
    ($name:ident, $($arg:tt)*) => (
        mod $name {
            #[allow(unused_imports)]
            use super::*;

            #[test]
            fn normal() {
                assert_appro_eq!($($arg)*);
            }

            #[test]
            fn debug() {
                debug_assert_appro_eq!($($arg)*);
            }
        }
    )
}

macro_rules! ok_test_all {
    ($name:ident, $($arg:tt)*) => (
        mod $name {
            #[allow(unused_imports)]
            use super::*;
            ok_test_none!(none, $($arg)*);
            ok_test_abs!(abs, $($arg)*);
            ok_test_rel!(rel, $($arg)*);
        }
    )
}

ok_test_all!(
    it_should_not_panic_if_values_are_appro_equal,
    8f32,
    8f32 + 1e-7
);
ok_test_all!(
    it_should_not_panic_if_values_are_appro_equal_f64,
    1e-12f64 + 1e-25,
    1e-12 as f64
);
ok_test_all!(compare_with_explicit_eps, 3f64, 4f64, 2f64);

panic_test_all!(
    it_should_panic_if_values_are_not_appro_equal,
    8f32,
    8f32 - 1e-5
);
panic_test_all!(bad_compare_with_explicit_eps, 3f64, 4f64, 1e-3f64);
panic_test_rel!(bad_compare_with_rel_div_zero_f32, 3f32, 0f32);
panic_test_rel!(bad_compare_with_rel_div_zero_f64, 3f64, 0f64);

ok_test_all!(
    it_should_not_panic_if_values_are_appro_equal_f32_f64,
    8f32,
    8f64 + 1e-7
);
panic_test_all!(
    it_should_panic_if_values_are_not_appro_equal_f32_f64,
    8f32,
    8f64 - 1e-5
);
panic_test_all!(bad_compare_with_explicit_eps_f32_f64, 3f32, 4f64, 1e-3f32);

ok_test_all!(
    it_should_not_panic_if_values_are_appro_equal_f64_f32,
    8f64,
    8f32 + 1e-7
);
panic_test_all!(
    it_should_panic_if_values_are_not_appro_equal_f64_f32,
    8f64,
    8f32 - 1e-5
);
panic_test_all!(bad_compare_with_explicit_eps_f64_f32, 3f64, 4f32, 1e-3f32);

ok_test_all!(
    compare_with_vector,
    vec![
        1f32,
        2.0,
        3.0,
        4.0,
        5.0,
        6.0,
        7.0 + 1e-8,
        8.0,
        9.0 - 1e-7,
        10.0,
    ],
    vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0 - 1e-9, 9.0, 10.0]
);

panic_test_all!(
    bad_compare_with_vector,
    vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.01],
    vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]
);
panic_test_all!(
    bad_len_compare_with_vector,
    vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0],
    vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]
);

ok_test_all!(
    compare_with_slice,
    &[
        1f32,
        2.0,
        3.0,
        4.0,
        5.0,
        6.0,
        7.0 + 1e-8,
        8.0,
        9.0 - 1e-7,
        10.0
    ] as &[f32],
    &[1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0 - 1e-9, 9.0, 10.0] as &[f32]
);

panic_test_all!(
    bad_compare_with_slice,
    &[1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.01] as &[f32],
    &[1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0] as &[f32]
);
panic_test_all!(
    bad_len_compare_with_slice,
    &[1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0] as &[f32],
    &[1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0] as &[f32]
);

ok_test_all!(
    compare_with_array,
    [
        1f32,
        2.0,
        3.0,
        4.0,
        5.0,
        6.0,
        7.0 + 1e-8,
        8.0,
        9.0 - 1e-7,
        10.0
    ],
    [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0 - 1e-9, 9.0, 10.0]
);

panic_test_all!(
    bad_compare_with_array,
    [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.01],
    [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]
);

#[cfg(feature = "num-complex")]
ok_test_all!(
    compare_with_complex,
    Complex::new(1.0f64, 0.0),
    Complex::new(1.0f64, 1e-12)
);

#[cfg(feature = "num-complex")]
panic_test_all!(
    bad_compare_with_complex,
    Complex::new(1.0f64, 0.0),
    Complex::new(1.0f64, 1e-8)
);

macro_rules! type_impls {
    ($($T:ident)+) => {
        $(
            mod $T {
                ok_test_all!(it_should_not_panic_if_values_are_appro_equal, 1 as $T, 1 as $T);
                panic_test_all!(it_should_panic_if_values_are_not_appro_equal, 0 as $T, 1 as $T);
                panic_test_rel!(it_should_panic_if_values_are_rel_div_zero, 1 as $T, 0 as $T);
            }
        )+
    }
}

type_impls! { i8 i16 i32 i64 u8 u16 u32 u64 }

#[cfg(feature = "i128")]
type_impls! { i128 u128 }

ok_test_all!(
    compare_with_option_both_some,
    Option::Some(1f64),
    Option::Some(1.0 + 1e-12)
);

ok_test_all!(
    compare_with_option_both_none,
    Option::None as Option<f64>,
    Option::None as Option<f64>
);

panic_test_all!(
    bad_compare_with_option_both_some,
    Option::Some(2f64),
    Option::Some(1f64)
);
panic_test_all!(
    bad_compare_with_option_left_some,
    Option::Some(2f64),
    Option::None as Option<f64>
);
panic_test_all!(
    bad_compare_with_option_right_some,
    Option::None as Option<f64>,
    Option::Some(1f64)
);

#[cfg(feature = "num-rational")]
ok_test_all!(
    compare_with_ratio_mindiff,
    Rational::new(1001, 1000),
    Rational::new(1002, 1001),
    Rational::new(1, 10000)
);

#[cfg(feature = "num-rational")]
ok_test_all!(
    compare_with_ratio_equal,
    Rational::new(1, 1000),
    Rational::new(1, 1000)
);

#[cfg(feature = "num-rational")]
panic_test_all!(
    bad_compare_with_ratio,
    Rational::new(1, 1000),
    Rational::new(1, 1001),
    Rational::new(1, 1000000000)
);

ok_test_all!(compare_with_rc, Rc::new(1.0), Rc::new(1.0));
panic_test_all!(bad_compare_with_rc, Rc::new(1.0), Rc::new(1.00001));

ok_test_all!(compare_with_arc, Arc::new(1.0), Arc::new(1.0));
panic_test_all!(bad_compare_with_arc, Arc::new(1.0), Arc::new(1.00001));

ok_test_all!(
    compare_with_weak,
    Rc::downgrade(&Rc::new(1.0)),
    Rc::downgrade(&Rc::new(1.0))
);
panic_test_all!(
    bad_compare_with_weak,
    Rc::downgrade(&Rc::new(1.0)),
    Rc::downgrade(&Rc::new(1.00001))
);

ok_test_all!(compare_with_cell, Cell::new(1.0), Cell::new(1.0));
panic_test_all!(bad_compare_with_cell, Cell::new(1.0), Cell::new(1.00001));


ok_test_all!(compare_with_refcell, RefCell::new(1.0), RefCell::new(1.0));
panic_test_all!(
    bad_compare_with_refcell,
    RefCell::new(1.0),
    RefCell::new(1.00001)
);

#[cfg(feature = "ndarray")]
ok_test_all!(
    compare_with_ndarray1d,
    arr1(&[1f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]),
    arr1(&[1f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0])
);

#[cfg(feature = "ndarray")]
ok_test_all!(
    compare_with_ndarray2d,
    arr2(&[[1f64, 2.0, 3.0, 4.0, 5.0], [6.0, 7.0, 8.0, 9.0, 10.0]]),
    arr2(&[[1f64, 2.0, 3.0, 4.0, 5.0], [6.0, 7.0, 8.0, 9.0, 10.0]])
);

#[cfg(feature = "ndarray")]
ok_test_all!(
    compare_with_ndarray3d,
    arr3(&[[[1f64, 2.0], [4.0, 5.0]], [[6.0, 7.0], [9.0, 10.0]]]),
    arr3(&[[[1f64, 2.0], [4.0, 5.0]], [[6.0, 7.0], [9.0, 10.0]]])
);

#[cfg(feature = "ndarray")]
ok_test_abs!(
    compare_with_ndarraynd,
    ArrayD::<f64>::zeros(IxDyn(&[2, 3, 4, 5, 6, 7])),
    ArrayD::<f64>::zeros(IxDyn(&[2, 3, 4, 5, 6, 7]))
);

#[cfg(feature = "ndarray")]
panic_test_all!(
    bad_compare_with_ndarray1d,
    arr1(&[1f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0001]),
    arr1(&[1f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0])
);

#[cfg(feature = "ndarray")]
panic_test_all!(
    bad_compare_with_ndarray2d_val,
    arr2(&[[1f64, 2.0, 3.0, 4.0, 5.0], [6.0, 7.0, 8.0, 9.0, 10.0001]]),
    arr2(&[[1f64, 2.0, 3.0, 4.0, 5.0], [6.0, 7.0, 8.0, 9.0, 10.0]])
);

#[cfg(feature = "ndarray")]
panic_test_all!(
    bad_compare_with_ndarray2d_len,
    arr2(&[[1f64, 2.0, 3.0, 4.0, 5.0], [6.0, 7.0, 8.0, 9.0, 10.0]]),
    arr2(&[
        [1f64, 2.0],
        [3.0, 4.0],
        [5.0, 6.0],
        [7.0, 8.0],
        [9.0, 10.0]
    ])
);

#[cfg(feature = "ndarray")]
panic_test_all!(
    bad_compare_with_ndarray3d,
    arr3(&[[[1f64, 2.0], [4.0, 5.0]], [[6.0, 7.0], [9.0, 10.0001]]]),
    arr3(&[[[1f64, 2.0], [4.0, 5.0]], [[6.0, 7.0], [9.0, 10.0]]])
);

#[cfg(feature = "ndarray")]
panic_test_all!(
    bad_compare_with_ndarraynd,
    ArrayD::<f64>::zeros(IxDyn(&[2, 3, 4, 5, 6, 7])),
    ArrayD::<f64>::zeros(IxDyn(&[2, 3, 4, 5, 6]))
);

#[cfg(feature = "ndarray")]
panic_test_rel!(
    compare_with_ndarraynd_rel,
    ArrayD::<f64>::zeros(IxDyn(&[2, 3, 4, 5, 6, 7])),
    ArrayD::<f64>::zeros(IxDyn(&[2, 3, 4, 5, 6, 7]))
);

#[cfg(feature = "ndarray")]
panic_test_all!(
    bad_compare_with_ndarray3d_f32_f64,
    arr3(&[[[1f32, 2.0], [4.0, 5.0]], [[6.0, 7.0], [9.0, 10.001]]]),
    arr3(&[[[1f64, 2.0], [4.0, 5.0]], [[6.0, 7.0], [9.0, 10.0]]])
);

ok_test_none!(
    compare_with_systemtime_none,
    SystemTime::now(),
    SystemTime::now()
);

ok_test_abs!(
    compare_with_systemtime_abs,
    SystemTime::now(),
    SystemTime::now()
);

panic_test_none!(
    bad_compare_with_systemtime_none,
    SystemTime::now() - Duration::new(10, 0),
    SystemTime::now()
);

panic_test_abs!(
    bad_compare_with_systemtime_abs,
    SystemTime::now() - Duration::new(10, 0),
    SystemTime::now()
);

ok_test_abs!(compare_with_instant_abs, Instant::now(), Instant::now());

ok_test_none!(compare_with_instant_none, Instant::now(), Instant::now());

panic_test_abs!(
    bad_compare_with_instant_abs,
    Instant::now() - Duration::new(10, 0),
    Instant::now()
);

panic_test_none!(
    bad_compare_with_instant_none,
    Instant::now() - Duration::new(10, 0),
    Instant::now()
);

ok_test_all!(
    compare_with_duration,
    Duration::new(10, 0),
    Duration::new(10, 1)
);

panic_test_all!(
    bad_compare_with_duration,
    Duration::new(9, 0),
    Duration::new(10, 1)
);
