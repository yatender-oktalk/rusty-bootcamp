struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }
}

trait Paint {
    fn make(&self) {
        println!("This is paint make");
    }
}

struct Car {
    make: String,
    max_speed: usize,
    brand: String,
    wheel_size: usize,
}

struct Truck {
    make: String,
    max_speed: usize,
    brand: String,
    wheel_size: usize,
}

impl Paint for Truck {}

fn main() {
    let yy = Wrapper::new("Hello");
    println!("{}", yy.value);

    let x = Wrapper::new(44);
    println!("{}", x.value);
}
