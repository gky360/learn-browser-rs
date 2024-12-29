use alloc::string::String;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Attribute {
    name: String,
    value: String,
}

impl Attribute {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }

    pub fn add_char(&mut self, c: char, is_name: bool) {
        if is_name {
            self.name.push(c);
        } else {
            self.value.push(c);
        }
    }
}
