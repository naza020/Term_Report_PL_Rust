fn main() {
    let num=2;
    match num{
        1|3|5|7|9=>println!("Odd"),
        2|4|6|8=>println!("Even"),
        _=>println!("Only one  digit allowed"),
    };
    //
    let num=19;
    match num{
        13..=19=>println!("Teenage"),
        _=>println!("Not Teenage"),
  };

    //
    let coin = "head";
    match coin{
        "head" =>println!("head"),
        "tail"=> println!("tail"),
        _=>println!("False coin"),
    };
  }
  //https://www.geeksforgeeks.org/rust-switch-case/