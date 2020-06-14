/*
fn type_of<T>(_: &T) -> &'static str {
    unsafe { std::intrinsics::type_name::<T>() }
}
*/

fn main() {
    let mut a = 5;  println!("The value of x is: {}, type : {:?}", a, a); dbg!(a);
    a = 6;    println!("The value of x is: {}", a);

    let b = 1.2;    println!("b = {}", b);
    let b = b + 1.2;    println!("b = {}", b); dbg!(b);
    
    let c = "cccc";    println!("c = {} || length = {}", c, c.len());
    let c = c.to_owned() + "ddd";    println!("c = {} || length = {}", c, c.len());
    
    let _d : i8 = 1;
    let _ddd : bool = false;
    
    let _tu_e = (1, 2.0, 'a', "a"); //tuple other data type
    let _ar = [1, 2, 3]; //array same data type
    let _ar_num = 2;
    let _ar_check = _ar[_ar_num];
    println!("tu {} // ar {} ~~~~~~~ {}", _tu_e.0, _ar[1], _ar_check);

}

fn print(x: isize){

}