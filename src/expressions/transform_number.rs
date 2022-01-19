use crate::prelude::*;

pub fn transform_number(number: &tokenizer::TokenReference) -> Expr {
	Expr::Lit(Lit::Num(Number {
		span: Default::default(),
		value: number
			.token()
			.to_string()
			.chars()
			.filter(|c| c != &'_')
			.collect::<String>()
			.parse()
			.unwrap(),
	}))
}
