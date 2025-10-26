use crate::registry::types::Outcome;
use core::fmt::Debug;

pub fn assert_eq<T: PartialEq + Debug>(got: T, expected: T) -> Outcome {
    if got == expected {
        Outcome::Pass
    } else {
        Outcome::Fail("not equal")
    }
}
pub fn assert_ne<T: PartialEq + Debug>(got: T, expected: T) -> Outcome {
    if got != expected {
        Outcome::Pass
    } else {
        Outcome::Fail("unexpected equality")
    }
}

pub fn assert_true(got: bool) -> Outcome {
    if got {
        Outcome::Pass
    } else {
        Outcome::Fail("not true")
    }
}

pub fn assert_false(got: bool) -> Outcome {
    if !got {
        Outcome::Pass
    } else {
        Outcome::Fail("unexpected true")
    }
}

// takes in a closure and equates the value obtained from closure to expected value
pub fn assert_eq_with<F, T>(f: F, expected: T) -> Outcome
where
    F: FnOnce() -> T,
    T: PartialEq + Debug,
{
    assert_eq(f(), expected)
}

pub fn assert_ok<T, E>(_got: Result<T, E>) -> Outcome {
    if _got.is_ok() {
        Outcome::Pass
    } else {
        Outcome::Fail("not ok")
    }
}

pub fn assert_err<T, E>(_got: Result<T, E>) -> Outcome {
    if _got.is_err() {
        Outcome::Pass
    } else {
        Outcome::Fail("not err")
    }
}

pub fn assert_some<T>(_got: Option<T>) -> Outcome {
    if _got.is_some() {
        Outcome::Pass
    } else {
        Outcome::Fail("not some")
    }
}

pub fn assert_none<T>(_got: Option<T>) -> Outcome {
    if _got.is_none() {
        Outcome::Pass
    } else {
        Outcome::Fail("not none")
    }
}
