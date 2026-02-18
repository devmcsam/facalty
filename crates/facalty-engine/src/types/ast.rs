use super::glue::{
    functions::{BuiltInFunction, FunctionId},
    number::Number,
};

// ── AST Node ────────────────────────────────────────────────────

/// A single node in the expression tree.
#[derive(Debug, Clone)]
pub enum Node {
    /// A numeric literal: `42`, `3/4`, `2.5`
    Number(Number),

    /// A single-character variable: `x`, `y`, …
    Variable(char),

    /// A binary operation: `left op right`
    BinaryOp {
        op: Operator,
        left: Box<Self>,
        right: Box<Self>,
    },

    /// Unary negation: `-expr`
    UnaryNeg(Box<Self>),

    /// Call to a built-in function: `sin(expr)`, `log(base, expr)`, …
    BuiltInCall {
        function: BuiltInFunction,
        args: Vec<Self>,
    },

    /// Call to a user-defined function: `f(expr, expr, …)`
    UserCall { id: FunctionId, args: Vec<Self> },
}

// ── Operators ───────────────────────────────────────────────────

/// A binary operator between two sub-expressions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Rem,
}

impl Operator {
    /// Precedence level (higher binds tighter). Useful for a Pratt / precedence-climbing parser.
    #[must_use]
    pub const fn precedence(self) -> u8 {
        match self {
            Self::Add | Self::Sub => 1,
            Self::Mul | Self::Div | Self::Rem => 2,
            Self::Pow => 3,
        }
    }

    /// Whether this operator is right-associative (`^` is, the rest aren't).
    #[must_use]
    pub const fn is_right_assoc(self) -> bool {
        matches!(self, Self::Pow)
    }
}

// ── Tokens (for the lexer) ──────────────────────────────────────

/// A token emitted by the lexer — consumed by the parser to build `Node`s.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(Number),
    Variable(char),
    Operator(Operator),
    LParen,
    RParen,
    Comma,
    Equals, // `=`  (for `f(x,y) = …`)
    BuiltInFunction(BuiltInFunction),
    Identifier(String), // unknown name → might be a user function
    Eof,
}
