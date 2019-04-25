use crate::*;
use ndarray::{arr1,arr2};

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn test_simplex() {
    let objective = arr1(&[-3.,1.,-2.]);
    let constraints = arr2(&[
        [2.,-2.,3.,1.,0.,0.],
        [1.,1.,-1.,0.,1.,0.],
        [1.,-1.,1.,0.,0.,1.]
    ]);
    let requirements = arr1(&[5.,3.,2.]);
    simplex(objective,constraints,requirements);
}

#[test]
fn test_setup(){
    let objective = arr1(&[-3.,1.,-2.]);
    let constraints = arr2(&[
        [2.,-2.,3.,1.,0.,0.],
        [1.,1.,-1.,0.,1.,0.],
        [1.,-1.,1.,0.,0.,1.]
    ]);
    let requirements = arr1(&[5.,3.,2.]);
    let table = initial_table(&objective,&constraints,&requirements);
    let test = arr2(&[
        [1.,3.,-1.,2.,0.,0.,0.,0.],
        [0.,2.,-2.,3.,1.,0.,0.,5.],
        [0.,1.,1.,-1.,0.,1.,0.,3.],
        [0.,1.,-1.,1.,0.,0.,1.,2.]
    ]);
    assert_eq!(table,test);
}

#[test]
fn test_select_pivot(){
    let objective = arr1(&[-3.,1.,-2.]);
    let constraints = arr2(&[
        [2.,-2.,3.,1.,0.,0.],
        [1.,1.,-1.,0.,1.,0.],
        [1.,-1.,1.,0.,0.,1.]
    ]);
    let requirements = arr1(&[5.,3.,2.]);
    let table = initial_table(&objective,&constraints,&requirements);
    let pivot = pivot_point(&table);
    assert_eq!(Some([3,1]),pivot);
}