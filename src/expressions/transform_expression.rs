use crate::prelude::*;

pub fn transform_expression(expr: &lua_ast::Expression) -> Box<Expr> {
	match expr {
		lua_ast::Expression::BinaryOperator { lhs, binop, rhs } => {
			transform_binary_expression(binop, lhs, rhs)
		}
		lua_ast::Expression::Parentheses {
			contained: _,
			expression,
		} => boxed(Expr::Paren(ParenExpr {
			span: Default::default(),
			expr: transform_expression(expression),
		})),
		lua_ast::Expression::UnaryOperator { unop, expression } => {
			transform_unary_expression(unop, expression)
		}
		lua_ast::Expression::Value {
			value,
			type_assertion,
		} => {
			let expr = transform_value(value);
			if let Some(type_assertion) = type_assertion {
				boxed(Expr::TsAs(TsAsExpr {
					span: Default::default(),
					expr,
					type_ann: transform_type(type_assertion.cast_to()),
				}))
			} else {
				expr
			}
		}
		_ => skip("Unknown expression variant", expr),
	}
}
