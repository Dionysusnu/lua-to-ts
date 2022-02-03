pub use crate::prelude::*;

pub fn boxed<T>(arg: T) -> Box<T> {
	Box::new(arg)
}

pub fn make_string(content: &str) -> Str {
	Str {
		span: Default::default(),
		has_escape: false,
		kind: StrKind::Synthesized,
		value: JsWord::from(content),
	}
}

pub fn skip(reason: &str, node: &(impl std::fmt::Debug + ToString)) -> Expr {
	let mut string = String::from("[lua-to-ts] Failed to transform: `");
	// eprintln!("{:?}", node);
	string.push_str(&node.to_string());
	string.push_str("` because: ");
	string.push_str(reason);
	Expr::Call(CallExpr {
		span: Default::default(),
		type_args: Default::default(),
		args: vec![ExprOrSpread {
			spread: None,
			expr: boxed(Expr::Lit(Lit::Str(make_string(&string)))),
		}],
		callee: Callee::Expr(boxed(Expr::Ident(Ident {
			span: Default::default(),
			sym: JsWord::from("error"),
			optional: false,
		}))),
	})
}

pub fn skip_stmt(reason: &str, node: &(impl std::fmt::Debug + ToString)) -> Stmt {
	Stmt::Expr(ExprStmt {
		span: Default::default(),
		expr: boxed(skip(reason, node)),
	})
}

pub fn skip_type(reason: &str, node: &(impl std::fmt::Debug + ToString)) -> TsType {
	let mut message = String::from("[lua-to-ts] Failed to transform: `");
	// eprintln!("{:?}", node);
	message.push_str(&node.to_string());
	message.push_str("` because: ");
	message.push_str(reason);
	TsType::TsLitType(TsLitType {
		span: Default::default(),
		lit: TsLit::Str(make_string(&message)),
	})
}
