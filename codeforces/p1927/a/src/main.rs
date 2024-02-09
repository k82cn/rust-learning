use std::io;

fn main() -> io::Result<()>{
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line)?;
 
    let n = line.trim().parse::<i32>().expect("n");

    for _ in 0..n {
        let mut line = String::new();
        let _ = io::stdin().read_line(&mut line)?;
        let _ = io::stdin().read_line(&mut line)?;
    
        let l = line.find("B").unwrap();
        let r = line.rfind("B").unwrap();
    
        println!("{}", r - l + 1);
    }

    Ok(())
}
