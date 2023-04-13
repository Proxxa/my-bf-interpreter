mod reader;
mod compiler;
mod interpreter;

fn main() -> std::io::Result<()> {
    let args : Vec<String> = std::env::args().collect();
    if args.len() < 1 {
        panic!("Collected no arguments");
    }
    if args.len() < 2 {
        println!("usage: {} <file>", args[0]);
        return Ok(());
    }

    let instructions = reader::read_file(&args[1].as_str())?;

    let program = compiler::compile(instructions);

    interpreter::interpret(program);


    Ok(())
}
