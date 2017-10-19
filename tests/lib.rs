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

#[test]
fn it_should_not_panic_if_values_are_appro_equal() {
    assert_appro_eq!(8f32, 8f32 + 1e-7);
}

#[test]
fn it_should_not_panic_if_values_are_appro_equal_f64() {
    assert_appro_eq!(0f64, 1e-12 as f64);
}

#[test]
fn it_should_not_panic_if_values_are_appro_equal_abs() {
    assert_appro_eq_abs!(8f32, 8f32 + 1e-7);
}

#[test]
fn it_should_not_panic_if_values_are_appro_equal_f64_abs() {
    assert_appro_eq_abs!(0f64, 1e-12 as f64);
}

#[test]
fn it_should_not_panic_if_values_are_appro_equal_rel() {
    assert_appro_eq_rel!(8f32, 8f32 + 1e-7);
}

#[test]
fn it_should_not_panic_if_values_are_appro_equal_f64_rel() {
    assert_appro_eq_rel!(1e-12f64 + 1e-25, 1e-12 as f64);
}

#[test]
#[should_panic]
fn it_should_panic_if_values_are_not_appro_equal() {
    assert_appro_eq!(8f32, 8f32 - 1e-5);
}

#[test]
#[should_panic]
fn it_should_panic_if_values_are_not_appro_equal_abs() {
    assert_appro_eq_abs!(8f32, 8f32 - 1e-5);
}

#[test]
#[should_panic]
fn it_should_panic_if_values_are_not_appro_equal_rel() {
    assert_appro_eq_rel!(8f32, 8f32 - 1e-5);
}

#[test]
fn compare_with_explicit_eps() {
    assert_appro_eq!(3f64, 4f64, 2f64);
}

#[test]
fn compare_with_explicit_eps_abs() {
    assert_appro_eq_abs!(3f64, 4f64, 2f64);
}

#[test]
fn compare_with_explicit_eps_rel() {
    assert_appro_eq_rel!(3f64, 4f64, 2f64);
}

#[test]
#[should_panic]
fn bad_compare_with_explicit_eps() {
    assert_appro_eq!(3f64, 4f64, 1e-3f64);
}

#[test]
#[should_panic]
fn bad_compare_with_explicit_eps_abs() {
    assert_appro_eq_abs!(3f64, 4f64, 1e-3f64);
}

#[test]
#[should_panic]
fn bad_compare_with_explicit_eps_rel() {
    assert_appro_eq_rel!(3f64, 4f64, 1e-3f64);
}

#[test]
fn compare_with_vector() {
    let left = vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let right = vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    assert_appro_eq!(left, right);
}

#[test]
fn compare_with_vector_abs() {
    let left = vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let right = vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    assert_appro_eq_abs!(left, right);
}

#[test]
fn compare_with_vector_rel() {
    let left = vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let right = vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    assert_appro_eq_rel!(left, right);
}

#[test]
#[should_panic]
fn bad_compare_with_vector() {
    let left = vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.01];
    let right = vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    assert_appro_eq!(left, right);
}

#[test]
#[should_panic]
fn bad_compare_with_vector_abs() {
    let left = vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.01];
    let right = vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    assert_appro_eq_abs!(left, right);
}

#[test]
#[should_panic]
fn bad_compare_with_vector_rel() {
    let left = vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.01];
    let right = vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    assert_appro_eq_rel!(left, right);
}

#[test]
#[should_panic]
fn bad_len_compare_with_vector() {
    let left = vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let right = vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    assert_appro_eq!(left, right);
}

#[test]
#[should_panic]
fn bad_len_compare_with_vector_abs() {
    let left = vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let right = vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    assert_appro_eq_abs!(left, right);
}

#[test]
#[should_panic]
fn bad_len_compare_with_vector_rel() {
    let left = vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let right = vec![1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    assert_appro_eq_rel!(left, right);
}

#[test]
fn compare_with_slice() {
    let left = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let right = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    assert_appro_eq!(&left as &[f32], &right as &[f32]);
}

#[test]
fn compare_with_slice_abs() {
    let left = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let right = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    assert_appro_eq_abs!(&left as &[f32], &right as &[f32]);
}

#[test]
fn compare_with_slice_rel() {
    let left = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let right = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    assert_appro_eq_rel!(&left as &[f32], &right as &[f32]);
}

#[test]
#[should_panic]
fn bad_compare_with_slice() {
    let left = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.01];
    let right = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    assert_appro_eq!(&left as &[f32], &right as &[f32]);
}

#[test]
#[should_panic]
fn bad_compare_with_slice_abs() {
    let left = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.01];
    let right = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    assert_appro_eq_abs!(&left as &[f32], &right as &[f32]);
}

#[test]
#[should_panic]
fn bad_compare_with_slice_rel() {
    let left = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.01];
    let right = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    assert_appro_eq_rel!(&left as &[f32], &right as &[f32]);
}

#[test]
#[should_panic]
fn bad_len_compare_with_slice() {
    let left = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let right = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    assert_appro_eq!(&left as &[f32], &right as &[f32]);
}

#[test]
#[should_panic]
fn bad_len_compare_with_slice_abs() {
    let left = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let right = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    assert_appro_eq_abs!(&left as &[f32], &right as &[f32]);
}

#[test]
#[should_panic]
fn bad_len_compare_with_slice_rel() {
    let left = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let right = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    assert_appro_eq_rel!(&left as &[f32], &right as &[f32]);
}

#[test]
fn compare_with_array() {
    let left = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let right = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    assert_appro_eq!(left, right);
}

#[test]
fn compare_with_array_abs() {
    let left = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let right = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    assert_appro_eq_abs!(left, right);
}

#[test]
fn compare_with_array_rel() {
    let left = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let right = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    assert_appro_eq_rel!(left, right);
}

#[test]
#[should_panic]
fn bad_compare_with_array() {
    let left = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.01];
    let right = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    assert_appro_eq!(left, right);
}

#[test]
#[should_panic]
fn bad_compare_with_array_abs() {
    let left = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.01];
    let right = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    assert_appro_eq_abs!(left, right);
}

#[test]
#[should_panic]
fn bad_compare_with_array_rel() {
    let left = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.01];
    let right = [1f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    assert_appro_eq_rel!(left, right);
}

#[test]
#[cfg(feature = "num-complex")]
fn compare_with_complex() {
    let left = Complex::new(1.0f64, 0.0);
    let right = Complex::new(1.0f64, 1e-12);
    assert_appro_eq!(left, right);
}

#[test]
#[cfg(feature = "num-complex")]
fn compare_with_complex_abs() {
    let left = Complex::new(1.0f64, 0.0);
    let right = Complex::new(1.0f64, 1e-12);
    assert_appro_eq_abs!(left, right);
}

#[test]
#[cfg(feature = "num-complex")]
fn compare_with_complex_rel() {
    let left = Complex::new(1.0f64, 0.0);
    let right = Complex::new(1.0f64, 1e-12);
    assert_appro_eq_rel!(left, right);
}

#[test]
#[should_panic]
#[cfg(feature = "num-complex")]
fn bad_compare_with_complex() {
    let left = Complex::new(1.0f64, 0.0);
    let right = Complex::new(1.0f64, 1e-8);
    assert_appro_eq!(left, right);
}

#[test]
#[should_panic]
#[cfg(feature = "num-complex")]
fn bad_compare_with_complex_abs() {
    let left = Complex::new(1.0f64, 0.0);
    let right = Complex::new(1.0f64, 1e-8);
    assert_appro_eq_abs!(left, right);
}

#[test]
#[should_panic]
#[cfg(feature = "num-complex")]
fn bad_compare_with_complex_rel() {
    let left = Complex::new(1.0f64, 0.0);
    let right = Complex::new(1.0f64, 1e-8);
    assert_appro_eq_rel!(left, right);
}

macro_rules! type_impls {
    ($($T:ident)+) => {
        $(
            mod $T {
                #[test]
                fn it_should_not_panic_if_values_are_appro_equal() {
                    assert_appro_eq!(0 as $T, 0 as $T);
                }

                #[test]
                fn it_should_not_panic_if_values_are_appro_equal_abs() {
                    assert_appro_eq_abs!(0 as $T, 0 as $T);
                }

                #[test]
                fn it_should_not_panic_if_values_are_appro_equal_rel() {
                    assert_appro_eq_rel!(1 as $T, 1 as $T);
                }

                #[test]
                #[should_panic]
                fn it_should_panic_if_values_are_not_appro_equal() {
                    assert_appro_eq!(0 as $T, 1 as $T);
                }

                #[test]
                #[should_panic]
                fn it_should_panic_if_values_are_not_appro_equal_abs() {
                    assert_appro_eq_abs!(0 as $T, 1 as $T);
                }

                #[test]
                #[should_panic]
                fn it_should_panic_if_values_are_not_appro_equal_rel() {
                    assert_appro_eq_rel!(0 as $T, 1 as $T);
                }
            }
        )+
    }
}

type_impls! { i8 i16 i32 i64 u8 u16 u32 u64 }

#[cfg(feature = "i128")]
type_impls! { i128 u128 }

#[test]
fn compare_with_option_both_some() {
    let left = Option::Some(0f64);
    let right = Option::Some(1e-12);
    assert_appro_eq!(left, right);
}

#[test]
fn compare_with_option_both_some_abs() {
    let left = Option::Some(0f64);
    let right = Option::Some(1e-12);
    assert_appro_eq_abs!(left, right);
}

#[test]
fn compare_with_option_both_some_rel() {
    let left = Option::Some(1f64);
    let right = Option::Some(1.0 + 1e-12);
    assert_appro_eq_rel!(left, right);
}

#[test]
fn compare_with_option_both_none() {
    let left: Option<f64> = Option::None;
    let right = Option::None;
    assert_appro_eq!(left, right);
}

#[test]
fn compare_with_option_both_none_abs() {
    let left: Option<f64> = Option::None;
    let right = Option::None;
    assert_appro_eq_abs!(left, right);
}

#[test]
fn compare_with_option_both_none_rel() {
    let left: Option<f64> = Option::None;
    let right = Option::None;
    assert_appro_eq_rel!(left, right);
}

#[test]
#[should_panic]
fn bad_compare_with_option_both_some() {
    let left = Option::Some(0f64);
    let right = Option::Some(1f64);
    assert_appro_eq!(left, right);
}

#[test]
#[should_panic]
fn bad_compare_with_option_both_some_abs() {
    let left = Option::Some(0f64);
    let right = Option::Some(1f64);
    assert_appro_eq_abs!(left, right);
}

#[test]
#[should_panic]
fn bad_compare_with_option_both_some_rel() {
    let left = Option::Some(2f64);
    let right = Option::Some(1f64);
    assert_appro_eq_rel!(left, right);
}

#[test]
#[should_panic]
fn bad_compare_with_option_left_some() {
    let left = Option::Some(0f64);
    let right = Option::None;
    assert_appro_eq!(left, right);
}

#[test]
#[should_panic]
fn bad_compare_with_option_left_some_abs() {
    let left = Option::Some(0f64);
    let right = Option::None;
    assert_appro_eq_abs!(left, right);
}

#[test]
#[should_panic]
fn bad_compare_with_option_left_some_rel() {
    let left = Option::Some(0f64);
    let right = Option::None;
    assert_appro_eq_rel!(left, right);
}

#[test]
#[should_panic]
fn bad_compare_with_option_right_some() {
    let left: Option<f64> = Option::None;
    let right = Option::Some(0f64);
    assert_appro_eq!(left, right);
}

#[test]
#[should_panic]
fn bad_compare_with_option_right_some_abs() {
    let left: Option<f64> = Option::None;
    let right = Option::Some(0f64);
    assert_appro_eq_abs!(left, right);
}

#[test]
#[should_panic]
fn bad_compare_with_option_right_some_rel() {
    let left: Option<f64> = Option::None;
    let right = Option::Some(0f64);
    assert_appro_eq_rel!(left, right);
}

#[test]
#[cfg(feature = "num-rational")]
fn compare_with_ratio_mindiff() {
    let left = Rational::new(1, 1000);
    let right = Rational::new(1, 1001);
    let eps = Rational::new(1, 10000);
    assert_appro_eq!(left, right, eps);
}

#[test]
#[cfg(feature = "num-rational")]
fn compare_with_ratio_mindiff_abs() {
    let left = Rational::new(1, 1000);
    let right = Rational::new(1, 1001);
    let eps = Rational::new(1, 10000);
    assert_appro_eq_abs!(left, right, eps);
}

#[test]
#[cfg(feature = "num-rational")]
fn compare_with_ratio_mindiff_rel() {
    let left = Rational::new(1, 1000);
    let right = Rational::new(1, 1001);
    let eps = Rational::new(1, 1);
    assert_appro_eq_rel!(left, right, eps);
}

#[test]
#[cfg(feature = "num-rational")]
fn compare_with_ratio_equal() {
    let left = Rational::new(1, 1000);
    let right = Rational::new(1, 1000);
    assert_appro_eq!(left, right);
}

#[test]
#[cfg(feature = "num-rational")]
fn compare_with_ratio_equal_abs() {
    let left = Rational::new(1, 1000);
    let right = Rational::new(1, 1000);
    assert_appro_eq_abs!(left, right);
}

#[test]
#[cfg(feature = "num-rational")]
fn compare_with_ratio_equal_rel() {
    let left = Rational::new(1, 1000);
    let right = Rational::new(1, 1000);
    assert_appro_eq_rel!(left, right);
}

#[test]
#[should_panic]
#[cfg(feature = "num-rational")]
fn bad_compare_with_ratio() {
    let left = Rational::new(1, 1000);
    let right = Rational::new(1, 1001);
    let eps = Rational::new(1, 1000000000);
    assert_appro_eq!(left, right, eps);
}

#[test]
#[should_panic]
#[cfg(feature = "num-rational")]
fn bad_compare_with_ratio_abs() {
    let left = Rational::new(1, 1000);
    let right = Rational::new(1, 1001);
    let eps = Rational::new(1, 1000000000);
    assert_appro_eq_abs!(left, right, eps);
}

#[test]
#[should_panic]
#[cfg(feature = "num-rational")]
fn bad_compare_with_ratio_rel() {
    let left = Rational::new(1, 1000);
    let right = Rational::new(1, 1001);
    let eps = Rational::new(1, 1000000000);
    assert_appro_eq_rel!(left, right, eps);
}

#[test]
fn compare_with_rc() {
    let left = Rc::new(1.0);
    let right = Rc::new(1.0);
    assert_appro_eq!(left, right);
}

#[test]
fn compare_with_rc_abs() {
    let left = Rc::new(1.0);
    let right = Rc::new(1.0);
    assert_appro_eq_abs!(left, right);
}

#[test]
fn compare_with_rc_rel() {
    let left = Rc::new(1.0);
    let right = Rc::new(1.0);
    assert_appro_eq_rel!(left, right);
}

#[test]
#[should_panic]
fn bad_compare_with_rc() {
    let left = Rc::new(1.0);
    let right = Rc::new(1.00001);
    assert_appro_eq!(left, right);
}

#[test]
#[should_panic]
fn bad_compare_with_rc_abs() {
    let left = Rc::new(1.0);
    let right = Rc::new(1.00001);
    assert_appro_eq_abs!(left, right);
}

#[test]
#[should_panic]
fn bad_compare_with_rc_rel() {
    let left = Rc::new(1.0);
    let right = Rc::new(1.00001);
    assert_appro_eq_rel!(left, right);
}

#[test]
fn compare_with_arc() {
    let left = Arc::new(1.0);
    let right = Arc::new(1.0);
    assert_appro_eq!(left, right);
}

#[test]
fn compare_with_arc_abs() {
    let left = Arc::new(1.0);
    let right = Arc::new(1.0);
    assert_appro_eq_abs!(left, right);
}

#[test]
fn compare_with_arc_rel() {
    let left = Arc::new(1.0);
    let right = Arc::new(1.0);
    assert_appro_eq_rel!(left, right);
}

#[test]
#[should_panic]
fn bad_compare_with_arc() {
    let left = Arc::new(1.0);
    let right = Arc::new(1.00001);
    assert_appro_eq!(left, right);
}

#[test]
#[should_panic]
fn bad_compare_with_arc_abs() {
    let left = Arc::new(1.0);
    let right = Arc::new(1.00001);
    assert_appro_eq_abs!(left, right);
}

#[test]
#[should_panic]
fn bad_compare_with_arc_rel() {
    let left = Arc::new(1.0);
    let right = Arc::new(1.00001);
    assert_appro_eq_rel!(left, right);
}

#[test]
fn compare_with_weak() {
    let left = Rc::new(1.0);
    let right = Rc::new(1.0);
    assert_appro_eq!(Rc::downgrade(&left), Rc::downgrade(&right));
}

#[test]
fn compare_with_weak_abs() {
    let left = Rc::new(1.0);
    let right = Rc::new(1.0);
    assert_appro_eq_abs!(Rc::downgrade(&left), Rc::downgrade(&right));
}

#[test]
fn compare_with_weak_rel() {
    let left = Rc::new(1.0);
    let right = Rc::new(1.0);
    assert_appro_eq_rel!(Rc::downgrade(&left), Rc::downgrade(&right));
}

#[test]
#[should_panic]
fn bad_compare_with_weak() {
    let left = Rc::new(1.0);
    let right = Rc::new(1.00001);
    assert_appro_eq!(Rc::downgrade(&left), Rc::downgrade(&right));
}

#[test]
#[should_panic]
fn bad_compare_with_weak_abs() {
    let left = Rc::new(1.0);
    let right = Rc::new(1.00001);
    assert_appro_eq_abs!(Rc::downgrade(&left), Rc::downgrade(&right));
}

#[test]
#[should_panic]
fn bad_compare_with_weak_rel() {
    let left = Rc::new(1.0);
    let right = Rc::new(1.00001);
    assert_appro_eq_rel!(Rc::downgrade(&left), Rc::downgrade(&right));
}

#[test]
fn compare_with_cell() {
    let left = Cell::new(1.0);
    let right = Cell::new(1.0);
    assert_appro_eq!(left, right);
}

#[test]
fn compare_with_cell_abs() {
    let left = Cell::new(1.0);
    let right = Cell::new(1.0);
    assert_appro_eq_abs!(left, right);
}

#[test]
fn compare_with_cell_rel() {
    let left = Cell::new(1.0);
    let right = Cell::new(1.0);
    assert_appro_eq_rel!(left, right);
}

#[test]
#[should_panic]
fn bad_compare_with_cell() {
    let left = Cell::new(1.0);
    let right = Cell::new(1.00001);
    assert_appro_eq!(left, right);
}

#[test]
#[should_panic]
fn bad_compare_with_cell_abs() {
    let left = Cell::new(1.0);
    let right = Cell::new(1.00001);
    assert_appro_eq_abs!(left, right);
}

#[test]
#[should_panic]
fn bad_compare_with_cell_rel() {
    let left = Cell::new(1.0);
    let right = Cell::new(1.00001);
    assert_appro_eq_rel!(left, right);
}

#[test]
fn compare_with_refcell() {
    let left = RefCell::new(1.0);
    let right = RefCell::new(1.0);
    assert_appro_eq!(left, right);
}

#[test]
fn compare_with_refcell_abs() {
    let left = RefCell::new(1.0);
    let right = RefCell::new(1.0);
    assert_appro_eq_abs!(left, right);
}

#[test]
fn compare_with_refcell_rel() {
    let left = RefCell::new(1.0);
    let right = RefCell::new(1.0);
    assert_appro_eq_rel!(left, right);
}

#[test]
#[should_panic]
fn bad_compare_with_refcell() {
    let left = RefCell::new(1.0);
    let right = RefCell::new(1.00001);
    assert_appro_eq!(left, right);
}

#[test]
#[should_panic]
fn bad_compare_with_refcell_abs() {
    let left = RefCell::new(1.0);
    let right = RefCell::new(1.00001);
    assert_appro_eq_abs!(left, right);
}

#[test]
#[should_panic]
fn bad_compare_with_refcell_rel() {
    let left = RefCell::new(1.0);
    let right = RefCell::new(1.00001);
    assert_appro_eq_rel!(left, right);
}

#[test]
#[cfg(debug_assertions)]
fn it_should_not_panic_if_values_are_nearly_equal_debug() {
    debug_assert_appro_eq!(8f32, 8f32 + 1e-7);
}

#[test]
#[cfg(debug_assertions)]
fn it_should_not_panic_if_values_are_nearly_equal_f64_debug() {
    debug_assert_appro_eq!(0f64, 1e-12 as f64);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic]
fn it_should_panic_if_values_are_not_nearly_equal_debug() {
    debug_assert_appro_eq!(8f32, 8f32 - 1e-5);
}

#[test]
#[cfg(debug_assertions)]
fn compare_with_explicit_eps_debug() {
    debug_assert_appro_eq!(3f64, 4f64, 2f64);
}

#[test]
#[cfg(not(debug_assertions))]
fn it_should_panic_if_values_are_not_nearly_equal_debug() {
    debug_assert_appro_eq!(8f32, 8f32 - 1e-5);
}

#[test]
#[cfg(debug_assertions)]
fn it_should_not_panic_if_values_are_nearly_equal_debug_abs() {
    debug_assert_appro_eq_abs!(8f32, 8f32 + 1e-7);
}

#[test]
#[cfg(debug_assertions)]
fn it_should_not_panic_if_values_are_nearly_equal_f64_debug_abs() {
    debug_assert_appro_eq_abs!(0f64, 1e-12 as f64);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic]
fn it_should_panic_if_values_are_not_nearly_equal_debug_abs() {
    debug_assert_appro_eq_abs!(8f32, 8f32 - 1e-5);
}

#[test]
#[cfg(debug_assertions)]
fn compare_with_explicit_eps_debug_abs() {
    debug_assert_appro_eq_abs!(3f64, 4f64, 2f64);
}

#[test]
#[cfg(not(debug_assertions))]
fn it_should_panic_if_values_are_not_nearly_equal_debug_abs() {
    debug_assert_appro_eq_abs!(8f32, 8f32 - 1e-5);
}

#[test]
#[cfg(debug_assertions)]
fn it_should_not_panic_if_values_are_nearly_equal_debug_rel() {
    debug_assert_appro_eq_rel!(8f32, 8f32 + 1e-7);
}

#[test]
#[cfg(debug_assertions)]
fn it_should_not_panic_if_values_are_nearly_equal_f64_debug_rel() {
    debug_assert_appro_eq_rel!(1f64, 1.0 + 1e-12 as f64);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic]
fn it_should_panic_if_values_are_not_nearly_equal_debug_rel() {
    debug_assert_appro_eq_rel!(8f32, 8f32 - 1e-5);
}

#[test]
#[cfg(debug_assertions)]
fn compare_with_explicit_eps_debug_rel() {
    debug_assert_appro_eq_rel!(3f64, 4f64, 2f64);
}

#[test]
#[cfg(not(debug_assertions))]
fn it_should_panic_if_values_are_not_nearly_equal_debug_rel() {
    debug_assert_appro_eq_rel!(8f32, 8f32 - 1e-3);
}

#[test]
#[cfg(feature = "ndarray")]
fn compare_with_ndarray1d() {
    let left = arr1(&[1f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]);
    let right = arr1(&[1f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]);
    assert_appro_eq!(left, right);
}

#[test]
#[should_panic]
#[cfg(feature = "ndarray")]
fn bad_compare_with_ndarray1d() {
    let left = arr1(&[1f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0001]);
    let right = arr1(&[1f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]);
    assert_appro_eq!(left, right);
}


#[test]
#[cfg(feature = "ndarray")]
fn compare_with_ndarray2d() {
    let left = arr2(&[[1f64, 2.0, 3.0, 4.0, 5.0], [6.0, 7.0, 8.0, 9.0, 10.0]]);
    let right = arr2(&[[1f64, 2.0, 3.0, 4.0, 5.0], [6.0, 7.0, 8.0, 9.0, 10.0]]);
    assert_appro_eq!(left, right);
}

#[test]
#[should_panic]
#[cfg(feature = "ndarray")]
fn bad_compare_with_ndarray2d_val() {
    let left = arr2(&[[1f64, 2.0, 3.0, 4.0, 5.0], [6.0, 7.0, 8.0, 9.0, 10.0001]]);
    let right = arr2(&[[1f64, 2.0, 3.0, 4.0, 5.0], [6.0, 7.0, 8.0, 9.0, 10.0]]);
    assert_appro_eq!(left, right);
}

#[test]
#[should_panic]
#[cfg(feature = "ndarray")]
fn bad_compare_with_ndarray2d_len() {
    let left = arr2(&[[1f64, 2.0, 3.0, 4.0, 5.0], [6.0, 7.0, 8.0, 9.0, 10.0]]);
    let right = arr2(&[
        [1f64, 2.0],
        [3.0, 4.0],
        [5.0, 6.0],
        [7.0, 8.0],
        [9.0, 10.0],
    ]);
    assert_appro_eq!(left, right);
}

#[test]
#[cfg(feature = "ndarray")]
fn compare_with_ndarray3d() {
    let left = arr3(&[[[1f64, 2.0], [4.0, 5.0]], [[6.0, 7.0], [9.0, 10.0]]]);
    let right = arr3(&[[[1f64, 2.0], [4.0, 5.0]], [[6.0, 7.0], [9.0, 10.0]]]);
    assert_appro_eq!(left, right);
}

#[test]
#[should_panic]
#[cfg(feature = "ndarray")]
fn bad_compare_with_ndarray3d() {
    let left = arr3(&[[[1f64, 2.0], [4.0, 5.0]], [[6.0, 7.0], [9.0, 10.0001]]]);
    let right = arr3(&[[[1f64, 2.0], [4.0, 5.0]], [[6.0, 7.0], [9.0, 10.0]]]);
    assert_appro_eq!(left, right);
}

#[test]
#[should_panic]
#[cfg(feature = "ndarray")]
fn bad_compare_with_ndarraynd() {
    let left = ArrayD::<f64>::zeros(IxDyn(&[2, 3, 4, 5, 6, 7]));
    let right = ArrayD::<f64>::zeros(IxDyn(&[2, 3, 4, 5, 6]));
    assert_appro_eq!(left, right);
}

#[test]
#[cfg(feature = "ndarray")]
fn compare_with_ndarray1d_abs() {
    let left = arr1(&[1f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]);
    let right = arr1(&[1f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]);
    assert_appro_eq_abs!(left, right);
}

#[test]
#[should_panic]
#[cfg(feature = "ndarray")]
fn bad_compare_with_ndarray1d_abs() {
    let left = arr1(&[1f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0001]);
    let right = arr1(&[1f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]);
    assert_appro_eq_abs!(left, right);
}


#[test]
#[cfg(feature = "ndarray")]
fn compare_with_ndarray2d_abs() {
    let left = arr2(&[[1f64, 2.0, 3.0, 4.0, 5.0], [6.0, 7.0, 8.0, 9.0, 10.0]]);
    let right = arr2(&[[1f64, 2.0, 3.0, 4.0, 5.0], [6.0, 7.0, 8.0, 9.0, 10.0]]);
    assert_appro_eq_abs!(left, right);
}

#[test]
#[should_panic]
#[cfg(feature = "ndarray")]
fn bad_compare_with_ndarray2d_val_abs() {
    let left = arr2(&[[1f64, 2.0, 3.0, 4.0, 5.0], [6.0, 7.0, 8.0, 9.0, 10.0001]]);
    let right = arr2(&[[1f64, 2.0, 3.0, 4.0, 5.0], [6.0, 7.0, 8.0, 9.0, 10.0]]);
    assert_appro_eq_abs!(left, right);
}

#[test]
#[should_panic]
#[cfg(feature = "ndarray")]
fn bad_compare_with_ndarray2d_len_abs() {
    let left = arr2(&[[1f64, 2.0, 3.0, 4.0, 5.0], [6.0, 7.0, 8.0, 9.0, 10.0]]);
    let right = arr2(&[
        [1f64, 2.0],
        [3.0, 4.0],
        [5.0, 6.0],
        [7.0, 8.0],
        [9.0, 10.0],
    ]);
    assert_appro_eq_abs!(left, right);
}

#[test]
#[cfg(feature = "ndarray")]
fn compare_with_ndarray3d_abs() {
    let left = arr3(&[[[1f64, 2.0], [4.0, 5.0]], [[6.0, 7.0], [9.0, 10.0]]]);
    let right = arr3(&[[[1f64, 2.0], [4.0, 5.0]], [[6.0, 7.0], [9.0, 10.0]]]);
    assert_appro_eq_abs!(left, right);
}

#[test]
#[should_panic]
#[cfg(feature = "ndarray")]
fn bad_compare_with_ndarray3d_abs() {
    let left = arr3(&[[[1f64, 2.0], [4.0, 5.0]], [[6.0, 7.0], [9.0, 10.0001]]]);
    let right = arr3(&[[[1f64, 2.0], [4.0, 5.0]], [[6.0, 7.0], [9.0, 10.0]]]);
    assert_appro_eq_abs!(left, right);
}

#[test]
#[should_panic]
#[cfg(feature = "ndarray")]
fn bad_compare_with_ndarraynd_abs() {
    let left = ArrayD::<f64>::zeros(IxDyn(&[2, 3, 4, 5, 6, 7]));
    let right = ArrayD::<f64>::zeros(IxDyn(&[2, 3, 4, 5, 6]));
    assert_appro_eq_abs!(left, right);
}

#[test]
#[cfg(feature = "ndarray")]
fn compare_with_ndarray1d_rel() {
    let left = arr1(&[1f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]);
    let right = arr1(&[1f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]);
    assert_appro_eq_rel!(left, right);
}

#[test]
#[should_panic]
#[cfg(feature = "ndarray")]
fn bad_compare_with_ndarray1d_rel() {
    let left = arr1(&[1f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0001]);
    let right = arr1(&[1f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]);
    assert_appro_eq_rel!(left, right);
}


#[test]
#[cfg(feature = "ndarray")]
fn compare_with_ndarray2d_rel() {
    let left = arr2(&[[1f64, 2.0, 3.0, 4.0, 5.0], [6.0, 7.0, 8.0, 9.0, 10.0]]);
    let right = arr2(&[[1f64, 2.0, 3.0, 4.0, 5.0], [6.0, 7.0, 8.0, 9.0, 10.0]]);
    assert_appro_eq_rel!(left, right);
}

#[test]
#[should_panic]
#[cfg(feature = "ndarray")]
fn bad_compare_with_ndarray2d_val_rel() {
    let left = arr2(&[[1f64, 2.0, 3.0, 4.0, 5.0], [6.0, 7.0, 8.0, 9.0, 10.0001]]);
    let right = arr2(&[[1f64, 2.0, 3.0, 4.0, 5.0], [6.0, 7.0, 8.0, 9.0, 10.0]]);
    assert_appro_eq_rel!(left, right);
}

#[test]
#[should_panic]
#[cfg(feature = "ndarray")]
fn bad_compare_with_ndarray2d_len_rel() {
    let left = arr2(&[[1f64, 2.0, 3.0, 4.0, 5.0], [6.0, 7.0, 8.0, 9.0, 10.0]]);
    let right = arr2(&[
        [1f64, 2.0],
        [3.0, 4.0],
        [5.0, 6.0],
        [7.0, 8.0],
        [9.0, 10.0],
    ]);
    assert_appro_eq_rel!(left, right);
}

#[test]
#[cfg(feature = "ndarray")]
fn compare_with_ndarray3d_rel() {
    let left = arr3(&[[[1f64, 2.0], [4.0, 5.0]], [[6.0, 7.0], [9.0, 10.0]]]);
    let right = arr3(&[[[1f64, 2.0], [4.0, 5.0]], [[6.0, 7.0], [9.0, 10.0]]]);
    assert_appro_eq_rel!(left, right);
}

#[test]
#[should_panic]
#[cfg(feature = "ndarray")]
fn bad_compare_with_ndarray3d_rel() {
    let left = arr3(&[[[1f64, 2.0], [4.0, 5.0]], [[6.0, 7.0], [9.0, 10.0001]]]);
    let right = arr3(&[[[1f64, 2.0], [4.0, 5.0]], [[6.0, 7.0], [9.0, 10.0]]]);
    assert_appro_eq_rel!(left, right);
}

#[test]
#[should_panic]
#[cfg(feature = "ndarray")]
fn bad_compare_with_ndarraynd_rel() {
    let left = ArrayD::<f64>::zeros(IxDyn(&[2, 3, 4, 5, 6, 7]));
    let right = ArrayD::<f64>::zeros(IxDyn(&[2, 3, 4, 5, 6]));
    assert_appro_eq_rel!(left, right);
}
