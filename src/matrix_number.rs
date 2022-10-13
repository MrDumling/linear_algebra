use std::ops::*;
use num_traits::identities::{self};

pub trait MatrixNumber:
    Sized + Add<Output = Self> + Mul<Output = Self> + Sub<Output = Self> + Div<Output = Self> + AddAssign + MulAssign + Copy + identities::One + identities::Zero
{
}


impl MatrixNumber for u8   {}
impl MatrixNumber for u16  {}
impl MatrixNumber for u32  {}
impl MatrixNumber for u64  {}
impl MatrixNumber for u128 {}

impl MatrixNumber for i8   {}
impl MatrixNumber for i16  {}
impl MatrixNumber for i32  {}
impl MatrixNumber for i64  {}
impl MatrixNumber for i128 {}

impl MatrixNumber for f32  {}
impl MatrixNumber for f64  {}

impl MatrixNumber for usize{}
impl MatrixNumber for isize{}