mod expressions;
mod statements;
mod types;
mod util;

mod prelude;
use crate::prelude::*;

#[cfg(feature = "progressbar")]
use console::style;
#[cfg(feature = "progressbar")]
use indicatif::{ProgressBar, ProgressStyle};
#[cfg(feature = "progressbar")]
use lazy_static::lazy_static;
#[cfg(feature = "progressbar")]
use std::convert::TryInto;

use std::{
	env,
	fs::{read_to_string, OpenOptions},
	io::{self, Write},
	path, process,
};
use swc_common::{sync::Lrc, SourceMap};
use swc_ecma_codegen::{text_writer::JsWriter, Emitter};

#[cfg(feature = "progressbar")]
lazy_static! {
	static ref PROGRESS_BAR_STYLE: ProgressStyle = ProgressStyle::default_bar()
		.template(
			"{spinner:.cyan} [{elapsed:.dim}] {msg}... [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})\n{prefix}",
		)
		// For more spinners check out the cli-spinners project:
		// https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
		.tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
		.progress_chars("=>-");

	static ref SPINNER_STYLE_RUNNING: ProgressStyle = ProgressStyle::default_spinner()
		.tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
		.template("{spinner:.cyan} [{elapsed:.dim}] {msg}...");

	static ref SPINNER_STYLE_WAITING: ProgressStyle = ProgressStyle::default_spinner()
		.tick_strings(&[".  ", ".. ", "...", "   "])
		.template("{spinner:.cyan} {msg}");

	static ref SPINNER_STYLE_FINISHED: ProgressStyle = ProgressStyle::default_spinner()
		.template(format!("{} {{msg:.dim}}", style("✔").green()).as_str());

	static ref SPINNER_STYLE_FAILED: ProgressStyle = ProgressStyle::default_spinner()
		.template(format!("{} {{msg:.dim}}", style("❌").red()).as_str());
}

fn process_files() -> i32 {
	let mut args = env::args();

	if args.len() < 2 {
		eprintln!(
			"Usage: {} <filename>",
			args.next().as_deref().unwrap_or("lua-to-ts")
		);
		process::exit(exitcode::USAGE);
	}

	// ignore filename of lua-to-ts itself
	args.next();

	let mut failure_messages = vec![];
	let mut exit_code = exitcode::OK;

	#[cfg(feature = "progressbar")]
	{
		let pb = ProgressBar::new(args.len().try_into().unwrap());
		pb.set_style(PROGRESS_BAR_STYLE.clone());
		let mut i = 0;
	}
	for filename in args {
		#[cfg(feature = "progressbar")]
		{
			i += 1;
			pb.set_position(i);
		}

		#[cfg(feature = "progressbar")]
		pb.set_message(format!("Reading {}", filename));
		let contents = match read_to_string(&filename) {
			Err(err) => {
				exit_code = exitcode::NOINPUT;
				failure_messages.push(format!("Error while reading `{}`: {:?}", filename, err));
				continue;
			}
			Ok(contents) => contents,
		};

		#[cfg(feature = "progressbar")]
		pb.set_message(format!("Parsing {}", filename));
		let ast = match full_moon::parse(&contents) {
			Err(err) => {
				exit_code = exitcode::DATAERR;
				failure_messages.push(format!("Error while parsing `{}`: {}", filename, err));
				continue;
			}
			Ok(ast) => ast,
		};

		#[cfg(feature = "progressbar")]
		pb.set_message(format!("Transforming {}", filename));
		let body = transform_module_block(ast.nodes());

		#[cfg(feature = "progressbar")]
		pb.set_message(format!("Emitting {}", filename));
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

		#[cfg(feature = "progressbar")]
		pb.set_message(format!("Writing {}", filename));
		let target = path::Path::new(&filename).with_extension("ts");
		let file = OpenOptions::new()
			.write(true)
			.create_new(true)
			.open(&target);

		// Handle common error cases gracefully
		let mut file = match file {
			Err(err) if err.kind() == io::ErrorKind::AlreadyExists => {
				exit_code = exitcode::CANTCREAT;
				failure_messages.push(format!(
					"Refusing to overwrite `{}`",
					target.to_string_lossy()
				));
				continue;
			}
			Err(err) => {
				exit_code = exitcode::CANTCREAT;
				failure_messages.push(format!(
					"Errored while opening file handle for `{}`: {:?}",
					target.to_string_lossy(),
					err
				));
				continue;
			}
			Ok(file) => file,
		};

		if let Err(err) = file.write_all(code.as_bytes()) {
			exit_code = exitcode::IOERR;
			failure_messages.push(format!(
				"Errored while writing `{}`: {:?}",
				target.to_string_lossy(),
				err
			));
			continue;
		};
	}
	#[cfg(feature = "progressbar")]
	{
		pb.set_style(SPINNER_STYLE_FINISHED.clone());
		pb.finish_with_message("Processing");
	}

	if !failure_messages.is_empty() {
		println!("{}", failure_messages.join("\n"));
	}

	exit_code
}

fn main() {
	process::exit(process_files());
}
