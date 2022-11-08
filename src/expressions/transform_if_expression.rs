use crate::prelude::*;

pub fn transform_if_expression(expression: &lua_ast::types::IfExpression) -> Box<Expr> {
	boxed(Expr::Cond(CondExpr {
		span: Default::default(),
		test: transform_expression(expression.condition()),
		cons: transform_expression(expression.if_expression()),
		alt: match expression.else_if_expressions() {
			None => parens(transform_expression(expression.else_expression())),
			Some(else_ifs) => else_ifs.iter().rev().fold(
				transform_expression(expression.else_expression()),
				|acc, else_if| {
					boxed(Expr::Cond(CondExpr {
						span: Default::default(),
						test: parens(transform_expression(else_if.condition())),
						cons: parens(transform_expression(else_if.expression())),
						alt: acc,
					}))
				},
			),
		},
	}))
}
