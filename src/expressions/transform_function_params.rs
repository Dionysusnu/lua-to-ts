use crate::prelude::*;

const REST_ARGS_NAME: &str = "args";

pub fn transform_function_params<'a>(
	params: impl Iterator<Item = &'a lua_ast::Parameter>,
) -> Vec<Pat> {
	let mut has_args_or_ellipse = false;
	params
		.map(|param| match param {
			lua_ast::Parameter::Name(name) => Pat::Ident(BindingIdent::from(Ident {
				span: Default::default(),
				sym: JsWord::from({
					let name = name.token().to_string();
					if name == "args" {
						if has_args_or_ellipse {
							return Pat::Expr(boxed(skip(
								"Double use of args or ... in function parameters",
								param,
							)));
						} else {
							has_args_or_ellipse = true;
						};
					}
					name
				}),
				optional: false,
			})),
			lua_ast::Parameter::Ellipse(_) => {
				if has_args_or_ellipse {
					return Pat::Expr(boxed(skip(
						"Double use of args or ... in function parameters",
						param,
					)));
				} else {
					has_args_or_ellipse = true;
				};
				Pat::Rest(RestPat {
					span: Default::default(),
					dot3_token: Default::default(),
					type_ann: None,
					arg: boxed(Pat::Ident(BindingIdent::from(Ident {
						span: Default::default(),
						sym: JsWord::from(REST_ARGS_NAME),
						optional: false,
					}))),
				})
			}
			_ => Pat::Expr(boxed(skip("Unknown parameter type", param))),
		})
		.collect()
}
