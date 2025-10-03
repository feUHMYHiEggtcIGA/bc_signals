use bc_utils_lg::statics::prices::{SRC, OPEN};

use bc_signals::train::mm::*;

#[test]
fn mm_coll_res_1(){
    let res =  mm_coll::<Vec<f64>>(
        SRC.as_slice(), 
        "open",
        &2, 
        &3,            
        &0.0001,
        &0.01,
    );
    assert_eq!(
        &[-1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0],
        &res[..10]
    );
    assert_eq!(&-1.0, &res[OPEN.iter().enumerate().max_by(|v1, v2| v1.1.partial_cmp(v2.1).unwrap()).unwrap().0],);
    assert_eq!(&1.0, &res[OPEN.iter().enumerate().min_by(|v1, v2| v1.1.partial_cmp(v2.1).unwrap()).unwrap().0],);
}
