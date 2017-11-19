use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
extern crate clipboard;
use clipboard::ClipboardProvider;
use clipboard::x11_clipboard::{X11ClipboardContext, Clipboard, Primary};
use std::{env, str};
fn main() {
    if let Some(mode) = env::args().nth(1) {
        if mode == "listen" {
            let listener = TcpListener::bind("127.0.0.1:6719").unwrap();
            for stream in listener.incoming() {
                let mut data: Vec<u8> = Vec::new();
                stream.unwrap().read_to_end(&mut data);
                if let Some((sel, txt)) = data.split_first() {
                    let clipboard = 99 as u8;
                    let primary = 112 as u8;
                    if sel == &clipboard {
                        let mut ctx: X11ClipboardContext<Clipboard> = ClipboardProvider::new()
                            .unwrap();
                        ctx.set_contents(str::from_utf8(txt).unwrap().to_owned())
                            .unwrap();
                    } else if sel == &primary {
                        let mut ctx: X11ClipboardContext<Primary> = ClipboardProvider::new()
                            .unwrap();
                        ctx.set_contents(str::from_utf8(txt).unwrap().to_owned())
                            .unwrap();
                    }
                }
            }
        } else if mode == "read" {
            let mut text: Vec<u8> = Vec::new();
            io::stdin().read_to_end(&mut text).unwrap();
            let mut stream = TcpStream::connect("127.0.0.1:6719").unwrap();
            if let Some(selection) = env::args().nth(2) {
                if selection == "primary" {
                    stream.write(String::from("p").as_bytes());
                } else if selection == "clipboard" {
                    stream.write(String::from("c").as_bytes());
                }
                stream.write_all(text.as_slice());
            } else {
                println!("{} read [primary|clipboard].", env::args().nth(0).unwrap());
            }
        } else {
            println!(
                "{} [listen|read [primary|clipboard]].",
                env::args().nth(0).unwrap()
            );
        }
    } else {
        println!(
            "{} [listen|read [primary|clipboard]].",
            env::args().nth(0).unwrap()
        );
    }
}
