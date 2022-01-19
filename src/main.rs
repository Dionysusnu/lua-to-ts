mod expressions;
mod statements;
mod util;

mod prelude;
use crate::prelude::*;

use std::error::Error;
use std::{env, fs, process};
use swc::common::{sync::Lrc, SourceMap};
use swc_ecma_codegen::{text_writer::JsWriter, Emitter};

fn main() -> Result<(), Box<dyn Error>> {
	let args: Vec<String> = env::args().collect();

	if args.len() < 2 {
		eprintln!("Usage: {} <filename>", args[0]);
		process::exit(1);
	}
	let filename = &args[1];

	let contents = fs::read_to_string(filename).unwrap();

	let ast = full_moon::parse(&contents)?;

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

			emitter
				.emit_module(&Module {
					body: transform_block_statements(ast.nodes())
						.into_iter()
						.map(ModuleItem::Stmt)
						.collect(),
					span: Default::default(),
					shebang: None,
				})
				.unwrap();
		}

		String::from_utf8_lossy(&buf).to_string()
	};

	print!("{}", code);

	Ok(())
}
