use std::fmt::Debug;

pub fn move_pivot_idx<T: PartialOrd + Debug>(
    vector: &mut Vec<T>,
    pivot_idx: usize,
    start_idx: usize,
    end_idx: usize,
) -> usize {
    println!("Pivot {:?} at index {:?}", vector[pivot_idx], pivot_idx);
    println!("O: {:?}", vector);
    let input_length = vector.len();
    if input_length <= 1 {
        return input_length;
    }
    if pivot_idx >= input_length {
        panic!(format!(
            "Error: Could not set pivot by index '{:?}'. Out of bounds of input length '{:?}'",
            pivot_idx, input_length
        ))
    }
    if start_idx >= input_length - 1 {
        panic!(format!(
            "Error: Could not set lower pivot loop boundary by index '{:?}'. Out of bounds of input length '{:?}'",
            start_idx, input_length
        ))
    }
    if end_idx > input_length {
        panic!(format!(
            "Error: Could not set upper pivot loop boundary by index '{:?}'. Out of bounds of input length '{:?}'",
            end_idx, input_length
        ))
    }
    if start_idx >= pivot_idx {
        panic!(format!(
            "Error: Could not set lower pivot boundary by index '{:?}'. Value is greater or equal than pivot index '{:?}'",
            start_idx, pivot_idx
        ))
    }
    if end_idx < pivot_idx {
        panic!(format!(
            "Error: Could not set upper pivot boundary by index '{:?}'. Value is lower than pivot index '{:?}'",
            end_idx, pivot_idx
        ))
    }

    let mut lower_than_idx: usize = start_idx;
    for idx in start_idx..end_idx - 1 {
        if idx == pivot_idx {
            continue;
        };
        if vector[idx] < vector[pivot_idx] {
            if idx != start_idx {
                lower_than_idx += 1;
            }
            if lower_than_idx != idx {
                vector.swap(lower_than_idx, idx);
                println!("{:?}: {:?}", idx, vector);
            }
        };
    }
    lower_than_idx += 1;
    vector.swap(pivot_idx, lower_than_idx);
    return lower_than_idx;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_move_pivot() {
        use super::*;
        let mut vector = vec![1, 3, 55, 7, 5, 100, 6, 41, 0, 2, 4];
        let vector_length = vector.len();
        let pivot_idx = 9; // element 2
        let pivot = vector[pivot_idx];
        move_pivot_idx(&mut vector, pivot_idx, 0, vector_length);
        for idx in 0..vector_length {
            // println!("idx: {:?}, pivot: {:?}, vector[idx]: {:?}",idx, pivot, vector[idx]);
            assert!((vector[idx] < pivot) == (idx < pivot));
        }
    }
}
