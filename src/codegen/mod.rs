use inkwell::{builder::Builder, context::Context, module::Module, types::BasicMetadataTypeEnum};

use crate::ast::{Ast, Function};
use into_llvm_type::IntoLlvmType;

mod into_llvm_type;
mod test;

struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new_with_module_name(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        Self {
            context,
            module,
            builder,
        }
    }
}

trait CodeGenTrait {
    type Ret;
    fn codegen(&self, code_gen: &CodeGen) -> Self::Ret;
}

impl CodeGenTrait for Ast {
    type Ret = ();
    fn codegen(&self, code_gen: &CodeGen) -> Self::Ret {
        for func in &self.program {
            func.codegen(code_gen);
        }
    }
}

impl CodeGenTrait for Function {
    type Ret = ();
    fn codegen(&self, code_gen: &CodeGen) -> Self::Ret {
        println!("Generating code for function: {}", self.name);

        let ret_type = self.ret_type.into_llvm_type(code_gen);
        let param_types = self
            .params
            .iter()
            .map(|p| p.param_type.into_llvm_type(code_gen))
            .collect::<Vec<_>>();
        let fn_type = match ret_type {
            BasicMetadataTypeEnum::IntType(ty) => ty.fn_type(&param_types, false),
            _ => panic!("Unsupported return type"),
        };

        // 用真正的函数名

        let function = code_gen.module.add_function(&self.name, fn_type, None);
        let basic_block = code_gen.context.append_basic_block(function, "entry");
        code_gen.builder.position_at_end(basic_block);

        // build body ...

        code_gen.builder.build_return(None).unwrap();
    }
}

#[test]
fn test() {
    use crate::parser::parse;

    let input = include_str!("../../examples/test.c");
    let ast = parse(input).unwrap();

    let context = Context::create();
    let codegen = CodeGen::new_with_module_name(&context, "my_module");

    ast.codegen(&codegen);

    codegen.module.print_to_stderr();
}
