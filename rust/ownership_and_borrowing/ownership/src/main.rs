fn take_ownership(s: String) -> String {
	println!("{}", s);
	s
}

fn give_ownership() -> String{
	let s = String::from("hello world");
	let _s = (s.clone()).into_bytes();
	s
}

fn print_str(s: String){
	println!("{}", s);
}


fn main() {
	let x = String::from("Hello WOrld");
	let y = x.clone();
	println!("{}, {}", x, y);
	let s1 = String::from("Hello World");
	let s2 = take_ownership(s1);
	println!("{}", s2);
	println!("Success");
	let s = give_ownership();
	println!("{}", s);
	let _s = String::from("Hello World");
	print_str(_s.clone()); // here we have take the owner ship of it
	println!("{}", _s);
	let _x = (1,2,(), "hell".to_string());
	let _y = x.clone();
	println!("{:?}, {:?}",_x,_y);

	let s_ = String::from("Hello ");
	let mut s1_ = s_.clone();
	s1_.push_str("World");

	let x_ = Box::new(5);
	let mut  y_ = x_.clone();
	*y_ = 4;
	assert_eq!(*x_, 5);
	let t = (String::from("hello"), String::from("world"));
	let ss = t.0.clone();
	
	println!("{:?}", t);	

}
