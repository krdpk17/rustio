use std::fs::File;

fn main(){
    File::open("hello.txt").expect("Failed to open hello.txt");
}
