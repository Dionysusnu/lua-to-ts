use crate::prelude::*;

pub fn transform_string(token: &tokenizer::TokenReference) -> Box<Expr> {
	match token.token().token_type() {
		tokenizer::TokenType::StringLiteral {
			literal,
			multi_line: _,
			quote_type: _,
		} => boxed(Expr::Lit(Lit::Str(make_string(literal)))),
		_ => skip(
			"transform_string token was not TokenType::StringLiteral",
			token,
		),
	}
}
