fn main() {
    let num = 40;
    if num < 50 {
        println!("num value is less than 50");
    } else {
        println!("num value is larger than 50");
    }

    let marks = 95;
    // let mut grade = 'N';
    // let grade = if marks >= 90 {
    //     'A'
    // } else if marks >= 80 {
    //     'B'
    // } else if marks >= 70 {
    //     'C'
    // } else {
    //     'D'
    // };

    // println!("{:?}", grade);

    let mut grade = 'A';
    match marks {
        90..=100 => grade = 'A',
        80..=89 => grade = 'B',
        70..=79 => grade = 'C',
        _ => grade = { 'F' },
    }

    let ret_grade = match marks {
        90..=100 => 'A',
        80..=90 => 'B',
        70..=79 => 'C',
        _ => 'F',
    };
    println!("final ret_grade={ret_grade}");
}
