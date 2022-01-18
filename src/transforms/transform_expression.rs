use crate::prelude::*;

pub fn transform_expression(expr: &lua_ast::Expression) -> Expr {
	match expr {
		lua_ast::Expression::Parentheses {
			contained: _,
			expression,
		} => Expr::Paren(ParenExpr {
			span: Default::default(),
			expr: boxed(transform_expression(expression)),
		}),
		lua_ast::Expression::UnaryOperator { unop, expression } => {
			transform_unary_expression(unop, expression)
		}
		lua_ast::Expression::Value {
			value,
			type_assertion,
		} => {
			let expr = transform_value(value);
			if let Some(type_assertion) = type_assertion {
				Expr::TsAs(TsAsExpr {
					span: Default::default(),
					expr: boxed(expr),
					type_ann: boxed(transform_type(type_assertion)),
				})
			} else {
				expr
			}
		}
		_ => skip("Unknown expression variant", expr),
	}
}
