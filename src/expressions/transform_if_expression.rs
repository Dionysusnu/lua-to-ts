use crate::prelude::*;

pub fn transform_if_expression(expression: &lua_ast::types::IfExpression) -> Expr {
	Expr::Cond(CondExpr {
		span: Default::default(),
		test: boxed(transform_expression(expression.condition())),
		cons: boxed(transform_expression(expression.if_expression())),
		alt: match expression.else_if_expressions() {
			None => boxed(transform_expression(expression.else_expression())),
			Some(else_ifs) => else_ifs.iter().rev().fold(
				boxed(transform_expression(expression.else_expression())),
				|acc, else_if| {
					boxed(Expr::Cond(CondExpr {
						span: Default::default(),
						test: boxed(transform_expression(else_if.condition())),
						cons: boxed(transform_expression(else_if.expression())),
						alt: acc,
					}))
				},
			),
		},
	})
}
