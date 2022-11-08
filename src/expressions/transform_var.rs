use crate::prelude::*;

pub fn transform_var(var: &lua_ast::Var) -> Box<Expr> {
	match var {
		lua_ast::Var::Expression(expr) => transform_prefix_suffixes(expr.prefix(), expr.suffixes()),
		lua_ast::Var::Name(name) => boxed(Expr::Ident(Ident {
			optional: false,
			span: Default::default(),
			sym: JsWord::from(name.token().to_string()),
		})),
		_ => skip("Unknown variable variant", var),
	}
}
