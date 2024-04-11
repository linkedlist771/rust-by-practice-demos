fn type_of<T> (_: &T) -> String{
	format!("{}", std::any::type_name::<T>())
}
use std::ops::{Range, RangeInclusive};

fn main() {
    let x: i32 =5;
    let mut y: u32 = 5;
    y = x as u32;
    let z = 10;
    let v: u16 = 38_u8 as u16;
    assert_eq!("i32".to_string(), type_of(&x));
 	assert_eq!(i8::MAX, 127);
	assert_eq!(u8::MAX, 255);
	let v1 = 251_u16 + 8;
	let v2 = i16::checked_add(251, 8).unwrap();
	println!("{},{}", v1, v1);
	let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
        println!("{}", v);
	let x7 = 1_000.000_1;
	let y: f32 = 0.12;
	let z = 0.01_f64;
	assert_eq!(type_of(&x7), "f64".to_string());
	assert!((0.1_f64+0.2_f64-0.3_f64).abs() < f64::EPSILON);
	// quiz 9, that should be the for loop case
	let mut sum = 0;
	for i in -3..2
	{
	sum +=i;	
}
	assert!(sum==-5);
	for c in 'a'..='z'{
	println!("{}",c );
}	
	assert_eq!((1..5), Range{start:1, end:5});
	assert_eq!((1..=5), RangeInclusive::new(1,5));
	
	assert_eq!(1u32 + 2, 3u32);
	assert_eq!(1i32-2, -1);
	assert_eq!(1i8-2, -1);
	assert_eq!(3 * 50 , 150);
	assert!(((9.6_f64 / 3.2_f64)- 3.0_f64).abs() < 1e-10);
	assert_eq!(24%5, 4);
	assert_eq!(true&&false, false);
	assert_eq!(true||false, true);
	assert_eq!(!true, false);
	
	println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);









    
println!("Success");
}
