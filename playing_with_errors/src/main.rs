use std::fs::File;

fn main() {
    let f = File::open("hey.txt");
    let f = match f {
        Ok(r) => r,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
