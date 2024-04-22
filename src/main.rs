#![feature(step_trait, trait_alias)]
use std::{
    fs::File,
    io::Write,
    iter::Step,
    ops::{Add, AddAssign, Div, Mul, Range, Sub},
};

#[cfg(target_pointer_width = "32")]
pub type fsize = f32;

#[cfg(target_pointer_width = "64")]
pub type fsize = f64;

pub trait EzIntFns {
    fn gen_zero() -> Self;
    fn is_zero(i: Self) -> bool;
    fn toZero(self);

    fn genOne() -> Self;
    fn isOne(i: Self) -> bool;
    fn toOne(self);
}

// trait EzFloatFuncs {
//     fn sin(self) -> Self;
// }

#[macro_export]
macro_rules! impl_int {
    ( $($t:ty), * ) => {
            $(impl EzIntFns for $t {
                fn gen_zero() -> Self {0}
                fn is_zero(i: Self) -> bool {if i == 0 {return true}; false}
                fn toZero(mut self) { self = 0 }

                fn genOne() -> Self {1}
                fn isOne(i: Self) -> bool {if i == 1 {return true}; false}
                fn toOne(mut self) { self = 1 }
            })*
    };
}

#[macro_export]
macro_rules! impl_float {
    ( $($t:ty), * ) => {
            $(impl EzIntFns for $t {
                fn gen_zero() -> Self {0.0}
                fn is_zero(i: Self) -> bool {if i == 0.0 {return true}; false}
                fn toZero(mut self) { self = 0.0 }

                fn genOne() -> Self {1.0}
                fn isOne(i: Self) -> bool {if i == 1.0 {return true}; false}
                fn toOne(mut self) { self = 1.0 }
            }

            // impl EzFloatFuncs for $t {
            //     fn sin(self) -> Self {
            //         self.sin()
            //     }
            // }
        )*
    };
}

#[macro_export]
macro_rules! default_numbase {
    ( $($t:ty), * ) => {
            $(impl Default for $t {
                fn default() -> Self {
                    gen_zero()
                }
            })*
    };
}

impl_int!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
impl_float!(f32, f64);

trait NumBase = Copy
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
    + std::fmt::Display;

trait Int = NumBase + Step;

trait Float = NumBase;

#[derive(Copy, Clone, Debug)]
struct FloatRange<GT: NumBase> {
    pub start: GT,
    pub end: GT,
    step: GT,
    cur: GT,
}

impl<GT: NumBase> FloatRange<GT> {
    pub fn new(start: GT, end: GT, step: GT) -> Self {
        Self {
            start,
            end,
            step,
            cur: start,
        }
    }

    pub fn accurize_graphing(&mut self) {
        self.start = self.start - self.step;
        self.cur = self.start;
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

pub fn res(i: usize) -> fsize {
    1.0 / (i as fsize)
}

struct Graph<GT: Float> {
    x: GT,
    x_step: FloatRange<GT>,
    out: Vec<(GT, Vec<GT>)>,
}

impl<GT: Float> Graph<GT> {
    pub fn new(start: GT, end: GT, step: GT) -> Self {
        let mut range = FloatRange::new(start, end, step);
        range.accurize_graphing();
        Self {
            x: start,
            x_step: range,
            out: Vec::new(),
        }
    }

    pub fn run_function(&mut self, f: fn(GT, GT) -> Vec<GT>, equ: fn(GT) -> GT) {
        for x in self.x_step {
            println!("{} {}", x, equ(x));
            self.out.push(
                (x, f(x, equ(x))
            ))
        }
    }

    pub fn graph_out_to_gnuplot(self, file_name: &str) -> std::io::Result<()> {
        let mut stringOut = String::new();
        for (i, x) in self.out.iter().enumerate() {
            if i != 0 {
                stringOut.push('\n');
            }
            stringOut.push_str(&format!("{}", x.0));
            for a in &x.1 {
                stringOut.push_str(&format!(" {}", a))
            }
        }
        let mut file = File::create(file_name)?;
        file.write_all(stringOut.as_bytes())?;
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let mut graph: Graph<f64> = Graph::new(-5.0, 5.0, res(20)); // Adjust the range accordingly
    let func = |x: f64, y: f64| {
        let r: f64 = 5.0; // Radius of the circle
        let yy = (r.powi(2) - x.powi(2)).sqrt();
        // Return only the positive y value
        vec![yy, -yy]};
    graph.run_function(func, |x| {x});
    graph.graph_out_to_gnuplot("out.log");
    Ok(())
}


