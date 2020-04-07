
pub struct Item {
    pub name: String,
    pub desc: String,
    pub value: i32,
}

impl Item {
    pub fn new(name: String, desc: String, value: i32) -> Item {
        Item{
            name: name,
            desc: desc,
            value: value,
        }
    }

    pub fn from(&self) -> Item {
        let result = Item::new(self.name.to_string(), self.desc.to_string(), self.value);
        result
    }

    pub fn get_name(&self) -> &String {return &self.name;}
    pub fn get_desc(&self) -> &String {return &self.desc;}
    pub fn get_value(&self) -> &i32 {return &self.value;}
}
