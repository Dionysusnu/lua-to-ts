use crate::prelude::*;

pub fn transform_type_info(type_info: &lua_ast::types::TypeInfo) -> Box<TsTypeAnn> {
	boxed(TsTypeAnn {
		span: Default::default(),
		type_ann: transform_type(type_info),
	})
}
