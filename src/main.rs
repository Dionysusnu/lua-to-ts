use std::error::Error;
use std::{env, fs, process};

use full_moon::{ast as lua_ast, tokenizer, visitors::Visitor};

use swc::atoms::JsWord;
use swc::common::{sync::Lrc, SourceMap};
use swc::ecmascript::ast::*;
use swc_ecma_codegen::{text_writer::JsWriter, Emitter};

fn skip(reason: &str, node: &(impl std::fmt::Debug + ToString)) -> Expr {
    Expr::Call(CallExpr {
        span: Default::default(),
        type_args: Default::default(),
        args: vec![ExprOrSpread {
            spread: None,
            expr: boxed(Expr::Lit(Lit::Str(Str {
                kind: StrKind::Synthesized,
                span: Default::default(),
                value: {
                    let mut string = String::from("[lua-to-ts] Failed to transform: `");
                    eprintln!("{:?}", node);
                    string.push_str(&node.to_string());
                    string.push_str("` because: ");
                    string.push_str(reason);
                    JsWord::from(string)
                },
                has_escape: false,
            }))),
        }],
        callee: Callee::Expr(boxed(Expr::Ident(Ident {
            span: Default::default(),
            sym: JsWord::from("error"),
            optional: false,
        }))),
    })
}

fn boxed<T>(arg: T) -> Box<T> {
    Box::new(arg)
}

fn property_chain(start: Expr, props: Vec<Ident>) -> Expr {
    let mut result = start;
    for prop in props.into_iter().rev() {
        result = Expr::Member(MemberExpr {
            span: Default::default(),
            obj: boxed(result),
            prop: MemberProp::Ident(prop),
        });
    }
    result
}

fn transform_type(_type_name: &lua_ast::types::TypeAssertion) -> TsType {
    todo!()
}

fn transform_variable(var: &lua_ast::Var) -> Expr {
    match var {
        lua_ast::Var::Expression(expr) => property_chain(
            match expr.prefix() {
                lua_ast::Prefix::Name(token) => Expr::Ident(Ident {
                    optional: false,
                    span: Default::default(),
                    sym: JsWord::from(token.token().to_string()),
                }),
                lua_ast::Prefix::Expression(expr) => transform_expression(expr),
                _ => skip("Unknown prefix variant", expr.prefix()),
            },
            expr.suffixes()
                .map(|suffix| Ident {
                    optional: false,
                    span: Default::default(),
                    sym: JsWord::from(suffix.to_string()),
                })
                .collect(),
        ),
        lua_ast::Var::Name(name) => Expr::Ident(Ident {
            optional: false,
            span: Default::default(),
            sym: JsWord::from(name.token().to_string()),
        }),
        _ => skip("Unknown variable variant", var),
    }
}

fn transform_value(value: &lua_ast::Value) -> Expr {
    match value {
        lua_ast::Value::ParenthesesExpression(expr) => transform_expression(expr),
        lua_ast::Value::Symbol(token)
            if matches!(
                token.token().token_type(),
                tokenizer::TokenType::Symbol {
                    symbol: tokenizer::Symbol::Nil
                }
            ) =>
        {
            Expr::Ident(Ident {
                span: Default::default(),
                sym: JsWord::from("undefined"),
                optional: false,
            })
        }
        _ => skip("Unknown value variant", value),
    }
}

fn transform_unary_expression(op: &lua_ast::UnOp, expression: &lua_ast::Expression) -> Expr {
    if matches!(op, lua_ast::UnOp::Hash(_)) {
        Expr::Call(CallExpr {
            span: Default::default(),
            args: vec![],
            type_args: None,
            callee: Callee::Expr(boxed(Expr::Member(MemberExpr {
                span: Default::default(),
                obj: boxed(transform_expression(expression)),
                prop: MemberProp::Ident(Ident {
                    span: Default::default(),
                    sym: JsWord::from("size"),
                    optional: false,
                }),
            }))),
        })
    } else {
        match op {
            lua_ast::UnOp::Not(_) => Expr::Unary(UnaryExpr {
                span: Default::default(),
                op: UnaryOp::Bang,
                arg: boxed(transform_expression(expression)),
            }),
            lua_ast::UnOp::Minus(_) => Expr::Unary(UnaryExpr {
                span: Default::default(),
                op: UnaryOp::Minus,
                arg: boxed(transform_expression(expression)),
            }),
            _ => skip("Unknown unary operator", op),
        }
    }
}

fn transform_expression(expr: &lua_ast::Expression) -> Expr {
    match expr {
        lua_ast::Expression::Parentheses {
            contained: _,
            expression,
        } => Expr::Paren(ParenExpr {
            span: Default::default(),
            expr: boxed(transform_expression(expression)),
        }),
        lua_ast::Expression::UnaryOperator { unop, expression } => {
            transform_unary_expression(unop, expression)
        }
        lua_ast::Expression::Value {
            value,
            type_assertion,
        } => {
            let expr = transform_value(value);
            if let Some(type_assertion) = type_assertion {
                Expr::TsAs(TsAsExpr {
                    span: Default::default(),
                    expr: boxed(expr),
                    type_ann: boxed(transform_type(type_assertion)),
                })
            } else {
                expr
            }
        }
        _ => skip("Unknown expression variant", expr),
    }
}

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
                        PatOrExpr::Expr(boxed(transform_variable(names.iter().next().unwrap())))
                    } else {
                        PatOrExpr::Pat(boxed(Pat::Array(ArrayPat {
                            span: Default::default(),
                            optional: false,
                            type_ann: None,
                            elems: names
                                .iter()
                                .map(|name| Some(Pat::Expr(boxed(transform_variable(name)))))
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
