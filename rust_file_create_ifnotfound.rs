use std::fs::File;
use std::io::ErrorKind;

fn main(){
    let f = File::open("hello.txt");
    match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Tried to create file but failed{:?}", e)
                },
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    };
    
}

