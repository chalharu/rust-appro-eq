//! # Licensing
//! This Source Code is subject to the terms of the Mozilla Public License
//! version 2.0 (the "License"). You can obtain a copy of the License at
//! http://mozilla.org/MPL/2.0/.

use crate::AbsError;
use crate::ApproEqError;
use crate::ApproEqResult;
use crate::RelError;
use crate::Tolerance;
use num_integer::Integer;
use num_rational::Ratio;
use num_traits::identities::Zero;

#[cfg_attr(feature = "docs", stable(feature = "num-rational", since = "0.1.0"))]
impl<A: Integer + Clone> Tolerance for Ratio<A> {
    fn tolerance() -> Ratio<A> {
        Ratio::zero()
    }
}

#[cfg_attr(feature = "docs", stable(feature = "num-rational", since = "0.1.0"))]
impl<A: Integer + Clone> AbsError<Ratio<A>, Ratio<A>> for Ratio<A> {
    fn abs_error(&self, expected: &Ratio<A>) -> ApproEqResult<Ratio<A>> {
        Ok(Some(if *self > *expected {
            self - expected
        } else {
            expected - self
        }))
    }
}

#[cfg_attr(feature = "docs", stable(feature = "num-rational", since = "0.1.0"))]
impl<A: Integer + Clone> RelError<Ratio<A>, Ratio<A>> for Ratio<A> {
    fn rel_error(&self, expected: &Ratio<A>) -> ApproEqResult<Ratio<A>> {
        if *expected == Ratio::zero() {
            Err(ApproEqError::DividedByZero)
        } else {
            Ok(Some(
                (if *self > *expected {
                    self - expected
                } else {
                    expected - self
                }) / expected,
            ))
        }
    }
}
