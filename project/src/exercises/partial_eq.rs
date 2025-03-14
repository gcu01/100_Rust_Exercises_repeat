use std::cmp::PartialEq;
#[derive(Debug)]
pub struct Ticket {
    pub title: String,
    pub description: String,
    pub status: String,
}

// TODO: Implement the `PartialEq` trait for `Ticket`.

impl PartialEq for Ticket {
    fn eq(&self, other: &Self) -> bool {
        if *self.title == *other.title && *self.description == *other.description && *self.status == *other.status { return true;}
        false
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_partialeq() {
        let t1: Ticket = Ticket{title: "One".into(), description: "To continue".into(), status: "In Progress".into()};
        let t2: Ticket = Ticket { title: "One".into(), description: "To continue".into(), status: "In Progress".into()};
        let t3: Ticket = Ticket { title: "One".into(), description: "To continue".into(), status: "To-Do".into()};
        assert_eq!(true, t1.eq(&t2));
        assert_eq!(false, t1==t3);
    }
}