use std::collections::HashMap;

fn main() {
	/*
	let mut map = HashMap::new();

	map.insert("Hello", "World");

	let _world: &str;
	
	match map.get(&("Hello")) {
		Some(_world) => println!("{}", *_world),
		None => ()
	}
	*/
	/*
	let mut map = HashMap::new();
	
	map.insert(String::from("World"), "Hello");
	
	let _hello: &str;
	
	match map.get("World") {
		Some(_hello) => println!("{}", *_hello),
		None => ()
	}
	*/

	let mut map = HashMap::new();
	
	map.insert(String::from("World"), "Hello");
	
	let _hello: &str;
	
	match map.get(&String::from("World")) {
		Some(_hello) => println!("{}", *_hello),
		None => ()
	}
}