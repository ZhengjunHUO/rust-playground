pub trait Informer {
    fn inform(&self, event: &str);
}

pub struct Watcher<'l, T: Informer> {
    informer: &'l T,
    consum: usize,
    quota: usize,
}

impl<'l, T> Watcher<'l, T>
where
    T: Informer,
{
    pub fn new(informer: &'l T, quota: usize) -> Watcher<'l, T> {
        Watcher {
            informer,
            consum: 0,
            quota,
        }
    }

    pub fn probe(&mut self, consum: usize) {
        self.consum = consum;
        let usage = self.consum as f64 / self.quota as f64;

        if usage >= 0.9 {
            self.informer.inform("[WARNING] You are running out of quota!");
        }else if usage >= 0.5 {
            self.informer.inform("[WARNING] You've used over 50% of quota!");
        }
    }
}
