pub struct Dollar {
    pub amount: u32,

}

impl Dollar {
    pub fn times(&mut self, multiplier: u32) {
        self.amount *= multiplier;
    }
}
