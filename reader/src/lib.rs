//! A `Reader<E, A>` is sort-of a glorified `E -> A` with a nice way to pass `E` around implicitly.

pub struct Reader<E, A> {
    run: Box<dyn FnOnce(&E) -> A>,
}

impl<E, A> Reader<E, A> {
    pub fn new<F>(run: F) -> Self
    where
        F: FnOnce(&E) -> A + 'static,
    {
        Self { run: Box::new(run) }
    }

    pub fn pure(value: A) -> Self
    where
        A: 'static,
    {
        Self::new(|_| value)
    }

    pub fn run_with(self, e: &E) -> A {
        (self.run)(e)
    }

    pub fn and_then<K, B>(self, k: K) -> Reader<E, B>
    where
        E: 'static,
        A: 'static,
        K: FnOnce(A) -> Reader<E, B> + 'static,
    {
        Reader::new(|e| k(self.run_with(e)).run_with(e))
    }

    pub fn then<K, B>(self, k: K) -> Reader<E, B>
    where
        E: 'static,
        A: 'static,
        K: FnOnce() -> Reader<E, B> + 'static,
    {
        self.and_then(|_| k())
    }

    pub fn map<F, B>(self, f: F) -> Reader<E, B>
    where
        E: 'static,
        A: 'static,
        F: FnOnce(A) -> B + 'static,
        B: 'static,
    {
        self.and_then(|a| Reader::pure(f(a)))
    }

    pub fn remap<F, B>(self, f: F) -> Reader<E, B>
    where
        E: 'static,
        A: 'static,
        F: FnOnce() -> B + 'static,
        B: 'static,
    {
        self.map(|_| f())
    }

    pub fn void(self) -> Reader<E, ()>
    where
        E: 'static,
        A: 'static,
    {
        self.remap(|| ())
    }

    pub fn local<F>(self, f: F) -> Reader<E, A>
    where
        E: 'static,
        A: 'static,
        F: FnOnce(&E) -> E + 'static,
    {
        Self::new(move |e| self.run_with(&f(e)))
    }
}

impl<F, E, A> From<F> for Reader<E, A>
where
    F: FnOnce(&E) -> A + 'static,
{
    fn from(run: F) -> Self {
        Self::new(run)
    }
}
