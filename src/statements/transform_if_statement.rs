use crate::prelude::*;

pub fn transform_if_statement(statement: &lua_ast::If) -> Stmt {
	Stmt::If(IfStmt {
		span: Default::default(),
		test: transform_expression(statement.condition()),
		cons: boxed(transform_block(statement.block())),
		alt: match statement.else_if() {
			None => statement.else_block().map(transform_block).map(boxed),
			Some(else_ifs) => else_ifs.iter().rev().fold(
				statement.else_block().map(transform_block).map(boxed),
				|acc, else_if| {
					Some(boxed(Stmt::If(IfStmt {
						span: Default::default(),
						test: transform_expression(else_if.condition()),
						cons: boxed(transform_block(else_if.block())),
						alt: acc,
					})))
				},
			),
		},
	})
}
