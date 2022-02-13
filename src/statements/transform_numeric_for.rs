use crate::prelude::*;

pub fn transform_numeric_for(numeric_for: &lua_ast::NumericFor) -> Stmt {
	let op = if let Some(expr) = numeric_for.step() {
		if let lua_ast::Expression::Value {
			value,
			type_assertion: _,
		} = expr
		{
			if let lua_ast::Value::Number(_) = **value {
				Some(BinaryOp::LtEq)
			} else {
				None
			}
		} else if let lua_ast::Expression::UnaryOperator {
			unop: lua_ast::UnOp::Minus(_),
			expression,
		} = expr
		{
			if let lua_ast::Expression::Value {
				ref value,
				type_assertion: _,
			} = **expression
			{
				if let lua_ast::Value::Number(_) = **value {
					Some(BinaryOp::GtEq)
				} else {
					None
				}
			} else {
				None
			}
		} else {
			None
		}
	} else {
		Some(BinaryOp::LtEq)
	};
	if op.is_none() {
		return skip_stmt(
			"Numeric for loops with non-literal step are not supported",
			numeric_for,
		);
	};
	Stmt::For(ForStmt {
		span: Default::default(),
		init: Some(VarDeclOrExpr::VarDecl(VarDecl {
			span: Default::default(),
			kind: VarDeclKind::Let,
			declare: false,
			decls: vec![VarDeclarator {
				span: Default::default(),
				init: Some(boxed(transform_expression(numeric_for.start()))),
				definite: false,
				name: Pat::Ident(BindingIdent {
					type_ann: numeric_for.type_specifier().map(transform_type_specifier),
					id: Ident {
						span: Default::default(),
						optional: false,
						sym: JsWord::from(numeric_for.index_variable().token().to_string()),
					},
				}),
			}],
		})),
		test: Some(boxed(Expr::Bin(BinExpr {
			span: Default::default(),
			left: boxed(Expr::Ident(Ident {
				span: Default::default(),
				optional: false,
				sym: JsWord::from(numeric_for.index_variable().token().to_string()),
			})),
			op: op.unwrap(),
			right: boxed(transform_expression(numeric_for.end())),
		}))),
		update: Some(boxed(Expr::Assign(AssignExpr {
			span: Default::default(),
			left: PatOrExpr::Expr(boxed(Expr::Ident(Ident {
				span: Default::default(),
				optional: false,
				sym: JsWord::from(numeric_for.index_variable().token().to_string()),
			}))),
			op: AssignOp::AddAssign,
			right: boxed(
				numeric_for
					.step()
					.map(transform_expression)
					.unwrap_or_else(|| {
						Expr::Lit(Lit::Num(Number {
							span: Default::default(),
							value: 1.0,
						}))
					}),
			),
		}))),
		body: boxed(transform_block(numeric_for.block())),
	})
}
