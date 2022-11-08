use crate::prelude::*;

pub fn transform_prefix_suffixes<'a>(
	prefix: &lua_ast::Prefix,
	suffixes: impl Iterator<Item = &'a lua_ast::Suffix>,
) -> Box<Expr> {
	suffixes.fold(
		match prefix {
			lua_ast::Prefix::Name(token) => boxed(Expr::Ident(Ident {
				optional: false,
				span: Default::default(),
				sym: JsWord::from(token.token().to_string()),
			})),
			lua_ast::Prefix::Expression(expr) => transform_expression(expr),
			_ => skip("Unknown prefix variant", prefix),
		},
		|base, suffix| match suffix {
			lua_ast::Suffix::Call(call) => transform_call(call, base),
			lua_ast::Suffix::Index(index) => transform_index(index, base),
			_ => skip("Unknown suffix variant", suffix),
		},
	)
}
