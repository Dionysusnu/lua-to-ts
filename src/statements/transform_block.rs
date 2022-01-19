use crate::prelude::*;

pub fn transform_block(block: &lua_ast::Block) -> Stmt {
	Stmt::Block(BlockStmt {
		span: Default::default(),
		stmts: transform_block_statements(block),
	})
}
