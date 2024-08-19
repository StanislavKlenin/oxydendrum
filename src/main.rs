use std::env;
use std::io;
use std::path::Path;

use oxydendrum::Node;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        for (i, dir) in args.iter().enumerate() {
            if i == 0 {
                continue;
            }
            let path = Path::new(dir);
            let tree = match Node::from_path(&path) {
                Ok(node) => node,
                Err(e) => return Err(e),
            };
            println!("{}", tree);
        }

        Ok(())
    }
    else {
        println!("usage: {} dir [dir2 ...]", args[0]);
        Ok(())
    }
}
