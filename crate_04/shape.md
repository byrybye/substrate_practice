程序代码
```
//图形 trait 包含一个计算面积函数
trait Shape {
    fn area(&self) -> f32;
}

//长方形
#[derive(Debug)]
struct Rectangle {
    pub width: f32,
    pub height: f32,
}

//三角形
#[derive(Debug)]
struct Triangle {
    pub side: f32,
}

//原形
#[derive(Debug)]
struct Circle {
    pub radius: f32,
}

//实现Shape
impl Shape for Rectangle {
    fn area(&self) -> f32 {
        &self.width * &self.height
    }
}

//实现Shape
impl Shape for Triangle {
    fn area(&self) -> f32 {
        self.side * 0.5 * 3.0_f32.sqrt() / 2.0 * self.side
    }
}

//实现Shape
impl Shape for Circle {
    fn area(&self) -> f32 {
        3.14 * &self.radius * &self.radius
    }
}

//计算面积函数 必须使用 &dyn 才能动态分发
fn area(shape: &dyn Shape) -> f32 {
    shape.area()
}

fn main() {    
    let c = Circle { radius: 2.0 };
    let r: Rectangle = Rectangle {
        width: 2.0,
        height: 3.0,
    };
    let t: Triangle = Triangle { side: 2.0 };
    println!("shape:{:#?} area:{}", &c, area(&c));
    println!("shape:{:#?} area:{}", &r, area(&r));
    println!("shape:{:#?} area:{}", &t, area(&t));    
}

```

运行命令 </BR>
cargo run --bin shape_area     </BR>

运行结果 </BR>
![image](https://github.com/byrybye/substrate_practice/blob/main/crate_04/shape.png)
