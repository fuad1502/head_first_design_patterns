mod menu_vec;

use menu_vec::MenuVec;

pub enum MenuComponent {
    Composite(MenuComposite),
    Leaf(MenuLeaf),
}

pub struct MenuComposite {
    name: String,
    description: String,
    items: MenuVec,
}

pub struct MenuLeaf {
    name: String,
    description: String,
    is_vegetarian: bool,
    price: f64,
}

pub struct Waittress {
    all_menu: MenuComponent,
}

impl MenuComponent {
    pub fn new_composite(name: &str, description: &str) -> MenuComponent {
        let c = MenuComposite {
            name: name.to_string(),
            description: description.to_string(),
            items: MenuVec::new(),
        };
        MenuComponent::Composite(c)
    }
    pub fn new_leaf(
        name: &str,
        description: &str,
        is_vegetarian: bool,
        price: f64,
    ) -> MenuComponent {
        let l = MenuLeaf {
            name: name.to_string(),
            description: description.to_string(),
            is_vegetarian,
            price,
        };
        MenuComponent::Leaf(l)
    }
    // Composite + Leaf
    pub fn get_name(&self) -> Option<String> {
        match self {
            MenuComponent::Composite(c) => Some(c.name.clone()),
            MenuComponent::Leaf(l) => Some(l.name.clone()),
        }
    }
    pub fn get_description(&self) -> Option<String> {
        match self {
            MenuComponent::Composite(c) => Some(c.description.clone()),
            MenuComponent::Leaf(l) => Some(l.description.clone()),
        }
    }
    pub fn print(&self) {
        match self {
            MenuComponent::Composite(c) => {
                println!("");
                println!("{}, {}", c.name, c.description);
                println!("---------------");
                for e in &c.items {
                    e.print();
                }
            }
            MenuComponent::Leaf(l) => {
                println!(
                    " {} {}, {}",
                    l.name,
                    if l.is_vegetarian { "(v)" } else { "" },
                    l.price
                );
                println!("  -- {}", l.description)
            }
        }
    }
    // Composite
    pub fn add(&mut self, component: MenuComponent) -> Result<(), ()> {
        match self {
            MenuComponent::Composite(c) => Ok(c.items.push(component)),
            MenuComponent::Leaf(_l) => Err(()),
        }
    }
    pub fn remove(&mut self, idx: usize) -> Result<MenuComponent, ()> {
        match self {
            MenuComponent::Composite(c) => {
                if idx < c.items.len() {
                    Ok(c.items.remove(idx))
                } else {
                    Err(())
                }
            }
            MenuComponent::Leaf(_l) => Err(()),
        }
    }
    // Leaf
    pub fn is_vegetarian(&self) -> Option<bool> {
        match self {
            MenuComponent::Composite(_c) => None,
            MenuComponent::Leaf(l) => Some(l.is_vegetarian),
        }
    }
    pub fn get_price(&self) -> Option<f64> {
        match self {
            MenuComponent::Composite(_c) => None,
            MenuComponent::Leaf(l) => Some(l.price),
        }
    }
}

impl Waittress {
    pub fn new(all_menu: MenuComponent) -> Waittress {
        Waittress { all_menu }
    }
    pub fn print(&self) {
        self.all_menu.print();
    }
}
