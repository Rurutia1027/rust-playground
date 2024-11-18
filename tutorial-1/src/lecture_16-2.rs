// Enumeration

enum TravelType {
    Car,
    Train,
    Aeroplance,
}

// another feature support in Rust for enum
// is it can add inner functions like struct
// then the inner defined funcutions can be get access by
// ${enum_type_name}::${inner_func_name}
impl TravelType {
    fn travel_allowance(&self, miles: f32) -> f32 {
        let allowance = match self {
            TravelType::Car => miles * 2.0,
            TravelType::Train => miles * 3.0,
            TravelType::Aeroplance => miles * 5.0,
        };

        // then we can calculate correspoinding allowance by passing
        // current travel type
        allowance
    }
}

fn main() {
    let participant = TravelType::Car;
    let allowance_value = participant.travel_allowance(60.0);
    println!(
        "allowance value for 60.0 miles by Car is {}",
        allowance_value
    );
}
