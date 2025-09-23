struct Ast {
    program: Vec<Function>,
}

struct Function {
    name: String,
    params: Vec<Parameter>,
    body: Block,
    return_type: Type,
}

struct Parameter {
    name: String,
    param_type: Type,
}

enum Type {
    Int,
    Float,
    Void,
}

struct Block {
    items: Vec<BlockItem>,
}

enum BlockItem {
    Decl(Declaration),
    Stmt(Statement),
}

struct Declaration {
    var_type: Type,
    name: String,
    init: Option<Expression>,
}

enum Statement {
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

enum Expression {
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

enum LValue {
    Var(String),
}

enum BinaryOperator {
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

enum UnaryOperator {
    Plus,
    Negate,
    Not,
}

enum Literal {
    Int(i32),
    Float(f32),
}
