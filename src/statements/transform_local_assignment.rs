use crate::prelude::*;

pub fn transform_local_assignment(local_assignment: &lua_ast::LocalAssignment) -> Stmt {
	Stmt::Decl(Decl::Var(VarDecl {
		span: Default::default(),
		declare: false,
		kind: VarDeclKind::Let,
		decls: {
			let names = local_assignment.names();
			let expressions = local_assignment.expressions();
			if local_assignment.equal_token().is_some() {
				vec![VarDeclarator {
					span: Default::default(),
					definite: false,
					name: if names.len() == 1 {
						Pat::Ident(BindingIdent::from(Ident::new(
							JsWord::from(names.iter().next().unwrap().token().to_string()),
							Default::default(),
						)))
					} else {
						Pat::Array(ArrayPat {
							span: Default::default(),
							optional: false,
							type_ann: None,
							elems: names
								.iter()
								.map(|name| {
									Some(Pat::Ident(BindingIdent::from(Ident::new(
										JsWord::from(name.token().to_string()),
										Default::default(),
									))))
								})
								.collect(),
						})
					},
					init: {
						if expressions.len() != 1 {
							Some(boxed(skip(
								"multiple expressions in variable assignment not supported",
								local_assignment,
							)))
						} else {
							expressions
								.iter()
								.next()
								.map(|e| boxed(transform_expression(e)))
						}
					},
				}]
			} else {
				names
					.iter()
					.map(|name| VarDeclarator {
						span: Default::default(),
						init: None,
						definite: false,
						name: Pat::Ident(BindingIdent::from(Ident::new(
							JsWord::from(name.token().to_string()),
							Default::default(),
						))),
					})
					.collect()
			}
		},
	}))
}
