pub struct DropExample {}

impl Drop for DropExample {
    fn drop(&mut self) {
        println!("drop");
    }
}

fn main() {
    let flag = true;

    let _obj = match flag {
        true => Some(DropExample {}),
        false => None,
    };

    println!("_obj");

    let _ = match flag {
        true => Some(DropExample {}),
        false => None,
    };

    println!("_");

    if flag {
        let _obj = DropExample {};
    }

    println!("end");
}
