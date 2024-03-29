use crate::prelude::*;

pub fn transform_assignment(assignment: &lua_ast::Assignment) -> Stmt {
	Stmt::Expr(ExprStmt {
		span: Default::default(),
		expr: boxed(Expr::Assign({
			let names = assignment.variables();
			let expressions = assignment.expressions();
			AssignExpr {
				span: Default::default(),
				op: AssignOp::Assign,
				left: if names.len() == 1 {
					PatOrExpr::Expr(transform_var(names.iter().next().unwrap()))
				} else {
					PatOrExpr::Pat(boxed(Pat::Array(ArrayPat {
						span: Default::default(),
						optional: false,
						type_ann: None,
						elems: names
							.iter()
							.map(|name| Some(Pat::Expr(transform_var(name))))
							.collect(),
					})))
				},
				right: {
					if expressions.len() != 1 {
						boxed(Expr::Array(ArrayLit {
							span: Default::default(),
							elems: expressions
								.iter()
								.map(|exp| {
									Some(ExprOrSpread {
										spread: None,
										expr: transform_expression(exp),
									})
								})
								.collect(),
						}))
					} else {
						transform_expression(expressions.iter().next().unwrap())
					}
				},
			}
		})),
	})
}
