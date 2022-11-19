use generics::Inspect;
use std::fmt;

#[derive(Debug)]
pub struct Kube {
    pub name: String,
    pub cni: String,
    pub size: u8,
    pub overlay: bool,
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

impl fmt::Display for Kube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut mode = String::from("DIRECT");
        if self.overlay {
            mode = String::from("TUNNELED");
        }
        write!(f, "cluster {} has {} nodes using {} as cni [{}]", self.name, self.size, self.cni, mode)
    }
}

impl Inspect for Kube {
    fn show_name(&self) -> String {
        format!("{}", self.name)
    }
}

pub struct Cat {
    name: String,
    age: u8,
}

impl Cat {
    pub fn new(name: String, age: u8) -> Self {
        Self {
            name,
            age,
        }
    }
}

// use trait's default implementation
//impl Inspect for Cat {}

impl Inspect for Cat {
    fn show_name(&self) -> String {
        format!("{}", self.name)
    }

    fn info(&self) -> String {
        format!("A cat named {} is {} years old.", self.name, self.age)
    }
}



#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn get_x(&self) -> &T {
        &self.x
    }

    pub fn get_y(&self) -> &T {
        &self.y
    }
}

pub struct PointX<X1, Y1> {
    pub x: X1,
    pub y: Y1,
}

impl<X1, Y1> PointX<X1, Y1> {
    pub fn melange<X2, Y2>(self, other: PointX<X2, Y2>) -> PointX<X1, Y2> {
        PointX {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<f32> {
    pub fn dist_from_zero_point(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
