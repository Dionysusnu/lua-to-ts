use crate::prelude::*;

pub fn transform_compound_assignment(assignment: &lua_ast::types::CompoundAssignment) -> Stmt {
	Stmt::Expr(ExprStmt {
		span: Default::default(),
		expr: boxed(Expr::Assign({
			AssignExpr {
				span: Default::default(),
				op: match assignment.compound_operator() {
					lua_ast::types::CompoundOp::PlusEqual(_) => AssignOp::AddAssign,
					lua_ast::types::CompoundOp::MinusEqual(_) => AssignOp::SubAssign,
					lua_ast::types::CompoundOp::StarEqual(_) => AssignOp::MulAssign,
					lua_ast::types::CompoundOp::SlashEqual(_) => AssignOp::DivAssign,
					lua_ast::types::CompoundOp::PercentEqual(_) => AssignOp::ModAssign,
					lua_ast::types::CompoundOp::CaretEqual(_) => AssignOp::ExpAssign,
					lua_ast::types::CompoundOp::TwoDotsEqual(_) => AssignOp::AddAssign,
					_ => {
						return skip_stmt(
							"Unknown compound assignment operator",
							assignment.compound_operator(),
						)
					}
				},
				left: PatOrExpr::Expr(boxed(transform_var(assignment.lhs()))),
				right: boxed(transform_expression(assignment.rhs())),
			}
		})),
	})
}
