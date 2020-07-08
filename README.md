# [RUST IN ACTION BOOK (Tim McNamara)](https://www.manning.com/books/rust-in-action)

## Chapter 4: The borrow checker: borrowing, lifetimes and ownership


## Ways to handle ownership

- 1. using reference:

```
struct GroundStation;

impl GroundStation {
    fn send(&self, to: &mut CubeSat, msg: Message) {
        to.mailbox.messages.push(msg);
    }
}


#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}

impl CubeSat {
    fn recv(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

type Message = String;

fn check_status(sat_id: CubeSat) -> CubeSat {
    println!("{:?}: {:?}", sat_id, StatusMessage::Ok);
    sat_id
}

fn main() {
    let base = GroundStation {};
    let mut sat_a = CubeSat { id:0001, mailbox: Mailbox { messages: vec![]}};
    
    println!("t0: {:?}", sat_a);
    base.send(&mut sat_a, Message::from("hello there!"));
    println!("t1: {:?}", sat_a);
    
    let msg = sat_a.recv();
    println!("t2: {:?}", sat_a);

    println!("msg: {:?}", msg);

}
```

- 2. Use Fewer Long-Lived Values
This methodology is not practical since the instance of the value must be valid for the whole program
