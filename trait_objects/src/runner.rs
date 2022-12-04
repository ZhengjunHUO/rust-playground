pub trait Runner {
    fn run(&self);
}

pub struct K8s {
    pub nodes: u32,
    pub version: String,
    pub cni: String,
}

impl Runner for K8s {
    fn run(&self) {
        println!("Running on a {} nodes k8s[{}] powered by {}...", self.nodes, self.version, self.cni);
    }
}

pub struct Baremetal {
    pub os: String,
    pub platform: String,
    pub release: String,
}

impl Runner for Baremetal {
    fn run(&self) {
       println!("Running on a {} {}[Release: {}] ...", self.platform, self.os, self.release);
    }
}

pub struct Project {
    pub envs: Vec<Box<dyn Runner>>,
}

impl Project {
    pub fn exec(&self) {
        for env in self.envs.iter() {
            env.run();
        }
    }
}
