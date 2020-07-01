extern crate rand;
use rand::Rng;

fn one_in(n: u32) -> bool {
    rand::thread_rng().gen_weighted_bool(n)
}

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState
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

// enums
// #[derive(Debug)]
// enum Event {
//     Update,
//     Delete,
//     Unknown,
// }

// type Message = String;

// fn parse_Log(line: &'static str) -> (Event, Message) {
//     let parts: Vec<&str> = line.splitn(2, ' ').collect();
//     if parts.len() == 1 {
//         return (Event::Unknown, String::from(line))
//     }

//     let event = parts[0];
//     let rest = String::from(parts[1]);

//     match event {
//         "UPDATE" | "update" => (Event::Update, rest),
//         "DELETE" | "delete" => (Event::Delete, rest),
//         _ => (Event::Unknown, String::from(line)),
//     }
// }






fn main() {

//     let log = "BEGIN Transaction XK342
// UPDATE 234:LS/32231 {\"price\": 31.00} -> {\"price\": 40.00}
// DELETE 342:LO/22111";

//     for line in log.lines() {
//         let parse_result = parse_Log(line);
//         println!("{:?}", parse_result);
//     }

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




