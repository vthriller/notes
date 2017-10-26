#[macro_use]
extern crate clap;
use clap::{App, Arg};

#[macro_use]
extern crate cfg_if;

#[cfg(target_os = "linux")]
fn foo() -> [&'static str; 2] { ["xxx", "yyy"] }
#[cfg(target_os = "freebsd")]
fn foo() -> [&'static str; 1] { ["xxx"] }

fn main() {
	/*
		// if and else have incompatible types
		// expected an array with a fixed size of 2 elements, found one with 1 elements
		let foo = 
			if cfg!(target_os = "linux") {
				["xxx", "yyy"]
			} else if cfg!(target_os = "freebsd") {
				["xxx"]
			} else {
				unimplemented!()
			};
	*/
	/*
		// error: expected an item keyword: >>>[<<<
		let foo = cfg_if! {
			if #[cfg(target_os = "linux")] {
				["xxx", "yyy"]
			} else if #[cfg(target_os = "freebsd")] {
				["xxx"]
			} else {
				unimplemented!()
			}
		};
	*/
	/*
		// error: expected an item keyword: >>>let<<<
		cfg_if! {
			if #[cfg(target_os = "linux")] {
				let foo = ["xxx", "yyy"];
			} else if #[cfg(target_os = "freebsd")] {
				let foo = ["xxx"];
			} else {
				let foo = unimplemented!();
			}
		}
	*/
        let args = App::new("lorem-ipsum")
                .about("dolors your precious sit amet")
                .version(crate_version!())
		// …bazillion of options omitted…
                .arg(Arg::with_name("foo")
                        .short("f")
                        .takes_value(true)
                        .possible_values(&foo())
                )
                .get_matches();
}
