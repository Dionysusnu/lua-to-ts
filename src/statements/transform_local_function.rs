use crate::prelude::*;

pub fn transform_local_function(declaration: &lua_ast::LocalFunction) -> Stmt {
	Stmt::Decl(Decl::Fn(FnDecl {
		declare: false,
		ident: Ident {
			span: Default::default(),
			sym: JsWord::from(declaration.name().token().to_string()),
			optional: false,
		},
		function: boxed(Function {
			span: Default::default(),
			is_async: false,
			is_generator: false,
			return_type: None,
			type_params: None,
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
		}),
	}))
}
