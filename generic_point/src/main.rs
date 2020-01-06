// struct Point<T> {
//     x: T,
//     y: T,
// }

struct Point<T, U> {
    x: T,
    y: U,
}

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
// enum Option<T> {
//     Some(T),
//     None
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn main() {
    // let integer = Point { x:5, y:10};
    // let float = Point {x:1.0, y:4.0};
    // let iteger_and_float = Point {x:1, y:4.0};

    // let p = Point { x: 5, y: 10 };
    // println!("p.x = {}", p.x());

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "nick", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
