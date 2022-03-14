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
		lua_ast::Stmt::GenericFor(generic_for) => transform_generic_for(generic_for),
		lua_ast::Stmt::If(if_stmt) => transform_if_statement(if_stmt),
		lua_ast::Stmt::LocalAssignment(local_assignment) => {
			transform_local_assignment(local_assignment)
		}
		lua_ast::Stmt::LocalFunction(declaration) => transform_local_function(declaration),
		lua_ast::Stmt::NumericFor(numeric_for) => transform_numeric_for(numeric_for),
		lua_ast::Stmt::Repeat(repeat) => Stmt::DoWhile(DoWhileStmt {
			span: Default::default(),
			body: boxed(transform_block(repeat.block())),
			test: boxed(Expr::Unary(UnaryExpr {
				span: Default::default(),
				op: UnaryOp::Bang,
				arg: boxed(transform_expression(repeat.until())),
			})),
		}),
		lua_ast::Stmt::While(while_stmt) => Stmt::While(WhileStmt {
			span: Default::default(),
			test: boxed(transform_expression(while_stmt.condition())),
			body: boxed(transform_block(while_stmt.block())),
		}),
		lua_ast::Stmt::CompoundAssignment(compound_assignment) => {
			transform_compound_assignment(compound_assignment)
		}
		lua_ast::Stmt::ExportedTypeDeclaration(declaration) => skip_stmt(
			"Exported type declaration only allowed as top-level",
			declaration,
		),
		lua_ast::Stmt::TypeDeclaration(declaration) => transform_type_declaration(declaration),
		_ => skip_stmt("Unknown statement type", stmt),
	}
}
