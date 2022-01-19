use crate::prelude::*;

pub fn transform_function_args(args: &lua_ast::FunctionArgs) -> Vec<ExprOrSpread> {
	match args {
		lua_ast::FunctionArgs::Parentheses {
			arguments,
			parentheses: _,
		} => arguments
			.iter()
			.map(|arg| ExprOrSpread {
				spread: None,
				expr: boxed(transform_expression(arg)),
			})
			.collect(),
		lua_ast::FunctionArgs::String(string) => {
			vec![ExprOrSpread {
				spread: None,
				expr: boxed(transform_string(string)),
			}]
		}
		lua_ast::FunctionArgs::TableConstructor(table) => vec![ExprOrSpread {
			spread: None,
			expr: boxed(transform_table_constructor(table)),
		}],
		_ => vec![ExprOrSpread {
			spread: None,
			expr: boxed(skip("Unknown function args type", args)),
		}],
	}
}
