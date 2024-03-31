// OOP languages share certain common characteristics, namely objects, encapsulation, and inheritance

// Objects - In rust, objects are represented by structs, enums and traits (impl)
// Encapsulation - Rust provides public and private access control - eg. below, list is private and can only be accessed by methods in the struct
// Inheritance - Rust does not have inheritance directly on struct, but it has trait objects which can be used to achieve similar functionality
// Default trait method implementations can be used to provide default implementations for methods in a trait but this cannot define fields
// Polymorphism - Rust uses generics + trait bounds OR trait objects to achieve polymorphism

pub struct AverageCollection {
    list: Vec<i32>,
    average: f64,
}

impl AverageCollection {
    pub fn new() -> Self {
        AverageCollection {
            list: vec![],
            average: 0.0,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

pub fn oops() {
    let mut avg = AverageCollection::new();
    avg.add(10);
    avg.add(20);
    avg.add(30);
    println!("Average: {}", avg.average());
    avg.remove();
    println!("Average: {}", avg.average());
    avg.remove();
    println!("Average: {}", avg.average());
    avg.remove();
    println!("Average: {}", avg.average());
}