pub fn start() {
    _1();
}

// 注释' i '和' borrow2 '的生命周期*///生命周期在下面用行注释，表示每个变量的创建//和销毁。
// ' i '具有最长的生命周期，
// 因为它的作用域完全包含了' borrow1 '和' borrow2 '。
// ' borrow1 '的持续时间比较
// ' borrow2 '是无关紧要的，因为它们是不相交的。
fn _1() {
    let i = 3;
    {
        let borrow1 = &i;
        println!("borrow1: {}", borrow1);
    }
    {
        let borrow2 = &i;
        println!("borrow2: {}", borrow2);
    }

    {
        let r;
        {
            let x = 5;
            r = &x;
        }
        println!("r: {}", r);
    }
}
