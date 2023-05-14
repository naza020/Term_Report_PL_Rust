static mut SOME_INT: i32 = 100;
static SOME_STR: &'static str = "hello world";
static SOME_STRUCT: MyStruct = MyStruct {
    number: 50,
    string: "global variable in rust",
};
fn main() {
    
    unsafe{
        SOME_INT=150;
        println!("{}", SOME_INT);
    }
    
    println!("{}", SOME_STR);
    println!("{}", SOME_STRUCT.number);
    println!("{}", SOME_STRUCT.string);
}
struct MyStruct {
    number: i32,
    string: &'static str,
}