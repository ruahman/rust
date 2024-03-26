#[derive(Debug)]
pub struct Pizza {
    pub dough: String,
}
impl Pizza {
    pub fn lunch(tooping: &str) -> Pizza {
        Pizza {
            dough: String::from(tooping),
        }
    }
}
