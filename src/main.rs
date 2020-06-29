extern crate rand;
use rand::Rng;

fn one_in(n: u32) -> bool {
    rand::thread_rng().gen_weighted_bool(n)
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(&name);
        f.data = data.clone();
        f
    }

    fn read(self: &File, save_to:&mut Vec<u8>) -> Result<usize, String> {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp); // tmp gets empty
        println!("tmp len: {}", read_length); // 5
        println!("tmp len2: {}", tmp.len()); // 0
    
        Ok(read_length)
    }
}

fn open(f: File) -> Result<File, String> {
    if one_in(10000) {
        let err_msg = String::from("Permission denied");
        return Err(err_msg);
    }
    Ok(f)
}

fn close(f: File) -> Result<File, String> {
    if one_in(100000) {
        let err_msg = String::from("Interrupted by signal!");
        return Err(err_msg);
    }
    Ok(f)
}


fn main() {
    println!("Hello, world!");

    let f_data = vec![114, 117, 115, 116, 33];
    let mut f = File::new_with_data("f3.txt", &f_data);

    let mut buffer: Vec<u8> = vec![];

    f = open(f).unwrap();
    let f_length = f.read(&mut buffer).unwrap();
    f = close(f).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f);
    println!("{} is {} bytes long", &f.name, f_length);
    println!("text: {}", text);
}
