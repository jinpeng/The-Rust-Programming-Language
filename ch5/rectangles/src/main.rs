#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, a_rect: &Rectangle) -> bool {
        self.width >= a_rect.width && self.height >= a_rect.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 20,
        height: 50,
    };

    println!("The area of {:?} is {}", &rect, rect.area());

    let second_rect = Rectangle {
        width: 10,
        height: 5,
    };

    let third_rect = Rectangle {
        width: 30,
        height: 50,
    };

    let fourth_rect = Rectangle::square(30);

    println!(
        "{:?} can hold {:?}: {}",
        &rect,
        &second_rect,
        rect.can_hold(&second_rect)
    );
    println!(
        "{:?} can hold {:?}: {}",
        &rect,
        &third_rect,
        rect.can_hold(&third_rect)
    );
    println!(
        "{:?} can hold {:?}: {}",
        &rect,
        &fourth_rect,
        rect.can_hold(&fourth_rect)
    );
}
