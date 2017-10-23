//! # Licensing
//! This Source Code is subject to the terms of the Mozilla Public License
//! version 2.0 (the "License"). You can obtain a copy of the License at
//! http://mozilla.org/MPL/2.0/.

use num_complex::Complex;
use num_traits::Float;
use AbsError;
use RelError;
use ApproEqResult;
use std::ops::{Div, Sub};

#[cfg_attr(feature = "docs", stable(feature = "num-complex", since = "0.1.0"))]
impl<A, D: Float, B: AbsError<A, D>> AbsError<Complex<A>, D> for Complex<B> {
    fn abs_error(&self, expected: &Complex<A>) -> ApproEqResult<D> {
        let diff_re = self.re.abs_error(&expected.re);
        let diff_im = self.im.abs_error(&expected.im);
        if match (&diff_re, &diff_im) {
            (&Err(_), _) => false,
            (_, &Err(_)) => true,
            (&Ok(ref diff_re), &Ok(ref diff_im)) => match (diff_re, diff_im) {
                (&None, _) => true,
                (_, &None) => false,
                (&Some(ref diff_re), &Some(ref diff_im)) => {
                    return Ok(Some(
                        Float::sqrt((*diff_re * *diff_re) + (*diff_im * *diff_im)),
                    ))
                }
            },
        } {
            diff_im
        } else {
            diff_re
        }
    }
}

#[cfg_attr(feature = "docs", stable(feature = "num-complex", since = "0.1.0"))]
impl<A: Float, D: Float + Div<A, Output = D>, B: Sub<A, Output = D> + Copy> RelError<Complex<A>, D>
    for Complex<B> {
    fn rel_error(&self, expected: &Complex<A>) -> ApproEqResult<D> {
        Ok(Some(
            Complex::new(self.re - expected.re, self.im - expected.im).norm() / expected.norm(),
        ))
    }
}
