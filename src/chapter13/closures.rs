// similar to functions without name (anonymous), closures are blocks of code that can take arguments and return a value
use std::thread;
use std::time::Duration;

pub fn closures() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
    closure_environment();
}

// MEMOIZATION PATTERN (caching the result of expensive computation)
// Regular functions also implement the Fn traits, hence can be used as arguments for Cacher

// implement Cahcer with HashMap
use std::collections::HashMap;
struct Cacher<T>
where
    T: Fn(u32) -> u32,  //Fn trait is provided by the standard library. All closures implement at least one of the traits: Fn, FnMut, or FnOnce
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_closure = |num| {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(1));
    //     num
    // };
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(1));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }

    // let example_closure = |x| x;
    // let s = example_closure(String::from("hello"));  // this type will be the concrete type of our closure
    // let n = example_closure(5); // error: expected struct `String`, found integer
}

// Capturing the Environment with Closures
// Closures can capture values from their environment in three ways, which directly map to the three ways a function can take parameters: taking ownership, borrowing mutably, and borrowing immutably.
// Functions cannot capture their environment, so closures have an advantage over functions in this context.
// Three ways: FnOnce, FnMut, and Fn, move keyword.
// FnOnce consumes the variables it captures from its enclosing scope, known as the closure’s environment. To consume the captured variables, the closure must take ownership of these variables and move them into the closure when it is defined. The Once part of the name represents the fact that the closure can’t take ownership of the same variables more than once, so it can be called only once.
// FnMut can change the environment because it mutably borrows values.
// Fn borrows values from the environment immutably.
fn closure_environment() {
    let x = 4;
    let equal_to_x = |z| z == x; // closure captures x from the environment
    
    let y = 4;
    assert!(equal_to_x(y));
    println!("can use x here: {:?}", x);

    // move keyword forces the closure to take ownership of the values it uses in the environment
    // let x = 2;
    // let equal_to_x = move |z: i32| z == x; // move keyword forces the closure to take ownership of the values it uses in the environment
    // assert!(equal_to_x(2));
    // println!("can't use x here: {:?}", x); // error: value borrowed here after move

}

// struct Cacher<T>
// where
//     T: Fn(u32) -> u32,
// {
//     calculation: T,
//     value: Option<u32>, // store a Hashmap to have multiple values cached
// }
// impl<T> Cacher<T>
// where
//     T: Fn(u32) -> u32,  // Fn trait is provided by the standard library. All closures implement at least one of the traits: Fn, FnMut, or FnOnce
// {
//     fn new(calculation: T) -> Cacher<T> {
//         Cacher {
//             calculation,
//             value: None,
//         }
//     }

//     fn value(&mut self, arg: u32) -> u32 {
//         match self.value {
//             Some(v) => v,
//             None => {
//                 let v = (self.calculation)(arg);
//                 self.value = Some(v);
//                 v
//             }
//         }
//     }
// }
// fn generate_workout(intensity: u32, random_number: u32) {
//     let expensive_closure = |num| {
//         println!("calculating slowly...");
//         thread::sleep(Duration::from_secs(1));
//         num
//     };

//     if intensity < 25 {
//         println!("Today, do {} pushups!", expensive_closure(intensity));
//         println!("Next, do {} situps!", expensive_closure(intensity));
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!("Today, run for {} minutes!", expensive_closure(intensity));
//         }
//     }

//     let example_closure = |x| x;
//     let s = example_closure(String::from("hello"));  // this type will be the concrete type of our closure
//     // let n = example_closure(5); // error: expected struct `String`, found integer
// }