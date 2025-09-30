#[derive(Clone)]
pub struct Ast {
    pub program: Vec<Function>,
}

#[derive(Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Parameter>,
    pub body: Block,
    pub ret_type: Type,
}

#[derive(Clone)]
pub struct Parameter {
    pub name: String,
    pub param_type: Type,
}

#[derive(Clone)]
pub enum Type {
    Int,
    Float,
    Void,
}

#[derive(Clone)]
pub struct Block {
    pub items: Vec<BlockItem>,
}

#[derive(Clone)]
pub enum BlockItem {
    Decl(Declaration),
    Stmt(Statement),
}

#[derive(Clone)]
pub struct Declaration {
    pub var_type: Type,
    pub name: String,
    pub init: Option<Expression>,
}

#[derive(Clone)]
pub enum Statement {
    Block(Block),
    Expr(Expression),
    Ret(Option<Expression>),
    If {
        condition: Expression,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },
    While {
        condition: Expression,
        body: Box<Statement>,
    },
    Empty,
}

#[derive(Clone)]
pub enum Expression {
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    Unary {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },
    Literal(Literal),
    Variable(String),
    Assignment {
        target: LValue,
        value: Box<Expression>,
    },
    FunctionCall {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },
}

#[derive(Clone)]
pub enum LValue {
    Var(String),
}

#[derive(Clone)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
}

#[derive(Clone)]
pub enum UnaryOperator {
    Plus,
    Negate,
    Not,
}

#[derive(Clone)]
pub enum Literal {
    Int(i32),
    Float(f32),
}
