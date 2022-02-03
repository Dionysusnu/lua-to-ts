use crate::prelude::*;

fn skip_type(reason: &str, node: &(impl std::fmt::Debug + ToString)) -> TsTypeParam {
	let mut message = String::from(reason);
	message.push_str(&node.to_string());
	TsTypeParam {
		span: Default::default(),
		name: Ident {
			span: Default::default(),
			optional: false,
			sym: JsWord::from("FailedToTransformType"),
		},
		constraint: Some(boxed(TsType::TsLitType(TsLitType {
			span: Default::default(),
			lit: TsLit::Str(make_string(&message)),
		}))),
		default: None,
	}
}

pub fn transform_type_declaration(type_declaration: &lua_ast::types::TypeDeclaration) -> Stmt {
	Stmt::Decl(Decl::TsTypeAlias(TsTypeAliasDecl {
		span: Default::default(),
		declare: false,
		id: Ident {
			span: Default::default(),
			optional: false,
			sym: JsWord::from(type_declaration.type_name().token().to_string()),
		},
		type_params: type_declaration.generics().map(|generic| TsTypeParamDecl {
			span: Default::default(),
			params: generic
				.generics()
				.iter()
				.map(|generic| match generic {
					lua_ast::types::GenericDeclarationParameter::Name(token) => TsTypeParam {
						span: Default::default(),
						name: Ident {
							span: Default::default(),
							optional: false,
							sym: JsWord::from(token.token().to_string()),
						},
						constraint: None,
						default: None,
					},
					lua_ast::types::GenericDeclarationParameter::Variadic { .. } => {
						skip_type("variadic type generic not supported", generic)
					}
					_ => skip_type("unknown type generic kind", generic),
				})
				.collect(),
		}),
		type_ann: boxed(transform_type(type_declaration.type_definition())),
	}))
}
