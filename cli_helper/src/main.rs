use clap::Parser;

#[derive(Parser)]
struct Cli{
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    
    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
    
    let content = std::fs::read_to_string(&args.path).unwrap();
    
    match content{
        Ok(content) => {
            for line in content.lines(){
                if line.contains(&args.pattern){
                    println!("{}", line);
                }
            }
        }
        Err(error) => { panic!("An error ocurred : {}", error);}
    }
    
    //let pattern = std::env::args().nth(1).expect("no pattern given");
    //let path = std::env::args().nth(2).expect("no path given");

}
