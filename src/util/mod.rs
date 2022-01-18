pub use crate::prelude::*;

pub fn skip(reason: &str, node: &(impl std::fmt::Debug + ToString)) -> Expr {
	Expr::Call(CallExpr {
		span: Default::default(),
		type_args: Default::default(),
		args: vec![ExprOrSpread {
			spread: None,
			expr: boxed(Expr::Lit(Lit::Str(Str {
				kind: StrKind::Synthesized,
				span: Default::default(),
				value: {
					let mut string = String::from("[lua-to-ts] Failed to transform: `");
					eprintln!("{:?}", node);
					string.push_str(&node.to_string());
					string.push_str("` because: ");
					string.push_str(reason);
					JsWord::from(string)
				},
				has_escape: false,
			}))),
		}],
		callee: Callee::Expr(boxed(Expr::Ident(Ident {
			span: Default::default(),
			sym: JsWord::from("error"),
			optional: false,
		}))),
	})
}

pub fn boxed<T>(arg: T) -> Box<T> {
	Box::new(arg)
}

pub fn property_chain(start: Expr, props: Vec<Ident>) -> Expr {
	let mut result = start;
	for prop in props.into_iter().rev() {
		result = Expr::Member(MemberExpr {
			span: Default::default(),
			obj: boxed(result),
			prop: MemberProp::Ident(prop),
		});
	}
	result
}
