pub fn solve_tsp(matrix: Vec<Vec<usize>>) -> (usize, Vec<usize>) {
    let n = matrix.len();
    
    if n == 1 {
        return (matrix[0][0], vec![0, 0]);
    }

    let inf = usize::MAX / 2;
    let size = 1 << n;
    
    let mut dp = vec![vec![inf; n]; size];
    let mut parent = vec![vec![0usize; n]; size];

    dp[1 << 0][0] = 0;

    for mask in 0..size {
        if mask & 1 == 0 {
            continue;
        }
        for j in 0..n {
            if j == 0 {
                continue;
            }
            if (mask & (1 << j)) == 0 {
                continue;
            }
            let prev_mask = mask ^ (1 << j);
            for i in 0..n {
                if (prev_mask & (1 << i)) == 0 {
                    continue;
                }
                let new_cost = dp[prev_mask][i].saturating_add(matrix[i][j]);
                if new_cost < dp[mask][j] {
                    dp[mask][j] = new_cost;
                    parent[mask][j] = i;
                }
            }
        }
    }

    let full_mask = size - 1;
    let mut best_cost = inf;
    let mut last = 0;
    for j in 1..n {
        let cost = dp[full_mask][j].saturating_add(matrix[j][0]);
        if cost < best_cost {
            best_cost = cost;
            last = j;
        }
    }

    let mut path_rev = Vec::new();
    let mut mask = full_mask;
    let mut curr = last;
    path_rev.push(curr);
    while curr != 0 {
        let p = parent[mask][curr];
        mask ^= 1 << curr;
        curr = p;
        path_rev.push(curr);
    }
    path_rev.reverse();
    let mut tour = path_rev.clone();
    tour.push(0);

    (best_cost, tour)
}