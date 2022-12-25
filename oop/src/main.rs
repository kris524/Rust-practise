
// THIS WAS BUILD USING ChatGPT for learning purposes

trait Shape {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn draw(&self) {
        // code to draw a circle with the specified radius
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn draw(&self) {
        // code to draw a rectangle with the specified width and height
        println!("I am a ")
    }
}

struct ShapeFactory {
    // other fields and methods
}

impl ShapeFactory {
    fn create_shape(&self, shape_type: &str) -> Box<dyn Shape> {
        match shape_type {
            "circle" => Box::new(Circle { radius: 1.0 }),
            "rectangle" => Box::new(Rectangle { width: 2.0, height: 3.0 }),
            _ => panic!("Invalid shape type"),
        }
    }
}

fn main() {
    let factory = ShapeFactory { /* other fields */ };

    let circle = factory.create_shape("circle");
    let rectangle = factory.create_shape("rectangle");

    circle.draw();
    rectangle.draw();

}
