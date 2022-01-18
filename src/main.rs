mod transforms;
mod util;

mod prelude;
use crate::prelude::*;

use full_moon::visitors::Visitor;
use std::error::Error;
use std::{env, fs, process};
use swc::common::{sync::Lrc, SourceMap};
use swc_ecma_codegen::{text_writer::JsWriter, Emitter};

struct LocalVariableVisitor {
	ts_ast: Module,
}

impl Visitor for LocalVariableVisitor {
	fn visit_assignment(&mut self, assignment: &lua_ast::Assignment) {
		self.ts_ast.body.push(ModuleItem::Stmt(Stmt::Expr(ExprStmt {
			span: Default::default(),
			expr: boxed(Expr::Assign({
				let names = assignment.variables();
				let expressions = assignment.expressions();
				AssignExpr {
					span: Default::default(),
					op: AssignOp::Assign,
					left: if names.len() == 1 {
						PatOrExpr::Expr(boxed(transform_var(names.iter().next().unwrap())))
					} else {
						PatOrExpr::Pat(boxed(Pat::Array(ArrayPat {
							span: Default::default(),
							optional: false,
							type_ann: None,
							elems: names
								.iter()
								.map(|name| Some(Pat::Expr(boxed(transform_var(name)))))
								.collect(),
						})))
					},
					right: {
						if expressions.len() != 1 {
							boxed(skip(
								"multiple expressions in assignment not implemented",
								assignment,
							))
						} else {
							boxed(transform_expression(expressions.iter().next().unwrap()))
						}
					},
				}
			})),
		})));
	}
	fn visit_local_assignment(&mut self, local_assignment: &lua_ast::LocalAssignment) {
		self.ts_ast
			.body
			.push(ModuleItem::Stmt(Stmt::Decl(Decl::Var(VarDecl {
				span: Default::default(),
				declare: false,
				kind: VarDeclKind::Let,
				decls: {
					let names = local_assignment.names();
					let expressions = local_assignment.expressions();
					if local_assignment.equal_token().is_some() {
						vec![VarDeclarator {
							span: Default::default(),
							definite: false,
							name: if names.len() == 1 {
								Pat::Ident(BindingIdent::from(Ident::new(
									JsWord::from(names.iter().next().unwrap().token().to_string()),
									Default::default(),
								)))
							} else {
								Pat::Array(ArrayPat {
									span: Default::default(),
									optional: false,
									type_ann: None,
									elems: names
										.iter()
										.map(|name| {
											Some(Pat::Ident(BindingIdent::from(Ident::new(
												JsWord::from(name.token().to_string()),
												Default::default(),
											))))
										})
										.collect(),
								})
							},
							init: {
								if expressions.len() != 1 {
									Some(boxed(skip("multiple expressions in variable assignment not implemented", local_assignment)))
								} else {
									expressions
										.iter()
										.next()
										.map(|e| boxed(transform_expression(e)))
								}
							},
						}]
					} else {
						names
							.iter()
							.map(|name| VarDeclarator {
								span: Default::default(),
								init: None,
								definite: false,
								name: Pat::Ident(BindingIdent::from(Ident::new(
									JsWord::from(name.token().to_string()),
									Default::default(),
								))),
							})
							.collect()
					}
				},
			}))));
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let args: Vec<String> = env::args().collect();

	if args.len() < 2 {
		eprintln!("Usage: {} <filename>", args[0]);
		process::exit(1);
	}
	let filename = &args[1];

	let contents = fs::read_to_string(filename).unwrap();

	let ast = full_moon::parse(&contents)?;

	let mut visitor = LocalVariableVisitor {
		ts_ast: Module {
			body: vec![],
			span: Default::default(),
			shebang: None,
		},
	};
	visitor.visit_ast(&ast);
	let cm = Lrc::new(SourceMap::default());
	let code = {
		let mut buf = vec![];

		{
			let mut emitter = Emitter {
				cfg: swc_ecma_codegen::Config {
					..Default::default()
				},
				cm: cm.clone(),
				comments: None,
				wr: JsWriter::new(cm.clone(), "\n", &mut buf, None),
			};

			emitter.emit_module(&visitor.ts_ast).unwrap();
		}

		String::from_utf8_lossy(&buf).to_string()
	};

	print!("{}", code);

	Ok(())
}
