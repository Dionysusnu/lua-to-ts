use crate::prelude::*;

pub fn transform_statement(stmt: &lua_ast::Stmt) -> Stmt {
	match stmt {
		lua_ast::Stmt::Assignment(assignment) => transform_assignment(assignment),
		lua_ast::Stmt::Do(do_stmt) => transform_block(do_stmt.block()),
		lua_ast::Stmt::FunctionCall(function_call) => Stmt::Expr(ExprStmt {
			span: Default::default(),
			expr: boxed(transform_function_call(function_call)),
		}),
		lua_ast::Stmt::FunctionDeclaration(declaration) => {
			transform_function_declaration(declaration)
		}
		lua_ast::Stmt::GenericFor(generic_for) => {
			skip_stmt("generic for loops not yet implemented", generic_for)
		}
		lua_ast::Stmt::If(if_stmt) => skip_stmt("if statements not yet implemented", if_stmt),
		lua_ast::Stmt::LocalAssignment(local_assignment) => {
			transform_local_assignment(local_assignment)
		}
		lua_ast::Stmt::LocalFunction(declaration) => skip_stmt(
			"local function declarations not yet implemented",
			declaration,
		),
		lua_ast::Stmt::NumericFor(numeric_for) => {
			skip_stmt("numeric for loops not yet implemented", numeric_for)
		}
		lua_ast::Stmt::Repeat(repeat) => skip_stmt("repeat not yet implemented", repeat),
		lua_ast::Stmt::While(while_stmt) => {
			skip_stmt("while loops not yet implemented", while_stmt)
		}
		lua_ast::Stmt::CompoundAssignment(compound_assignment) => skip_stmt(
			"compound assignments not yet implemented",
			compound_assignment,
		),
		lua_ast::Stmt::ExportedTypeDeclaration(declaration) => {
			skip_stmt("exported type declaration not yet implemented", declaration)
		}
		lua_ast::Stmt::TypeDeclaration(declaration) => {
			skip_stmt("type declaration not yet implemented", declaration)
		}
		_ => skip_stmt("Unknown statement type", stmt),
	}
}
