use super::MenuComponent;

pub struct MenuVec {
    elements: Vec<MenuComponent>,
}

impl MenuVec {
    pub fn new() -> MenuVec {
        MenuVec { elements: vec![] }
    }
    pub fn len(&self) -> usize {
        self.elements.len()
    }
    pub fn remove(&mut self, idx: usize) -> MenuComponent {
        self.elements.remove(idx)
    }
    pub fn push(&mut self, component: MenuComponent) {
        self.elements.push(component)
    }
    pub fn get(&self, idx: usize) -> Option<&MenuComponent> {
        self.elements.get(idx)
    }
}

impl<'a> IntoIterator for &'a MenuVec {
    type Item = &'a MenuComponent;

    type IntoIter = MenuVecIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MenuVecIter::new(self)
    }
}

pub struct MenuVecIter<'a> {
    curr: usize,
    menu_vec: &'a MenuVec,
}

impl<'a> MenuVecIter<'a> {
    fn new(menu_vec: &MenuVec) -> MenuVecIter {
        MenuVecIter { curr: 0, menu_vec }
    }
}

impl<'a> Iterator for MenuVecIter<'a> {
    type Item = &'a MenuComponent;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr < self.menu_vec.len() {
            self.curr += 1;
            self.menu_vec.get(self.curr - 1)
        } else {
            None
        }
    }
}
