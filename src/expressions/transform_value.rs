use crate::prelude::*;

pub fn transform_value(value: &lua_ast::Value) -> Box<Expr> {
	match value {
		lua_ast::Value::Function((_, body)) => boxed(Expr::Arrow(ArrowExpr {
			span: Default::default(),
			is_async: false,
			is_generator: false,
			return_type: body.return_type().map(transform_type_specifier),
			type_params: transform_type_generic(body.generics()),
			params: transform_function_params(body.parameters().iter(), body.type_specifiers()),
			body: BlockStmtOrExpr::BlockStmt(BlockStmt {
				span: Default::default(),
				stmts: transform_block_statements(body.block()),
			}),
		})),
		lua_ast::Value::FunctionCall(call) => transform_function_call(call),
		lua_ast::Value::IfExpression(if_expression) => transform_if_expression(if_expression),
		lua_ast::Value::TableConstructor(table_constructor) => {
			transform_table_constructor(table_constructor)
		}
		lua_ast::Value::Number(number) => transform_number(number),
		// `expr` will also be wrapped in Expression::Parentheses
		lua_ast::Value::ParenthesesExpression(expr) => transform_expression(expr),
		lua_ast::Value::String(number) => transform_string(number),
		lua_ast::Value::Symbol(token)
			if matches!(
				token.token().token_type(),
				tokenizer::TokenType::Symbol {
					symbol: tokenizer::Symbol::Nil
				}
			) =>
		{
			boxed(Expr::Ident(Ident {
				span: Default::default(),
				sym: JsWord::from("undefined"),
				optional: false,
			}))
		}
		lua_ast::Value::Symbol(token)
			if matches!(
				token.token().token_type(),
				tokenizer::TokenType::Symbol {
					symbol: tokenizer::Symbol::True
				}
			) =>
		{
			boxed(Expr::Lit(Lit::Bool(Bool {
				span: Default::default(),
				value: true,
			})))
		}
		lua_ast::Value::Symbol(token)
			if matches!(
				token.token().token_type(),
				tokenizer::TokenType::Symbol {
					symbol: tokenizer::Symbol::False
				}
			) =>
		{
			boxed(Expr::Lit(Lit::Bool(Bool {
				span: Default::default(),
				value: false,
			})))
		}
		lua_ast::Value::Symbol(token)
			if matches!(
				token.token().token_type(),
				tokenizer::TokenType::Symbol {
					symbol: tokenizer::Symbol::Ellipse
				}
			) =>
		{
			// Use ident hack because can't use ExprOrSpread
			boxed(Expr::Ident(Ident {
				span: Default::default(),
				optional: false,
				sym: JsWord::from(format!("...{}", REST_ARGS_NAME)),
			}))
		}
		lua_ast::Value::Var(var) => transform_var(var),
		_ => skip("Unknown value variant", value),
	}
}
