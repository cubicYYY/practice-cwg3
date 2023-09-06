pub mod hashmap_macro;
pub mod my_rc;
pub mod my_stack;

fn main() {
    println!("Nothing to do in main(), run `cargo test` instead.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_macro() {
        use hash_map;
        let map = hash_map! {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "invalid" => -1
        };
        assert_eq!(map["one"], 1);
        assert_eq!(map["two"], 2);
        assert_eq!(map["three"], 3);
        assert_eq!(map["invalid"], -1);
        assert_eq!(map.len(), 4);
    }

    #[test]
    fn test_my_stack() {
        use my_stack::SimpleStack;
        let stack = SimpleStack::<i32>::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));

        stack.push(4);

        assert_eq!(stack.pop(), Some(4));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_my_rc() {
        use my_rc::MyRc;
        {
            let x = MyRc::new(7);
            {
                let y = x.clone();
                {
                    let z = x.clone();
                    assert_eq!(x.ref_count(), 3);
                    assert_eq!(y.ref_count(), 3);
                    assert_eq!(z.ref_count(), 3);

                    assert_eq!(*x, 7);
                    assert_eq!(*y, 7);
                    assert_eq!(*z, 7);
                } // drop
                assert_eq!(x.ref_count(), 2);
                assert_eq!(y.ref_count(), 2);

                assert_eq!(*x, 7);
                assert_eq!(*y, 7);
            } // drop
            assert_eq!(x.ref_count(), 1);
            assert_eq!(*x, 7);
        } // drop, you can check stdout
    }
}
