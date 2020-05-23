pub mod errors {
    pub fn illegal_char(position: usize) {
        panic!(format!("Illegal character detected at position {}", position));
    }
}

pub mod errors;