use crate::prelude::*;

pub fn transform_type(type_info: &lua_ast::types::TypeInfo) -> Box<TsType> {
	match type_info {
		lua_ast::types::TypeInfo::Array {
			type_info,
			braces: _,
		} => boxed(TsType::TsArrayType(TsArrayType {
			span: Default::default(),
			elem_type: transform_type(type_info),
		})),
		lua_ast::types::TypeInfo::Basic(token) => boxed(TsType::TsTypeRef(TsTypeRef {
			span: Default::default(),
			type_params: None,
			type_name: TsEntityName::Ident(Ident {
				span: Default::default(),
				optional: false,
				sym: JsWord::from(token.token().to_string()),
			}),
		})),
		lua_ast::types::TypeInfo::String(token) => boxed(TsType::TsLitType(TsLitType {
			span: Default::default(),
			lit: TsLit::Str(make_string(&token.token().to_string())),
		})),
		lua_ast::types::TypeInfo::Boolean(token) => boxed(TsType::TsLitType(TsLitType {
			span: Default::default(),
			lit: TsLit::Bool(Bool {
				span: Default::default(),
				value: token.token().to_string() == "true",
			}),
		})),
		lua_ast::types::TypeInfo::Callback {
			generics,
			arguments,
			return_type,
			parentheses: _,
			arrow: _,
		} => boxed(TsType::TsFnOrConstructorType(
			TsFnOrConstructorType::TsFnType(TsFnType {
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
										.as_deref()
										.unwrap_or("_"),
								),
							},
							type_ann: Some(transform_type_info(argument.type_info())),
						})
					})
					.collect(),
				type_ann: boxed(TsTypeAnn {
					span: Default::default(),
					type_ann: transform_type(return_type),
				}),
			}),
		)),
		lua_ast::types::TypeInfo::Generic {
			base,
			generics,
			arrows: _,
		} => boxed(TsType::TsTypeRef(TsTypeRef {
			span: Default::default(),
			type_params: Some(boxed(TsTypeParamInstantiation {
				span: Default::default(),
				params: generics.iter().map(transform_type).collect(),
			})),
			type_name: TsEntityName::Ident(Ident {
				span: Default::default(),
				optional: false,
				sym: JsWord::from(base.token().to_string()),
			}),
		})),
		lua_ast::types::TypeInfo::GenericPack {
			name: _,
			ellipse: _,
		} => skip_type("TS does not allow variadic type arguments", type_info),
		lua_ast::types::TypeInfo::Intersection {
			left,
			right,
			ampersand: _,
		} => boxed(TsType::TsUnionOrIntersectionType(
			TsUnionOrIntersectionType::TsIntersectionType(TsIntersectionType {
				span: Default::default(),
				types: vec![transform_type(left), transform_type(right)],
			}),
		)),
		lua_ast::types::TypeInfo::Module {
			module,
			type_info,
			punctuation: _,
		} => boxed(TsType::TsTypeRef(TsTypeRef {
			span: Default::default(),
			type_name: TsEntityName::TsQualifiedName(boxed(TsQualifiedName {
				left: TsEntityName::Ident(Ident {
					span: Default::default(),
					optional: false,
					sym: JsWord::from(module.token().to_string()),
				}),
				right: Ident {
					span: Default::default(),
					optional: false,
					sym: JsWord::from(match **type_info {
						lua_ast::types::IndexedTypeInfo::Basic(ref token) => {
							token.token().to_string()
						}
						lua_ast::types::IndexedTypeInfo::Generic {
							ref base,
							arrows: _,
							generics: _,
						} => base.token().to_string(),
						_ => unimplemented!("Unknown IndexedTypeInfo kind"),
					}),
				},
			})),
			type_params: match **type_info {
				lua_ast::types::IndexedTypeInfo::Basic(_) => None,
				lua_ast::types::IndexedTypeInfo::Generic {
					base: _,
					arrows: _,
					ref generics,
				} => Some(boxed(TsTypeParamInstantiation {
					span: Default::default(),
					params: generics.iter().map(transform_type).collect(),
				})),
				_ => unreachable!("Already panicked for unknown IndexedTypeInfo above"),
			},
		})),
		lua_ast::types::TypeInfo::Optional {
			base,
			question_mark: _,
		} => boxed(TsType::TsUnionOrIntersectionType(
			TsUnionOrIntersectionType::TsUnionType(TsUnionType {
				span: Default::default(),
				types: vec![
					transform_type(base),
					boxed(TsType::TsKeywordType(TsKeywordType {
						span: Default::default(),
						kind: TsKeywordTypeKind::TsUndefinedKeyword,
					})),
				],
			}),
		)),
		lua_ast::types::TypeInfo::Table { fields, braces: _ } => transform_table_type(fields),
		lua_ast::types::TypeInfo::Typeof {
			inner: _,
			typeof_token: _,
			parentheses: _,
		} => skip_type("TS has no functional equivalent of Luau typeof", type_info),
		lua_ast::types::TypeInfo::Tuple {
			types,
			parentheses: _,
		} => {
			if types.len() == 1 {
				boxed(TsType::TsParenthesizedType(TsParenthesizedType {
					span: Default::default(),
					type_ann: transform_type(types.iter().next().unwrap()),
				}))
			} else {
				boxed(TsType::TsTypeRef(TsTypeRef {
					span: Default::default(),
					type_name: TsEntityName::Ident(Ident {
						span: Default::default(),
						optional: false,
						sym: JsWord::from("LuaTuple"),
					}),
					type_params: Some(boxed(TsTypeParamInstantiation {
						span: Default::default(),
						params: vec![boxed(TsType::TsTupleType(TsTupleType {
							span: Default::default(),
							elem_types: types
								.iter()
								.map(|type_info| TsTupleElement {
									span: Default::default(),
									label: None,
									ty: match type_info {
										lua_ast::types::TypeInfo::Variadic {
											type_info,
											ellipse: _,
										} => boxed(TsType::TsRestType(TsRestType {
											span: Default::default(),
											type_ann: boxed(TsType::TsArrayType(TsArrayType {
												span: Default::default(),
												elem_type: transform_type(type_info),
											})),
										})),
										_ => transform_type(type_info),
									},
								})
								.collect(),
						}))],
					})),
				}))
			}
		}
		lua_ast::types::TypeInfo::Union {
			left,
			right,
			pipe: _,
		} => boxed(TsType::TsUnionOrIntersectionType(
			TsUnionOrIntersectionType::TsUnionType(TsUnionType {
				span: Default::default(),
				types: vec![transform_type(left), transform_type(right)],
			}),
		)),
		lua_ast::types::TypeInfo::Variadic {
			type_info,
			ellipse: _,
		} => boxed(TsType::TsTypeRef(TsTypeRef {
			span: Default::default(),
			type_name: TsEntityName::Ident(Ident {
				span: Default::default(),
				optional: false,
				sym: JsWord::from("LuaTuple"),
			}),
			type_params: Some(boxed(TsTypeParamInstantiation {
				span: Default::default(),
				params: vec![boxed(TsType::TsArrayType(TsArrayType {
					span: Default::default(),
					elem_type: transform_type(type_info),
				}))],
			})),
		})),
		lua_ast::types::TypeInfo::VariadicPack {
			name: _,
			ellipse: _,
		} => skip_type("TS does not allow variadic type arguments", type_info),
		_ => skip_type("Unknown TypeInfo kind", type_info),
	}
}
