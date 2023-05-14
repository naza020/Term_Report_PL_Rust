fn main(){
    let (a,b):(i32,i32)= (5,2);
    let c:f32;
    // c=a/b;
    // c =a as f32/b as f32;
    c= (a/b) as f32;
    println!("{c}");
    println!("{:?}",c);
}