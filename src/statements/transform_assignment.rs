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
					PatOrExpr::Expr(boxed(transform_var(names.iter().next().unwrap())))
				} else {
					PatOrExpr::Pat(boxed(Pat::Array(ArrayPat {
						span: Default::default(),
						optional: false,
						type_ann: None,
						elems: names
							.iter()
							.map(|name| Some(Pat::Expr(boxed(transform_var(name)))))
							.collect(),
					})))
				},
				right: {
					if expressions.len() != 1 {
						boxed(skip(
							"multiple expressions in assignment not implemented",
							assignment,
						))
					} else {
						boxed(transform_expression(expressions.iter().next().unwrap()))
					}
				},
			}
		})),
	})
}
