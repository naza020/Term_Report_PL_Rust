fn main() {
    let mut n_main: usize = 100;
    let n_main_ref: &usize = &n_main;
    println!("{}", inc_ref(n_main_ref));
    println!("n main ref = {n_main_ref}");
    inc(&mut n_main);
    println!("n main = {n_main}");
  }
  
  fn inc_ref(n_inc_ref: &usize) -> usize {
    n_inc_ref + 1
  }
  fn inc(t:&mut usize){
    *t=200;
  }