use crate::*;
use ndarray::arr2;

#[test]
fn build_simplex_table_1(){
    let program = Simplex::minimize(&vec![-3.0, 1.0, -2.0])
        .with(vec![
            SimplexConstraint::LessThan(vec![2.0, -2.0, 3.0], 5.0),
            SimplexConstraint::LessThan(vec![1.0, 1.0, -1.0], 3.0),
            SimplexConstraint::LessThan(vec![1.0, -1.0, 1.0], 2.0),
        ]);
    let simplex = program.unwrap();
    assert_eq!(simplex.table, arr2(&[
        [1.0, 3.0, -1.0, 2.0, 0.0, f64::MIN, 0.0, f64::MIN, 0.0, f64::MIN, 0.0],
        [0.0, 2.0, -2.0, 3.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 5.0],
        [0.0, 1.0, 1.0, -1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 3.0],
        [0.0, 1.0, -1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 2.0]
    ]));

    assert_eq!(simplex.base, vec![5, 7, 9]);
    assert_eq!(simplex.vars.len(), 9);
}

#[test]
fn build_simplex_table_2(){
    let simplex = Simplex::minimize(&vec![-1.0, 1.0])
        .with(vec![
            SimplexConstraint::LessThan(vec![3.0, -4.0], 12.0),
            SimplexConstraint::LessThan(vec![1.0, -1.0], 10.0)
        ]).unwrap();

    assert_eq!(simplex.table, arr2(&[
        [1.0, 1.0, -1.0, 0.0, f64::MIN, 0.0, f64::MIN, 0.0],
        [0.0, 3.0, -4.0, 1.0, 1.0, 0.0, 0.0, 12.0],
        [0.0, 1.0, -1.0, 0.0, 0.0, 1.0, 1.0, 10.0]
    ]));

    assert_eq!(simplex.base, vec![4, 6]);
    assert_eq!(simplex.vars.len(), 6);
}

#[test]
fn build_simplex_table_3(){
    let simplex = Simplex::minimize(&vec![-1.0, 2.0])
        .with(vec![
            SimplexConstraint::GreaterThan(vec![1.0, 1.0], 2.0),
            SimplexConstraint::GreaterThan(vec![-1.0, 1.0], 1.0),
            SimplexConstraint::LessThan(vec![0.0, 1.0], 3.0)
        ]).unwrap();
    assert_eq!(simplex.table, arr2(&[
        [1.0, 1.0, -2.0, 0.0, f64::MIN, 0.0, f64::MIN, 0.0, f64::MIN, 0.0],
        [0.0, 1.0, 1.0, -1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 2.0],
        [0.0, -1.0, 1.0, 0.0, 0.0, -1.0, 1.0, 0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 3.0]
    ]));
    assert_eq!(simplex.base, vec![4, 6, 8]);
    assert_eq!(simplex.vars.len(), 8);
}

#[test]
fn get_entry_var_1(){
    let program = Simplex::minimize(&vec![-3.0, 1.0, -2.0])
    .with(vec![
        SimplexConstraint::LessThan(vec![2.0, -2.0, 3.0], 5.0),
        SimplexConstraint::LessThan(vec![1.0, 1.0, -1.0], 3.0),
        SimplexConstraint::LessThan(vec![1.0, -1.0, 1.0], 2.0),
    ]);
    let simplex = program.unwrap();
    assert_eq!(simplex.get_entry_var(), Some(1));
}

#[test]
fn get_entry_var_2(){
    let simplex = Simplex::minimize(&vec![-1.0, 1.0])
        .with(vec![
            SimplexConstraint::LessThan(vec![3.0, -4.0], 12.0),
            SimplexConstraint::LessThan(vec![1.0, -1.0], 10.0)
        ]).unwrap();
    assert_eq!(simplex.get_entry_var(), Some(1));
}

#[test]
fn get_entry_var_3(){
    let simplex = Simplex::minimize(&vec![-1.0, 2.0])
        .with(vec![
            SimplexConstraint::GreaterThan(vec![1.0, 1.0], 2.0),
            SimplexConstraint::GreaterThan(vec![-1.0, 1.0], 1.0),
            SimplexConstraint::LessThan(vec![0.0, 1.0], 3.0)
        ]).unwrap();
    assert_eq!(simplex.get_entry_var(), Some(1));
}

#[test]
fn get_exit_var_1(){
    let program = Simplex::minimize(&vec![-3.0, 1.0, -2.0])
    .with(vec![
        SimplexConstraint::LessThan(vec![2.0, -2.0, 3.0], 5.0),
        SimplexConstraint::LessThan(vec![1.0, 1.0, -1.0], 3.0),
        SimplexConstraint::LessThan(vec![1.0, -1.0, 1.0], 2.0),
    ]);
    let simplex = program.unwrap();
    assert_eq!(simplex.get_exit_var(1), Some(9));
}

#[test]
fn get_exit_var_2(){
    let simplex = Simplex::minimize(&vec![-1.0, 1.0])
        .with(vec![
            SimplexConstraint::LessThan(vec![3.0, -4.0], 12.0),
            SimplexConstraint::LessThan(vec![1.0, -1.0], 10.0)
        ]).unwrap();
    assert_eq!(simplex.get_exit_var(1), Some(4));
}

#[test]
fn get_exit_var_3(){
    let simplex = Simplex::minimize(&vec![-1.0, 2.0])
        .with(vec![
            SimplexConstraint::GreaterThan(vec![1.0, 1.0], 2.0),
            SimplexConstraint::GreaterThan(vec![-1.0, 1.0], 1.0),
            SimplexConstraint::LessThan(vec![0.0, 1.0], 3.0)
        ]).unwrap();
    assert_eq!(simplex.get_exit_var(1), Some(4));
}

#[test]
fn step_example_1(){
    let program = Simplex::minimize(&vec![-3.0, 1.0, -2.0])
    .with(vec![
        SimplexConstraint::LessThan(vec![2.0, -2.0, 3.0], 5.0),
        SimplexConstraint::LessThan(vec![1.0, 1.0, -1.0], 3.0),
        SimplexConstraint::LessThan(vec![1.0, -1.0, 1.0], 2.0),
    ]);
    let mut simplex = program.unwrap();
    let entry_var = simplex.get_entry_var().unwrap();
    let exit_var = simplex.get_exit_var(entry_var).unwrap();
    simplex.step(entry_var, exit_var);
    assert_eq!(simplex.base, vec![5, 7, 1]);
    assert_eq!(simplex.table.column(10), arr1(
        &[-6.0, 1.0, 1.0, 2.0]
    ));
}

#[test]
fn solve_example_1(){
    let program = Simplex::minimize(&vec![-3.0, 1.0, -2.0])
    .with(vec![
        SimplexConstraint::LessThan(vec![2.0, -2.0, 3.0], 5.0),
        SimplexConstraint::LessThan(vec![1.0, 1.0, -1.0], 3.0),
        SimplexConstraint::LessThan(vec![1.0, -1.0, 1.0], 2.0),
    ]);
    let mut simplex = program.unwrap();
    assert_eq!(simplex.solve(), SimplexOutput::MultipleOptimum(-8.0));
    assert_eq!(simplex.get_var(1), Some(2.5));
    assert_eq!(simplex.get_var(2), Some(1.5));
    assert_eq!(simplex.get_var(3), Some(1.0));
}

#[test]
fn solve_example_2(){
    let mut simplex = Simplex::minimize(&vec![-1.0, 1.0])
        .with(vec![
            SimplexConstraint::LessThan(vec![3.0, -4.0], 12.0),
            SimplexConstraint::LessThan(vec![1.0, -1.0], 10.0)
        ]).unwrap();
    assert_eq!(simplex.solve(), SimplexOutput::MultipleOptimum(-10.0));
}

#[test]
fn solve_example_3(){
    let mut simplex = Simplex::minimize(&vec![1.0, -2.0])
        .with(vec![
            SimplexConstraint::GreaterThan(vec![1.0, 1.0], 2.0),
            SimplexConstraint::GreaterThan(vec![-1.0, 1.0], 1.0),
            SimplexConstraint::LessThan(vec![0.0, 1.0], 3.0),
        ]).unwrap();
    assert_eq!(simplex.solve(), SimplexOutput::MultipleOptimum(-6.0));
}

#[test]
fn solve_example_4(){
    let mut simplex = Simplex::minimize(&vec![-2.0, -3.0, -4.0])
    .with(vec![
        SimplexConstraint::LessThan(vec![3.0, 2.0, 1.0], 10.0),
        SimplexConstraint::LessThan(vec![2.0, 5.0, 3.0], 15.0),
    ])
    .unwrap();
    assert_eq!(simplex.solve(), SimplexOutput::MultipleOptimum(-20.0));
    assert_eq!(simplex.get_var(1), Some(0.0));
    assert_eq!(simplex.get_var(2), Some(0.0));
    assert_eq!(simplex.get_var(3), Some(5.0));
}

#[test]
fn solve_example_5(){
    let mut simplex = Simplex::minimize(&vec![-1.0])
    .with(vec![
        SimplexConstraint::LessThan(vec![1.0], 1.0)
    ])
    .unwrap();
    assert_eq!(simplex.solve(), SimplexOutput::MultipleOptimum(-1.0));
    assert_eq!(simplex.get_var(1), Some(1.0));
}
