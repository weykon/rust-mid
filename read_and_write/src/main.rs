<<<<<<< HEAD
fn main() {
    // let we describe a data in ram.
    let can_read_data = 10;
    let mut can_write_data = 20; // of course, can read.

    // if a diff block, we can borrow the readable, and writable.
    let readable = &can_read_data;
    let writable = &mut can_write_data;

    person_use(*readable);
    person_write(writable);

    println!("can_read_data: {}", can_read_data);
    println!("can_write_data: {}", can_write_data);
}

fn person_use(read: i32) {
    println!("read: {}", read);
}

fn person_write(write: &mut i32) {
    *write = 100;
}

struct OwnStory {
    only_read: i32,
    can_rw: i32,
}
// this part seem simple, but this is important for rust mentality.
=======
fn main() {
    // let we describe a data in ram.
    let can_read_data = 10;
    let mut can_write_data = 20; // of course, can read.

    // if a diff block, we can borrow the readable, and writable.
    let readable = &can_read_data;
    let writable = &mut can_write_data;

    person_use(*readable);
    person_write(writable);

    println!("can_read_data: {}", can_read_data);
    println!("can_write_data: {}", can_write_data);
    
}

fn person_use(read: i32) {
    println!("read: {}", read);
}

fn person_write(write: &mut i32) {
    *write = 100;
}

struct OwnStory {
    only_read: i32,
    can_rw: i32,
}
// this part seem simple, but this is important for rust mentality.
>>>>>>> origin/main
