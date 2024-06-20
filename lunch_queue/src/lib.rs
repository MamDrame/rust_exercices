#[derive(Debug, Clone)]
pub struct Queue {
    pub node: PersonLink,
}

pub type PersonLink = Option<Box<Person>>;

#[derive(Debug, Clone)]
pub struct Person {
    pub discount: i32,
    pub name: String,
    pub next_person: PersonLink,
}

impl Queue {
    pub fn new() -> Queue {
        Self { node: None }
    }
    pub fn add(&mut self, name: String, discount: i32) {
        let new_node = Some(Box::new(Person {
            name,
            discount,
            next_person: self.node.take(),
        }));
        self.node = new_node;
    }
    pub fn invert_queue(&mut self) {
        let a: Vec<Person> = self.unroll_queue();
        let mut new_node = Self::new();
        for person in a {
            new_node.add(person.name, person.discount)
        }
        self.node = new_node.node;
    }
    pub fn rm(&mut self) -> Option<(String, i32)> {
        let mut a: Vec<Person> = self.unroll_queue();
        let removed_person = a.pop();
        a.reverse();
        let mut new_node = Self::new();
        for person in a {
            new_node.add(person.name, person.discount)
        }
        self.node = new_node.node;
        removed_person.map(|person| (person.name, person.discount))
    }
    fn unroll_queue(&mut self) -> Vec<Person> {
        let mut a: Vec<Person> = Vec::new();
        let mut current = self.node.take();
        while let Some(mut person) = current {
            a.push(Person {
                name: person.name.clone(),
                discount: person.discount,
                next_person: None,
            });
            current = person.next_person.take();
        }
        a
    }
    pub fn search(&self, name: &str) -> Option<(String, i32)> {
        let mut current = self.node.as_ref();
        while let Some(person) = current {
            if person.name == name {
                return Some((person.name.clone(), person.discount));
            }
            current = person.next_person.as_ref();
        }
        None
    }
}
