use crate::prelude::*;

pub fn transform_statement(stmt: &lua_ast::Stmt) -> Vec<Stmt> {
	match stmt {
		lua_ast::Stmt::Assignment(assignment) => vec![transform_assignment(assignment)],
		lua_ast::Stmt::Do(do_stmt) => vec![transform_block(do_stmt.block())],
		lua_ast::Stmt::FunctionCall(function_call) => vec![Stmt::Expr(ExprStmt {
			span: Default::default(),
			expr: boxed(transform_function_call(function_call)),
		})],
		lua_ast::Stmt::FunctionDeclaration(declaration) => {
			vec![transform_function_declaration(declaration)]
		}
		lua_ast::Stmt::GenericFor(generic_for) => vec![transform_generic_for(generic_for)],
		lua_ast::Stmt::If(if_stmt) => vec![Stmt::If(IfStmt {
			span: Default::default(),
			test: boxed(transform_expression(if_stmt.condition())),
			cons: boxed(transform_block(if_stmt.block())),
			alt: if_stmt.else_block().map(transform_block).map(boxed),
		})],
		lua_ast::Stmt::LocalAssignment(local_assignment) => {
			vec![transform_local_assignment(local_assignment)]
		}
		lua_ast::Stmt::LocalFunction(declaration) => vec![transform_local_function(declaration)],
		lua_ast::Stmt::NumericFor(numeric_for) => vec![transform_numeric_for(numeric_for)],
		lua_ast::Stmt::Repeat(repeat) => vec![Stmt::DoWhile(DoWhileStmt {
			span: Default::default(),
			body: boxed(transform_block(repeat.block())),
			test: boxed(Expr::Unary(UnaryExpr {
				span: Default::default(),
				op: UnaryOp::Bang,
				arg: boxed(transform_expression(repeat.until())),
			})),
		})],
		lua_ast::Stmt::While(while_stmt) => vec![Stmt::While(WhileStmt {
			span: Default::default(),
			test: boxed(transform_expression(while_stmt.condition())),
			body: boxed(transform_block(while_stmt.block())),
		})],
		lua_ast::Stmt::CompoundAssignment(compound_assignment) => {
			vec![transform_compound_assignment(compound_assignment)]
		}
		lua_ast::Stmt::ExportedTypeDeclaration(declaration) => {
			vec![
				transform_type_declaration(declaration.type_declaration()),
				/*
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
				*/
				skip_stmt(
					"ModuleItem not allowed in architecture, export manually",
					declaration,
				),
			]
		}
		lua_ast::Stmt::TypeDeclaration(declaration) => {
			vec![transform_type_declaration(declaration)]
		}
		_ => vec![skip_stmt("Unknown statement type", stmt)],
	}
}
