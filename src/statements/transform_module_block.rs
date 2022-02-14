use crate::prelude::*;

pub fn transform_module_block(block: &lua_ast::Block) -> Vec<ModuleItem> {
	let mut out: Vec<ModuleItem> = block
		.stmts()
		.flat_map(|stmt| {
			if let lua_ast::Stmt::ExportedTypeDeclaration(declaration) = stmt {
				vec![
					ModuleItem::Stmt(transform_type_declaration(declaration.type_declaration())),
					ModuleItem::ModuleDecl(ModuleDecl::ExportNamed(NamedExport {
						asserts: None,
						src: None,
						type_only: true,
						span: Default::default(),
						specifiers: vec![ExportSpecifier::Named(ExportNamedSpecifier {
							span: Default::default(),
							is_type_only: true,
							exported: None,
							orig: ModuleExportName::Ident(Ident {
								span: Default::default(),
								optional: false,
								sym: JsWord::from(
									declaration.type_declaration().type_name().to_string(),
								),
							}),
						})],
					})),
				]
			} else {
				vec![ModuleItem::Stmt(transform_statement(stmt))]
			}
		})
		.collect();
	if let Some(lua_ast::LastStmt::Return(return_statement)) = block.last_stmt() {
		out.push(ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultExpr(
			ExportDefaultExpr {
				span: Default::default(),
				expr: boxed(transform_expression(
					return_statement.returns().iter().next().unwrap(),
				)),
			},
		)));
	};
	out
}
