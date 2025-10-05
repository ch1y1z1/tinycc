use inkwell::types::BasicMetadataTypeEnum;

use crate::{ast::Type, codegen::CodeGen};

pub trait IntoLlvmType<'ctx> {
    fn into_llvm_type(&self, code_gen: &CodeGen<'ctx>) -> BasicMetadataTypeEnum<'ctx>;
}

impl<'ctx> IntoLlvmType<'ctx> for Type {
    fn into_llvm_type(&self, code_gen: &CodeGen<'ctx>) -> BasicMetadataTypeEnum<'ctx> {
        match self {
            Type::Int => BasicMetadataTypeEnum::IntType(code_gen.context.i32_type()),
            Type::Float => BasicMetadataTypeEnum::FloatType(code_gen.context.f32_type()),
            Type::Void => panic!("Void type cannot be used as a function parameter or return type"),
        }
    }
}
