extern crate rand;
use rand::Rng;
use std::fmt;

fn one_in(n: u32) -> bool {
    rand::thread_rng().gen_weighted_bool(n)
}

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

impl fmt::Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED")
        }
        
    }
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The File {} has {} bytes and his state is {}", self.name, self.data.len(), self.state)
    }
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(&name);
        f.data = data.clone();
        f
    }

    fn read(self: &File, save_to:&mut Vec<u8>) -> Result<usize, String> {

        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading!"));
        }


        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp); // tmp gets empty
        println!("tmp len: {}", read_length); // 5
        println!("tmp len2: {}", tmp.len()); // 0
    
        Ok(read_length)
    }
}

fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    if one_in(10000) {
        let err_msg = String::from("Permission denied");
        return Err(err_msg);
    }
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    if one_in(100000) {
        let err_msg = String::from("Interrupted by signal!");
        return Err(err_msg);
    }
    Ok(f)
}

fn main() {

    let f_data = vec![114, 117, 115, 116, 33];
    let mut f = File::new_with_data("f3.txt", &f_data);

    let mut buffer: Vec<u8> = vec![];

    f = open(f).unwrap();
    let f_length = f.read(&mut buffer).unwrap();
    f = close(f).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("File INFO: {}", f);
    println!("{}", f.state);
    println!("{} is {} bytes long", &f.name, f_length);
    println!("text: {}", text);
}




