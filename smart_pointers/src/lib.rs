use std::rc::Rc;

pub struct Noeud {
    pub name: String,
    pub children: Vec<Rc<Noeud>>,
}

impl Noeud {
    pub fn new(name: &str) -> Noeud {
        Noeud {
            name: name.to_string(),
            children: vec![],
        }
    }

    pub fn append_to(self: Rc<Self>, parent: &mut Noeud) {
        parent.children.push(self);
    }
}
