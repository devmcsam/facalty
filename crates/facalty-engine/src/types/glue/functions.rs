use crate::types::ast::Node;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltInFunction {
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Sinpi,
    Cospi,
    Tanpi,
    ASinPi,
    ACosPi,
    ATanPi,
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

impl BuiltInFunction {
    pub fn map_str(input: &str) -> Option<Self> {
        match input {
            "sin" => Some(Self::Sin),
            "cos" => Some(Self::Cos),
            "tan" => Some(Self::Tan),
            "asin" => Some(Self::Asin),
            "acos" => Some(Self::Acos),
            "atan" => Some(Self::Atan),
            "sinh" => Some(Self::Sinh),
            "cosh" => Some(Self::Cosh),
            "tanh" => Some(Self::Tanh),
            "sqrt" => Some(Self::Sqrt),
            "cbrt" => Some(Self::Cbrt),
            "root" => Some(Self::Root),
            "factorial" => Some(Self::Factorial),
            "abs" => Some(Self::Abs),
            "ln" => Some(Self::Ln),
            "log10" => Some(Self::Log10),
            "log" => Some(Self::LogBase),
            "round" => Some(Self::Round),
            "truncate" => Some(Self::Truncate),
            "ceil" => Some(Self::Ceil),
            "floor" => Some(Self::Floor),
            "integral" => Some(Self::Integral),
            "derivative" => Some(Self::Derivative),
            "max" => Some(Self::Max),
            "min" => Some(Self::Min),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FunctionId(u32);

pub struct FunctionInterner {
    to_id: HashMap<String, FunctionId>,
    to_name: Vec<String>, // indexed by id
}

impl FunctionInterner {
    pub fn new() -> Self {
        Self {
            to_id: HashMap::new(),
            to_name: Vec::new(),
        }
    }

    pub fn intern(&mut self, name: &str) -> FunctionId {
        if let Some(&id) = self.to_id.get(name) {
            return id;
        }
        let id = FunctionId(self.to_name.len() as u32);
        self.to_name.push(name.to_owned());
        self.to_id.insert(name.to_owned(), id);
        id
    }
    pub fn resolve(&self, id: FunctionId) -> &str {
        &self.to_name[id.0 as usize]
    }
}

pub struct UserFunction {
    pub(crate) id: FunctionId,
    pub params: Vec<char>,
    pub body: Box<Node>,
}

pub struct FunctionRegistry {
    interner: FunctionInterner,
    functions: HashMap<FunctionId, UserFunction>,
}

pub enum FunctionError {
    AlreadyRegistered,
}

impl FunctionRegistry {
    pub fn register(
        &mut self,
        name: &str,
        params: Vec<char>,
        body: Box<Node>,
    ) -> Result<FunctionId, FunctionError> {
        let id = self.interner.intern(name);
        if self.functions.contains_key(&id) {
            return Err(FunctionError::AlreadyRegistered);
        }
        self.functions.insert(id, UserFunction { id, params, body });
        Ok(id)
    }

    pub fn get(&self, id: FunctionId) -> Option<&UserFunction> {
        self.functions.get(&id)
    }
}

pub enum Function {
    BuiltIn(BuiltInFunction),
    User(UserFunction),
}
