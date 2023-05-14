// An attribute to hide warnings for unused code.
#![allow(dead_code)]
enum Days{
    Sun,
    Mon,
    Tue,
    Wed,
    Thur,
    Fri,
    Sat
}

fn main(){
    let my_day =Days::Sat;
    print(my_day);
    print(Days::Sun);

}

fn print(d:Days){
    match d{
        Days::Sun =>println!("myDay = Sunday"),
        Days::Mon =>println!("myDay = Monday"),
        // Tue =>println!("myDay = Tueday"),
        // Wed =>println!("myDay = Wedday"),
        // Thur =>println!("myDay = Thurday"),
        // Fri =>println!("myDay = Friday"),
        // Sat =>println!("myDay = Satday"),
        _ => println!("not found"),
    }
}