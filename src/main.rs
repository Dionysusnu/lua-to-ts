mod expressions;
mod statements;
mod types;
mod util;

mod prelude;
use crate::prelude::*;

use std::error::Error;
use std::{
	env,
	fs::{read_to_string, OpenOptions},
	io::{self, Write},
	path, process,
};
use swc_common::{sync::Lrc, SourceMap};
use swc_ecma_codegen::{text_writer::JsWriter, Emitter};

fn main() -> Result<(), Box<dyn Error>> {
	let mut args = env::args();

	if args.len() < 2 {
		eprintln!(
			"Usage: {} <filename>",
			args.next().unwrap_or_else(|| "lua-to-ts".to_string())
		);
		process::exit(1);
	}

	// ignore filename of lua-to-ts itself
	args.next();
	for filename in args.progress() {
		println!("Reading {}", filename);
		let contents = read_to_string(&filename)?;
		println!("Parsing {}", filename);
		let ast = full_moon::parse(&contents)?;
		println!("Transforming {}", filename);
		let body = transform_module_block(ast.nodes());
		println!("Emitting {}", filename);
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
						body,
						span: Default::default(),
						shebang: None,
					})
					.unwrap();
			}

			String::from_utf8_lossy(&buf).to_string()
		};

		println!("Writing {}", filename);
		let target = path::Path::new(&filename).with_extension("ts");
		let file = OpenOptions::new()
			.write(true)
			.create_new(true)
			.open(&target);

		// Handle common error cases gracefully
		let mut file = match file {
			Err(err) if err.kind() == io::ErrorKind::AlreadyExists => {
				eprintln!("Refusing to overwrite `{}`", target.to_string_lossy());
				process::exit(1);
			}
			Err(err) => {
				eprintln!(
					"Errored while opening file handle for `{}`: {:#?} {}",
					target.to_string_lossy(),
					err.source(),
					err
				);
				process::exit(1);
			}
			Ok(file) => file,
		};

		file.write_all(code.as_bytes())?;
	}

	Ok(())
}
