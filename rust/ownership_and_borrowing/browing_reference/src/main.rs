fn borrow_object(s: &String){

}

fn main() {
	// let x = 5; 
	// let p = &x;
	//  println!("{:p}", p);
	let x = 5;
	let y = &x;
	assert_eq!(&5, y);	
	let mut s = String::from("hello. ");
	borrow_object(&s);
	println!("Hello, world!");
}
