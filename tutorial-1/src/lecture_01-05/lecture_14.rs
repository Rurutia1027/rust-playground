// Structs and its Types

struct Car {
    owener: String,
    year: u32,
    fuel_level: f32,
    price: u32,
}

fn main() {
    // this shows how to create an instance of a struct
    // all assignments should be wrapped by curly braces
    // and all fields should assigend correspoinding values
    let my_car = Car {
        owener: String::from("Mae"),
        year: 2023,
        fuel_level: 0.0,
        price: 5000,
    };

    let car_year = my_car.year;

    // fiels in the instance of struct are immutable created, so the expression below is not allowed here:
    // -- not -- allowed --> my_car.fuel_level = 30.3;

    // if we want the struct instance supports re-assign value, we need to initialize it as mutable struct instance
    let mut my_carr = Car {
        owener: String::from("rurutia"),
        year: 2048,
        fuel_level: 30.0,
        price: 234123,
    };

    my_carr.fuel_level = 50.3;

    // here we create a variable and let it point the inner field of the
    // owner(which is in type of String which means it is heap allocated variable)

    // and we know that, a re-assign of a heap-allocated variable means hand over the
    // ownershp from my_carr to the variable of extracted_owner
    let extracted_owner = my_carr.owener;
    println!("owner: {extracted_owner}");
    // this should be cause compiler error -->
    // println!(
    //     "owner cannot be get access from my_carr invoking {}",
    //     my_carr.owener
    // );
    // to this we can use clone to avoid hand over the ownership from struct instance to the variable

    let mut mutable_extracted_fuel_level = my_carr.fuel_level.clone();

    // modify clone value
    mutable_extracted_fuel_level = 550.0;

    let immutable_extracted_fuel_level_v2 = my_carr.fuel_level.clone();

    // in this way we can get both access to variable and struct instance's field value
    println!(
        "mutable extracted_fuel_level: {}, immutable_extracted_fuel_level_v2: {}, my_carr fuel level value: {}",
        mutable_extracted_fuel_level, immutable_extracted_fuel_level_v2, my_carr.fuel_level
    );

    // sometimes we may copy lot of same value from a struct
    // this is an example to copy all of the field values from my_carr
    // but exclude the owner field

    // and this expression goes right because we exclude the heap-located owner:String out of the copy
    // because if we use this method to invoke the copy the ownership of the owner in my_carr
    // will be transfer to the another_car, then my_carr trying to get access to the owner field
    // it will go wrong
    let another_car = Car {
        owener: "rurutia".to_string(),
        ..my_carr
    };

    // Tuple Structs
    // tuple with two integers
    let point1 = (1, 3);

    // tuple twith three integers
    let point2 = (1, 2, 3);

    // tuple structs are similar to normal tuple structs
    struct Point_2D(i32, i32);
    struct Point_3D(i32, i32, i32);

    // instances of those two tuple structs
    // tuple structs help us define variables with specific numbers of variables
    // and make sure specific variable appears at the correct place
    let p_2D = Point_2D(1, 2);
    let p_3D = Point_3D(3, 4, 5);

    // Unit Struct
    struct ABC;
}
