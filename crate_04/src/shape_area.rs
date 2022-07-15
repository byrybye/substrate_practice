trait Shape {
    fn area(&self) -> f32;
}

#[derive(Debug)]
struct Rectangle {
    pub width: f32,
    pub height: f32,
}
#[derive(Debug)]
struct Triangle {
    pub side: f32,
}
#[derive(Debug)]
struct Circle {
    pub radius: f32,
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        &self.width * &self.height
    }
}

impl Shape for Triangle {
    fn area(&self) -> f32 {
        self.side * 0.5 * 3.0_f32.sqrt() / 2.0 * self.side
    }
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        3.14 * &self.radius * &self.radius
    }
}

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
