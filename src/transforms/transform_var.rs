use crate::prelude::*;

pub fn transform_var(var: &lua_ast::Var) -> Expr {
	match var {
		lua_ast::Var::Expression(expr) => property_chain(
			match expr.prefix() {
				lua_ast::Prefix::Name(token) => Expr::Ident(Ident {
					optional: false,
					span: Default::default(),
					sym: JsWord::from(token.token().to_string()),
				}),
				lua_ast::Prefix::Expression(expr) => transform_expression(expr),
				_ => skip("Unknown prefix variant", expr.prefix()),
			},
			expr.suffixes()
				.map(|suffix| Ident {
					optional: false,
					span: Default::default(),
					sym: JsWord::from(suffix.to_string()),
				})
				.collect(),
		),
		lua_ast::Var::Name(name) => Expr::Ident(Ident {
			optional: false,
			span: Default::default(),
			sym: JsWord::from(name.token().to_string()),
		}),
		_ => skip("Unknown variable variant", var),
	}
}
