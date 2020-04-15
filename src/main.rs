mod land;
mod dice;

use land::Land;
use std::fs::File;
use std::io::Write;
use std::env::current_dir;

fn main() {
    println!("\0\0\0\0\0TABLESTUCK\nPRESS ENTER TO BEGIN");
    //let mut landexample = land::section_gen(1,1);
    let mut landexample = land::Land::new(1);
    println!("{}", landexample.get_section(1)[6][6]);
    println!("{}", landexample.get_section(2)[1][3]);
    let mut path = current_dir().unwrap().to_str().unwrap().to_string();
    let mut landpath = path.clone() + "/land.txt";
    println!("Saving to {}", landpath);
    let mut landfile = File::create(landpath).expect("Unable to create land file");
    let mut b = [0; 2];
    for k in 1..5{
        for i in 0..11{
            for j in 0..11{
                landfile.write_all(landexample.get_section(k)[i][j].encode_utf8(&mut b).as_bytes()).expect("Can't save land");
            }
            landfile.write_all('\n'.encode_utf8(&mut b).as_bytes()).expect("Can't write new line");
        }
        landfile.write_all('\n'.encode_utf8(&mut b).as_bytes()).expect("Can't write new line");
    }

}
