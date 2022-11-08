use crate::prelude::*;

pub fn transform_function_call(call: &lua_ast::FunctionCall) -> Box<Expr> {
	transform_prefix_suffixes(call.prefix(), call.suffixes())
}
