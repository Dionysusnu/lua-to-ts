use crate::prelude::*;

pub fn transform_binary_expression(
	op: &lua_ast::BinOp,
	lhs: &lua_ast::Expression,
	rhs: &lua_ast::Expression,
) -> Expr {
	Expr::Bin(BinExpr {
		span: Default::default(),
		op: match op {
			lua_ast::BinOp::And(_) => BinaryOp::LogicalAnd,
			lua_ast::BinOp::Caret(_) => BinaryOp::Exp,
			lua_ast::BinOp::GreaterThan(_) => BinaryOp::Gt,
			lua_ast::BinOp::GreaterThanEqual(_) => BinaryOp::GtEq,
			lua_ast::BinOp::LessThan(_) => BinaryOp::Lt,
			lua_ast::BinOp::LessThanEqual(_) => BinaryOp::LtEq,
			lua_ast::BinOp::Minus(_) => BinaryOp::Sub,
			lua_ast::BinOp::Or(_) => BinaryOp::LogicalOr,
			lua_ast::BinOp::Percent(_) => BinaryOp::Mod,
			lua_ast::BinOp::Plus(_) => BinaryOp::Add,
			lua_ast::BinOp::Slash(_) => BinaryOp::Div,
			lua_ast::BinOp::Star(_) => BinaryOp::Mul,
			lua_ast::BinOp::TildeEqual(_) => BinaryOp::NotEqEq,
			lua_ast::BinOp::TwoDots(_) => BinaryOp::Add,
			lua_ast::BinOp::TwoEqual(_) => BinaryOp::EqEqEq,
			_ => return skip("Unknown binary operator", op),
		},
		left: boxed(transform_expression(lhs)),
		right: boxed(transform_expression(rhs)),
	})
}
