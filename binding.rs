fn main() {
    //explicit declarelation
    // static type binding
    let money:i32 =200;
    let pi:f32 =3.14;
    println!("money = {}",money);
    println!("pi = {}",pi);
    // pi=2.0;
    // println!("pi = {}",pi);
    println!("//");
    

    //implicit declarelation
    let money =200;
    let pi =3.14;
    println!("money = {}",money);
    println!("pi = {}",pi);

    println!("//");
    // static type binding
    let x=5;
    println!("x = {}",x);
    // x=10;
    // println!("x = {}",x);
    // x="s";
    let x ="s";//// Shadow
    println!("x = {}",x);

    println!("//");
    //declare multiple variable
    let (mut x,mut y) = (10,"hello");
    println!("x = {} , y = {}",x,y);
    x=20;
    y="hi";
    println!("x = {} , y = {}",x,y);
    let (x,y):(i32,f64) = (10,3.14);
    println!("x = {} , y = {}",x,y);
}