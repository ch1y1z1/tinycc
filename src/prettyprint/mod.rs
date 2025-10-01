use crate::ast::{Ast, Block, BlockItem, Expression, Function, LValue, Literal, Statement, Type};

trait PpWithCtx<C> {
    fn pp(&self, f: &mut std::fmt::Formatter<'_>, ctx: C);
}

#[derive(Clone)]
struct PpCtx {
    indent: usize,
}

impl PpCtx {
    fn sub_ctx(&self) -> Self {
        PpCtx {
            indent: self.indent + 1,
        }
    }
}
macro_rules! write_indent {
    ($ctx:expr, $f:expr) => {
        for _ in 0..$ctx.indent {
            write!($f, "  ").unwrap();
        }
    };

    ($ctx:expr, $f:expr, $($arg:tt)*) => {
        write_indent!($ctx, $f);
        write!($f, $($arg)*).unwrap();
    };
}

macro_rules! writeln_indent {
    ($ctx:expr, $f:expr) => {
        write_indent!($ctx, $f);
        writeln!($f).unwrap();
    };

    ($ctx:expr, $f:expr, $($arg:tt)*) => {
        write_indent!($ctx, $f);
        writeln!($f, $($arg)*).unwrap();
    };
}

impl PpWithCtx<PpCtx> for Ast {
    fn pp(&self, f: &mut std::fmt::Formatter<'_>, ctx: PpCtx) {
        writeln_indent!(ctx, f, "Program:");
        for func in &self.program {
            func.pp(f, ctx.sub_ctx());
            writeln_indent!(ctx, f);
        }
    }
}

impl PpWithCtx<PpCtx> for Function {
    fn pp(&self, f: &mut std::fmt::Formatter<'_>, ctx: PpCtx) {
        writeln_indent!(ctx, f, "Function: {}", self.name);
        write_indent!(ctx.sub_ctx(), f, "Args: ");
        for param in &self.params {
            write!(f, "{} {}, ", &param.param_type, param.name).unwrap();
        }
        writeln!(f).unwrap();
        writeln_indent!(ctx.sub_ctx(), f, "Return Type: {}", self.ret_type);
        writeln_indent!(ctx.sub_ctx(), f, "Body: ");
        self.body.pp(f, ctx.sub_ctx().sub_ctx());
    }
}

impl PpWithCtx<PpCtx> for Block {
    fn pp(&self, f: &mut std::fmt::Formatter<'_>, ctx: PpCtx) {
        writeln_indent!(ctx, f, "Block:");
        for item in &self.items {
            item.pp(f, ctx.sub_ctx());
        }
    }
}

impl PpWithCtx<PpCtx> for BlockItem {
    fn pp(&self, f: &mut std::fmt::Formatter<'_>, ctx: PpCtx) {
        match self {
            BlockItem::Decl(decl) => {
                writeln_indent!(ctx, f, "Declaration: {} {}", decl.var_type, decl.name);
                // if let Some(init) = &decl.init {
                //     write_indent!(ctx.sub_ctx(), f, "Init: ");
                //     init.pp(f, ctx.sub_ctx());
                //     writeln!(f).unwrap();
                // }
            }
            BlockItem::Stmt(stmt) => {
                writeln_indent!(ctx, f, "Statement:");
                stmt.pp(f, ctx.sub_ctx());
            }
        }
    }
}

impl PpWithCtx<PpCtx> for Statement {
    fn pp(&self, f: &mut std::fmt::Formatter<'_>, ctx: PpCtx) {
        match self {
            Statement::Block(block) => {
                block.pp(f, ctx);
            }
            Statement::Expr(expr) => {
                write_indent!(ctx, f, "Expr: ");
                expr.pp(f, ctx);
                writeln!(f).unwrap();
            }
            Statement::Ret(expr_opt) => {
                write_indent!(ctx, f, "Return");
                if let Some(expr) = expr_opt {
                    write!(f, " ");
                    expr.pp(f, ctx);
                }
                writeln!(f).unwrap();
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                writeln_indent!(ctx, f, "If:");
                write_indent!(ctx.sub_ctx(), f, "Condition: ");
                condition.pp(f, ctx.sub_ctx());
                writeln!(f).unwrap();
                writeln_indent!(ctx.sub_ctx(), f, "Then:");
                then_branch.pp(f, ctx.sub_ctx().sub_ctx());
                if let Some(else_branch) = else_branch {
                    writeln_indent!(ctx.sub_ctx(), f, "Else:");
                    else_branch.pp(f, ctx.sub_ctx().sub_ctx());
                }
            }
            Statement::While { condition, body } => {
                writeln_indent!(ctx, f, "While:");
                write_indent!(ctx.sub_ctx(), f, "Condition: ");
                condition.pp(f, ctx.sub_ctx());
                writeln!(f).unwrap();
                writeln_indent!(ctx.sub_ctx(), f, "Body:");
                body.pp(f, ctx.sub_ctx().sub_ctx());
            }
            Statement::Empty => {
                writeln_indent!(ctx, f, "Empty;");
            }
        }
    }
}

impl PpWithCtx<PpCtx> for Expression {
    fn pp(&self, f: &mut std::fmt::Formatter<'_>, ctx: PpCtx) {
        match self {
            Expression::Literal(lit) => {
                write!(f, "Literal({})", lit).unwrap();
            }
            Expression::Variable(name) => {
                write!(f, "Variable({})", name).unwrap();
            }
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                write!(f, "Binary(").unwrap();
                left.pp(f, ctx.clone());
                write!(f, " {:?} ", operator).unwrap();
                right.pp(f, ctx);
                write!(f, ")").unwrap();
            }
            Expression::Unary { operator, operand } => {
                write!(f, "Unary({:?} ", operator).unwrap();
                operand.pp(f, ctx);
                write!(f, ")").unwrap();
            }
            Expression::Assignment { target, value } => {
                write!(f, "Assignment(").unwrap();
                write!(f, "{}", target).unwrap();
                write!(f, " = ").unwrap();
                value.pp(f, ctx);
                write!(f, ")").unwrap();
            }
            Expression::FunctionCall { callee, arguments } => {
                write!(f, "FunctionCall(").unwrap();
                callee.pp(f, ctx.clone());
                write!(f, ", [").unwrap();
                for arg in arguments {
                    arg.pp(f, ctx.clone());
                    write!(f, ", ").unwrap();
                }
                write!(f, "])").unwrap();
            }
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::Void => write!(f, "void"),
        }
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Int(n) => write!(f, "Int({})", n),
            Literal::Float(n) => write!(f, "Float({})", n),
        }
    }
}

impl std::fmt::Display for LValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LValue::Var(name) => write!(f, "Var({})", name),
        }
    }
}

impl std::fmt::Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ctx = PpCtx { indent: 0 };
        self.pp(f, ctx);
        Ok(())
    }
}

#[test]
fn test() {
    use crate::parser::parse;

    let input = include_str!("../../examples/test.c");
    let ast = parse(input).unwrap();
    println!("{}", ast);
}
