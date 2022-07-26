use std::process::Command;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   #[clap(short, long)]
   copier_version: bool,

   #[clap(short, long)]
   source: String,

   #[clap(short, long)]
   destination: String
}

fn main() {
    let args = Args::parse();
    let mut copier = Command::new("copier");
    if args.copier_version {
        copier.arg("-v");
    }
    copier.arg(args.source);
    copier.arg(args.destination);
    let _hello_1 = copier.spawn().expect("failed to execute process").wait();
    //print!("{}", String::from_utf8(hello_1.stdout.unwrap().bytes().collect()).unwrap());
}
