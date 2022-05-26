// quiz4.rs
// This quiz covers the sections:
// - Modules
// - Macros

// Write a macro that passes the quiz! No hints this time, you can do it!

mod my_macro_mod {
    #[macro_export]
    macro_rules! custom_macro {
        ( $x:expr) => {
            {
                let mut str: String = String::from("Hello ");
                str.push_str($x);
                str
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::custom_macro;

    #[test]
    fn test_my_macro_world() {
        assert_eq!(custom_macro!("world!"), "Hello world!");
    }

    #[test]
    fn test_my_macro_goodbye() {
        assert_eq!(custom_macro!("goodbye!"), "Hello goodbye!");
    }
}
