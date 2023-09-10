// Complete the code by addressing the TODO.

trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement trait `AppendBar` for a vector of strings.
// Learning: if we are getting non mutable reference then we can assign that to a mutable reference
// then we can change it and return it.
impl AppendBar for Vec<String> {
    fn append_bar(self) -> Self {
        let mut res = self;
        res.push(String::from("Bar"));
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
