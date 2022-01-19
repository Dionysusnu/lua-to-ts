use crate::prelude::*;

pub fn transform_call(call: &lua_ast::Call, base: Expr) -> Expr {
	let mut result = base;
	let args = if let lua_ast::Call::MethodCall(method_call) = call {
		result = Expr::Member(MemberExpr {
			span: Default::default(),
			obj: boxed(result),
			prop: MemberProp::Ident(Ident {
				span: Default::default(),
				optional: false,
				sym: JsWord::from(method_call.name().token().to_string()),
			}),
		});
		method_call.args()
	} else if let lua_ast::Call::AnonymousCall(args) = call {
		args
	} else {
		return skip("Unknown call type", call);
	};
	Expr::Call(CallExpr {
		span: Default::default(),
		type_args: None,
		callee: Callee::Expr(boxed(result)),
		args: transform_function_args(args),
	})
}
