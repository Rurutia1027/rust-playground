// Associated Types in Traits

#[derive(Debug)]
struct Km {
    value: u32,
}

#[derive(Debug)]
struct Kmh {
    value: u32,
}

#[derive(Debug)]
struct Miles {
    value: u32,
}

#[derive(Debug)]
struct Mph {
    value: u32,
}

impl Kmh {
    fn distance_in_three_hours(&self) -> Km {
        Km {
            value: self.value * 3,
        }
    }
}

impl Mph {
    fn distance_in_three_hours(&self) -> Miles {
        Miles {
            value: self.value * 3,
        }
    }
}

// from the examples above, we can see that there is a funciton `distance_in_three_hours` that defined
// for multiple time, to avoid this, we can declare a trait with one function define in it
// and let the structs impl the trait and provide the different implementaiton logic for that
trait DistanceThreeHours {
    // Why do we need to declare an associated type within the trait ?
    // This is because we want the trait to be implemented by different structs,
    // but the return types of the inner functions in those structs are not the same.
    // By declaring an associated type within the trait, we create a way to unify these differing return types
    // while still allowing each implementation to specify its own concrete type.
    type Distance;
    fn distance_in_three_hours_interface(&self) -> Self::Distance;
}

impl DistanceThreeHours for Kmh {
    type Distance = Km;

    fn distance_in_three_hours_interface(&self) -> Self::Distance {
        Km {
            value: self.value * 3,
        }
    }
}

impl DistanceThreeHours for Mph {
    type Distance = Miles;
    fn distance_in_three_hours_interface(&self) -> Self::Distance {
        Miles {
            value: self.value * 3,
        }
    }
}

fn main() {
    let speed_Kmh = Kmh { value: 90 };
    let distance_Km = speed_Kmh.distance_in_three_hours();

    println!(
        "At {:?} you will travel {:?} in 3 hours",
        speed_Kmh, distance_Km
    );

    let speed_Mph = Mph { value: 90 };
    let distance_Miles = speed_Mph.distance_in_three_hours();

    println!(
        "At {:?} you will travel {:?} in 3 hours",
        speed_Mph, distance_Miles
    );

    /// ----- invoke Associated Types in Traits Function ----
    
    println!("----- invoke Associated Types in Traits Function ----");
    let kmh_intance = Kmh { value: 90 };
    let km_instance = kmh_intance.distance_in_three_hours_interface();
    println!(
        "At {:?} you will travel {:?} in 3 hours",
        kmh_intance, km_instance
    );

    let mph_instance = Mph { value: 39 };
    let mile_instance = mph_instance.distance_in_three_hours_interface();

    println!(
        "At {:?} you will travel {:?} in 3 hours",
        mph_instance, mile_instance
    );
}
