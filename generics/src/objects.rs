use generics::Inspect;

#[derive(Debug)]
pub struct Kube {
    name: String,
    cni: String,
    size: u8,
    overlay: bool,
}

impl Kube {
    pub fn new(name: String, cni: String, size: u8, overlay: bool) -> Self {
        Self {
            name,
            cni,
            size,
            overlay,
        }
    }
}

impl Inspect for Kube {
    fn info(&self) -> String {
        let mut mode = String::from("DIRECT");
        if self.overlay {
            mode = String::from("TUNNELED");
        }
        format!("cluster {} has {} nodes using {} as cni [{}]", self.name, self.size, self.cni, mode)
    }
}

#[derive(Debug)]
pub struct Cat {
    pub name: String,
    pub age: u8,
}

impl Cat {
    pub fn new(name: String, age: u8) -> Self {
        Self {
            name,
            age,
        }
    }
}

impl Inspect for Cat {
    fn info(&self) -> String {
        format!("A cat named {} is {} years old.", self.name, self.age)
    }
}
