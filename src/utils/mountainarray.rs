pub struct MountainArray {
    array: Vec<i32>,
}

impl MountainArray {
    pub fn new(values: &[i32]) -> Self {
        Self {
            array: values.into(),
        }
    }

    pub fn get(&self, index: i32) -> i32 {
        assert!(index < self.array.len() as i32);

        *self.array.get(index as usize).unwrap()
    }

    pub fn length(&self) -> i32 {
        self.array.len() as i32
    }
}
