use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Blog {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>,
}

impl Blog {
    pub fn new() -> Self {
        Blog {
            drops: Cell::new(0),
            states: RefCell::new(Vec::new()),
        }
    }

    pub fn new_article(&self, body: String) -> (usize, Rc<Article>) {
        let id = self.new_id();
        let article = Article::new(id, body, self);
        (id, article)
    }

    pub fn new_id(&self) -> usize {
        let mut states = self.states.borrow_mut();
        states.push(false);
        states.len() - 1
    }

    pub fn is_dropped(&self, id: usize) -> bool {
        self.states.borrow()[id]
    }

    pub fn add_drop(&self, id: usize) {
        let mut states = self.states.borrow_mut();
        if states[id] {
            panic!("{} is already dropped", id);
        } else {
            states[id] = true;
            let drops = self.drops.get();
            self.drops.set(drops + 1);
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Article<'a> {
    pub id: usize,
    pub body: String,
    pub parent: &'a Blog,
}

impl<'a> Article<'a> {
    pub fn new(id: usize, body: String, parent: &'a Blog) -> Rc<Article<'a>> {
        Rc::new(Article { id, body, parent })
    }

    pub fn discard(self: Rc<Self>) {
        if Rc::strong_count(&self) == 1 {
            drop(self);
        } else {
            panic!("{} is already dropped", self.id);
        }
    }
}

impl<'a> Drop for Article<'a> {
    fn drop(&mut self) {
        self.parent.add_drop(self.id);
    }
}