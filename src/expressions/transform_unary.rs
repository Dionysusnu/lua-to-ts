use crate::prelude::*;

pub fn transform_unary_expression(
	op: &lua_ast::UnOp,
	expression: &lua_ast::Expression,
) -> Box<Expr> {
	match op {
		lua_ast::UnOp::Hash(_) => boxed(Expr::Call(CallExpr {
			span: Default::default(),
			args: vec![],
			type_args: None,
			callee: Callee::Expr(boxed(Expr::Member(MemberExpr {
				span: Default::default(),
				obj: transform_expression(expression),
				prop: MemberProp::Ident(Ident {
					span: Default::default(),
					sym: JsWord::from("size"),
					optional: false,
				}),
			}))),
		})),
		lua_ast::UnOp::Minus(_) => boxed(Expr::Unary(UnaryExpr {
			span: Default::default(),
			op: UnaryOp::Minus,
			arg: parens(transform_expression(expression)),
		})),
		lua_ast::UnOp::Not(_) => boxed(Expr::Unary(UnaryExpr {
			span: Default::default(),
			op: UnaryOp::Bang,
			arg: parens(transform_expression(expression)),
		})),
		_ => skip("Unknown unary operator", op),
	}
}
