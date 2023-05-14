struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

// impl of Val
impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

// impl of GenVal for a generic type `T`
impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

fn main() {
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };
    
    let x1 = Val { val: 4.0 };
    // let x1 = Val { val: 5 };
    let y1 = GenVal { gen_val: 3.0 };

    let z = GenVal { gen_val: "hello"};
    let z1 = GenVal { gen_val: true };
    println!("x = {:?}, y = {:?}", x.value(), y.value());
    println!("x1 = {:?}, y1 = {:?}", x1.value(), y1.value());
    println!("z = {:?}, z1 ={:?}", z.value(), z1.value());
}
//https://doc.rust-lang.org/rust-by-example/generics/impl.html