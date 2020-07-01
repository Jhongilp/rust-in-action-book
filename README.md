# [RUST IN ACTION BOOK (Tim McNamara)](https://www.manning.com/books/rust-in-action)

## Chapters 3

In this chapter I learnt about:

- struct
- enums
- traits

## struct
A struct allow us to store several properties

```
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState
}
```

## enum

```
enum FileState {
    Open,
    Closed,
}
```

Both, structs and enums can have implementations. I initially thought that only struct has implementations

## traits

Traits are a way to force that an enum or struct have determine functionality. In this chapter we implement Display trait to create a better info.

```
impl fmt::Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED")
        }
        
    }
}
```

```
impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The File {} has {} bytes and his state is {}", self.name, self.data.len(), self.state)
    }
}
```



