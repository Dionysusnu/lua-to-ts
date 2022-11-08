use crate::prelude::*;

pub fn transform_last_statement(stmt: &lua_ast::LastStmt) -> Stmt {
	match stmt {
		lua_ast::LastStmt::Break(_) => Stmt::Break(BreakStmt {
			span: Default::default(),
			label: None,
		}),
		lua_ast::LastStmt::Continue(_) => Stmt::Continue(ContinueStmt {
			span: Default::default(),
			label: None,
		}),
		lua_ast::LastStmt::Return(return_statement) => Stmt::Return(ReturnStmt {
			span: Default::default(),
			arg: {
				let returns = return_statement.returns();
				match returns.len() {
					0 => None,
					1 => Some(transform_expression(returns.iter().next().unwrap())),
					_ => Some(boxed(Expr::TsAs(TsAsExpr {
						span: Default::default(),
						expr: boxed(Expr::Array(ArrayLit {
							span: Default::default(),
							elems: returns
								.iter()
								.map(transform_expression)
								.map(|expr| ExprOrSpread { spread: None, expr })
								.map(Some)
								.collect(),
						})),
						type_ann: boxed(TsType::TsTypeRef(TsTypeRef {
							span: Default::default(),
							type_name: TsEntityName::Ident(Ident {
								span: Default::default(),
								optional: false,
								sym: JsWord::from("LuaTuple"),
							}),
							type_params: Some(boxed(TsTypeParamInstantiation {
								span: Default::default(),
								params: vec![boxed(TsType::TsTupleType(TsTupleType {
									span: Default::default(),
									elem_types: vec![TsTupleElement {
										span: Default::default(),
										label: None,
										ty: skip_type("Can't infer return types", return_statement),
									}],
								}))],
							})),
						})),
					}))),
				}
			},
		}),
		_ => skip_stmt("Unknown last statement type", stmt),
	}
}
