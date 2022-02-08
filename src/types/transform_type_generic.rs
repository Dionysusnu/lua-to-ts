use crate::prelude::*;

fn skipped_type_name() -> Ident {
	Ident {
		span: Default::default(),
		optional: false,
		sym: JsWord::from("FailedToTransformType"),
	}
}

fn transform_type_generic_name(
	generic: &lua_ast::types::GenericParameterInfo,
) -> (Ident, Option<Box<TsType>>) {
	match generic {
		lua_ast::types::GenericParameterInfo::Name(token) => (
			Ident {
				span: Default::default(),
				optional: false,
				sym: JsWord::from(token.token().to_string()),
			},
			None,
		),
		lua_ast::types::GenericParameterInfo::Variadic { .. } => (
			skipped_type_name(),
			Some(boxed(skip_type(
				"variadic type generic not supported",
				generic,
			))),
		),
		_ => (
			skipped_type_name(),
			Some(boxed(skip_type("unknown type generic kind", generic))),
		),
	}
}

pub fn transform_type_generic(
	generic: Option<&lua_ast::types::GenericDeclaration>,
) -> Option<TsTypeParamDecl> {
	generic.map(|generic| TsTypeParamDecl {
		span: Default::default(),
		params: generic
			.generics()
			.iter()
			.map(|generic| {
				let (name, constraint) = transform_type_generic_name(generic.parameter());
				TsTypeParam {
					span: Default::default(),
					name,
					constraint,
					default: generic
						.default_type()
						.map(|default_type| boxed(transform_type(default_type))),
				}
			})
			.collect(),
	})
}
