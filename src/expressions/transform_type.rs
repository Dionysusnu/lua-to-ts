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
		_ => skip_type("TODO", type_info),
	}
}
