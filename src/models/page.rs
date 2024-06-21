pub struct Article {
    title: String,
    body: String,
    published: bool,
    content: String,
}

pub struct Section {
    name: String,
    content: String,
}

pub type MenuItem = String;

pub struct Menu {
    items: Vec<MenuItem>,
}

pub struct Home {
    hero: String,
    sections: Vec<Section>,
    top_menu: Menu,
    footer_menu: Menu,
}