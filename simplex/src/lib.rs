use ndarray::{Array1,Array2,arr1,arr2};
use ndarray::*;

use num_traits::Float;

#[cfg(test)]
mod test;

pub struct SimplexResult {

}

type SimplexTable = Array2<std::convert::From<i32>>;

fn initial_table<T: Float + std::convert::From<i32>>
    (objective: &Array1<T>, constraints: &Array2<T>, requirements: &Array1<T>)
    -> Array2<T>
    {
    let n_variables = objective.len();
    // Margen izquierdo, valor de cada variable en la restriccion, columna de requerimientos, variables artificiales
    let dimension_j = 1+n_variables+1+constraints.len_of(Axis(0));
    // Cada restriccion y el renglon z
    let dimension_i = constraints.len_of(Axis(0)) + 1;
    let mut table = Array2::<T>::zeros((dimension_i,dimension_j));
    // Renglon Z
    table[[0,0]] = 1i32.into();
    for j in 0..objective.len(){
        table[[0,j+1]] = objective[j];
        table[[0,j+1]] = table[[0,j+1]] * (-1).into();
    }
    // Restricciones
    for i in 0..constraints.len_of(Axis(0)){
        for j in 0..constraints.len_of(Axis(1)){
            table[[i+1,j+1]] = constraints[[i,j]];
        }
    }
    // Requerimientos
    for i in 0..requirements.len(){
        table[[i+1,dimension_j-1]] = requirements[i];
    }
    table
}

pub fn simplex<T: Float + std::convert::From<i32> + std::fmt::Debug>
    (objective: Array1<T>, constraints: Array2<T>, requirements: Array1<T>){
    /*
    Min z = cx
    s.a
        Ax = b
        x >= 0
     */
    let n_variables = objective.len();
    // Margen izquierdo, valor de cada variable en la restriccion, columna de requerimientos, variables artificiales
    let dimension_j = 1+n_variables+1+constraints.len_of(Axis(0));
    // Cada restriccion y el renglon z
    let dimension_i = constraints.len_of(Axis(0)) + 1;
    println!("I: {} J: {}",dimension_i,dimension_j);
    let mut table = Array2::<T>::zeros((dimension_i,dimension_j));
    // Renglon Z
    table[[0,0]] = 1i32.into();
    println!("{:?}",table);
    for j in 0..objective.len(){
        table[[0,j+1]] = objective[j];
    }
     println!("{:?}",table);
    // Restricciones
    for i in 0..constraints.len_of(Axis(0)){
        for j in 0..constraints.len_of(Axis(1)){
            table[[i+1,j+1]] = constraints[[i,j]];
        }
    }
    // Requerimientos
    for i in 0..requirements.len(){
        table[[i+1,dimension_j-1]] = requirements[i];
    }
    println!("{:?}",table);
    unimplemented!();
}