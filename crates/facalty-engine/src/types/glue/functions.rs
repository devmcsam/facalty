use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Function {
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Sinh,
    Cosh,
    Tanh,
    Sqrt,
    Cbrt,
    Root,
    Factorial,
    Abs,
    Ln,
    Log10,
    LogBase,
    Round,
    Truncate,
    Ceil,
    Floor,
    Integral,
    Derivative,
    Max,
    Min,
}

pub struct UserFunction {
    id: u32,
}
