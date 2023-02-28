use clap::{Parser};
use std::{
    fs::File,
    io::{self, stdin, BufRead, BufReader,Write},
    path::PathBuf,
};
use serde_json::json;
use human_panic::setup_panic;


#[derive(Parser)]
#[command(arg_required_else_help = true)]

struct Cli {
    path:  PathBuf,
    pattern: String,
    #[arg(long = "json")]
    json: bool,
}

fn open_file(filename: &PathBuf) -> BufReader<File> {
    let file = File::open(filename).expect("Can´t open filename");
    let buffer= BufReader::new(file);
    buffer
}

fn read_lines<R>(reader: &mut BufReader<R>, patt: &String, json: bool) where R: std::io::Read {
    let mut counter: u128 = 0;
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for line in reader.lines() {
        let text = line.unwrap();
        if text.contains(patt) {
            if json {
                counter += 1;
                writeln!(handle, "{}",json!({
                    "type" : "match",
                    "content": text
                })).expect("An error encountered");
            } else {
                counter += 1;
                writeln!(handle, "{}", text).expect("An error encountered");
            }
        }
    }
    writeln!(handle, "{} lines encounter", counter).expect("An error encountered");
}


fn main() -> io::Result<()> {
    setup_panic!();
    let args = Cli::parse();
    let file = args.path;
    if file == PathBuf::from("-") {
        let mut data = BufReader::new(stdin().lock());
        read_lines(&mut data, &args.pattern, args.json);
    } else {
        let mut reader: BufReader<File>= open_file(&file);
        read_lines(&mut reader, &args.pattern, args.json);
    }

    Ok(())
}