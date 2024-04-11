fn print_char(c : char)
{
println!("{}", c);
}
fn implicitly_ret_unit(){
	println!("i will return a ()");
}
fn explicityly_ret_unit(){
	println!("i will return a ()");
}

use std::mem::size_of_val;
fn main() {
	let c1 = 'a';
	assert_eq!(size_of_val(&c1), 4);
	
	let c2 = '中';
	assert_eq!(size_of_val(&c2), 4);
	print_char(c2);	
	let _v: () = ();
	let v = (2,3);
	assert_eq!(_v, implicitly_ret_unit());	
	assert_eq!(size_of_val(&_v), 0);	
	println!("Success!");
}
