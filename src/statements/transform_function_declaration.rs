use crate::prelude::*;

fn transform_function_name(name: &lua_ast::FunctionName) -> PatOrExpr {
	let mut iter = name.names().iter();
	let mut result = Expr::Ident(Ident {
		span: Default::default(),
		sym: JsWord::from(iter.next().unwrap().token().to_string()),
		optional: false,
	});
	for token in iter {
		result = Expr::Member(MemberExpr {
			span: Default::default(),
			obj: boxed(result),
			prop: MemberProp::Ident(Ident {
				span: Default::default(),
				sym: JsWord::from(token.token().to_string()),
				optional: false,
			}),
		});
	}
	if let Some(token) = name.method_name() {
		result = Expr::Member(MemberExpr {
			span: Default::default(),
			obj: boxed(result),
			prop: MemberProp::Ident(Ident {
				span: Default::default(),
				sym: JsWord::from(token.token().to_string()),
				optional: false,
			}),
		});
	}
	if let Expr::Ident(ident) = result {
		PatOrExpr::Pat(boxed(Pat::Ident(BindingIdent::from(ident))))
	} else {
		PatOrExpr::Expr(boxed(result))
	}
}

pub fn transform_function_declaration(declaration: &lua_ast::FunctionDeclaration) -> Stmt {
	Stmt::Expr(ExprStmt {
		span: Default::default(),
		expr: boxed(Expr::Assign(AssignExpr {
			span: Default::default(),
			left: transform_function_name(declaration.name()),
			op: AssignOp::Assign,
			right: boxed(Expr::Fn(FnExpr {
				ident: None,
				function: Function {
					span: Default::default(),
					is_async: false,
					is_generator: false,
					return_type: declaration
						.body()
						.return_type()
						.map(transform_type_specifier),
					type_params: transform_type_generic(declaration.body().generics()),
					decorators: vec![],
					params: transform_function_params(
						declaration.body().parameters().iter(),
						declaration.body().type_specifiers(),
					)
					.into_iter()
					.map(|param| Param {
						span: Default::default(),
						decorators: vec![],
						pat: param,
					})
					.collect(),
					body: Some(BlockStmt {
						span: Default::default(),
						stmts: transform_block_statements(declaration.body().block()),
					}),
				},
			})),
		})),
	})
}
