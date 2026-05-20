use proconio::input;

fn main() {
    input! {n: usize, a: String}
    
    let a: Vec<usize> = a.chars().map(|c| (c == '1') as usize).collect();
    
    // Get current final value
    let current = get_value(&a);
    
    // Calculate minimum cost to make it 0 or 1
    let (cost_to_0, cost_to_1) = min_cost(&a);
    
    // Output the cost to flip the current value
    let answer = if current == 0 { cost_to_1 } else { cost_to_0 };
    println!("{}", answer);
}

fn get_value(a: &[usize]) -> usize {
    if a.len() == 1 {
        return a[0];
    }
    if a.len() == 3 {
        let sum: usize = a.iter().sum();
        return (sum >= 2) as usize;
    }
    
    let mut next_layer = Vec::new();
    for i in 0..a.len() / 3 {
        let chunk = &a[i * 3..(i + 1) * 3];
        // Majority: if sum >= 2, it's 1, else 0
        let majority = (chunk.iter().sum::<usize>() >= 2) as usize;
        next_layer.push(majority);
    }
    
    get_value(&next_layer)
}

fn min_cost(a: &[usize]) -> (usize, usize) {
    /* Returns (min cost to make this subtree 0, min cost to make it 1)
    Uses DP to find minimum changes needed */
    
    if a.len() == 1 {
        // Leaf node: cost is 0 if already target, 1 if need to flip
        return (a[0], 1 - a[0]);
    }
    
    let mut next_layer_costs = Vec::new();
    
    // Process each group of 3 and get costs for each child
    for i in 0..a.len() / 3 {
        let chunk = &a[i * 3..(i + 1) * 3];
        let (c0, c1) = min_cost(chunk);
        next_layer_costs.push((c0, c1));
    }
    
    // Now we need to determine the cost to make the next layer 0 or 1
    // For each group of 3 children, we need at least 2 to be the target value
    min_cost_for_majorities(&next_layer_costs)
}

fn min_cost_for_majorities(costs: &[(usize, usize)]) -> (usize, usize) {
    /* Given costs for each child, compute minimum cost to make parent 0 or 1
    We need majority (at least 2 out of 3) to achieve each target */
    
    if costs.len() == 1 {
        return costs[0];
    }
    
    let mut next_costs = Vec::new();
    
    for i in 0..costs.len() / 3 {
        let group = &costs[i * 3..(i + 1) * 3];
        
        // To make this group evaluate to 0: need at least 2 children to be 0
        let cost_to_0 = min_two_costs(&[group[0].0, group[1].0, group[2].0]);
        
        // To make this group evaluate to 1: need at least 2 children to be 1
        let cost_to_1 = min_two_costs(&[group[0].1, group[1].1, group[2].1]);
        
        next_costs.push((cost_to_0, cost_to_1));
    }
    
    min_cost_for_majorities(&next_costs)
}

fn min_two_costs(three_costs: &[usize]) -> usize {
    /* Return minimum cost to make at least 2 out of 3 equal to target value */
    let mut sorted = three_costs.to_vec();
    sorted.sort();
    // We pick the 2 cheapest options
    sorted[0] + sorted[1]
}