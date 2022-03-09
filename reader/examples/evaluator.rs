use reader::Reader;
use std::collections::HashMap;

fn main() {
    let expr = Expr::let_in(
        "y".into(),
        Expr::add(Expr::lit(1), Expr::lit(2)),
        Expr::cond(
            Expr::var("x".into()),
            Expr::add(Expr::lit(100), Expr::var("y".into())),
            Expr::bool(false),
        ),
    );
    let env = Env::from_iter(vec![("x".into(), Value::Bool(true))]);
    let r = expr.eval().run_with(&env);

    println!("eval = {:?}", r)
}

type BoxedExpr = Box<Expr>;

#[derive(Debug)]
enum Expr {
    Lit(i32),
    Bool(bool),
    Var(Ident),
    Add(BoxedExpr, BoxedExpr),
    LetIn(Ident, BoxedExpr, BoxedExpr),
    Cond(BoxedExpr, BoxedExpr, BoxedExpr),
}

type Ident = String;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Value {
    Lit(i32),
    Bool(bool),
}

type Env = HashMap<Ident, Value>;

type Eval = Reader<Env, Option<Value>>;

impl Expr {
    fn lit(i: i32) -> Self {
        Self::Lit(i)
    }

    fn bool(b: bool) -> Self {
        Self::Bool(b)
    }

    fn var(name: Ident) -> Self {
        Self::Var(name)
    }

    fn add(l: Expr, r: Expr) -> Self {
        Self::Add(BoxedExpr::new(l), BoxedExpr::new(r))
    }

    fn let_in(n: Ident, h: Expr, b: Expr) -> Self {
        Self::LetIn(n, BoxedExpr::new(h), BoxedExpr::new(b))
    }

    fn cond(c: Expr, t: Expr, e: Expr) -> Self {
        Self::Cond(c.into(), t.into(), e.into())
    }

    fn eval(self) -> Eval {
        use Expr::*;
        match self {
            Lit(i) => Self::wrap_lit(i),
            Bool(b) => Self::wrap_bool(b),
            Var(name) => Self::access(name),
            Add(l, r) => l
                .eval()
                .and_then(|l| r.eval().map(move |r| lift2(l, r, Value::add).flatten())),
            LetIn(n, h, b) => h.eval().and_then(|h| {
                h.map(|h| {
                    b.eval().local(move |e| {
                        let mut e = e.clone();
                        e.insert(n, h);
                        e
                    })
                })
                .unwrap_or_else(|| Eval::pure(None))
            }),
            Cond(c, t, e) => c.eval().and_then(|c| Self::conditional(c, t, e)),
        }
    }

    fn wrap_lit(i: i32) -> Eval {
        Self::wrap(Value::Lit(i))
    }

    fn wrap_bool(b: bool) -> Eval {
        Self::wrap(Value::Bool(b))
    }

    fn access<A>(name: A) -> Eval
    where
        A: AsRef<str> + 'static,
    {
        Eval::new(move |env| env.get(name.as_ref()).copied())
    }

    fn conditional(c: Option<Value>, t: BoxedExpr, e: BoxedExpr) -> Eval {
        match c {
            Some(Value::Bool(true)) => t.eval(),
            Some(Value::Bool(false)) => e.eval(),
            _ => Eval::pure(None),
        }
    }

    fn wrap(value: Value) -> Eval {
        Eval::pure(value.into())
    }
}

impl Value {
    fn add(l: Value, r: Value) -> Option<Value> {
        match (l, r) {
            (Value::Lit(l), Value::Lit(r)) => Value::Lit(l + r).into(),
            _ => None,
        }
    }
}

fn lift2<A, B, C>(a: Option<A>, b: Option<B>, f: impl FnOnce(A, B) -> C) -> Option<C> {
    match (a, b) {
        (Some(a), Some(b)) => f(a, b).into(),
        _ => None,
    }
}
