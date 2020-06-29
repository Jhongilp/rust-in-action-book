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

    fn read(f: &File, save_to:&mut Vec<u8>) -> usize {
        let mut tmp = f.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp); // tmp gets empty
        println!("tmp len: {}", read_length); // 5
        println!("tmp len2: {}", tmp.len()); // 0
    
        read_length
    }
}

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

fn main() {
    println!("Hello, world!");

    let f3_data = vec![114, 117, 115, 116, 33];
    let mut f3 = File::new_with_data("f3.txt", &f3_data);

    let mut buffer: Vec<u8> = vec![];

    open(&mut f3);
    let f3_length = File::read(&f3, &mut buffer);
    close(&mut f3);

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f3);
    println!("{} is {} bytes long", &f3.name, f3_length);
    println!("text: {}", text);
}
