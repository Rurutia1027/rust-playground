// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(0, 4);
//         assert_eq!(result, 4);
//     }
// }
mod shapes {
    pub struct Circle {
        radius: f32,
    }

    impl Circle {
        pub fn new(radius: f32) -> Self {
            Circle { radius }
        }

        pub fn new_v2(radius: f32) -> Result<Self, String> {
            if radius >= 0.0 {
                Ok(Circle::new(radius))
            } else {
                Err(String::from("illegal radius received, it should be positive"))
            }
        }

        pub fn new_v3(radius: f32) -> Self {
            match radius {
                // use this panic will not cause exectuion crush during the test period
                -10.0..=0.0 => {
                    panic!("radius should be positive")
                }
                ..=-10.0 => {
                    panic!("is less than -10.0")
                }
                _ => Circle { radius },
            }
        }

        pub fn contains(&self, other: &Circle) -> bool {
            self.radius > other.radius
        }
    }
}

#[cfg(test)]
mod tests {
    use shapes::Circle;

    use super::*;

    #[test]
    fn larger_circule_should_contain_smaller() {
        let larger_circle = Circle::new(20.1);
        let smaller_circule = Circle::new(20.0);

        assert_eq!(
            larger_circle.contains(&smaller_circule),
            true,
            "Custom failure message"
        );

        assert_ne!(
            larger_circle.contains(&smaller_circule),
            false
        );

        assert!(larger_circle.contains(&smaller_circule));
    }

    #[test]
    fn smaller_circule_should_not_contain_larger_one() {
        let larger_circle = Circle::new(20.1);
        let smaller_circule = Circle::new(20.0);

        assert_eq!(
            smaller_circule.contains(&larger_circle),
            false
        );
    }

    #[test]
    fn create_circle_with_negative_value() -> Result<(), String>
    {
        let some_circle = Circle::new_v2(-10.2 * -1.0)?;
        Ok(())
    }

    #[test]
    #[should_panic(expected = "is lesser than -10.0")]
    fn should_not_create_and_panic() {
        // here we use the panic to get the result like this:
        // test tests::should_not_create_and_panic - should panic ... ok
        // even though here return an Error but it is expected will not terminate during unit test cases' execution
        let some_circle = shapes::Circle::new_v3(-10.0);
    }
}
