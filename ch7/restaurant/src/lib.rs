mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Add to waitlist.");
        }

        fn seat_at_table() {
            println!("Seat at table.");
        }
    }

    mod serving {
        fn take_order() {
            println!("Take order.");
        }

        fn serve_order() {
            println!("Serve order.");
        }

        fn take_payment() {
            println!("Take payment.");
        }
    }
}

mod back_of_house {
    fn fix_incorrect_order() {
        println!("Fix incorrect order.");
    }

    fn cook_order() {
        println!("Cook order.");
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
