#[derive(Debug)]
#[allow(dead_code)]
pub struct Pizza {
    pub dough: String,
    pub cheese: String,
    pub topping: String,
}
impl Pizza {
    pub fn lunch(tooping: &str) -> Pizza {
        Pizza {
            dough: String::from(tooping),
            cheese: String::from(tooping),
            topping: String::from(tooping),
        }
    }
}
pub mod help_customer {
    pub fn seat_at_table() {
        println!("seat at table");
    }
    #[allow(dead_code)]
    pub fn take_order() {
        seat_at_table();
        println!("take order");
    }
}
