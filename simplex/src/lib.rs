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

fn pivot_point<T: Float + std::convert::From<i32>>(table: &Array2<T>) -> Option<[usize;2]> {
    let mut out_var = None;
    let mut out_var_max = 0.into();
    let mut in_var = None;
    let mut in_var_min = None;

    for j in 1..(table.len_of(Axis(1))-1){
        if table[[0,j]] > out_var_max {
            out_var_max = table[[0,j]];
            out_var = Some(j);
        }
    }
    
    if let Some(j) = out_var {
        let req = table.len_of(Axis(1)) - 1;
        for i in 1..table.len_of(Axis(0)) {
            if let Some(m) = in_var_min{
                if table[[i,req]]/table[[i,j]] < m {
                    in_var_min = Some(table[[i,req]]/table[[i,j]]);
                    in_var = Some(i);
                }
            }else{
                in_var_min = Some(table[[i,req]]/table[[i,j]]);
                in_var = Some(i);
            }
        }
    }
    match (out_var,in_var) {
        (Some(j),Some(i)) => Some([i,j]),
        _ => None
    }
}

pub fn simplex<T: Float + std::convert::From<i32> + std::fmt::Debug>
    (objective: Array1<T>, constraints: Array2<T>, requirements: Array1<T>){
    let table = initial_table(&objective, &constraints, &requirements);
    println!("{:?}",table);
    unimplemented!();
}