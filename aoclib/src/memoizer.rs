use std::hash::Hash;
use std::sync::Mutex;
use rustc_hash::FxHashMap;

pub struct Memoizer<A, B, R> {
    fixed_input: B,
    cache: Mutex<FxHashMap<A, R>>,
    func: Box<dyn Fn(&A, &B, &Memoizer<A, B, R>) -> R>,
}

impl<A: Hash + Clone + Eq, B, R: Clone> Memoizer<A, B, R> {
    pub fn new(func: Box<dyn Fn(&A, &B, &Memoizer<A, B, R>) -> R>, fixed_input: B) -> Memoizer<A, B, R> {
        Memoizer {
            cache: Mutex::new(FxHashMap::default()),
            fixed_input,
            func
        }
    }

    pub fn call(&self, arg: A) -> R {
        if let Some(v) = self.cache.lock().unwrap().get(&arg) {
            return v.clone();
        }

        let v = (self.func)(&arg, &self.fixed_input, &self);
        self.cache.lock().unwrap().insert(arg.clone(), v.clone());
        v
    }
}

#[test]
fn memoize_test() {
    let mut mem = Memoizer::new(Box::new(crate::memoizer::testfunc::test_func), ());
    assert_eq!(mem.call(()), 42);
    assert_eq!(mem.call(()), 42);
    assert_eq!(mem.call(()), 42);
}

#[cfg(test)]
mod testfunc {
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering;
    use crate::memoizer::Memoizer;

    const BOOL: AtomicBool = AtomicBool::new(false);

    pub fn test_func(_arg: &(), _fixed_input: &(), mem: &Memoizer<(), (), u8>) -> u8 {
        assert!(!BOOL.load(Ordering::Relaxed));
        BOOL.store(true, Ordering::Relaxed);
        42
    }
}
