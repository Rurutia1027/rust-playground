// Adding Functions to Structs

struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32,
}

// we want add a ability to print the car's information
// but this function does  not belong the scope of the Car
fn display_car_info(car: &Car) {
    println!(
        "Owner: {}, Year: {}, Price: {}",
        car.owner, car.year, car.price
    );
}

// here is how to declare & implement an inner associated print info function
impl Car {
    // this is the associated functions
    // which declared and implemented in the scope of the struct
    // but it's inner logic will not depend on 'self'
    // so parameter do not contain self or &mut self
    // it also belongs to Car, and can be invoked by ${struct_name}::monthly_insurance()
    fn monthly_insurance() -> u32 {
        123
    }

    fn selling_price(&self) -> u32 {
        // the associted function can be
        self.price + Car::monthly_insurance()
    }

    fn new(name: String, year: u32) -> Self {
        Self {
            owner: name,
            year: year,
            fuel_level: 0.0,
            price: 0,
        }
    }

    fn display_info(&self) {
        println!(
            "[display_info]: Owner: {}, Year: {}, Price: {}",
            self.owner, self.year, self.price
        );
    }

    // here we create a function to re-fuel the car
    // which means do modificaitons to car's inner fields
    // this should passing a mutable reference to the function
    fn refuel(&mut self, refuel_val: f32) {
        self.fuel_level += refuel_val;
        println!(
            "Refuel done with total fuel value {}",
            self.fuel_level
        );
    }

    fn sell(self) -> Self {
        self
    }
}

fn main() {
    let mut my_car = Car {
        owner: String::from("Mae"),
        year: 2023,
        fuel_level: 20.3,
        price: 5600,
    };

    display_car_info(&my_car);

    my_car.display_info();

    my_car.refuel(10.5);
    let new_owner = my_car.sell();
    // since we transfer the ownership from mycar to new_owner
    // so here my_car cannot invoke its heap-object's function anymore
    // --> this will go error: my_car.refuel(10.4);

    let new_car = Car::new("XYZ".to_string(), 2011);
}
