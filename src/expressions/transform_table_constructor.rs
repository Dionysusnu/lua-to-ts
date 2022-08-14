use crate::prelude::*;

fn get_unpack_data(original_expression: &lua_ast::Expression) -> (Option<()>, Expr) {
	let default = || (None, transform_expression(original_expression));
	let value = match original_expression {
		lua_ast::Expression::Value {
			value,
			type_assertion: _,
		} => value,
		_ => return default(),
	};
	let function_call = match **value {
		lua_ast::Value::FunctionCall(ref function_call) => function_call,
		_ => return default(),
	};
	let token = match function_call.prefix() {
		lua_ast::Prefix::Name(token) => token,
		_ => return default(),
	};
	let call = if token.token().to_string() == "unpack" {
		function_call.suffixes().next().unwrap()
	} else {
		return default();
	};
	let function_args = match call {
		lua_ast::Suffix::Call(lua_ast::Call::AnonymousCall(function_args)) => function_args,
		_ => unreachable!(),
	};
	// unpack only ever has one argument
	let arg = transform_function_args(function_args).remove(0);
	(Some(()), *arg.expr)
}

pub fn transform_table_constructor(args: &lua_ast::TableConstructor) -> Expr {
	let is_array = args
		.fields()
		.iter()
		.all(|field| matches!(field, lua_ast::Field::NoKey(_)));
	if is_array {
		Expr::Array(ArrayLit {
			span: Default::default(),
			elems: args
				.fields()
				.iter()
				.map(|field| {
					if let lua_ast::Field::NoKey(original_expression) = field {
						// Detect usage of `unpack` in table constructor
						// This needs to convert to the `...` spread operator in TS
						let (spread, expression) = get_unpack_data(original_expression);
						Some(ExprOrSpread {
							spread: spread.map(|()| Default::default()),
							expr: boxed(expression),
						})
					} else {
						unreachable!()
					}
				})
				.collect(),
		})
	} else {
		Expr::Object(ObjectLit {
			span: Default::default(),
			props: args
				.fields()
				.iter()
				.map(|field| {
					PropOrSpread::Prop(boxed(Prop::KeyValue(match field {
						lua_ast::Field::NameKey {
							key,
							value,
							equal: _,
						} => KeyValueProp {
							// Not PropName::Ident because Luau has different ident validity rules
							key: PropName::Str(make_string(&key.token().to_string())),
							value: boxed(transform_expression(value)),
						},
						lua_ast::Field::ExpressionKey {
							key,
							value,
							equal: _,
							brackets: _,
						} => KeyValueProp {
							key: PropName::Computed(ComputedPropName {
								span: Default::default(),
								expr: boxed(transform_expression(key)),
							}),
							value: boxed(transform_expression(value)),
						},
						lua_ast::Field::NoKey(value) => KeyValueProp {
							key: PropName::Num(Number::from(
								args.fields()
									.iter()
									.filter(|f| matches!(f, lua_ast::Field::NoKey(_)))
									.position(|f| f == field)
									// unwrap: field will always be NoKey and thus found in iterator
									// +1: Luau tables start indexing at 1
									.unwrap() + 1,
							)),
							value: boxed(transform_expression(value)),
						},
						_ => KeyValueProp {
							key: PropName::Ident(Ident {
								optional: false,
								span: Default::default(),
								sym: JsWord::from("UnknownTableField"),
							}),
							value: boxed(skip("Unknown Field kind", field)),
						},
					})))
				})
				.collect(),
		})
	}
}
