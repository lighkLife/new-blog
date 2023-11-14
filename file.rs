#![no_std]
#![no_main]

extern crate alloc;
#[macro_use]
extern crate user_lib;

use alloc::string::String;

use user_lib::{close, current_time, open, OpenFlags, read, write};

#[no_mangle]
pub fn main() -> i32 {
    for i in 0..10 {
        let name = file_name(i);
        let content = file_content(i);
        io_test(name.as_str(), content.as_str());
    }
    0
}

fn file_name(i: u8) -> String {
    let mut name = String::from("file");
    name.push(char::from(i));
    name
}

fn file_content(i: u8) -> String {
    let mut name = String::from("content");
    name.push(char::from(i));
    name
}

fn io_test(file: &str, content: &str) {
    let start = current_time();
    let fd = open(file, OpenFlags::CREATE | OpenFlags::WRONLY | OpenFlags::TRUNC);
    assert!(fd > 0);
    let fd = fd as usize;
    write(fd, content.as_bytes());
    close(fd);

    let fd = open(file, OpenFlags::RDONLY);
    assert!(fd > 0);
    let fd = fd as usize;
    let mut buffer = [0u8; 100];
    read(fd, &mut buffer) as usize;
    close(fd);

    let stop = current_time();
    println!("duration = {}", stop - start);
    // assert_eq!(content, core::str::from_utf8(&buffer[..read_len]).unwrap(), );
}
