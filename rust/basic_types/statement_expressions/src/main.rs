fn sum(x: i32, y: i32) -> i32 {
x+y

}

fn main() {
	let x = 5_u32;
	let y = {
	let x_squared = x * x;
	let x_cube = x_squared * x;
	x_cube + x_squared + x
};
	let z = {
	2 * x
};
	let v = {
		let mut x = 1;
		x += 2;
		x
};
	assert_eq!(v, 3);
	let v = {let x = 3;
	x
};
	assert_eq!(v, 3);
	let s = sum(1, 2);
	assert_eq!(s, 3);
    println!("Success");
}
