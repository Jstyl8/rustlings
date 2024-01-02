// traits1.rs
//
// Time to implement some traits! Your task is to implement the trait
// `AppendBar` for the type `String`. The trait AppendBar has only one function,
// which appends "Bar" to any object implementing this trait.
//
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a
// hint.

trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // TODO: Implement `AppendBar` for type `String`.
    fn append_bar(self) -> Self {
        // - respecting signature, self without mut but cloning
        // let mut bared = self.clone();
        // bared.push_str("Bar");
        // bared

        /*
        This code will work just as efficiently as if you had declared self
        as mutable because self here is not a reference;
        it's the actual owner of the string data.
        By assigning self to new_string, we're just transferring the ownership,
        not copying the data, which avoids the clone operation.
        */
        let mut new_string = self;
        new_string.push_str("Bar");
        new_string

        // - is this allowed? I am changing the parameter to mut self
        // self.push_str("Bar");
        // self
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
