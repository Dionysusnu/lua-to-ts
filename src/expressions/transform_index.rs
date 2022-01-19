use crate::prelude::*;

pub fn transform_index(index: &lua_ast::Index, base: Expr) -> Expr {
	Expr::Member(MemberExpr {
		span: Default::default(),
		obj: boxed(base),
		prop: match index {
			lua_ast::Index::Brackets {
				expression,
				brackets: _,
			} => MemberProp::Computed(ComputedPropName {
				span: Default::default(),
				expr: boxed(transform_expression(expression)),
			}),
			lua_ast::Index::Dot { name, dot: _ } => MemberProp::Ident(Ident {
				span: Default::default(),
				optional: false,
				sym: JsWord::from(name.token().to_string()),
			}),
			_ => MemberProp::Computed(ComputedPropName {
				span: Default::default(),
				expr: boxed(skip("Unknown index variant", index)),
			}),
		},
	})
}
