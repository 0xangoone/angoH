use std::{
    fs,
    env,
    path,
};
fn main() {
    let _path = env::args().nth(1).unwrap_or(String::new());
    println!("reading the {} bytes",_path);
    let file = fs::read(_path);
    match  file{
        Err(e) => println!("{e}"),
        Ok(e) =>{

            let mut j = 0;
            let mut line = 0x00;
            print!("  01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f");
            println!("\n");
            for i in 0..e.len(){
                if j == 0{
                    print!("{:X}:  ",line);
                }
                print!("{:X} ",e[i]);
                if j == 0xf{
                    line += 0x10;
                    j = 0;
                    println!();
                    continue;
                }
                j += 1;
            }
        },
        _ => {}
    }
}
