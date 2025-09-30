use crate::{
    ast::{Ast, Block, Function, Type},
    parser::parse,
};

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
    fn pp(&self, f: &mut std::fmt::Formatter<'_>, ctx: PpCtx) {}
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

impl std::fmt::Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ctx = PpCtx { indent: 0 };
        self.pp(f, ctx);
        Ok(())
    }
}

#[test]
fn test() {
    let input = include_str!("../../examples/test.c");
    let ast = parse(input).unwrap();
    println!("{}", ast);
}
