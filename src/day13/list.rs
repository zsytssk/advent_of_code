pub enum ListItem {
    Num(i32),
    List(Vec<ListItem>),
}

impl ListItem {
    pub fn parse() -> Self {
        ListItem::Num(0)
    }
    pub fn get_num(num: i32) -> Self {
        ListItem::Num(num)
    }
}
