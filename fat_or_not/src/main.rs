struct Cat;
trait Animal {
    fn speak();
}

fn main() {
    println!("Hello, Fat Pointer!");

    // 这里讨论关于类型和指针到类型的实际内存下的视野的事情
    // 以下讨论都基于64位系统下，所以在指针的长度上都是8字节（64位/8位一字节 = 8）
    // &Cat 这个用一个8位空间保存
    // &<dyn Animal>而Animal作为了trait是用fat指针来保存，有16位空间
}
