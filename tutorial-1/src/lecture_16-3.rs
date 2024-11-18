// Enumeration

enum TravelType {
    Car(f32, String),
    Train(f32, String),
    Aeroplance(f32, String),
}

// add a TravelType inner function
impl TravelType {
    fn travel_allowance(&self) -> f32 {
        let ret = match self {
            TravelType::Car(miles, name) => {
                println!("calculating travelling by {} car with miles {} allowance",
                name, miles);
                miles * 2.0
            }
            TravelType::Train(miles, name) => {
                println!("calculating travelling by {} train with miles {} allowance",
                name, miles * 3.0);
                miles * 3.0
            }
            TravelType::Aeroplance(miles, name) => {
                println!("calculating travelling by {} aeroplance with miles {} allowance",
                name, miles * 5.0);
                miles * 5.0
            }
        };

        ret
    }
}

fn main() {
    let train =
        TravelType::Train(50.0, "K-919".to_string().to_owned());
    let allowance = train.travel_allowance();
    println!("traveling by train allowance is {}", allowance);
}
