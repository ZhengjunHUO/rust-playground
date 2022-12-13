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
            self.informer
                .inform("[WARNING] You are running out of quota!");
        } else if usage >= 0.5 {
            self.informer
                .inform("[WARNING] You've used over 50% of quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct FakeInformer {
        cache: RefCell<Vec<String>>,
    }

    impl FakeInformer {
        fn new() -> FakeInformer {
            FakeInformer {
                cache: RefCell::new(vec![]),
            }
        }
    }

    impl Informer for FakeInformer {
        fn inform(&self, event: &str) {
            self.cache.borrow_mut().push(String::from(event))
        }
    }

    #[test]
    fn get_informed() {
        let fake = FakeInformer::new();
        let mut watcher = Watcher::new(&fake, 100);
        watcher.probe(60);
        watcher.probe(95);
        assert_eq!(
            fake.cache.borrow()[0],
            String::from("[WARNING] You've used over 50% of quota!")
        );
        assert_eq!(fake.cache.borrow().len(), 2);
    }
}
