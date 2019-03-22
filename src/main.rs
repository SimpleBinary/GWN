use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => gwn::run_repl(),
        2 => gwn::run_file(args[0].clone()),
        _ => panic!("Too many arguments."), // TODO: Argv
    }
}
