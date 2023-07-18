use rand::prelude::*;
use std::collections::VecDeque;

fn main() {
    let mut rng = rand::thread_rng();
    let candidates = "コメダ".chars().collect::<Vec<_>>();

    let mut que = VecDeque::new();
    loop {
        let c = candidates.choose(&mut rng).unwrap();
        print!("{}", c);

        que.push_back(c);
        while que.len() > 3 {
            que.pop_front();
        }
        let tail = que.iter().copied().collect::<String>();
        if tail == "コメダ" {
            println!("行きなさい");
            break;
        }
    }
}
