use wasm_bindgen::prelude::JsValue;
use std::collections::HashSet;

use extended_matrix::{SquareMatrix, Vector, SquareMatrixTrait, BasicOperationsTrait, Position};

use crate::types::FEFloat;


pub(super) fn find_lines_intersection(
    l_1_p_1: &[FEFloat; 3], 
    l_1_p_2: &[FEFloat; 3], 
    l_2_p_1: &[FEFloat; 3], 
    l_2_p_2: &[FEFloat; 3], 
    abs_tol: FEFloat, 
    rel_tol: FEFloat
) 
    -> Result<[FEFloat; 3], JsValue>
{
    let a_values_1 = [
        l_1_p_2[0] - l_1_p_1[0], l_2_p_1[0] - l_2_p_2[0], 
        l_1_p_2[1] - l_1_p_1[1], l_2_p_1[1] - l_2_p_2[1],
    ];
    let b_values_1 = [l_2_p_1[0] - l_1_p_1[0], l_2_p_1[1] - l_1_p_1[1]];
    let index_1 = 2;

    let a_values_2 = [
        l_1_p_2[0] - l_1_p_1[0], l_2_p_1[0] - l_2_p_2[0], 
        l_1_p_2[2] - l_1_p_1[2], l_2_p_1[2] - l_2_p_2[2],
    ];
    let b_values_2 = [l_2_p_1[0] - l_1_p_1[0], l_2_p_1[2] - l_1_p_1[2]];
    let index_2 = 1;

    let a_values_3= [
        l_1_p_2[1] - l_1_p_1[1], l_2_p_1[1] - l_2_p_2[1], 
        l_1_p_2[2] - l_1_p_1[2], l_2_p_1[2] - l_2_p_2[2],
    ];
    let b_values_3 = [l_2_p_1[1] - l_1_p_1[1], l_2_p_1[2] - l_1_p_1[2]];
    let index_3 = 0;

    for (a_values, b_values, index) in [
        (a_values_1, b_values_1, index_1), (a_values_2, b_values_2, index_2), (a_values_3, b_values_3, index_3),
    ].iter()
    {
        let a = SquareMatrix::create(2, a_values);
        let b = Vector::create(b_values);
        let mut x = Vector::create(&[0.0, 0.0]);
        match a.gauss_gep(&b, &mut x, rel_tol)
        {
            Ok(()) => 
            {
                let x_elements = x.get_elements();
                let m = x_elements.get(&Position(0, 0)).ok_or("Incorrect position!".to_string())?;
                let n = x_elements.get(&Position(1, 0)).ok_or("Incorrect position!".to_string())?;
                if (
                    l_1_p_1[*index] + m * (l_1_p_2[*index] - l_1_p_1[*index]) - 
                    l_2_p_1[*index] - n * (l_2_p_2[*index] - l_2_p_1[*index])
                ).abs() > abs_tol
                {
                    return Err(JsValue::from("Lines didn't lie on the same plane!"));
                }
                else
                {
                    let intersection = [
                        l_2_p_1[0] + n * (l_2_p_2[0] - l_2_p_1[0]),
                        l_2_p_1[1] + n * (l_2_p_2[1] - l_2_p_1[1]),
                        l_2_p_1[2] + n * (l_2_p_2[2] - l_2_p_1[2]),
                    ];
                    if !intersection.iter().any(|component| component.is_nan()) 
                    {
                        return Ok(intersection);
                    }
                }
            }
            Err(_e) => (),
        }
    }

    Err(JsValue::from("Could not find intersection!"))
}


pub(super) fn generate_uid(uids: &mut HashSet<u32>) -> u32
{
    let uid =
    {
        let mut current_uid = rand::random::<u32>();
        while uids.contains(&current_uid)
        {
            current_uid = rand::random::<u32>();
        }
        current_uid
    };
    uids.insert(uid);
    uid
}