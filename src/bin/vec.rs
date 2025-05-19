struct Tree(Vec<usize>);

impl Tree {
    pub fn create(depth: i32) -> Self {
        fn create_node(nodes: &mut Vec<usize>, index: usize, depth: i32) {
            if depth > 0 {
                let left = index * 2 + 1;
                let right = index * 2 + 2;
                create_node(nodes, left, depth - 1);
                create_node(nodes, right, depth - 1);
            }
            nodes[index] = 1;
        }
        let mut nodes = vec![0; 1 << depth + 1];
        create_node(&mut nodes, 0, depth);
        Tree(nodes)
    }

    pub fn check(&self) -> i32 {
        self.check_rec(0)
    }

    fn check_rec(&self, index: usize) -> i32 {
        let mut res = 1;

        let left = index * 2 + 1;
        if left < self.0.len() && self.0[left] != 0 {
            res += self.check_rec(left);
        }

        let right = index * 2 + 2;
        if right < self.0.len() && self.0[right] != 0 {
            res += self.check_rec(right);
        }
        res
    }
}

impl Drop for Tree {
    fn drop(&mut self) {
        println!("dropping tree");
    }
}

const MIN_DEPTH: i32 = 4;

fn main() {
    let n = std::env::args_os()
        .nth(1)
        .and_then(|s| s.into_string().ok())
        .and_then(|n| n.parse().ok())
        .unwrap_or(10);

    let max_depth = if MIN_DEPTH + 2 > n { MIN_DEPTH + 2 } else { n };
    {
        let depth = max_depth + 1;
        let tree = Tree::create(max_depth + 1);

        println!("stretch tree of depth {}\t check: {}", depth, tree.check());
    }

    let long_lived_tree = Tree::create(max_depth);

    for d in (MIN_DEPTH..max_depth + 1).step_by(2) {
        let iterations = 1 << ((max_depth - d + MIN_DEPTH) as u32);
        let mut chk = 0;
        let timer = std::time::Instant::now();
        for _i in 0..iterations {
            let a = Tree::create(d);
            chk += a.check();
        }
        println!(
            "{}\t trees of depth {}\t check: {}\t ({:.0?})",
            iterations,
            d,
            chk,
            timer.elapsed()
        )
    }

    println!(
        "long lived tree of depth {}\t check: {}",
        max_depth,
        long_lived_tree.check()
    );
}
