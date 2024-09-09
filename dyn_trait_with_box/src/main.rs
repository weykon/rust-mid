trait Book {
    fn open_animation(&self);
}

struct Gelin;
struct Linux_Niao;

impl Book for Linux_Niao {
    fn open_animation (&self) { }
}
impl Book for Gelin {
    fn open_animation (&self) { }
}

fn just_book (
    a_book :Box <dyn Book> 
) {

}

fn main() {
    println!("Hello, world!");

}
