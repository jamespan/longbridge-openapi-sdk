use std::{cmp::Ordering, ffi::CStr, os::raw::c_char};

use rust_decimal::{prelude::ToPrimitive, Decimal, MathematicalOps};

use crate::types::ToFFI;

#[derive(Debug)]
pub struct CDecimal(pub(crate) Decimal);

impl From<Decimal> for CDecimal {
    #[inline]
    fn from(value: Decimal) -> Self {
        Self(value)
    }
}

impl ToFFI for CDecimal {
    type FFIType = *const CDecimal;

    #[inline]
    fn to_ffi_type(&self) -> Self::FFIType {
        self
    }
}

/// Create a decimal value with a 64 bit `m` representation and corresponding
/// `e` scale.
#[no_mangle]
pub extern "C" fn lb_decimal_new(num: i64, scale: u32) -> *mut CDecimal {
    Box::into_raw(Box::new(CDecimal(Decimal::new(num, scale))))
}

/// Clone the decimal value
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_clone(value: *const CDecimal) -> *mut CDecimal {
    Box::into_raw(Box::new(CDecimal((*value).0)))
}

/// Create a decimal value from string
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_from_str(value: *const c_char) -> *mut CDecimal {
    match CStr::from_ptr(value)
        .to_str()
        .ok()
        .and_then(|value| value.parse::<Decimal>().ok())
    {
        Some(value) => Box::into_raw(Box::new(CDecimal(value))),
        None => std::ptr::null_mut(),
    }
}

/// Create a decimal value from double
#[no_mangle]
pub extern "C" fn lb_decimal_from_double(value: f64) -> *mut CDecimal {
    match Decimal::from_f64_retain(value) {
        Some(value) => Box::into_raw(Box::new(CDecimal(value))),
        None => std::ptr::null_mut(),
    }
}

/// Free the decimal value
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_free(value: *mut CDecimal) {
    let _ = Box::from_raw(value);
}

#[no_mangle]
pub unsafe extern "C" fn lb_decimal_to_double(value: *const CDecimal) -> f64 {
    (*value).0.to_f64().unwrap_or_default()
}

/// Computes the absolute value.
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_abs(value: *mut CDecimal) {
    let new_value = (*value).0.abs();
    (*value).0 = new_value;
}

/// Returns the smallest integer greater than or equal to a number.
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_ceil(value: *mut CDecimal) {
    let new_value = (*value).0.ceil();
    (*value).0 = new_value;
}

/// Returns the largest integer less than or equal to a number.
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_floor(value: *mut CDecimal) {
    let new_value = (*value).0.floor();
    (*value).0 = new_value;
}

/// Returns a new Decimal representing the fractional portion of the number.
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_fract(value: *mut CDecimal) {
    let new_value = (*value).0.fract();
    (*value).0 = new_value;
}

/// Returns `true` if the decimal is negative.
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_is_negative(value: *const CDecimal) -> bool {
    (*value).0.is_sign_negative()
}

/// Returns `true` if the decimal is positive.
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_is_positive(value: *const CDecimal) -> bool {
    (*value).0.is_sign_positive()
}

/// Returns `true` if this Decimal number is equivalent to zero.
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_is_zero(value: *const CDecimal) -> bool {
    (*value).0.is_zero()
}

/// Returns the maximum of the two numbers.
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_max(a: *const CDecimal, b: *const CDecimal) -> *const CDecimal {
    if (*a).0 > (*b).0 {
        a
    } else {
        b
    }
}

/// Returns the minimum of the two numbers.
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_min(a: *const CDecimal, b: *const CDecimal) -> *const CDecimal {
    if (*a).0 < (*b).0 {
        a
    } else {
        b
    }
}

/// Strips any trailing zero’s from a Decimal and converts `-0` to `0`.
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_normalize(value: *mut CDecimal) {
    let new_value = (*value).0.normalize();
    (*value).0 = new_value;
}

/// Returns a new Decimal number with no fractional portion (i.e. an
/// integer). Rounding currently follows “Bankers Rounding” rules. e.g.
/// `6.5` -> `6`, `7.5` -> `8`
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_round(value: *mut CDecimal) {
    let new_value = (*value).0.round();
    (*value).0 = new_value;
}

/// Returns a new Decimal integral with no fractional portion. This is a
/// true truncation whereby no rounding is performed.
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_trunc(value: *mut CDecimal) {
    let new_value = (*value).0.trunc();
    (*value).0 = new_value;
}

/// Performs the `+` operation.
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_add(a: *mut CDecimal, b: *const CDecimal) {
    let new_value = (*a).0 + (*b).0;
    (*a).0 = new_value;
}

/// Performs the `-` operation.
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_sub(a: *mut CDecimal, b: *const CDecimal) {
    let new_value = (*a).0 - (*b).0;
    (*a).0 = new_value;
}

/// Performs the `*` operation.
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_mul(a: *mut CDecimal, b: *const CDecimal) {
    let new_value = (*a).0 * (*b).0;
    (*a).0 = new_value;
}

/// Performs the `/` operation.
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_div(a: *mut CDecimal, b: *const CDecimal) {
    let new_value = (*a).0 / (*b).0;
    (*a).0 = new_value;
}

/// Performs the `%` operation.
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_rem(a: *mut CDecimal, b: *const CDecimal) {
    let new_value = (*a).0 % (*b).0;
    (*a).0 = new_value;
}

/// Performs the unary `-` operation.
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_neg(value: *mut CDecimal) {
    let new_value = -(*value).0;
    (*value).0 = new_value;
}

/// Returns `true` if the value of this Decimal is greater than the value of
/// `x`, otherwise returns `false`.
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_gt(a: *const CDecimal, b: *const CDecimal) -> bool {
    (*a).0 > (*b).0
}

/// Returns `true` if the value of this Decimal is greater than or equal to
/// the value of `x`, otherwise returns `false`.
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_gte(a: *const CDecimal, b: *const CDecimal) -> bool {
    (*a).0 >= (*b).0
}

/// Returns `true` if the value of this Decimal equals the value of `x`,
/// otherwise returns `false`.
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_eq(a: *const CDecimal, b: *const CDecimal) -> bool {
    (*a).0 == (*b).0
}

/// Returns `true` if the value of this Decimal is less than the value of
/// `x`, otherwise returns `false`.
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_lt(a: *const CDecimal, b: *const CDecimal) -> bool {
    (*a).0 < (*b).0
}

/// Returns `true` if the value of this Decimal is less than or equal to the
/// value of `x`, otherwise returns `false`.
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_lte(a: *const CDecimal, b: *const CDecimal) -> bool {
    (*a).0 <= (*b).0
}

/// Compares the values of two Decimals.
///
/// Returns `-1` if the value of this Decimal is less than the value of
/// `x`.
///
/// Returns `1` if the value of this Decimal is greater than the value of
/// `x`.
///
/// Returns `0` if the value of this Decimal equals the value of `x`.
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_cmp(a: *const CDecimal, b: *const CDecimal) -> i32 {
    match (*a).0.cmp(&(*b).0) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

/// Computes the sine of a number (in radians)
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_sin(value: *mut CDecimal) {
    let new_value = (*value).0.checked_sin().expect("overflowed");
    (*value).0 = new_value;
}

/// Computes the cosine of a number (in radians)
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_cos(value: *mut CDecimal) {
    let new_value = (*value).0.checked_cos().expect("overflowed");
    (*value).0 = new_value;
}

/// Computes the tangent of a number (in radians). Panics upon overflow or
/// upon approaching a limit.
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_tan(value: *mut CDecimal) {
    let new_value = (*value).0.checked_tan().expect("overflowed");
    (*value).0 = new_value;
}

/// The square root of a Decimal. Uses a standard Babylonian method.
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_sqrt(value: *mut CDecimal) {
    let new_value = (*value).0.sqrt().expect("overflowed");
    (*value).0 = new_value;
}

/// Raise self to the given Decimal exponent: x<sup>y</sup>. If `exp` is not
/// whole then the approximation e<sup>y*ln(x)</sup> is used.
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_pow(value: *mut CDecimal, exp: *const CDecimal) {
    let new_value = (*value).0.checked_powd((*exp).0).expect("overflowed");
    (*value).0 = new_value;
}

/// Calculates the natural logarithm for a Decimal calculated using Taylor’s
/// series.
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_ln(value: *mut CDecimal) {
    let new_value = (*value).0.checked_ln().expect("overflowed");
    (*value).0 = new_value;
}

/// Calculates the base 10 logarithm of a specified Decimal number.
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_log10(value: *mut CDecimal) {
    let new_value = (*value).0.checked_log10().expect("overflowed");
    (*value).0 = new_value;
}

/// The estimated exponential function, ex. Stops calculating when it is
/// within tolerance of roughly `0.0000002`.
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_exp(value: *mut CDecimal) {
    let new_value = (*value).0.checked_exp().expect("overflowed");
    (*value).0 = new_value;
}

/// The estimated exponential function, e<sup>x</sup> using the `tolerance`
/// provided as a hint as to when to stop calculating. A larger
/// tolerance will cause the number to stop calculating sooner at the
/// potential cost of a slightly less accurate result.
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_exp_with_tolerance(
    value: *mut CDecimal,
    tolerance: *const CDecimal,
) {
    let new_value = (*value)
        .0
        .checked_exp_with_tolerance((*tolerance).0)
        .expect("overflowed");
    (*value).0 = new_value;
}

/// Abramowitz Approximation of Error Function from [wikipedia](https://en.wikipedia.org/wiki/Error_function#Numerical_approximations)
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_erf(value: *mut CDecimal) {
    let new_value = (*value).0.erf();
    (*value).0 = new_value;
}

/// The Cumulative distribution function for a Normal distribution
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_normal_cdf(value: *mut CDecimal) {
    let new_value = (*value).0.norm_cdf();
    (*value).0 = new_value;
}

/// The Probability density function for a Normal distribution.
#[no_mangle]
pub unsafe extern "C" fn lb_decimal_norm_pdf(value: *mut CDecimal) {
    let new_value = (*value).0.norm_pdf();
    (*value).0 = new_value;
}
