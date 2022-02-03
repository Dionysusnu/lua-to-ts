use crate::prelude::*;

pub fn transform_block_statements(block: &lua_ast::Block) -> Vec<Stmt> {
	let mut out: Vec<Stmt> = block.stmts().flat_map(transform_statement).collect();
	if let Some(stmt) = block.last_stmt() {
		out.push(transform_last_statement(stmt));
	};
	out
}
