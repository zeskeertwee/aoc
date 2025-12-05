use std::hash::Hash;
use rustc_hash::FxHashMap;

pub struct Memoizer<A, R> {
    cache: FxHashMap<A, R>,
    func: Box<dyn Fn(&A) -> R>,
}

impl<A: Hash + Clone + Eq, R: Clone> Memoizer<A, R> {
    pub fn new(func: Box<dyn Fn(&A) -> R>) -> Memoizer<A, R> {
        Memoizer {
            cache: FxHashMap::default(),
            func
        }
    }

    pub fn call(&mut self, arg: A) -> R {
        if let Some(v) = self.cache.get(&arg) {
            return v.clone();
        }

        let v = (self.func)(&arg);
        self.cache.insert(arg.clone(), v.clone());
        v
    }
}

#[test]
fn memoize_test() {
    let mut mem = Memoizer::new(Box::new(crate::memoizer::testfunc::test_func));
    assert_eq!(mem.call(()), 42);
    assert_eq!(mem.call(()), 42);
    assert_eq!(mem.call(()), 42);
}

#[cfg(test)]
mod testfunc {
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering;

    const BOOL: AtomicBool = AtomicBool::new(false);

    pub fn test_func(_arg: &()) -> u8 {
        assert!(!BOOL.load(Ordering::Relaxed));
        BOOL.store(true, Ordering::Relaxed);
        42
    }
}
