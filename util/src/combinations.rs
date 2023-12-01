/*
pub fn combinations<T: Clone>(v: &Vec<T>, n: usize) -> Vec<Vec<T>> {
    match n {
        0 => vec![],
        1 => v.iter().map(|i| vec![i.clone()]).collect::<Vec<_>>(),
        _ => {
            let mut out: Vec<Vec<T>> = Vec::new();
            (1..v.len()).for_each(|i| {
                let (current, rest) = (&v[i - 1], &v.iter().skip(i).cloned().collect::<Vec<_>>());
                combinations(rest, n - 1).into_iter().for_each(|mut c| {
                    c.push(current.clone());
                    out.push(c);
                })
            });
            out
        }
    }
}
*/

pub fn combinations<T: Clone>(v: &Vec<T>, n: usize) -> Vec<Vec<T>> {
    if n == 0 {
        return vec![vec![]];
    }

    if n > v.len() {
        return vec![];
    }

    let mut result = vec![];
    let mut stack: Vec<(usize, usize, Vec<T>)> = vec![(0, 0, vec![])];

    while let Some((i, count, mut current)) = stack.pop() {
        if count == n {
            result.push(current);
        } else {
            for j in i..v.len() {
                current.push(v[j].clone());
                stack.push((j + 1, count + 1, current.clone()));
                current.pop();
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combinations() {
        let v = vec!["A", "B", "C", "D", "E", "F"];
        assert_eq!(combinations(&v, 1).len(), 6);
        assert_eq!(combinations(&v, 2).len(), 15);
        assert_eq!(combinations(&v, 3).len(), 20);
        assert_eq!(combinations(&v, 4).len(), 15);
        assert_eq!(combinations(&v, 5).len(), 6);
        assert_eq!(combinations(&v, 6).len(), 1);
    }
}
