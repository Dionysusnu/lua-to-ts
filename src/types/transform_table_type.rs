use crate::prelude::*;

pub fn transform_table_type(
	fields: &lua_ast::punctuated::Punctuated<lua_ast::types::TypeField>,
) -> TsType {
	TsType::TsTypeLit(TsTypeLit {
		span: Default::default(),
		members: fields
			.iter()
			.map(|field| match field.key() {
				lua_ast::types::TypeFieldKey::Name(token) => {
					TsTypeElement::TsPropertySignature(TsPropertySignature {
						span: Default::default(),
						readonly: false,
						key: boxed(Expr::Ident(Ident {
							span: Default::default(),
							optional: false,
							sym: JsWord::from(token.token().to_string()),
						})),
						computed: false,
						optional: matches!(
							field.value(),
							lua_ast::types::TypeInfo::Optional { .. }
						),
						init: None,
						params: vec![],
						type_ann: Some(TsTypeAnn {
							span: Default::default(),
							type_ann: boxed(transform_type(field.value())),
						}),
						type_params: None,
					})
				}
				lua_ast::types::TypeFieldKey::IndexSignature { inner, brackets: _ } => {
					TsTypeElement::TsIndexSignature(TsIndexSignature {
						span: Default::default(),
						readonly: false,
						is_static: false,
						params: vec![TsFnParam::Ident(BindingIdent {
							id: Ident {
								span: Default::default(),
								optional: false,
								sym: JsWord::from("_"),
							},
							type_ann: Some(TsTypeAnn {
								span: Default::default(),
								type_ann: boxed(transform_type(inner)),
							}),
						})],
						type_ann: Some(TsTypeAnn {
							span: Default::default(),
							type_ann: boxed(transform_type(field.value())),
						}),
					})
				}
				_ => unimplemented!("Unknown TypeFieldKey kind"),
			})
			.collect(),
	})
}
