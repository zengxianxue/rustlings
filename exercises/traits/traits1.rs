// traits1.rs
// Time to implement some traits!
//
// Your task is to implement the trait
// `AppendBar' for the type `String'.
//
// The trait AppendBar has only one function,
// which appends "Bar" to any object
// implementing this trait.

trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    //Add your code here
    // trait 中的`append_bar`方法是 `self` ==> `self:Self`（需要move）。实现的时候可以使用`mut self` ==> `mut self:Self`
    // 函数传参同变量绑定类似，可以理解为：
    //        let self = Self; // 绑定Self到self
    //        let mut self = self; // 将self move为一个可变的self
    fn append_bar(mut self) -> Self {
        self.push_str("Bar");
        self
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
