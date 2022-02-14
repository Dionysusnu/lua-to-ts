use crate::prelude::*;

pub fn transform_block_statements(block: &lua_ast::Block) -> Vec<Stmt> {
	let mut out: Vec<Stmt> = block.stmts().map(transform_statement).collect();
	if let Some(stmt) = block.last_stmt() {
		out.push(transform_last_statement(stmt));
	};
	out
}
