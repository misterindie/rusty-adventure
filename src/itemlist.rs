use item::Item;

pub struct ItemList {
    list: Vec<Item>
}
#[allow(dead_code)]
impl ItemList {
    pub fn new() -> ItemList {
        ItemList { list: vec![] }
    }

    pub fn from(items: &[&Item]) -> ItemList {
        let mut result = Self::new();

        for item in items {
            result.add(item, None);
        }

        result
    }

    pub fn add(&mut self, item: &Item, message: Option<String>){
        self.list.push(Item::from(item));
        match message {
            None => (),
            Some(ref x) => println!("{} {}", item.get_name(), x)
        }
    }

    pub fn render(&self){
        for i in self.list.iter(){
            println!("{}:   {}",i.get_name(), i.get_value());
        }
    }

    pub fn get_list(&self) -> &Vec<Item> {
        &self.list
    }

    //TODO: delete an item from itemlist
    pub fn delete(&mut self, _item: &Item){

    }
}
