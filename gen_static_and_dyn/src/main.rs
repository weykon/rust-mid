trait Shape {
    type T;
    fn area(&self) -> Self::T;
}

struct Point<T> {
    x: T,
    y: T,
}

pub struct Rectangle<T> {
    top_left: Point<T>,
    bottom_right: Point<T>,
}


impl<T> Shape for Rectangle<T>
// 这个T可以减法和乘法
where
    T: std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + Copy,
{
    type T = T;
    fn area(&self) -> Self::T {
        let w = self.bottom_right.x - self.top_left.x;
        let h = self.top_left.y - self.bottom_right.y;
        h * w
    }
}

//计算两个形状的面积并返回包含该面积的元组的函数。
//该函数要求两个形状实现Shape特性。该函数将通过静态调度调用
// area函数。只有在为' a '和' b '指定了具体的
//类型时，才会为函数生成代码。
pub fn area_pair_static(a: impl Shape<T = f64>, b: impl Shape<T = f64>) -> (f64, f64) {
    (a.area(), b.area())
}

pub fn static_dispatch_pair(a: Rectangle<f64>, b: Rectangle<f64>) -> (f64, f64) {
    //当为' a '和' b '指定具体类型时，下面一行将为函数生成代码。
    area_pair_static(a, b)
}

//此函数执行与' area_pair_static '相同的函数，但使用动态分派。编译器将为这个函数生成代码。对' area '的调用是通过' vtable '进行的。
pub fn area_pair_dynamic(a: &dyn Shape<T = f64>, b: &dyn Shape<T = f64>) -> (f64, f64) {
    (a.area(), b.area())
}
// 以上是区别于Box的使用的，上面的仅仅是动态的引用，是借用，Box会拿所有权
// 两个内容： 一个是data的指针，一个是vtable（virtual method table）是包含对象实现的特征的方法的地址

fn main() {}
