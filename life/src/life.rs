// here try function with life description
pub fn start<'start>() {
    let may1 = step1();
    println!("may1: {}", may1.data);
    step2();
}

struct OnlyStep1<'step1> {
    data: &'step1 i32,
}
fn step1<'step1>() -> OnlyStep1<'step1> {
    OnlyStep1 { data: &10 }
}

fn step2<'step2>() {}