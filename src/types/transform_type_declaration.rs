use crate::prelude::*;

pub fn transform_type_declaration(type_declaration: &lua_ast::types::TypeDeclaration) -> Stmt {
	Stmt::Decl(Decl::TsTypeAlias(TsTypeAliasDecl {
		span: Default::default(),
		declare: false,
		id: Ident {
			span: Default::default(),
			optional: false,
			sym: JsWord::from(type_declaration.type_name().token().to_string()),
		},
		type_params: transform_type_generic(type_declaration.generics()),
		type_ann: boxed(transform_type(type_declaration.type_definition())),
	}))
}
