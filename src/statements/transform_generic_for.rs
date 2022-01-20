use crate::prelude::*;

pub fn transform_generic_for(generic_for: &lua_ast::GenericFor) -> Stmt {
	let expression = generic_for.expressions();
	if expression.len() != 1 {
		return skip_stmt(
			"For-in loops with multiple expressions not supported",
			generic_for,
		);
	}
	Stmt::ForOf(ForOfStmt {
		span: Default::default(),
		await_token: None,
		left: VarDeclOrPat::VarDecl(VarDecl {
			span: Default::default(),
			kind: VarDeclKind::Let,
			declare: false,
			decls: vec![VarDeclarator {
				span: Default::default(),
				init: None,
				definite: false,
				name: Pat::Array(ArrayPat {
					span: Default::default(),
					optional: false,
					type_ann: None,
					elems: generic_for
						.names()
						.iter()
						.map(|name| Ident {
							span: Default::default(),
							optional: false,
							sym: JsWord::from(name.token().to_string()),
						})
						.map(BindingIdent::from)
						.map(Pat::Ident)
						.map(Some)
						.collect(),
				}),
			}],
		}),
		right: boxed(transform_expression(expression.iter().next().unwrap())),
		body: boxed(transform_block(generic_for.block())),
	})
}
