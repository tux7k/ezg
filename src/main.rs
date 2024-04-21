#![feature(step_trait, trait_alias)]
use std::{iter::Step, ops::{Add, AddAssign, Div, Mul, Range, Sub}};

pub trait EzIntFns {
    fn genZero() -> Self;
    fn isZero(i: Self) -> bool;
    fn toZero(self);

    fn genOne() -> Self;
    fn isOne(i: Self) -> bool;
    fn toOne(self);
}

#[macro_export]
macro_rules! one_zero_int {
    ( $($t:ty), * ) => {  
            $(impl EzIntFns for $t {
                fn genZero() -> Self {0}
                fn isZero(i: Self) -> bool {if i == 0 {return true}; false}
                fn toZero(mut self) { self = 0 }

                fn genOne() -> Self {1}
                fn isOne(i: Self) -> bool {if i == 1 {return true}; false}
                fn toOne(mut self) { self = 1 }
            })*
    };
}

#[macro_export]
macro_rules! one_zero_float {
    ( $($t:ty), * ) => {  
            $(impl EzIntFns for $t {
                fn genZero() -> Self {0.0}
                fn isZero(i: Self) -> bool {if i == 0.0 {return true}; false}
                fn toZero(mut self) { self = 0.0 }

                fn genOne() -> Self {1.0}
                fn isOne(i: Self) -> bool {if i == 1.0 {return true}; false}
                fn toOne(mut self) { self = 1.0 }
            })*
    };
}


one_zero_int!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
one_zero_float!(f32, f64);



trait NumBase = 
    Copy
    + Clone
    + Add<Output = Self>
    + Sub<Output = Self>
    + Div<Output = Self>
    + Mul<Output = Self>
    + PartialEq
    + PartialOrd
    + AddAssign
    // easy functions to be able to use in type generics
    + EzIntFns
    + Sized
;

trait Int = NumBase + Step;

trait Float = NumBase;

struct FloatRange<GT: NumBase> {
    pub start: GT,
    pub end: GT,
    step: GT,
    cur: GT
}

impl<GT: NumBase> FloatRange<GT> {
    pub fn new(start: GT, end: GT, step: GT, sp: GT) -> Self {
        Self {
            start,
            end,
            step,
            cur: GT::genZero()
        }
    }
}

impl<GT: NumBase> std::iter::Iterator for FloatRange<GT> {
    type Item = GT;

    fn next(&mut self) -> Option<Self::Item> {
        self.cur += self.step;

        if self.cur < self.end {
            Some(self.cur)
        } else {
            None
        }
    }
}


struct Graph<GT: Float> {

}

fn main() {

}
