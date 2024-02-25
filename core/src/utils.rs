/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

/// [subspace] is a function for creating subsets of a given [Vec] and set size
pub fn subspace<T: Clone>(args: Vec<T>, size: usize) -> Vec<Vec<T>> {
    let mut res = Vec::<Vec<T>>::new();
    for i in 0..args.len() {
        let tmp = (1..size)
            .map(|z: usize| (i + z) % size)
            .collect::<Vec<usize>>();
        for j in 0..tmp.len() {
            let mut subset = vec![args[i].clone()];
            subset.append(
                &mut (0..tmp.len())
                    .map(|k: usize| args[tmp[(j + k) % tmp.len()]].clone())
                    .collect(),
            );
            res.push(subset.clone());
        }
    }
    res
}
