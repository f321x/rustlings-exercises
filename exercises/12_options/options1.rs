// options1.rs
//
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    let amount: u16 = 5;

    if time_of_day <= 21 {
        Option::Some(amount)
    }   else if time_of_day > 21 && time_of_day < 24 {
        Option::Some(0)
    }   else {
        Option::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get at the value contained in the
        // Option?
        let icecreams = maybe_icecream(12);
        assert_eq!(icecreams, Option::Some(5));
    }
}
