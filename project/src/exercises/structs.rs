struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    // TODO: implement the `new` function.
    //  The following requirements should be met:
    //   - Only `To-Do`, `In Progress`, and `Done` statuses are allowed.
    //   - The `title` and `description` fields should not be empty.
    //   - the `title` should be at most 50 bytes long.
    //   - the `description` should be at most 500 bytes long.
    //  The method should panic if any of the requirements are not met.
    //  You can find the needed panic messages in the tests.
    //
    // You'll have to use what you learned in the previous exercises,
    // as well as some `String` methods. Use the documentation of Rust's standard library
    // to find the most appropriate options -> https://doc.rust-lang.org/std/string/struct.String.html

    pub fn new(&mut self, title: String, description :String, status: String) {
        if title.len()<1  {
            panic!("the Title field should not be empty!");
        } else if title.len() > 50 {
            panic!("the Title field should not exceed 50 bytes");
        }

        if description.len()<1 {
            panic!("the Description field should not be empty!");
        } else if description.len() > 500 {
            panic!("the Description field should not exceed 500 bytes");
        }

        match status.as_str() {
           "To-Do" | "In Progress" | "Done"   => (),
            _ => panic!("the Status is not one of the required values: To-Do, In Progress or Done"), 
        }
        
        self.title = title;
        self.description = description;
        self.status = status;
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test1_new() {
        let mut t: Ticket = Ticket{title: "".to_string(), description: "".to_string(), status: "".to_string(),};
        t.new(String::from("title1"), "description1".to_string(), "To-Do".to_string());
        assert_eq!("title1".to_string(), t.title);
        assert_eq!("description1".to_string(), t.description);
        assert_eq!("To-Do".to_string(), t.status);
    }

    #[test]
    #[should_panic]
    fn test2_new() {
        let mut t: Ticket = Ticket { title: "".to_string(), description: "".to_string(), status: String::from("")};
        Ticket::new(&mut t, "".to_string(), "d1".to_string(), "s1".to_string());
        t.title;
    }
    #[test]
    #[should_panic]
    fn test3_new() {
        let mut t: Ticket = Ticket { title: "".to_string(), description: "".to_string(), status: String::from("")};
        Ticket::new(&mut t, "".to_string(), "d1".to_string(), "s1".to_string());
        t.description;
    }
    #[test]
    #[should_panic]
    fn test4_new() {
        let mut t: Ticket = Ticket { title: "".to_string(), description: "".to_string(), status: String::from("")};
        Ticket::new(&mut t, "".to_string(), "d1".to_string(), "s1".to_string());
        t.status;
    }
}