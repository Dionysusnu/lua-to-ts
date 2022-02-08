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

fn get_node_text(node: &impl node::Node) -> String {
	let mut message = String::new();
	let mut tokens = node.tokens().peekable();
	while let Some(token_ref) = tokens.next() {
		if tokens.peek().is_some() {
			message.push_str(&token_ref.to_string());
		} else {
			for token in token_ref.leading_trivia() {
				message.push_str(&token.to_string());
			}
			message.push_str(&token_ref.token().to_string());
			break;
		}
	}
	message
}

fn get_fail_string(reason: &str, node: &impl node::Node) -> Str {
	#[cfg(debug)]
	eprintln!("{}: {:#?}", reason, node);
	make_string(&format!(
		"[lua-to-ts] Failed to transform: `{}` because: {}",
		get_node_text(node),
		reason
	))
}

pub fn skip(reason: &str, node: &(impl node::Node + std::fmt::Debug + ToString)) -> Expr {
	Expr::Call(CallExpr {
		span: Default::default(),
		type_args: Default::default(),
		args: vec![ExprOrSpread {
			spread: None,
			expr: boxed(Expr::Lit(Lit::Str(get_fail_string(reason, node)))),
		}],
		callee: Callee::Expr(boxed(Expr::Ident(Ident {
			span: Default::default(),
			sym: JsWord::from("error"),
			optional: false,
		}))),
	})
}

pub fn skip_stmt(reason: &str, node: &(impl node::Node + std::fmt::Debug + ToString)) -> Stmt {
	Stmt::Expr(ExprStmt {
		span: Default::default(),
		expr: boxed(skip(reason, node)),
	})
}

pub fn skip_type(reason: &str, node: &(impl node::Node + std::fmt::Debug + ToString)) -> TsType {
	TsType::TsLitType(TsLitType {
		span: Default::default(),
		lit: TsLit::Str(get_fail_string(reason, node)),
	})
}

pub const REST_ARGS_NAME: &str = "_args";
