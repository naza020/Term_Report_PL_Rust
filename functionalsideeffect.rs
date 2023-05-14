static mut A:i32=5;

fn func()->i32{
    unsafe{
        A=6;
    }
    
    return 10;
}

fn main(){
    let b:i32;
    unsafe{
        // b=func()+A; //16
        b=A+func(); //15
        println!("A = {A}");
    }
    println!("b = {b}");
}