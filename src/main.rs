mod expressions;
mod statements;
mod types;
mod util;

mod roblox;
use roblox::transform_dom;

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
	fs::{read_to_string, File, OpenOptions},
	io::{BufReader, Write},
	path::Path,
	process,
};
use swc_common::{sync::Lrc, SourceMap};
use swc_ecma_codegen::{text_writer::JsWriter, Emitter};

#[cfg(feature = "progressbar")]
lazy_static! {
	static ref PROGRESS_BAR_STYLE: ProgressStyle = ProgressStyle::default_bar()
		.template(
			"{spinner:.cyan} [{elapsed:.dim}] {msg}... [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})\n{prefix}",
		)
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

use clap::Parser;

#[derive(Parser)]
struct Cli {
	#[clap(required = true)]
	files: Vec<String>,
	#[clap(long)]
	overwrite: bool,
}

#[allow(clippy::large_enum_variant)]
enum TransformFile {
	Lua(lua_ast::Ast),
	Model(rbx_dom_weak::WeakDom),
}

impl TransformFile {
	fn from(filename: &str) -> Result<TransformFile, String> {
		match Path::new(filename)
			.extension()
			.and_then(std::ffi::OsStr::to_str)
		{
			Some("lua" | "luau") => match full_moon::parse(
				&read_to_string(filename)
					.map_err(|err| format!("Error while opening `{}`: {}", filename, err))?,
			) {
				Err(err) => Err(format!("Error while parsing `{}`: {}", filename, err))?,
				Ok(ast) => Ok(TransformFile::Lua(ast)),
			},
			Some("rbxm") => match rbx_binary::from_reader(BufReader::new(
				File::open(filename)
					.map_err(|err| format!("Error while opening `{}`: {}", filename, err))?,
			)) {
				Err(err) => Err(format!("Error while parsing `{}`: {}", filename, err))?,
				Ok(ast) => Ok(TransformFile::Model(ast)),
			},
			Some("rbxmx") => {
				match rbx_xml::from_reader_default(BufReader::new(
					File::open(filename)
						.map_err(|err| format!("Error while opening `{}`: {}", filename, err))?,
				)) {
					Err(err) => Err(format!("Error while parsing `{}`: {}", filename, err))?,
					Ok(ast) => Ok(TransformFile::Model(ast)),
				}
			}
			_ => Err(format!("Could not detect file kind of `{}`", filename)),
		}
	}
	fn transform(self) -> Vec<ModuleItem> {
		match self {
			TransformFile::Lua(ast) => transform_module_block(ast.nodes()),
			TransformFile::Model(dom) => transform_dom(dom),
		}
	}
}

fn process_files(args: Cli) -> i32 {
	let mut failure_messages = vec![];
	let mut exit_code = exitcode::OK;

	#[cfg(feature = "progressbar")]
	let pb = ProgressBar::new(args.files.len().try_into().unwrap());
	#[cfg(feature = "progressbar")]
	pb.set_style(PROGRESS_BAR_STYLE.clone());
	#[cfg(feature = "progressbar")]
	let mut i = 0;

	for filename in args.files {
		#[cfg(feature = "progressbar")]
		{
			i += 1;
			pb.set_position(i);
		}

		#[cfg(feature = "progressbar")]
		pb.set_message(format!("Parsing {}", filename));
		let ast = match TransformFile::from(&filename) {
			Err(err) => {
				exit_code = exitcode::DATAERR;
				failure_messages.push(err);
				continue;
			}
			Ok(ast) => ast,
		};

		#[cfg(feature = "progressbar")]
		pb.set_message(format!("Transforming {}", filename));
		let body = ast.transform();

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
		let target = Path::new(&filename).with_extension("ts");
		let file = OpenOptions::new()
			.write(true)
			.truncate(true)
			.create_new(!args.overwrite)
			.open(&target);

		// Handle common error cases gracefully
		let mut file = match file {
			Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => {
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
					"Error while opening file handle for `{}`: {:?}",
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
				"Error while writing `{}`: {:?}",
				target.to_string_lossy(),
				err
			));
			continue;
		};
	}
	#[cfg(feature = "progressbar")]
	{
		pb.set_style(SPINNER_STYLE_FINISHED.clone());
		pb.finish_with_message("Processed files");
	}

	if !failure_messages.is_empty() {
		println!("{}", failure_messages.join("\n"));
	}

	exit_code
}

fn main() {
	process::exit(process_files(Cli::parse()));
}
