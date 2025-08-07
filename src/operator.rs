//! Operator overloads for VipsImage
//!
//! `+`, `-`, `*`, `/`
//!
//! `lt`(<), `le`(<=), `gt`(>), `ge`(>=), and `at`([])
//!
//! Every overload returns VipsImage as the result of Vips operation.
use crate::{
    ops::{OperationBoolean, OperationRelational},
    VipsImage,
};
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Rem, Shl, Shr, Sub};

pub trait Index<Idx> {
    type Output: ?Sized;
    fn at(&self, index: Idx) -> Self::Output;
}

fn invert(vector: &[f64]) -> Vec<f64> {
    let mut new_vector = Vec::with_capacity(vector.len());

    for value in vector {
        new_vector.push(1.0 / value);
    }

    new_vector
}

fn negate(vector: &[f64]) -> Vec<f64> {
    let mut new_vector = Vec::with_capacity(vector.len());

    for value in vector {
        new_vector.push(value * -1.0);
    }

    new_vector
}

// index
impl Index<i32> for VipsImage {
    type Output = VipsImage;
    fn at(&self, index: i32) -> Self::Output {
        self.extract_band(index)
            .unwrap()
    }
}

// add
impl Add for VipsImage {
    type Output = VipsImage;
    fn add(self, b: VipsImage) -> VipsImage {
        self.add_image(&b)
            .unwrap()
    }
}

impl Add<VipsImage> for f64 {
    type Output = VipsImage;
    fn add(self, b: VipsImage) -> VipsImage {
        b.linear(
            &[1.0],
            &[self],
        )
        .unwrap()
    }
}

impl Add<f64> for VipsImage {
    type Output = VipsImage;
    fn add(self, b: f64) -> VipsImage {
        self.linear(
            &[1.0],
            &[b],
        )
        .unwrap()
    }
}

impl Add<VipsImage> for &[f64] {
    type Output = VipsImage;
    fn add(self, b: VipsImage) -> VipsImage {
        b.linear(
            &[1.0],
            self,
        )
        .unwrap()
    }
}

impl Add<&[f64]> for VipsImage {
    type Output = VipsImage;
    fn add(self, b: &[f64]) -> VipsImage {
        self.linear(&[1.0], b)
            .unwrap()
    }
}

impl<const N: usize> Add<VipsImage> for &[f64; N] {
    type Output = VipsImage;
    fn add(self, b: VipsImage) -> VipsImage {
        b.linear(
            &[1.0],
            self,
        )
        .unwrap()
    }
}

impl<const N: usize> Add<&[f64; N]> for VipsImage {
    type Output = VipsImage;
    fn add(self, b: &[f64; N]) -> VipsImage {
        self.linear(&[1.0], b)
            .unwrap()
    }
}

// add ref
impl Add for &VipsImage {
    type Output = VipsImage;
    fn add(self, b: &VipsImage) -> VipsImage {
        self.add_image(b)
            .unwrap()
    }
}

impl Add<&VipsImage> for f64 {
    type Output = VipsImage;
    fn add(self, b: &VipsImage) -> VipsImage {
        b.linear(
            &[1.0],
            &[self],
        )
        .unwrap()
    }
}

impl Add<f64> for &VipsImage {
    type Output = VipsImage;
    fn add(self, b: f64) -> VipsImage {
        self.linear(
            &[1.0],
            &[b],
        )
        .unwrap()
    }
}

impl Add<&VipsImage> for &[f64] {
    type Output = VipsImage;
    fn add(self, b: &VipsImage) -> VipsImage {
        b.linear(
            &[1.0],
            self,
        )
        .unwrap()
    }
}

impl Add<&[f64]> for &VipsImage {
    type Output = VipsImage;
    fn add(self, b: &[f64]) -> VipsImage {
        self.linear(&[1.0], b)
            .unwrap()
    }
}

impl<const N: usize> Add<&VipsImage> for &[f64; N] {
    type Output = VipsImage;
    fn add(self, b: &VipsImage) -> VipsImage {
        b.linear(
            &[1.0],
            self,
        )
        .unwrap()
    }
}

impl<const N: usize> Add<&[f64; N]> for &VipsImage {
    type Output = VipsImage;
    fn add(self, b: &[f64; N]) -> VipsImage {
        self.linear(&[1.0], b)
            .unwrap()
    }
}

// sub
impl Sub for VipsImage {
    type Output = VipsImage;
    fn sub(self, b: VipsImage) -> Self::Output {
        self.subtract(&b)
            .unwrap()
    }
}

impl Sub<VipsImage> for f64 {
    type Output = VipsImage;
    fn sub(self, b: VipsImage) -> Self::Output {
        b.linear(
            &[-1.0],
            &[self],
        )
        .unwrap()
    }
}

impl Sub<f64> for VipsImage {
    type Output = VipsImage;
    fn sub(self, b: f64) -> Self::Output {
        self.linear(
            &[1.0],
            &[b],
        )
        .unwrap()
    }
}

impl Sub<VipsImage> for &[f64] {
    type Output = VipsImage;
    fn sub(self, b: VipsImage) -> VipsImage {
        b.linear(
            &[-1.0],
            self,
        )
        .unwrap()
    }
}

impl Sub<&[f64]> for VipsImage {
    type Output = VipsImage;
    fn sub(self, b: &[f64]) -> VipsImage {
        self.linear(
            &[1.0],
            &negate(b),
        )
        .unwrap()
    }
}

impl<const N: usize> Sub<VipsImage> for &[f64; N] {
    type Output = VipsImage;
    fn sub(self, b: VipsImage) -> VipsImage {
        b.linear(
            &[-1.0],
            self,
        )
        .unwrap()
    }
}

impl<const N: usize> Sub<&[f64; N]> for VipsImage {
    type Output = VipsImage;
    fn sub(self, b: &[f64; N]) -> VipsImage {
        self.linear(
            &[1.0],
            &negate(b),
        )
        .unwrap()
    }
}

// sub ref
impl Sub for &VipsImage {
    type Output = VipsImage;
    fn sub(self, b: &VipsImage) -> Self::Output {
        self.subtract(b)
            .unwrap()
    }
}

impl Sub<&VipsImage> for f64 {
    type Output = VipsImage;
    fn sub(self, b: &VipsImage) -> Self::Output {
        b.linear(
            &[-1.0],
            &[self],
        )
        .unwrap()
    }
}

impl Sub<f64> for &VipsImage {
    type Output = VipsImage;
    fn sub(self, b: f64) -> Self::Output {
        self.linear(
            &[1.0],
            &[b],
        )
        .unwrap()
    }
}

impl Sub<&VipsImage> for &[f64] {
    type Output = VipsImage;
    fn sub(self, b: &VipsImage) -> VipsImage {
        b.linear(
            &[-1.0],
            self,
        )
        .unwrap()
    }
}

impl Sub<&[f64]> for &VipsImage {
    type Output = VipsImage;
    fn sub(self, b: &[f64]) -> VipsImage {
        self.linear(
            &[1.0],
            &negate(b),
        )
        .unwrap()
    }
}

impl<const N: usize> Sub<&VipsImage> for &[f64; N] {
    type Output = VipsImage;
    fn sub(self, b: &VipsImage) -> VipsImage {
        b.linear(
            &[-1.0],
            self,
        )
        .unwrap()
    }
}

impl<const N: usize> Sub<&[f64; N]> for &VipsImage {
    type Output = VipsImage;
    fn sub(self, b: &[f64; N]) -> VipsImage {
        self.linear(
            &[1.0],
            &negate(b),
        )
        .unwrap()
    }
}

// multiply
impl Mul for VipsImage {
    type Output = VipsImage;
    fn mul(self, b: VipsImage) -> VipsImage {
        self.multiply(&b)
            .unwrap()
    }
}

impl Mul<VipsImage> for f64 {
    type Output = VipsImage;
    fn mul(self, b: VipsImage) -> VipsImage {
        b.linear(
            &[self],
            &[0.0],
        )
        .unwrap()
    }
}

impl Mul<f64> for VipsImage {
    type Output = VipsImage;
    fn mul(self, b: f64) -> VipsImage {
        self.linear(
            &[b],
            &[0.0],
        )
        .unwrap()
    }
}

impl Mul<VipsImage> for &[f64] {
    type Output = VipsImage;
    fn mul(self, b: VipsImage) -> VipsImage {
        b.linear(
            self,
            &[0.0],
        )
        .unwrap()
    }
}

impl Mul<&[f64]> for VipsImage {
    type Output = VipsImage;
    fn mul(self, b: &[f64]) -> VipsImage {
        self.linear(b, &[0.0])
            .unwrap()
    }
}

impl<const N: usize> Mul<VipsImage> for &[f64; N] {
    type Output = VipsImage;
    fn mul(self, b: VipsImage) -> VipsImage {
        b.linear(
            self,
            &[0.0],
        )
        .unwrap()
    }
}

impl<const N: usize> Mul<&[f64; N]> for VipsImage {
    type Output = VipsImage;
    fn mul(self, b: &[f64; N]) -> VipsImage {
        self.linear(b, &[0.0])
            .unwrap()
    }
}

// multiply ref
impl Mul for &VipsImage {
    type Output = VipsImage;
    fn mul(self, b: &VipsImage) -> VipsImage {
        self.multiply(b)
            .unwrap()
    }
}

impl Mul<&VipsImage> for f64 {
    type Output = VipsImage;
    fn mul(self, b: &VipsImage) -> VipsImage {
        b.linear(
            &[self],
            &[0.0],
        )
        .unwrap()
    }
}

impl Mul<f64> for &VipsImage {
    type Output = VipsImage;
    fn mul(self, b: f64) -> VipsImage {
        self.linear(
            &[b],
            &[0.0],
        )
        .unwrap()
    }
}

impl Mul<&VipsImage> for &[f64] {
    type Output = VipsImage;
    fn mul(self, b: &VipsImage) -> VipsImage {
        b.linear(
            self,
            &[0.0],
        )
        .unwrap()
    }
}

impl Mul<&[f64]> for &VipsImage {
    type Output = VipsImage;
    fn mul(self, b: &[f64]) -> VipsImage {
        self.linear(b, &[0.0])
            .unwrap()
    }
}

impl<const N: usize> Mul<&VipsImage> for &[f64; N] {
    type Output = VipsImage;
    fn mul(self, b: &VipsImage) -> VipsImage {
        b.linear(
            self,
            &[0.0],
        )
        .unwrap()
    }
}

impl<const N: usize> Mul<&[f64; N]> for &VipsImage {
    type Output = VipsImage;
    fn mul(self, b: &[f64; N]) -> VipsImage {
        self.linear(b, &[0.0])
            .unwrap()
    }
}

// div
impl Div for VipsImage {
    type Output = VipsImage;
    fn div(self, b: VipsImage) -> VipsImage {
        self.divide(&b)
            .unwrap()
    }
}

impl Div<VipsImage> for f64 {
    type Output = VipsImage;
    fn div(self, b: VipsImage) -> VipsImage {
        b.boolean_const(
            OperationBoolean::Eor,
            &[-1.0],
        )
        .unwrap()
        .linear(
            &[self],
            &[0.0],
        )
        .unwrap()
    }
}

impl Div<f64> for VipsImage {
    type Output = VipsImage;
    fn div(self, b: f64) -> VipsImage {
        self.linear(
            &[1.0 / b],
            &[0.0],
        )
        .unwrap()
    }
}

impl Div<VipsImage> for &[f64] {
    type Output = VipsImage;
    fn div(self, b: VipsImage) -> VipsImage {
        b.boolean_const(
            OperationBoolean::Eor,
            &[-1.0],
        )
        .unwrap()
        .linear(
            self,
            &[0.0],
        )
        .unwrap()
    }
}

impl Div<&[f64]> for VipsImage {
    type Output = VipsImage;
    fn div(self, b: &[f64]) -> VipsImage {
        self.linear(
            &invert(b),
            &[0.0],
        )
        .unwrap()
    }
}

impl<const N: usize> Div<VipsImage> for &[f64; N] {
    type Output = VipsImage;
    fn div(self, b: VipsImage) -> VipsImage {
        b.boolean_const(
            OperationBoolean::Eor,
            &[-1.0],
        )
        .unwrap()
        .linear(
            self,
            &[0.0],
        )
        .unwrap()
    }
}

impl<const N: usize> Div<&[f64; N]> for VipsImage {
    type Output = VipsImage;
    fn div(self, b: &[f64; N]) -> VipsImage {
        self.linear(
            &invert(b),
            &[0.0],
        )
        .unwrap()
    }
}

// div ref
impl Div for &VipsImage {
    type Output = VipsImage;
    fn div(self, b: &VipsImage) -> VipsImage {
        self.divide(b)
            .unwrap()
    }
}

impl Div<&VipsImage> for f64 {
    type Output = VipsImage;
    fn div(self, b: &VipsImage) -> VipsImage {
        b.boolean_const(
            OperationBoolean::Eor,
            &[-1.0],
        )
        .unwrap()
        .linear(
            &[self],
            &[0.0],
        )
        .unwrap()
    }
}

impl Div<f64> for &VipsImage {
    type Output = VipsImage;
    fn div(self, b: f64) -> VipsImage {
        self.linear(
            &[1.0 / b],
            &[0.0],
        )
        .unwrap()
    }
}

impl Div<&VipsImage> for &[f64] {
    type Output = VipsImage;
    fn div(self, b: &VipsImage) -> VipsImage {
        b.boolean_const(
            OperationBoolean::Eor,
            &[-1.0],
        )
        .unwrap()
        .linear(
            self,
            &[0.0],
        )
        .unwrap()
    }
}

impl Div<&[f64]> for &VipsImage {
    type Output = VipsImage;
    fn div(self, b: &[f64]) -> VipsImage {
        self.linear(
            &invert(b),
            &[0.0],
        )
        .unwrap()
    }
}

impl<const N: usize> Div<&VipsImage> for &[f64; N] {
    type Output = VipsImage;
    fn div(self, b: &VipsImage) -> VipsImage {
        b.boolean_const(
            OperationBoolean::Eor,
            &[-1.0],
        )
        .unwrap()
        .linear(
            self,
            &[0.0],
        )
        .unwrap()
    }
}

impl<const N: usize> Div<&[f64; N]> for &VipsImage {
    type Output = VipsImage;
    fn div(self, b: &[f64; N]) -> VipsImage {
        self.linear(
            &invert(b),
            &[0.0],
        )
        .unwrap()
    }
}

// rem
impl Rem for VipsImage {
    type Output = VipsImage;
    fn rem(self, b: VipsImage) -> VipsImage {
        self.remainder(&b)
            .unwrap()
    }
}

impl Rem<f64> for VipsImage {
    type Output = VipsImage;
    fn rem(self, b: f64) -> VipsImage {
        self.remainder_const(&[b])
            .unwrap()
    }
}

impl Rem<&[f64]> for VipsImage {
    type Output = VipsImage;
    fn rem(self, b: &[f64]) -> VipsImage {
        self.remainder_const(b)
            .unwrap()
    }
}

impl<const N: usize> Rem<&[f64; N]> for VipsImage {
    type Output = VipsImage;
    fn rem(self, b: &[f64; N]) -> VipsImage {
        self.remainder_const(b)
            .unwrap()
    }
}

// BitAnd
impl BitAnd for VipsImage {
    type Output = VipsImage;
    fn bitand(self, b: VipsImage) -> VipsImage {
        self.boolean(
            &b,
            OperationBoolean::And,
        )
        .unwrap()
    }
}

impl BitAnd<VipsImage> for f64 {
    type Output = VipsImage;
    fn bitand(self, b: VipsImage) -> VipsImage {
        b.boolean_const(
            OperationBoolean::And,
            &[self],
        )
        .unwrap()
    }
}

impl BitAnd<f64> for VipsImage {
    type Output = VipsImage;
    fn bitand(self, b: f64) -> VipsImage {
        self.boolean_const(
            OperationBoolean::And,
            &[b],
        )
        .unwrap()
    }
}

impl BitAnd<VipsImage> for &[f64] {
    type Output = VipsImage;
    fn bitand(self, b: VipsImage) -> VipsImage {
        b.boolean_const(
            OperationBoolean::And,
            self,
        )
        .unwrap()
    }
}

impl BitAnd<&[f64]> for VipsImage {
    type Output = VipsImage;
    fn bitand(self, b: &[f64]) -> VipsImage {
        self.boolean_const(
            OperationBoolean::And,
            b,
        )
        .unwrap()
    }
}

impl<const N: usize> BitAnd<VipsImage> for &[f64; N] {
    type Output = VipsImage;
    fn bitand(self, b: VipsImage) -> VipsImage {
        b.boolean_const(
            OperationBoolean::And,
            self,
        )
        .unwrap()
    }
}

impl<const N: usize> BitAnd<&[f64; N]> for VipsImage {
    type Output = VipsImage;
    fn bitand(self, b: &[f64; N]) -> VipsImage {
        self.boolean_const(
            OperationBoolean::And,
            b,
        )
        .unwrap()
    }
}

// BitAnd ref
impl BitAnd for &VipsImage {
    type Output = VipsImage;
    fn bitand(self, b: &VipsImage) -> VipsImage {
        self.boolean(
            b,
            OperationBoolean::And,
        )
        .unwrap()
    }
}

impl BitAnd<&VipsImage> for f64 {
    type Output = VipsImage;
    fn bitand(self, b: &VipsImage) -> VipsImage {
        b.boolean_const(
            OperationBoolean::And,
            &[self],
        )
        .unwrap()
    }
}

impl BitAnd<f64> for &VipsImage {
    type Output = VipsImage;
    fn bitand(self, b: f64) -> VipsImage {
        self.boolean_const(
            OperationBoolean::And,
            &[b],
        )
        .unwrap()
    }
}

impl BitAnd<&VipsImage> for &[f64] {
    type Output = VipsImage;
    fn bitand(self, b: &VipsImage) -> VipsImage {
        b.boolean_const(
            OperationBoolean::And,
            self,
        )
        .unwrap()
    }
}

impl BitAnd<&[f64]> for &VipsImage {
    type Output = VipsImage;
    fn bitand(self, b: &[f64]) -> VipsImage {
        self.boolean_const(
            OperationBoolean::And,
            b,
        )
        .unwrap()
    }
}

impl<const N: usize> BitAnd<&VipsImage> for &[f64; N] {
    type Output = VipsImage;
    fn bitand(self, b: &VipsImage) -> VipsImage {
        b.boolean_const(
            OperationBoolean::And,
            self,
        )
        .unwrap()
    }
}

impl<const N: usize> BitAnd<&[f64; N]> for &VipsImage {
    type Output = VipsImage;
    fn bitand(self, b: &[f64; N]) -> VipsImage {
        self.boolean_const(
            OperationBoolean::And,
            b,
        )
        .unwrap()
    }
}

// BitOr
impl BitOr for VipsImage {
    type Output = VipsImage;
    fn bitor(self, b: VipsImage) -> VipsImage {
        self.boolean(
            &b,
            OperationBoolean::Or,
        )
        .unwrap()
    }
}

impl BitOr<VipsImage> for f64 {
    type Output = VipsImage;
    fn bitor(self, b: VipsImage) -> VipsImage {
        b.boolean_const(
            OperationBoolean::Or,
            &[self],
        )
        .unwrap()
    }
}

impl BitOr<f64> for VipsImage {
    type Output = VipsImage;
    fn bitor(self, b: f64) -> VipsImage {
        self.boolean_const(
            OperationBoolean::Or,
            &[b],
        )
        .unwrap()
    }
}

impl BitOr<VipsImage> for &[f64] {
    type Output = VipsImage;
    fn bitor(self, b: VipsImage) -> VipsImage {
        b.boolean_const(
            OperationBoolean::Or,
            self,
        )
        .unwrap()
    }
}

impl BitOr<&[f64]> for VipsImage {
    type Output = VipsImage;
    fn bitor(self, b: &[f64]) -> VipsImage {
        self.boolean_const(
            OperationBoolean::Or,
            b,
        )
        .unwrap()
    }
}

impl<const N: usize> BitOr<VipsImage> for &[f64; N] {
    type Output = VipsImage;
    fn bitor(self, b: VipsImage) -> VipsImage {
        b.boolean_const(
            OperationBoolean::Or,
            self,
        )
        .unwrap()
    }
}

impl<const N: usize> BitOr<&[f64; N]> for VipsImage {
    type Output = VipsImage;
    fn bitor(self, b: &[f64; N]) -> VipsImage {
        self.boolean_const(
            OperationBoolean::Or,
            b,
        )
        .unwrap()
    }
}

// BitOr ref
impl BitOr for &VipsImage {
    type Output = VipsImage;
    fn bitor(self, b: &VipsImage) -> VipsImage {
        self.boolean(
            b,
            OperationBoolean::Or,
        )
        .unwrap()
    }
}

impl BitOr<&VipsImage> for f64 {
    type Output = VipsImage;
    fn bitor(self, b: &VipsImage) -> VipsImage {
        b.boolean_const(
            OperationBoolean::Or,
            &[self],
        )
        .unwrap()
    }
}

impl BitOr<f64> for &VipsImage {
    type Output = VipsImage;
    fn bitor(self, b: f64) -> VipsImage {
        self.boolean_const(
            OperationBoolean::Or,
            &[b],
        )
        .unwrap()
    }
}

impl BitOr<&VipsImage> for &[f64] {
    type Output = VipsImage;
    fn bitor(self, b: &VipsImage) -> VipsImage {
        b.boolean_const(
            OperationBoolean::Or,
            self,
        )
        .unwrap()
    }
}

impl BitOr<&[f64]> for &VipsImage {
    type Output = VipsImage;
    fn bitor(self, b: &[f64]) -> VipsImage {
        self.boolean_const(
            OperationBoolean::Or,
            b,
        )
        .unwrap()
    }
}

impl<const N: usize> BitOr<&VipsImage> for &[f64; N] {
    type Output = VipsImage;
    fn bitor(self, b: &VipsImage) -> VipsImage {
        b.boolean_const(
            OperationBoolean::Or,
            self,
        )
        .unwrap()
    }
}

impl<const N: usize> BitOr<&[f64; N]> for &VipsImage {
    type Output = VipsImage;
    fn bitor(self, b: &[f64; N]) -> VipsImage {
        self.boolean_const(
            OperationBoolean::Or,
            b,
        )
        .unwrap()
    }
}

// BitXor
impl BitXor for VipsImage {
    type Output = VipsImage;
    fn bitxor(self, b: VipsImage) -> VipsImage {
        self.boolean(
            &b,
            OperationBoolean::Eor,
        )
        .unwrap()
    }
}

impl BitXor<VipsImage> for f64 {
    type Output = VipsImage;
    fn bitxor(self, b: VipsImage) -> VipsImage {
        b.boolean_const(
            OperationBoolean::Eor,
            &[self],
        )
        .unwrap()
    }
}

impl BitXor<f64> for VipsImage {
    type Output = VipsImage;
    fn bitxor(self, b: f64) -> VipsImage {
        self.boolean_const(
            OperationBoolean::Eor,
            &[b],
        )
        .unwrap()
    }
}

impl BitXor<VipsImage> for &[f64] {
    type Output = VipsImage;
    fn bitxor(self, b: VipsImage) -> VipsImage {
        b.boolean_const(
            OperationBoolean::Eor,
            self,
        )
        .unwrap()
    }
}

impl BitXor<&[f64]> for VipsImage {
    type Output = VipsImage;
    fn bitxor(self, b: &[f64]) -> VipsImage {
        self.boolean_const(
            OperationBoolean::Eor,
            b,
        )
        .unwrap()
    }
}

impl<const N: usize> BitXor<VipsImage> for &[f64; N] {
    type Output = VipsImage;
    fn bitxor(self, b: VipsImage) -> VipsImage {
        b.boolean_const(
            OperationBoolean::Eor,
            self,
        )
        .unwrap()
    }
}

impl<const N: usize> BitXor<&[f64; N]> for VipsImage {
    type Output = VipsImage;
    fn bitxor(self, b: &[f64; N]) -> VipsImage {
        self.boolean_const(
            OperationBoolean::Eor,
            b,
        )
        .unwrap()
    }
}

// BitXor ref
impl BitXor for &VipsImage {
    type Output = VipsImage;
    fn bitxor(self, b: &VipsImage) -> VipsImage {
        self.boolean(
            b,
            OperationBoolean::Eor,
        )
        .unwrap()
    }
}

impl BitXor<&VipsImage> for f64 {
    type Output = VipsImage;
    fn bitxor(self, b: &VipsImage) -> VipsImage {
        b.boolean_const(
            OperationBoolean::Eor,
            &[self],
        )
        .unwrap()
    }
}

impl BitXor<f64> for &VipsImage {
    type Output = VipsImage;
    fn bitxor(self, b: f64) -> VipsImage {
        self.boolean_const(
            OperationBoolean::Eor,
            &[b],
        )
        .unwrap()
    }
}

impl BitXor<&VipsImage> for &[f64] {
    type Output = VipsImage;
    fn bitxor(self, b: &VipsImage) -> VipsImage {
        b.boolean_const(
            OperationBoolean::Eor,
            self,
        )
        .unwrap()
    }
}

impl BitXor<&[f64]> for &VipsImage {
    type Output = VipsImage;
    fn bitxor(self, b: &[f64]) -> VipsImage {
        self.boolean_const(
            OperationBoolean::Eor,
            b,
        )
        .unwrap()
    }
}

impl<const N: usize> BitXor<&VipsImage> for &[f64; N] {
    type Output = VipsImage;
    fn bitxor(self, b: &VipsImage) -> VipsImage {
        b.boolean_const(
            OperationBoolean::Eor,
            self,
        )
        .unwrap()
    }
}

impl<const N: usize> BitXor<&[f64; N]> for &VipsImage {
    type Output = VipsImage;
    fn bitxor(self, b: &[f64; N]) -> VipsImage {
        self.boolean_const(
            OperationBoolean::Eor,
            b,
        )
        .unwrap()
    }
}

// Shl
impl Shl for VipsImage {
    type Output = VipsImage;
    fn shl(self, b: VipsImage) -> VipsImage {
        self.boolean(
            &b,
            OperationBoolean::Lshift,
        )
        .unwrap()
    }
}

impl Shl<f64> for VipsImage {
    type Output = VipsImage;
    fn shl(self, b: f64) -> VipsImage {
        self.boolean_const(
            OperationBoolean::Lshift,
            &[b],
        )
        .unwrap()
    }
}

impl Shl<&[f64]> for VipsImage {
    type Output = VipsImage;
    fn shl(self, b: &[f64]) -> VipsImage {
        self.boolean_const(
            OperationBoolean::Lshift,
            b,
        )
        .unwrap()
    }
}

impl<const N: usize> Shl<&[f64; N]> for VipsImage {
    type Output = VipsImage;
    fn shl(self, b: &[f64; N]) -> VipsImage {
        self.boolean_const(
            OperationBoolean::Lshift,
            b,
        )
        .unwrap()
    }
}

// Shl ref
impl Shl for &VipsImage {
    type Output = VipsImage;
    fn shl(self, b: &VipsImage) -> VipsImage {
        self.boolean(
            b,
            OperationBoolean::Lshift,
        )
        .unwrap()
    }
}

impl Shl<f64> for &VipsImage {
    type Output = VipsImage;
    fn shl(self, b: f64) -> VipsImage {
        self.boolean_const(
            OperationBoolean::Lshift,
            &[b],
        )
        .unwrap()
    }
}

impl Shl<&[f64]> for &VipsImage {
    type Output = VipsImage;
    fn shl(self, b: &[f64]) -> VipsImage {
        self.boolean_const(
            OperationBoolean::Lshift,
            b,
        )
        .unwrap()
    }
}

impl<const N: usize> Shl<&[f64; N]> for &VipsImage {
    type Output = VipsImage;
    fn shl(self, b: &[f64; N]) -> VipsImage {
        self.boolean_const(
            OperationBoolean::Lshift,
            b,
        )
        .unwrap()
    }
}

// Shr
impl Shr for VipsImage {
    type Output = VipsImage;
    fn shr(self, b: VipsImage) -> VipsImage {
        self.boolean(
            &b,
            OperationBoolean::Rshift,
        )
        .unwrap()
    }
}

impl Shr<f64> for VipsImage {
    type Output = VipsImage;
    fn shr(self, b: f64) -> VipsImage {
        self.boolean_const(
            OperationBoolean::Rshift,
            &[b],
        )
        .unwrap()
    }
}

impl Shr<&[f64]> for VipsImage {
    type Output = VipsImage;
    fn shr(self, b: &[f64]) -> VipsImage {
        self.boolean_const(
            OperationBoolean::Rshift,
            b,
        )
        .unwrap()
    }
}

impl<const N: usize> Shr<&[f64; N]> for VipsImage {
    type Output = VipsImage;
    fn shr(self, b: &[f64; N]) -> VipsImage {
        self.boolean_const(
            OperationBoolean::Rshift,
            b,
        )
        .unwrap()
    }
}

// Shr ref
impl Shr for &VipsImage {
    type Output = VipsImage;
    fn shr(self, b: &VipsImage) -> VipsImage {
        self.boolean(
            b,
            OperationBoolean::Rshift,
        )
        .unwrap()
    }
}

impl Shr<f64> for &VipsImage {
    type Output = VipsImage;
    fn shr(self, b: f64) -> VipsImage {
        self.boolean_const(
            OperationBoolean::Rshift,
            &[b],
        )
        .unwrap()
    }
}

impl Shr<&[f64]> for &VipsImage {
    type Output = VipsImage;
    fn shr(self, b: &[f64]) -> VipsImage {
        self.boolean_const(
            OperationBoolean::Rshift,
            b,
        )
        .unwrap()
    }
}

impl<const N: usize> Shr<&[f64; N]> for &VipsImage {
    type Output = VipsImage;
    fn shr(self, b: &[f64; N]) -> VipsImage {
        self.boolean_const(
            OperationBoolean::Rshift,
            b,
        )
        .unwrap()
    }
}

// Not in ops
pub trait Eq<T> {
    type Output: ?Sized;
    fn eq(self, b: T) -> Self::Output;
}

pub trait Lt<T> {
    type Output: ?Sized;
    fn lt(self, b: T) -> Self::Output;
}

pub trait Le<T> {
    type Output: ?Sized;
    fn le(self, b: T) -> Self::Output;
}

pub trait Gt<T> {
    type Output: ?Sized;
    fn gt(self, b: T) -> Self::Output;
}

pub trait Ge<T> {
    type Output: ?Sized;
    fn ge(self, b: T) -> Self::Output;
}

// eq
impl Eq<VipsImage> for VipsImage {
    type Output = VipsImage;
    fn eq(self, b: VipsImage) -> Self::Output {
        self.relational(
            &b,
            OperationRelational::Equal,
        )
        .unwrap()
    }
}

impl Eq<VipsImage> for f64 {
    type Output = VipsImage;
    fn eq(self, b: VipsImage) -> Self::Output {
        b.relational_const(
            OperationRelational::Equal,
            &[self],
        )
        .unwrap()
    }
}

impl Eq<f64> for VipsImage {
    type Output = VipsImage;
    fn eq(self, b: f64) -> Self::Output {
        self.relational_const(
            OperationRelational::Equal,
            &[b],
        )
        .unwrap()
    }
}

impl Eq<VipsImage> for &[f64] {
    type Output = VipsImage;
    fn eq(self, b: VipsImage) -> Self::Output {
        b.relational_const(
            OperationRelational::Equal,
            self,
        )
        .unwrap()
    }
}

impl Eq<&[f64]> for VipsImage {
    type Output = VipsImage;
    fn eq(self, b: &[f64]) -> Self::Output {
        self.relational_const(
            OperationRelational::Equal,
            b,
        )
        .unwrap()
    }
}

impl<const N: usize> Eq<VipsImage> for &[f64; N] {
    type Output = VipsImage;
    fn eq(self, b: VipsImage) -> Self::Output {
        b.relational_const(
            OperationRelational::Equal,
            self,
        )
        .unwrap()
    }
}

impl<const N: usize> Eq<&[f64; N]> for VipsImage {
    type Output = VipsImage;
    fn eq(self, b: &[f64; N]) -> Self::Output {
        self.relational_const(
            OperationRelational::Equal,
            b,
        )
        .unwrap()
    }
}

// lt
impl Lt<VipsImage> for VipsImage {
    type Output = VipsImage;
    fn lt(self, b: VipsImage) -> Self::Output {
        self.relational(
            &b,
            OperationRelational::Less,
        )
        .unwrap()
    }
}

impl Lt<VipsImage> for f64 {
    type Output = VipsImage;
    fn lt(self, b: VipsImage) -> Self::Output {
        b.relational_const(
            OperationRelational::More,
            &[self],
        )
        .unwrap()
    }
}

impl Lt<f64> for VipsImage {
    type Output = VipsImage;
    fn lt(self, b: f64) -> Self::Output {
        self.relational_const(
            OperationRelational::Less,
            &[b],
        )
        .unwrap()
    }
}

impl Lt<VipsImage> for &[f64] {
    type Output = VipsImage;
    fn lt(self, b: VipsImage) -> Self::Output {
        b.relational_const(
            OperationRelational::More,
            self,
        )
        .unwrap()
    }
}

impl Lt<&[f64]> for VipsImage {
    type Output = VipsImage;
    fn lt(self, b: &[f64]) -> Self::Output {
        self.relational_const(
            OperationRelational::Less,
            b,
        )
        .unwrap()
    }
}

impl<const N: usize> Lt<VipsImage> for &[f64; N] {
    type Output = VipsImage;
    fn lt(self, b: VipsImage) -> Self::Output {
        b.relational_const(
            OperationRelational::More,
            self,
        )
        .unwrap()
    }
}

impl<const N: usize> Lt<&[f64; N]> for VipsImage {
    type Output = VipsImage;
    fn lt(self, b: &[f64; N]) -> Self::Output {
        self.relational_const(
            OperationRelational::Less,
            b,
        )
        .unwrap()
    }
}

// lt ref
impl Lt<&VipsImage> for &VipsImage {
    type Output = VipsImage;
    fn lt(self, b: &VipsImage) -> Self::Output {
        self.relational(
            b,
            OperationRelational::Less,
        )
        .unwrap()
    }
}

impl Lt<&VipsImage> for f64 {
    type Output = VipsImage;
    fn lt(self, b: &VipsImage) -> Self::Output {
        b.relational_const(
            OperationRelational::More,
            &[self],
        )
        .unwrap()
    }
}

impl Lt<f64> for &VipsImage {
    type Output = VipsImage;
    fn lt(self, b: f64) -> Self::Output {
        self.relational_const(
            OperationRelational::Less,
            &[b],
        )
        .unwrap()
    }
}

impl Lt<&VipsImage> for &[f64] {
    type Output = VipsImage;
    fn lt(self, b: &VipsImage) -> Self::Output {
        b.relational_const(
            OperationRelational::More,
            self,
        )
        .unwrap()
    }
}

impl Lt<&[f64]> for &VipsImage {
    type Output = VipsImage;
    fn lt(self, b: &[f64]) -> Self::Output {
        self.relational_const(
            OperationRelational::Less,
            b,
        )
        .unwrap()
    }
}

impl<const N: usize> Lt<&VipsImage> for &[f64; N] {
    type Output = VipsImage;
    fn lt(self, b: &VipsImage) -> Self::Output {
        b.relational_const(
            OperationRelational::More,
            self,
        )
        .unwrap()
    }
}

impl<const N: usize> Lt<&[f64; N]> for &VipsImage {
    type Output = VipsImage;
    fn lt(self, b: &[f64; N]) -> Self::Output {
        self.relational_const(
            OperationRelational::Less,
            b,
        )
        .unwrap()
    }
}

// le
impl Le<VipsImage> for VipsImage {
    type Output = VipsImage;
    fn le(self, b: VipsImage) -> Self::Output {
        self.relational(
            &b,
            OperationRelational::Lesseq,
        )
        .unwrap()
    }
}

impl Le<VipsImage> for f64 {
    type Output = VipsImage;
    fn le(self, b: VipsImage) -> Self::Output {
        b.relational_const(
            OperationRelational::Moreeq,
            &[self],
        )
        .unwrap()
    }
}

impl Le<f64> for VipsImage {
    type Output = VipsImage;
    fn le(self, b: f64) -> Self::Output {
        self.relational_const(
            OperationRelational::Lesseq,
            &[b],
        )
        .unwrap()
    }
}

impl Le<VipsImage> for &[f64] {
    type Output = VipsImage;
    fn le(self, b: VipsImage) -> Self::Output {
        b.relational_const(
            OperationRelational::Moreeq,
            self,
        )
        .unwrap()
    }
}

impl Le<&[f64]> for VipsImage {
    type Output = VipsImage;
    fn le(self, b: &[f64]) -> Self::Output {
        self.relational_const(
            OperationRelational::Lesseq,
            b,
        )
        .unwrap()
    }
}

impl<const N: usize> Le<VipsImage> for &[f64; N] {
    type Output = VipsImage;
    fn le(self, b: VipsImage) -> Self::Output {
        b.relational_const(
            OperationRelational::Moreeq,
            self,
        )
        .unwrap()
    }
}

impl<const N: usize> Le<&[f64; N]> for VipsImage {
    type Output = VipsImage;
    fn le(self, b: &[f64; N]) -> Self::Output {
        self.relational_const(
            OperationRelational::Lesseq,
            b,
        )
        .unwrap()
    }
}

// le ref
impl Le<&VipsImage> for &VipsImage {
    type Output = VipsImage;
    fn le(self, b: &VipsImage) -> Self::Output {
        self.relational(
            b,
            OperationRelational::Lesseq,
        )
        .unwrap()
    }
}

impl Le<&VipsImage> for f64 {
    type Output = VipsImage;
    fn le(self, b: &VipsImage) -> Self::Output {
        b.relational_const(
            OperationRelational::Moreeq,
            &[self],
        )
        .unwrap()
    }
}

impl Le<f64> for &VipsImage {
    type Output = VipsImage;
    fn le(self, b: f64) -> Self::Output {
        self.relational_const(
            OperationRelational::Lesseq,
            &[b],
        )
        .unwrap()
    }
}

impl Le<&VipsImage> for &[f64] {
    type Output = VipsImage;
    fn le(self, b: &VipsImage) -> Self::Output {
        b.relational_const(
            OperationRelational::Moreeq,
            self,
        )
        .unwrap()
    }
}

impl Le<&[f64]> for &VipsImage {
    type Output = VipsImage;
    fn le(self, b: &[f64]) -> Self::Output {
        self.relational_const(
            OperationRelational::Lesseq,
            b,
        )
        .unwrap()
    }
}

impl<const N: usize> Le<&VipsImage> for &[f64; N] {
    type Output = VipsImage;
    fn le(self, b: &VipsImage) -> Self::Output {
        b.relational_const(
            OperationRelational::Moreeq,
            self,
        )
        .unwrap()
    }
}

impl<const N: usize> Le<&[f64; N]> for &VipsImage {
    type Output = VipsImage;
    fn le(self, b: &[f64; N]) -> Self::Output {
        self.relational_const(
            OperationRelational::Lesseq,
            b,
        )
        .unwrap()
    }
}

// gt
impl Gt<VipsImage> for VipsImage {
    type Output = VipsImage;
    fn gt(self, b: VipsImage) -> Self::Output {
        self.relational(
            &b,
            OperationRelational::More,
        )
        .unwrap()
    }
}

impl Gt<VipsImage> for f64 {
    type Output = VipsImage;
    fn gt(self, b: VipsImage) -> Self::Output {
        b.relational_const(
            OperationRelational::Less,
            &[self],
        )
        .unwrap()
    }
}

impl Gt<f64> for VipsImage {
    type Output = VipsImage;
    fn gt(self, b: f64) -> Self::Output {
        self.relational_const(
            OperationRelational::More,
            &[b],
        )
        .unwrap()
    }
}

impl Gt<VipsImage> for &[f64] {
    type Output = VipsImage;
    fn gt(self, b: VipsImage) -> Self::Output {
        b.relational_const(
            OperationRelational::Less,
            self,
        )
        .unwrap()
    }
}

impl Gt<&[f64]> for VipsImage {
    type Output = VipsImage;
    fn gt(self, b: &[f64]) -> Self::Output {
        self.relational_const(
            OperationRelational::More,
            b,
        )
        .unwrap()
    }
}

impl<const N: usize> Gt<VipsImage> for &[f64; N] {
    type Output = VipsImage;
    fn gt(self, b: VipsImage) -> Self::Output {
        b.relational_const(
            OperationRelational::Less,
            self,
        )
        .unwrap()
    }
}

impl<const N: usize> Gt<&[f64; N]> for VipsImage {
    type Output = VipsImage;
    fn gt(self, b: &[f64; N]) -> Self::Output {
        self.relational_const(
            OperationRelational::More,
            b,
        )
        .unwrap()
    }
}

// gt ref
impl Gt<&VipsImage> for &VipsImage {
    type Output = VipsImage;
    fn gt(self, b: &VipsImage) -> Self::Output {
        self.relational(
            b,
            OperationRelational::More,
        )
        .unwrap()
    }
}

impl Gt<&VipsImage> for f64 {
    type Output = VipsImage;
    fn gt(self, b: &VipsImage) -> Self::Output {
        b.relational_const(
            OperationRelational::Less,
            &[self],
        )
        .unwrap()
    }
}

impl Gt<f64> for &VipsImage {
    type Output = VipsImage;
    fn gt(self, b: f64) -> Self::Output {
        self.relational_const(
            OperationRelational::More,
            &[b],
        )
        .unwrap()
    }
}

impl Gt<&VipsImage> for &[f64] {
    type Output = VipsImage;
    fn gt(self, b: &VipsImage) -> Self::Output {
        b.relational_const(
            OperationRelational::Less,
            self,
        )
        .unwrap()
    }
}

impl Gt<&[f64]> for &VipsImage {
    type Output = VipsImage;
    fn gt(self, b: &[f64]) -> Self::Output {
        self.relational_const(
            OperationRelational::More,
            b,
        )
        .unwrap()
    }
}

impl<const N: usize> Gt<&VipsImage> for &[f64; N] {
    type Output = VipsImage;
    fn gt(self, b: &VipsImage) -> Self::Output {
        b.relational_const(
            OperationRelational::Less,
            self,
        )
        .unwrap()
    }
}

impl<const N: usize> Gt<&[f64; N]> for &VipsImage {
    type Output = VipsImage;
    fn gt(self, b: &[f64; N]) -> Self::Output {
        self.relational_const(
            OperationRelational::More,
            b,
        )
        .unwrap()
    }
}

// ge
impl Ge<VipsImage> for VipsImage {
    type Output = VipsImage;
    fn ge(self, b: VipsImage) -> Self::Output {
        self.relational(
            &b,
            OperationRelational::Moreeq,
        )
        .unwrap()
    }
}

impl Ge<VipsImage> for f64 {
    type Output = VipsImage;
    fn ge(self, b: VipsImage) -> Self::Output {
        b.relational_const(
            OperationRelational::Lesseq,
            &[self],
        )
        .unwrap()
    }
}

impl Ge<f64> for VipsImage {
    type Output = VipsImage;
    fn ge(self, b: f64) -> Self::Output {
        self.relational_const(
            OperationRelational::Moreeq,
            &[b],
        )
        .unwrap()
    }
}

impl Ge<VipsImage> for &[f64] {
    type Output = VipsImage;
    fn ge(self, b: VipsImage) -> Self::Output {
        b.relational_const(
            OperationRelational::Lesseq,
            self,
        )
        .unwrap()
    }
}

impl Ge<&[f64]> for VipsImage {
    type Output = VipsImage;
    fn ge(self, b: &[f64]) -> Self::Output {
        self.relational_const(
            OperationRelational::Moreeq,
            b,
        )
        .unwrap()
    }
}

impl<const N: usize> Ge<VipsImage> for &[f64; N] {
    type Output = VipsImage;
    fn ge(self, b: VipsImage) -> Self::Output {
        b.relational_const(
            OperationRelational::Lesseq,
            self,
        )
        .unwrap()
    }
}

impl<const N: usize> Ge<&[f64; N]> for VipsImage {
    type Output = VipsImage;
    fn ge(self, b: &[f64; N]) -> Self::Output {
        self.relational_const(
            OperationRelational::Moreeq,
            b,
        )
        .unwrap()
    }
}

// ge ref
impl Ge<&VipsImage> for &VipsImage {
    type Output = VipsImage;
    fn ge(self, b: &VipsImage) -> Self::Output {
        self.relational(
            b,
            OperationRelational::Moreeq,
        )
        .unwrap()
    }
}

impl Ge<&VipsImage> for f64 {
    type Output = VipsImage;
    fn ge(self, b: &VipsImage) -> Self::Output {
        b.relational_const(
            OperationRelational::Lesseq,
            &[self],
        )
        .unwrap()
    }
}

impl Ge<f64> for &VipsImage {
    type Output = VipsImage;
    fn ge(self, b: f64) -> Self::Output {
        self.relational_const(
            OperationRelational::Moreeq,
            &[b],
        )
        .unwrap()
    }
}

impl Ge<&VipsImage> for &[f64] {
    type Output = VipsImage;
    fn ge(self, b: &VipsImage) -> Self::Output {
        b.relational_const(
            OperationRelational::Lesseq,
            self,
        )
        .unwrap()
    }
}

impl Ge<&[f64]> for &VipsImage {
    type Output = VipsImage;
    fn ge(self, b: &[f64]) -> Self::Output {
        self.relational_const(
            OperationRelational::Moreeq,
            b,
        )
        .unwrap()
    }
}

impl<const N: usize> Ge<&VipsImage> for &[f64; N] {
    type Output = VipsImage;
    fn ge(self, b: &VipsImage) -> Self::Output {
        b.relational_const(
            OperationRelational::Lesseq,
            self,
        )
        .unwrap()
    }
}

impl<const N: usize> Ge<&[f64; N]> for &VipsImage {
    type Output = VipsImage;
    fn ge(self, b: &[f64; N]) -> Self::Output {
        self.relational_const(
            OperationRelational::Moreeq,
            b,
        )
        .unwrap()
    }
}
