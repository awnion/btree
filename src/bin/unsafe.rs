struct TreeNode {
    l: *mut TreeNode,
    r: *mut TreeNode,
}

impl TreeNode {
    unsafe fn check(&self) -> i32 {
        unsafe {
            let mut ret = 1;
            if !self.l.is_null() {
                ret += (*self.l).check();
            }
            if !self.r.is_null() {
                ret += (*self.r).check();
            }
            ret
        }
    }

    fn create(depth: i32) -> *mut TreeNode {
        if depth > 0 {
            let node = Box::new(TreeNode {
                l: TreeNode::create(depth - 1),
                r: TreeNode::create(depth - 1),
            });
            Box::into_raw(node)
        } else {
            let node = Box::new(TreeNode {
                l: std::ptr::null_mut(),
                r: std::ptr::null_mut(),
            });
            Box::into_raw(node)
        }
    }
}

const MIN_DEPTH: i32 = 4;

fn main() {
    let n = std::env::args_os()
        .nth(1)
        .and_then(|s| s.into_string().ok())
        .and_then(|n| n.parse().ok())
        .unwrap_or(18);

    let max_depth = if MIN_DEPTH + 2 > n { MIN_DEPTH + 2 } else { n };
    {
        let depth = max_depth + 1;
        let tree = TreeNode::create(depth);
        unsafe {
            println!(
                "stretch tree of depth {}\t check: {}",
                depth,
                (*tree).check()
            );
        }
    }

    let long_lived_tree = TreeNode::create(max_depth);

    for depth in (MIN_DEPTH..=max_depth).step_by(2) {
        let iterations = 1 << (max_depth - depth + MIN_DEPTH);
        let mut sum = 0;
        let timer = std::time::Instant::now();
        for _i in 0..iterations {
            let tree = TreeNode::create(depth);
            unsafe {
                sum += (*tree).check();
            }
        }
        println!(
            "{}\t trees of depth {}\t check: {}\t ({:.0?})",
            iterations,
            depth,
            sum,
            timer.elapsed()
        );
    }

    unsafe {
        println!(
            "long lived tree of depth {}\t check: {}",
            max_depth,
            (*long_lived_tree).check()
        );
    }
}
