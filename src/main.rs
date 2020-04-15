mod land;
mod dice;

use std::fs::File;
use std::io::Write;
use std::env::current_dir;

fn main() {
    println!("\0\0\0\0\0TABLESTUCK\nPRESS ENTER TO BEGIN");
    println!("{}",dice::roll(1,6)+3);
    println!("{}",dice::roll(1,6)+3);
    println!("{}",dice::roll(1,6)+3);
    println!("{}",dice::roll(1,6)+3);
    println!("{}",dice::roll(1,6)+3);
    let mut landexample = land::section_gen(1,1);
    println!("{}", landexample[6][6]);
    println!("{}", landexample[1][3]);
    let mut path = current_dir().unwrap().to_str().unwrap().to_string();
    let mut landpath = path.clone() + "/../land.txt";
    println!("Saving to {}", landpath);
    let mut landfile = File::create(landpath).expect("Unable to create land file");
    let mut b = [0; 2];
    for i in 0..11{
        for j in 0..11{
            landfile.write_all(landexample[i][j].encode_utf8(&mut b).as_bytes()).expect("Can't save land");
        }
        landfile.write_all('\n'.encode_utf8(&mut b).as_bytes()).expect("Can't write new line");
    }

}
