fn sum(x: i32, y: i32) -> i32{
	x+y
}
fn print() -> () {
	println!("Success!");
}

fn never_return() -> !{
	panic!("never return");

}


fn main() {
	let (x, y) = (1, 2);
	let s = sum(x, y);
	assert_eq!(s, 3);
	print(); 
	never_return();   
println!("Hello, world!");
}
