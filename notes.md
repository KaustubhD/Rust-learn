
## Notes
* char
	* 4 bytes in size
	* represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid char values

* function
	* There is no type inference in function parameters. You **need** to specify the type of each parameter

* loop
	* Returning Values from Loops.
	
		One of the uses of a loop is to retry an operation you know might fail, such as checking whether a thread has completed its job. However, you might need to pass the result of that operation to the rest of your code. To do this, you can add the value you want returned after the break expression you use to stop the loop; that value will be returned out of the loop so you can use it, as shown here:
		```rust
		fn main() {
			let mut counter = 0;

			let result = loop {
				counter += 1;

				if counter == 10 {
					break counter * 2;
				}
			};

			println!("The result is {}", result);
		}
		```

	* Iteration
		```rust
		fn main() {
			let a = [10, 20, 30, 40, 50];

			for element in a.iter() {
				println!("the value is: {}", element);
			}
		}
		```
	* Range
		```rust
		fn main() {
			for number in (1..4).rev() {
				println!("{}!", number);
			}
		}
		```
* Modules
	Modules can be defined in two ways
	```rust
	pub mod hosting; //declaration
	fn http() {}
	fn https() {}

	// or

	pub mod hosting { // block
		fn http() {}
		fn https() {}
	}
	```
	When using the declaration method, Rust will treat the contents of the file with name of module name to be the contents of that module

* Nested Paths

	We can combine `use` statements to reduce vertical space in larger applications

	Eg.
	```rust
	use std::io;
	use std::cmp::Ordering;
	// can be changed to
	use std::{cmp::Ordering, io};
	```
	```rust
	use std::io;
	use std::io::Write;
	// can be changed to
	use std::io::{self, Write};
	```

* Collections

	* Vector
		Dynamic array
		```rust
		// Create new empty vector
		let v: Vec<i32> = Vec::new();
		
		// Or create with initial values without specifying type
		let v = vec![1, 2, 3];

		// Add element to the end
		v.push(5);

		// Remove element from the start
		v.pop();

		// Access elements
		let first: i32 = &v[100]; // Will panic on incorrect index
		let first: i32 = v.get(1000); // Will return None on incorrect index

		// Iterate
		for i in &v { // Immutable
			println!("{}", i);
		}
		for i in &mut v { // Mutable
			*i += 50;
			println!("{}", i);
		}
		```
	* String
		
		* A `String` is a wrapper over a `Vec<u8>`

		* Creating a string
			```rust
			let mut s = String::new();
			let s = "Hello World"; // String literals
			```
		
		* Copy from literals
			```rust
			let data = "initial contents";
			let s = data.to_string(); // Method 1
			let s = String::from(data); // Method 2
			```
		
		* Adding to string
			```rust
			// Method 1
			let mut s = String::from("lo");
			s.push('l');

			// Method 2
			let s1 = String::from("Hello, ");
			let s2 = String::from("world!");
			let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

			// Method 3
			let s1 = String::from("tic");
			let s2 = String::from("tac");
			let s3 = String::from("toe");

			let s = format!("{}-{}-{}", s1, s2, s3);
			```
		* `+` operator concatenates the string but takes the ownership of the first variable and it can't be used further
		* `String` cannot be indexed or sliced. This is because String stores unicode values and they can be more than 1 byte each and so the output may be different than expected.
			
			That is why Rust doesn't allow indexing to be compiled.
		* For iterating over letters of a unicode string, we can use `chars()`
			```rust
			for c in "नमस्ते".chars() {
				println!("{}", c);
			}
			// Outputs
			// ['न', 'म', 'स', '्', 'त', 'े']
			```
		* The `bytes()` method returns each raw byte
			```rust
			for b in "नमस्ते".bytes() {
				println!("{}", b);
			}
			// Outputs
			// [224, 164 ,...., 165, 135]
			```
	
	* HashMap

		* Create a HashMap using constructor
			```rust
			use std::collections::HashMap;

			let mut scores = HashMap::new();

			scores.insert(String::from("Blue"), 10);
			scores.insert(String::from("Yellow"), 50);
			```
		* Create a HashMap using another collection
			```rust
			use std::collections::HashMap;

			let teams = vec![String::from("Blue"), String::from("Yellow")];
			let initial_scores = vec![10, 50];

			let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
			```
		* For types that implement the Copy trait, like i32, the values are copied into the hash map. For owned values like String, the values will be moved and the hash map will be the owner of those values (this is valid for both key and value)
		* Iteration
			We can iterate over key values of a hashmap using a for loop
			```rust
			for (key, value) in &scores {
				println!("{}: {}", key, value);
			}
			```
			Note that the pairs will appear unsorted.
		* Update
			```rust
			// 1. Overwrite
			let mut scores = HashMap::new();

			scores.insert(String::from("Blue"), 10);
			scores.insert(String::from("Blue"), 25);

			// 2. Insert if not present
			let mut scores = HashMap::new();

			scores.insert(String::from("Blue"), 10);
			scores.entry(String::from("Yellow")).or_insert(50);

			// 3. Update using previous values
			let mut map = HashMap::new();

			for word in text.split_whitespace() {
				let count = map.entry(word).or_insert(0); // get a ref of previous or newly inserted value
				*count += 1; // update it
			}
			```
			More info from docs
			> The `or_insert` method on [Entry]("https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html") is defined to return a mutable reference to the value for the corresponding Entry key if that key exists, and if not, inserts the parameter as the new value for this key and returns a mutable reference to the new value
* Errors
	* panic!
		Rust offers a panic! macro to throw off errors in the code.
		> By default, when a panic occurs, the program starts unwinding, which means Rust walks back up the stack and cleans up the data from each function it encounters. But this walking back and cleanup is a lot of work. The alternative is to immediately abort, which ends the program without cleaning up. Memory that the program was using will then need to be cleaned up by the operating system.
	* Result Type
		Result enum is defined as having two variants, `Ok` and `Err`
		```rust
		enum Result<T, E> {
			Ok(T),
			Err(E),
		}
		```
		Usage:
		```rust
		use std::fs::File;

		fn main() {
			let f = File::open("hello.txt");

			let f = match f {
				Ok(file) => file,
				Err(error) => panic!("Problem opening the file: {:?}", error),
			};
		}
		```
		Check out https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#matching-on-different-errors for an example of error handling with `match`
	
	* Shortcuts to error handling
		
		* `unwrap()`
			
			If the Result value is the `Ok` variant, unwrap will return the value inside the Ok. If the Result is the `Err` variant, unwrap will call the panic! macro for us.
			
			Usage:
			```rust
			fn main() {
				let f = File::open("hello.txt").unwrap();
			}
			```
		* `expect()`
			> expectlets us also choose the panic! error message. Using expect instead of unwrap and providing good error messages can convey your intent and make tracking down the source of a panic easier.
			Usage:
			```rust
			fn main() {
				let f = File::open("hello.txt").expect("Failed to open hello.txt");
			}
			```
		* Propogate errors with `?` operator
			```rust
			// Without '?'
			fn read_username_from_file() -> Result<String, io::Error> {
				let f = File::open("hello.txt");

				let mut f = match f {
					Ok(file) => file,
					Err(e) => return Err(e),
				};

				let mut s = String::new();

				match f.read_to_string(&mut s) {
					Ok(_) => Ok(s),
					Err(e) => Err(e),
				}
			}

			// With '?'
			fn read_username_from_file() -> Result<String, io::Error> {
				let mut f = File::open("hello.txt")?;
				let mut s = String::new();
				f.read_to_string(&mut s)?;
				Ok(s)
			}

			// With chaining
			fn read_username_from_file() -> Result<String, io::Error> {
				let mut s = String::new();
				File::open("hello.txt")?.read_to_string(&mut s)?;
				Ok(s)
			}
			```
			**NOTE:** We are only allowed to use the ? operator in a function that returns `Result` or `Option` or another type that implements `std::ops::Try`
		
		* Error handling best practices
			[https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html)

## Unclear

1. To explicitly handle the possibility of overflow, you can use these families of methods that the standard library provides on primitive numeric types:

	* Wrap in all modes with the wrapping_* methods, such as wrapping_add
	* Return the None value if there is overflow with the checked_* methods
	* Return the value and a boolean indicating whether there was overflow with the overflowing_* methods
	* Saturate at the value's minimum or maximum values with saturating_* methods
