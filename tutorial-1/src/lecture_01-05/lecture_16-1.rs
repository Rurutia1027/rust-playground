// Enums

fn main() {
    let mut day = "Saturday".to_string();
    let week_day = vec![
        "Monday".to_string(),
        "Tuesday".to_string(),
        "Wednesday".to_string(),
        "Thursday".to_string(),
        "Friday".to_string(),
        "Saturday".to_string(),
        "Sunday".to_string(),
    ];

    // directly re-assign is not allowed in Rust
    // because week_day[index] value is String which is heap-located data
    // and directly re-assign will result in ownership transfer which is not
    // allowed in Rust -- yes, this is the vector is a little bit different from the struct instance items can get access to
    // so we use clone to create a new item instead of retriving from vector directly
    day = week_day[2].clone();

    // but in rust it provides a better solution for this scenario
    // that is enumeration

    // here we create a variable and set it to the variable defined in the
    // enum of WeekDay like this:
    let day = WeekDay::Saturday;

    // enum define a type and help developer avoid mis-spelling wrong words
     
}

enum WeekDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
