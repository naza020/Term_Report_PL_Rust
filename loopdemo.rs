fn main(){
    let mut last = 0;
    for x in 1..100 {
        if x > 12 {
            break;
        }
    last = x;
    println!("x ={last}")
    }
    println!("//");
    
    //
    let v = &["apples", "cake", "coffee"];
    for text in v {
        println!("I like {}.", text);
    }
    println!("//");
    //
    let mut sum = 0;
    for n in 1..11 {
        sum += n;
    }
    println!("sum = {sum}");
    println!("//");
    //
    let mut i = 0;
    while i < 10 {
        println!("hello {i}");
        // i = i + 1;
        i+=1;
    }

}
//https://doc.rust-lang.org/reference/expressions/loop-expr.html