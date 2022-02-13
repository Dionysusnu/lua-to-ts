use crate::prelude::*;

fn skip_double_rest(param: &lua_ast::Parameter) -> Pat {
	Pat::Expr(boxed(skip(
		&format!(
			"Double use of `{}` or ... in function parameters",
			REST_ARGS_NAME
		),
		param,
	)))
}

pub fn transform_function_params<'a>(
	params: impl Iterator<Item = &'a lua_ast::Parameter>,
	mut type_specifiers: impl Iterator<Item = Option<&'a lua_ast::types::TypeSpecifier>>,
) -> Vec<Pat> {
	let mut has_args_or_ellipse = false;
	params
		.map(|param| {
			let type_specifier = type_specifiers.next().flatten();
			match param {
				lua_ast::Parameter::Name(name) => Pat::Ident(BindingIdent {
					id: Ident {
						span: Default::default(),
						sym: JsWord::from({
							let name = name.token().to_string();
							if name == REST_ARGS_NAME {
								if has_args_or_ellipse {
									return skip_double_rest(param);
								} else {
									has_args_or_ellipse = true;
								};
							}
							name
						}),
						optional: false,
					},
					type_ann: type_specifier.map(transform_type_specifier),
				}),
				lua_ast::Parameter::Ellipse(_) => {
					if has_args_or_ellipse {
						return skip_double_rest(param);
					} else {
						has_args_or_ellipse = true;
					};
					Pat::Rest(RestPat {
						span: Default::default(),
						dot3_token: Default::default(),
						type_ann: type_specifier.map(|t| TsTypeAnn {
							span: Default::default(),
							// Lua rest param is the individual type
							// TS requires array of individual type
							type_ann: boxed(TsType::TsArrayType(TsArrayType {
								span: Default::default(),
								elem_type: boxed(transform_type(t.type_info())),
							})),
						}),
						arg: boxed(Pat::Ident(BindingIdent {
							// type_ann already done above
							type_ann: None,
							id: Ident {
								span: Default::default(),
								sym: JsWord::from(REST_ARGS_NAME),
								optional: false,
							},
						})),
					})
				}
				_ => Pat::Expr(boxed(skip("Unknown parameter type", param))),
			}
		})
		.collect()
}
