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

#[test]
fn test_gauss(){
    let objective = arr1(&[-3.,1.,-2.]);
    let constraints = arr2(&[
        [2.,-2.,3.,1.,0.,0.],
        [1.,1.,-1.,0.,1.,0.],
        [1.,-1.,1.,0.,0.,1.]
    ]);
    let requirements = arr1(&[5.,3.,2.]);
    let mut table = initial_table(&objective,&constraints,&requirements);
    let pivot = pivot_point(&table).unwrap();
    let after_gauss = arr2(&[
        [1.,0.,2.,-1.,0.,0.,-3.,-6.],
        [0.,0.,0.,1.,1.,0.,-2.,1.],
        [0.,0.,2.,-2.,0.,1.,-1.,1.],
        [0.,1.,-1.,1.,0.,0.,1.,2.]
    ]);
    gauss(pivot,&mut table);
    assert_eq!(table,after_gauss);
}

#[test]
fn test_simplex_all(){
    let objective = arr1(&[-3.,1.,-2.]);
    let constraints = arr2(&[
        [2.,-2.,3.,1.,0.,0.],
        [1.,1.,-1.,0.,1.,0.],
        [1.,-1.,1.,0.,0.,1.]
    ]);
    let requirements = arr1(&[5.,3.,2.]);
    let mut table = initial_table(&objective,&constraints,&requirements);
    while let Some(pivot) = pivot_point(&table) {
        gauss(pivot,&mut table);
    }
    let expected = arr2(&[
        [1.,0.,0.,0.,-1.,-1.,0.,-8.],
        [0.,0.,0.,1.,1.,0.,-2.,1.],
        [0.,0.,1.,0.,1.,0.5,-5./2.,3./2.],
        [0.,1.,0.,0.,0.,0.5,0.5,5./2.]
    ]);
    assert_eq!(table,expected);
}