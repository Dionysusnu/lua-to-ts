use crate::prelude::*;

pub fn transform_type(type_info: &lua_ast::types::TypeInfo) -> TsType {
	match type_info {
		lua_ast::types::TypeInfo::Array {
			type_info,
			braces: _,
		} => TsType::TsArrayType(TsArrayType {
			span: Default::default(),
			elem_type: boxed(transform_type(type_info)),
		}),
		lua_ast::types::TypeInfo::Basic(token) => TsType::TsTypeRef(TsTypeRef {
			span: Default::default(),
			type_params: None,
			type_name: TsEntityName::Ident(Ident {
				span: Default::default(),
				optional: false,
				sym: JsWord::from(token.token().to_string()),
			}),
		}),
		lua_ast::types::TypeInfo::String(token) => TsType::TsLitType(TsLitType {
			span: Default::default(),
			lit: TsLit::Str(make_string(&token.token().to_string())),
		}),
		lua_ast::types::TypeInfo::Boolean(token) => TsType::TsLitType(TsLitType {
			span: Default::default(),
			lit: TsLit::Bool(Bool {
				span: Default::default(),
				value: token.token().to_string() == "true",
			}),
		}),
		lua_ast::types::TypeInfo::Callback {
			generics,
			arguments,
			return_type,
			parentheses: _,
			arrow: _,
		} => TsType::TsFnOrConstructorType(TsFnOrConstructorType::TsFnType(TsFnType {
			span: Default::default(),
			type_params: transform_type_generic(generics.as_ref()),
			params: arguments
				.iter()
				.map(|argument| {
					TsFnParam::Ident(BindingIdent {
						id: Ident {
							span: Default::default(),
							optional: false,
							sym: JsWord::from(
								argument
									.name()
									.map(|name| name.0.token().to_string())
									.unwrap_or_else(|| String::from("_")),
							),
						},
						type_ann: Some(TsTypeAnn {
							span: Default::default(),
							type_ann: boxed(transform_type(argument.type_info())),
						}),
					})
				})
				.collect(),
			type_ann: TsTypeAnn {
				span: Default::default(),
				type_ann: boxed(transform_type(return_type)),
			},
		})),
		_ => skip_type("TODO", type_info),
	}
}
