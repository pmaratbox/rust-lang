struct Tarjan {
    adj: Vec<Vec<usize>>,
    index: Vec<i32>,
    low: Vec<i32>,
    on_stack: Vec<bool>,
    stack: Vec<usize>,
    counter: i32,
    scc_count: i32,
}

impl Tarjan {
    fn dfs(&mut self, u: usize) {
        self.index[u] = self.counter;
        self.low[u] = self.counter;
        self.counter += 1;
        self.stack.push(u);
        self.on_stack[u] = true;

        for i in 0..self.adj[u].len() {
            let v = self.adj[u][i];
            if self.index[v] == -1 {
                self.dfs(v);
                self.low[u] = self.low[u].min(self.low[v]);
            } else if self.on_stack[v] {
                self.low[u] = self.low[u].min(self.index[v]);
            }
        }

        if self.low[u] == self.index[u] {
            self.scc_count += 1;
            loop {
                let w = self.stack.pop().unwrap();
                self.on_stack[w] = false;
                if w == u {
                    break;
                }
            }
        }
    }
}

fn main() {
    let edges: [(usize, usize); 4] = [(0, 1), (1, 2), (2, 0), (2, 3)];
    let n = 4;
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for &(u, v) in &edges {
        adj[u].push(v);
    }

    let mut t = Tarjan {
        adj,
        index: vec![-1; n],
        low: vec![-1; n],
        on_stack: vec![false; n],
        stack: Vec::new(),
        counter: 0,
        scc_count: 0,
    };

    for s in 0..n {
        if t.index[s] == -1 {
            t.dfs(s);
        }
    }

    println!("{}", t.scc_count);
}
