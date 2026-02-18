use super::glue::{
    functions::{BuiltInFunction, FunctionId},
    number::Number,
};

// ── AST Node ────────────────────────────────────────────────────

/// A single node in the expression tree.
#[derive(Debug, Clone)]
pub enum Node {
    /// floating point or integer
    Number(Number),

    /// single character
    Variable(char),

    /// A binary operation: 'left op right'
    BinaryOp {
        op: Operator,
        left: Box<Self>,
        right: Box<Self>,
    },

    /// Unary negation: '-expr'
    UnaryNeg(Box<Self>),

    /// Call to a built in, this is just an enum with a match
    BuiltInCall {
        function: BuiltInFunction,
        args: Vec<Self>,
    },

    /// Call to a user defined function
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
    /// Precedence level (higher binds tighter).
    #[must_use]
    pub const fn precedence(self) -> u8 {
        match self {
            Self::Add | Self::Sub => 1,
            Self::Mul | Self::Div | Self::Rem => 2,
            Self::Pow => 3,
        }
    }

    /// Whether this operator is right-associative ('^' is, the rest aren't).
    #[must_use]
    pub const fn is_right_assoc(self) -> bool {
        matches!(self, Self::Pow)
    }
}

// ── Tokens (for the lexer) ──────────────────────────────────────

/// A token emitted by the lexer
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(Number),
    Variable(char),
    Operator(Operator),
    LParen,
    RParen,
    Comma,
    Equals, // '=' for "f(x) ="
    BuiltInFunction(BuiltInFunction),
    Identifier(String), // unknown name → might be a user function
    Eof,
}
