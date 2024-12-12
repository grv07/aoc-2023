use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut args = std::env::args();

    let _ = args.next();
    let file_name = args.next().unwrap();
    let mut lines = BufReader::new(File::open(file_name).unwrap()).lines();

    let mut res = 0;
    let mut res_ep = 0;
    while let Some(Ok(line)) = lines.next() {
        let input = line
            .trim()
            .split_whitespace()
            .into_iter()
            .map(|v| v.parse().unwrap())
            .collect::<Vec<i64>>();
        let (pr, _next_diff) = solu1(input.to_vec(), 0);
        let (ep, _next_diff) = solu2(input.to_vec(), 0);

        let pr = pr + input.last().unwrap();
        res += pr;
        res_ep += ep;
    }
    println!("Part 1: {res}");
    println!("Part 2: {res_ep}");
}

fn solu1(input: Vec<i64>, pr: i64) -> (i64, Vec<i64>) {
    if input.iter().all(|v| *v == 0) {
        return (pr, input);
    }

    let mut next_diff = vec![];
    for p in input.windows(2) {
        next_diff.push(p[1] - p[0]);
    }

    let n_pr = pr + next_diff.last().unwrap();

    solu1(next_diff, n_pr)
}

fn solu2(input: Vec<i64>, ep: i64) -> (i64, Vec<i64>) {
    if input.iter().all(|v| *v == 0) {
        return (0, input);
    }

    let mut next_diff = vec![];
    for p in input.windows(2) {
        next_diff.push(p[1] - p[0]);
    }

    let (ep, res) = solu2(next_diff, ep);

    let n_ep = input.first().unwrap() - ep;
    (n_ep, res)
}

#[cfg(test)]
mod test {
    use solu1;

    #[test]
    fn sol1_test() {
        let input = vec![1, 3, 6, 10, 15, 21, 28];
        let (pr, _) = solu1(input.to_vec(), 0);
        let pr = pr + input.last().unwrap();

        assert_eq!(pr, 36);

        let input = vec![1, 3, 6, 10, 15];
        let (pr, _) = solu1(input.to_vec(), 0);
        let pr = pr + input.last().unwrap();

        assert_eq!(pr, 21);

        let input = vec![1, 1];
        let (pr, _) = solu1(input.to_vec(), 0);
        let pr = pr + input.last().unwrap();

        assert_eq!(pr, 1);

        let input = vec![-7, -14, -27, -48, -70, -69, 4, 231, 735];
        let (pr, _) = solu1(input.to_vec(), 0);
        let pr = pr + input.last().unwrap();

        assert_eq!(pr, 1688);
    }
}
