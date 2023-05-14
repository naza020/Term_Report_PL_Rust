// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_mut)]
// #![allow(unused_assignments)]
fn main(){
    let mut  u=3;
    let mut v=5;
    let  pu =&u;
    // let  pu =u;
    v=*pu;
    // v=pu;
    let mut pv = &v;
    // let  pv = v;
    u=5;
    v=6;
    pv=&5;
    println!("u ={u} , v = {v} , pu={pu}, pv={pv}");

    let mut a:i32 =3;
    let mut b:i32 =4;
    println!("Before swap : a = {a} , b={b}");
    swap(&mut a,&mut b);
    println!("After swap : a = {a} , b={b}");
}

fn swap(a:&mut i32,b:&mut i32){
    let temp = *a;
    *a=*b;
    *b=temp;
    // println!("After swap : a = {a} , b={b}");
}