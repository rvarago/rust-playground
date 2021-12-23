//! A set of partition algorithms, where we split `Vec<A>` according to a predicate `A -> bool`.

pub mod predicate {
    pub fn partition<P, A>(input: Vec<A>, pred: P) -> (Vec<A>, Vec<A>)
    where
        P: Fn(&A) -> bool,
    {
        let (mut matches, mut mismatches) = (Vec::default(), Vec::default());

        input.into_iter().for_each(|x| {
            if pred(&x) {
                matches.push(x)
            } else {
                mismatches.push(x)
            }
        });

        (matches, mismatches)
    }
}

pub mod predicate_named {
    pub fn partition<P, A>(input: Vec<A>, pred: P) -> Partitioned<A>
    where
        P: Fn(&A) -> bool,
    {
        let (mut matches, mut mismatches) = (Vec::default(), Vec::default());

        input.into_iter().for_each(|x| {
            if pred(&x) {
                matches.push(x)
            } else {
                mismatches.push(x)
            }
        });

        Partitioned {
            matches,
            mismatches,
        }
    }

    #[derive(Debug)]
    pub struct Partitioned<A> {
        matches: Vec<A>,
        mismatches: Vec<A>,
    }
}

pub mod either {
    pub fn partition<P, A, B, C>(input: Vec<A>, select: P) -> (Vec<B>, Vec<C>)
    where
        P: Fn(A) -> Result<B, C>,
    {
        let (mut matches, mut mismatches) = (Vec::default(), Vec::default());

        input.into_iter().map(select).for_each(|r| match r {
            Ok(x) => matches.push(x),
            Err(x) => mismatches.push(x),
        });

        (matches, mismatches)
    }

    pub fn partition_p<P, A>(input: Vec<A>, pred: P) -> (Vec<A>, Vec<A>)
    where
        P: Fn(&A) -> bool,
    {
        partition(input, |x| if pred(&x) { Ok(x) } else { Err(x) })
    }
}
