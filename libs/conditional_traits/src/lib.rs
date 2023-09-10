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

#[derive(Clone, Debug, PartialEq)]
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
    fn paint(&self) -> &str {
        "This is painted"
    }
}

trait Park {
    fn paint(&self) {
        println!("This is parked")
    }
}

impl Default for Car {
    fn default() -> Self {
        Car::new("2020".to_owned())
    }
}

// impl Park for Car {}
impl Paint for Truck {
    fn paint(&self) -> &str {
        "Truck is painted"
    }
}

impl Paint for Car {
    fn paint(&self) -> &str {
        "Car is painted"
    }
}

// impl Paint for Truck {}
impl Paint for House {
    fn paint(&self) -> &str {
        "House is painted"
    }
}

// the object who is calling should send the `as_ref()`
fn paint_red(object: &dyn Paint) -> &str {
    object.paint()
}

fn create_paintable_or_parkable_object(vehicle: bool) -> Box<dyn Paint> {
    if vehicle {
        Box::new(Car {
            make: "2020".to_owned(),
        })
    } else {
        Box::new(House {
            address: "Chaussestraße 7, 10115".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let house = House::new(String::from("Khurja"));
        let car = Car::new(String::from("2020"));
        let house = House::new(String::from("2020"));
        // assert_eq!(String::from("Painting red"), paint_red(car.clone()));
        // assert_eq!(String::from("Painting red 2"), paint_red_2(car.clone()));
        assert_eq!(car, Car::default());
        let x = create_paintable_or_parkable_object(true);
        // assert_eq!(car, create_paintable_or_parkable_object(true));
        assert_eq!("Car is painted", paint_red(x.as_ref()));

        let xx = create_paintable_or_parkable_object(false);
        // assert_eq!(car, create_paintable_or_parkable_object(true));
        assert_eq!("House is painted", paint_red(xx.as_ref()));

        // create a vector of objects which implements the same traits
        let objects: Vec<&dyn Paint> = vec![&car, &house];
    }
}
