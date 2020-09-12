# simplex
A Linear Programming solver.

Use it to solve your linear programming problems directly from Rust. For example, for this problem:
```
minimize -3x +  y - 2z
    with  2x - 2y + 3z <= 5
           x +  y -  z <= 3
           x -  y +  z <= 2
```

You can do:
```rust
use simplex::*;

fn main(){
    let program = Simplex::minimize(&vec![-3.0, 1.0, -2.0])
    .with(vec![
        SimplexConstraint::LessThan(vec![2.0, -2.0, 3.0], 5.0),
        SimplexConstraint::LessThan(vec![1.0, 1.0, -1.0], 3.0),
        SimplexConstraint::LessThan(vec![1.0, -1.0, 1.0], 2.0),
    ]);
    let mut simplex = program.unwrap();
    match simplex.solve() {
        SimplexOutput::UniqueOptimum(x) => println!("{}", x),
        SimplexOutput::MultipleOptimum(x) => println!("{}", x),
        _ => println!("No solution or unbounded") => return
    }
    println!("{:?}", simplex.get_var(1));
    println!("{:?}", simplex.get_var(2));
    println!("{:?}", simplex.get_var(3));
}
```