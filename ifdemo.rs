
fn main(){
   let x =4;
    if x == 4 {
        println!("x is four");
    } else if x == 3 {
        println!("x is three");
    } else {
        println!("x is something else");
    }
    println!("//");

    let y = if 12 * 15 > 150 {
        "Bigger"
    } else {
        "Smaller"
    };
    println!("y ={y}");
    println!("//");

    if let _ = 5 {
        println!("Irrefutable patterns are always true");
    }
    println!("//");
    //
    enum E {
        X(u8),
        Y(u8),
        Z(u8),
    }
    let n:u8=12;
    let v = E::Y(12);
    if let E::X(n) | E::Y(n) = v {
        println!("true");
    }
}
//https://doc.rust-lang.org/reference/expressions/if-expr.html