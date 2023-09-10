// https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/traits.html
//
struct House {
    address: String,
}

impl House {
    fn new(address: String) -> Self {
        House { address }
    }
}

#[derive(Clone)]
struct Car {
    make: String,
}

impl Car {
    fn new(make: String) -> Self {
        Car { make }
    }
}

struct Truck {
    make: String,
}

trait Paint {
    fn paint(&self) {
        println!("This is painted")
    }
}

trait Park {
    fn paint(&self) {
        println!("This is parked")
    }
}

impl Park for Car {}
impl Park for Truck {}

impl Paint for Car {}
impl Paint for Truck {}
impl Paint for House {}

fn paint_red<T: Paint>(object: T) -> String {
    String::from("Painting red")
}

fn paint_red_2(object: impl Paint + Park) -> String {
    String::from("Painting red 2")
}

fn paint_park<T>(object: T) -> String
where
    T: Paint + Park,
{
    String::from("Painting red 3")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let house = House::new(String::from("Khurja"));
        let car = Car::new(String::from("2020"));
        assert_eq!(String::from("Painting red"), paint_red(car.clone()));
        assert_eq!(String::from("Painting red 2"), paint_red_2(car.clone()));
        assert_eq!(String::from("Painting red 3"), paint_red_2(car));
    }
}
