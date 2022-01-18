use crate::prelude::*;

pub fn transform_value(value: &lua_ast::Value) -> Expr {
	match value {
		lua_ast::Value::ParenthesesExpression(expr) => transform_expression(expr),
		lua_ast::Value::Symbol(token)
			if matches!(
				token.token().token_type(),
				tokenizer::TokenType::Symbol {
					symbol: tokenizer::Symbol::Nil
				}
			) =>
		{
			Expr::Ident(Ident {
				span: Default::default(),
				sym: JsWord::from("undefined"),
				optional: false,
			})
		}
		_ => skip("Unknown value variant", value),
	}
}
