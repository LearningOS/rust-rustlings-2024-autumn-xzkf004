// structs2.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs2` or use the `hint` watch subcommand for a
// hint.



#[derive(Debug)]
struct Order {
    name: String,
    year: u32,
    made_by_phone: bool,
    made_by_mobile: bool,
    made_by_email: bool,
    item_number: u32,
    count: u32,
}

fn create_order_template() -> Order {
    Order {
        name: String::from("Bob"),
        year: 2019,
        made_by_phone: false,
        made_by_mobile: false,
        made_by_email: true,
        item_number: 123,
        count: 0,
    }
}

fn creat_my_order() -> Order {
    Order {
        name: String::from("Hacker in Rust"),
        year: 2019,
        made_by_phone: false,
        made_by_mobile: false,
        made_by_email: true,
        item_number: 123,
        count: 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn your_order() {
        let order_template = create_order_template();
        let my_order = creat_my_order();
        // TODO: Create your own order using the update syntax and template above!
        // let your_order =
        assert_eq!(my_order.name, "Hacker in Rust");
        assert_eq!(my_order.year, order_template.year);
        assert_eq!(my_order.made_by_phone, order_template.made_by_phone);
        assert_eq!(my_order.made_by_mobile, order_template.made_by_mobile);
        assert_eq!(my_order.made_by_email, order_template.made_by_email);
        assert_eq!(my_order.item_number, order_template.item_number);
        assert_eq!(my_order.count, 1);
    }
}
