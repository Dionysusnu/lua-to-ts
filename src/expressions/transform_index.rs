use crate::prelude::*;

pub fn transform_index(index: &lua_ast::Index, base: Box<Expr>) -> Box<Expr> {
	boxed(Expr::Member(MemberExpr {
		span: Default::default(),
		obj: base,
		prop: match index {
			lua_ast::Index::Brackets {
				expression,
				brackets: _,
			} => MemberProp::Computed(ComputedPropName {
				span: Default::default(),
				expr: transform_expression(expression),
			}),
			lua_ast::Index::Dot { name, dot: _ } => MemberProp::Ident(Ident {
				span: Default::default(),
				optional: false,
				sym: JsWord::from(name.token().to_string()),
			}),
			_ => MemberProp::Computed(ComputedPropName {
				span: Default::default(),
				expr: skip("Unknown index variant", index),
			}),
		},
	}))
}
