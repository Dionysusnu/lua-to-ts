use crate::prelude::*;

pub fn transform_type_specifier(type_specifier: &lua_ast::types::TypeSpecifier) -> Box<TsTypeAnn> {
	transform_type_info(type_specifier.type_info())
}
