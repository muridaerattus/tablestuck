mod dice;

fn main() {
    println!("\0\0\0\0\0TABLESTUCK\nPRESS ENTER TO BEGIN");
    println!("{}",dice::roll(1,6)+3);
    println!("{}",dice::roll(1,6)+3);
    println!("{}",dice::roll(1,6)+3);
    println!("{}",dice::roll(1,6)+3);
    println!("{}",dice::roll(1,6)+3);
}
