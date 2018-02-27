pub struct Dollar {
    pub amount: u32,

}

impl Dollar {
    pub fn times(&self, multiplier: u32) -> Dollar {
        Dollar{amount: self.amount * multiplier}
    }
}
