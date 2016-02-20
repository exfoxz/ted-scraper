extern crate stopwatch;
extern crate hyper;

// Stopwatch
use stopwatch::{Stopwatch};

use hyper::Client;
use hyper::header::Connection;

use std::sync::mpsc;
use std::thread;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

static NTHREADS: i32 = 10;

fn main() {
	
	let sw = Stopwatch::start_new();
	let max = 100;


	let (tx, rx) = mpsc::channel();

	for i in 0..NTHREADS {
		// Channel
		let tx = tx.clone();	

		thread::spawn(move || {
			// Create a client
			let client = Client::new();

			// Scrape HTMLs
			let range = max / NTHREADS;	
			for index in 1 + (i * range).. ((i+1) * range) {
				// Create an outgoing request 
				let mut res = client.get(&format!("https://www.ted.com/talks/{}/transcript?language=en", index))
					// set header
					.header(Connection::close())
					// let go
					.send().unwrap();

				// Read the Response
				let mut body = String::new();
				res.read_to_string(&mut body).unwrap();

				// Write res to file
				let link = format!("out/{}.html", index);
				let path = Path::new(&link);
				let display = path.display();

				// Open a file in w-o mode
				let mut file = match File::create(&path) {
					Err(why) => panic!("couldn't create {}:{}", display, Error::description(&why)),
					Ok(file) => file,
				};

				match file.write_all(body.as_bytes()) {
					Err(why) => panic!("couldn't create {}:{}", display, Error::description(&why)),
					Ok(_) => println!("Thread{}: wrote to {}", i, display),
				}
			}

			tx.send(()).unwrap();
		});
	}

	for _ in 0..NTHREADS {
		rx.recv().unwrap();
	}

	println!("Thing took {}ms", sw.elapsed_ms());
}
