use crate::prelude::*;

pub fn transform_type_specifier(type_specifier: &lua_ast::types::TypeSpecifier) -> TsTypeAnn {
	TsTypeAnn {
		span: Default::default(),
		type_ann: boxed(transform_type(type_specifier.type_info())),
	}
}
