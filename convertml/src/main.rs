use clap::Parser;
use std::{ffi::OsString, io::Write, os::windows::process::CommandExt, process::Command};
use std::env;

#[derive(Parser)]
struct CliArgs {
    #[arg(short, long)]
    latex: bool,
    expr: OsString,
}

fn main() {
    simple_logger::init().unwrap();
    env::set_current_dir(env::current_exe().unwrap().parent().unwrap()).unwrap();
    let args = CliArgs::parse();
    let output = Command::new("node")
        .raw_arg("convertor.js")
        .raw_arg(&args.expr)
        .output()
        .unwrap();
    if output.status.success() {
        let mathml = String::from_utf8(output.stdout).unwrap();
        let res = rbmath::render(&mathml);
        if args.latex {
            println!(
                "{}",
                res.iter()
                    .map(|c| format!("\\braillebox{{{}}} ", c.to_dots()))
                    .collect::<String>()
            );
        } else {
            println!("{}", res);
        }
    } else {
        std::io::stderr().write_all(&output.stdout).unwrap();
        std::io::stderr().write_all(&output.stderr).unwrap();
    }
}
