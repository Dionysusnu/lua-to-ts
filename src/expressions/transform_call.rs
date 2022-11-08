use crate::prelude::*;

pub fn transform_call(call: &lua_ast::Call, base: Box<Expr>) -> Box<Expr> {
	let (result, args) = match call {
		lua_ast::Call::MethodCall(method_call) => (
			boxed(Expr::Member(MemberExpr {
				span: Default::default(),
				obj: base,
				prop: MemberProp::Ident(Ident {
					span: Default::default(),
					optional: false,
					sym: JsWord::from(method_call.name().token().to_string()),
				}),
			})),
			transform_function_args(method_call.args()),
		),
		lua_ast::Call::AnonymousCall(args) => {
			if let Expr::Member(MemberExpr {
				prop: MemberProp::Ident(Ident {
					sym: js_word!("new"),
					..
				}),
				obj,
				span: _,
			}) = *base
			{
				return boxed(Expr::New(NewExpr {
					span: Default::default(),
					callee: obj,
					args: Some(transform_function_args(args)),
					type_args: None,
				}));
			}
			(base, transform_function_args(args))
		}
		_ => (
			base,
			vec![ExprOrSpread {
				spread: None,
				expr: skip("Unknown call type", call),
			}],
		),
	};
	boxed(Expr::Call(CallExpr {
		span: Default::default(),
		type_args: None,
		callee: Callee::Expr(result),
		args,
	}))
}
