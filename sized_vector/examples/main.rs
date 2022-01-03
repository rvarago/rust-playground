use sized_vector::{
    nat::{Succ, Zero},
    IVec,
};

#[allow(unused_variables)]

fn main() {
    // We statically know the size of the xs and ys.
    let xs: IVec<Succ<Succ<Zero>>, i32> = IVec::default().push(1).push(2);
    let ys: IVec<Succ<Zero>, String> = IVec::default().push("a".to_owned());

    // A call to map preserves the length, but may optionally change the carrier type.
    let xs = xs.map(|x| (x + 10).to_string());

    // If we append two vectors, then their lengths are statically added.
    let xys: IVec<Succ<Succ<Zero>>, String> = xs.clone().append(ys.clone());

    // An attempt to call:
    // xs.zip(ys);
    // would fail as the lengths don't match up (i.e. Succ<Succ<Zero>> != Succ<Zero>).
    // but if we push another element to `ys`, then the call succeeds as the lengths are the same.
    let zs: IVec<Succ<Succ<Zero>>, (String, String)> = xs.zip(ys.push("b".to_owned()));

    // An attempt to call:
    // let a = IVec::<_, i32>::default().first();
    // would fail as there's no method named `first()` for an empty vector (i.e. IVec<Zero, A>).
    let b: &i32 = IVec::default().push(1).first();
}
