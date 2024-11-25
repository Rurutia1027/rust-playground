// Lifetimes in Structs

struct Object<'a> {
    data: &'a [i32],
}

/*
there are two rules we need to follow when we use the generic lifetime type.

The first is , each parameter that is a reference, gets its own lifetime parameter.
If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
If there are multiple input lifetime parameters, but one of them is &self, or &mut self,
the lifetime of self is assigned to all output lifetime parameters.
*/

impl<'a> Object<'a> {
    // here we follow the rule two of generic type lifetime:
    // if there are multiple input lifetime parameters, but one of them is &self, or &mut self,
    // then the lifetime of self is assigned to all output lifetime parameters.

    // and that's why we add the generic type of 'b to both function, and return value type and the self
    // we want to rely on the generic type to bind the self, function and return value's lifetime period together
    fn update_data<'b>(&'b mut self, new_data: &'a [i32]) -> &'b [i32] {
        let previous_data = self.data;

        // we need to add a 'a to the new_data
        // because the new data is gonna allocate to the struct object as an inner field as a input parameter
        // so, we need to let this input parameter has the same lifetime as the struct instance
        // so, add &'a [i32] is necessary
        self.data = new_data;
        previous_data
    }
}

fn main() {
    let mut data = Object {
        data: &[2, 3, 4, 5, 6],
    };

    let prev_data = data.update_data(&[3, 4, 5, 6, 7, 7]);
    println!("prev_data content {:?}", prev_data);
    println!("data's new data content  {:?}", data.data);
}
