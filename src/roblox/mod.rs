use crate::prelude::*;
use itertools::Itertools;
use rbx_dom_weak::*;
use rbx_types::Ref;

fn refs_to_sorted_instances<'a>(
	dom: &WeakDom,
	iter: impl Iterator<Item = &'a Ref>,
) -> impl Iterator<Item = &Instance> {
	iter.map(|&r| dom.get_by_ref(r).unwrap())
		.unique_by(|c| &c.name)
		.sorted_by(|a, b| Ord::cmp(&a.name, &b.name))
}

fn transform_children(dom: &WeakDom, children: &[Ref]) -> Vec<TsTypeElement> {
	refs_to_sorted_instances(dom, children.iter())
		.map(|child| transform_instance(dom, child))
		.collect()
}

fn transform_instance(dom: &WeakDom, instance: &Instance) -> TsTypeElement {
	TsTypeElement::TsPropertySignature(TsPropertySignature {
		span: Default::default(),
		readonly: false,
		key: boxed(Expr::Ident(ident(instance.name.clone()))),
		computed: false,
		optional: false,
		type_params: None,
		type_ann: Some(boxed(TsTypeAnn {
			span: Default::default(),
			type_ann: boxed({
				let class_type = TsType::TsTypeRef(TsTypeRef {
					span: Default::default(),
					type_name: TsEntityName::Ident(ident(instance.class.clone())),
					type_params: None,
				});
				let children = instance.children();
				if !children.is_empty() {
					TsType::TsUnionOrIntersectionType(
						TsUnionOrIntersectionType::TsIntersectionType(TsIntersectionType {
							span: Default::default(),
							types: vec![
								boxed(class_type),
								boxed(TsType::TsTypeLit(TsTypeLit {
									span: Default::default(),
									members: transform_children(dom, instance.children()),
								})),
							],
						}),
					)
				} else {
					class_type
				}
			}),
		})),
		init: None,
		params: vec![],
	})
}

pub fn transform_dom(dom: WeakDom) -> Vec<ModuleItem> {
	refs_to_sorted_instances(&dom, dom.root().children().iter())
		.map(|instance| {
			ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl {
				span: Default::default(),
				decl: Decl::TsInterface(boxed(TsInterfaceDecl {
					span: Default::default(),
					id: Ident {
						span: Default::default(),
						optional: false,
						sym: JsWord::from(instance.name.clone()),
					},
					declare: false,
					type_params: None,
					extends: vec![TsExprWithTypeArgs {
						span: Default::default(),
						expr: boxed(Expr::Ident(ident(instance.class.clone()))),
						type_args: None,
					}],
					body: TsInterfaceBody {
						span: Default::default(),
						body: transform_children(&dom, instance.children()),
					},
				})),
			}))
		})
		.collect()
}
