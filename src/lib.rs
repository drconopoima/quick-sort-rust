use rayon;
use std::usize::MAX;
mod random_generator;

pub fn move_pivot<T: PartialOrd>(
    vector: &mut [T],
    mut pivot_idx: usize,
    start_idx: usize,
    end_idx: usize,
) -> usize {
    // println!("Pivot {:?} at index {:?}", vector[pivot_idx], pivot_idx);
    // println!("O: {:?}", vector);
    let input_length = vector.len();
    if input_length <= 1 {
        return 0;
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
    if start_idx > pivot_idx {
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

    let mut lower_than_idx: usize = MAX;
    for idx in start_idx..end_idx {
        if idx == pivot_idx {
            continue;
        };
        if vector[idx] < vector[pivot_idx] {
            // at this time unchecked_add is nightly, so check done manually
            lower_than_idx = lower_than_idx.wrapping_add(1);
            if lower_than_idx != idx {
                vector.swap(lower_than_idx, idx);
                if pivot_idx == lower_than_idx {
                    pivot_idx = idx;
                }
                // println!("{:?}: {:?}", idx, vector);
            }
        };
    }
    lower_than_idx = lower_than_idx.wrapping_add(1);
    vector.swap(pivot_idx, lower_than_idx);
    // println!("vector {:?}", vector);
    // println!("return_index {:?}", lower_than_idx);
    return lower_than_idx;
}

pub fn quick_sort<T: PartialOrd>(vector: &mut [T]) {
    let input_length = vector.len();
    if input_length <= 1 {
        return;
    }
    let initial_pivot = random_generator::rand(input_length);
    let pivot = move_pivot(vector, initial_pivot, 0, input_length);
    let (a, b) = vector.split_at_mut(pivot);
    quick_sort(a);
    quick_sort(&mut b[1..])
}

struct RawSend<T>(*mut [T]);

unsafe impl<T> Send for RawSend<T> {}

pub fn threaded_quick_sort<T: 'static + Send + PartialOrd>(vector: &mut [T]) {
    let input_length = vector.len();
    if input_length <= 1 {
        return;
    }
    let initial_pivot = random_generator::rand(input_length);
    let pivot = move_pivot(vector, initial_pivot, 0, input_length);
    let (a, b) = vector.split_at_mut(pivot);

    let raw_a: *mut [T] = a as *mut [T];
    let send_a = RawSend(raw_a);

    unsafe {
        let handle = std::thread::spawn(move || {
            threaded_quick_sort(&mut *send_a.0);
        });
        threaded_quick_sort(&mut b[1..]);
        handle.join().ok();
    }
}

pub fn quick_sort_rayon<T: Send + PartialOrd>(vector: &mut [T]) {
    let input_length = vector.len();
    if input_length <= 1 {
        return;
    }
    let initial_pivot = random_generator::rand(input_length);
    let pivot = move_pivot(vector, initial_pivot, 0, input_length);
    let (a, b) = vector.split_at_mut(pivot);

    rayon::join(|| quick_sort_rayon(a), || quick_sort_rayon(&mut b[1..]));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_move_pivot() {
        use super::*;
        let mut vector = vec![100, 3, 55, 7, 5, 1, 6, 41, 0, 2, 4];
        let vector_length = vector.len();
        let pivot_idx = 9; // element 2
        let pivot = vector[pivot_idx];
        move_pivot(&mut vector, pivot_idx, 0, vector_length);
        // assert_eq!(vector, vec![100, 3, 55, 7, 5, 1, 6, 41, 0, 2, 4]);
        for idx in 0..vector_length {
            // println!("idx: {:?}, pivot: {:?}, vector[idx]: {:?}",idx, pivot, vector[idx]);
            assert!((vector[idx] < pivot) == (idx < pivot));
        }
    }
    #[test]
    fn test_quick_sort() {
        use super::*;
        let mut vector = vec![1, 3, 55, 7, 5, 100, 6, 41, 0, 2, 4];
        quick_sort(&mut vector);
        assert_eq!(vector, vec![0, 1, 2, 3, 4, 5, 6, 7, 41, 55, 100]);
    }
    #[test]
    fn test_threaded_quick_sort() {
        use super::*;
        let mut vector = vec![1, 3, 55, 7, 5, 100, 6, 41, 0, 2, 4];
        threaded_quick_sort(&mut vector);
        assert_eq!(vector, vec![0, 1, 2, 3, 4, 5, 6, 7, 41, 55, 100]);
    }
    #[test]
    fn test_rayon_quick_sort() {
        use super::*;
        let mut vector = vec![1, 3, 55, 7, 5, 100, 6, 41, 0, 2, 4];
        quick_sort_rayon(&mut vector);
        assert_eq!(vector, vec![0, 1, 2, 3, 4, 5, 6, 7, 41, 55, 100]);
    }
    #[test]
    fn sort_empty() {
        use super::*;
        let vector_empty: Vec<f32> = Vec::new();
        let mut sorted_empty = vector_empty.clone();
        quick_sort(&mut sorted_empty);
        assert_eq!(vector_empty, sorted_empty);
    }
    #[test]
    fn random_sort() {
        use super::*;
        use rand::distributions::Uniform;
        use rand::Rng;
        // use std::time::SystemTime;
        let mut rng = rand::thread_rng();
        let range = Uniform::new(0, 20);
        let random_vector: Vec<usize> = (0..100).map(|_| rng.sample(&range)).collect();
        let mut quick_sorted = random_vector.clone();
        let mut std_sorted = random_vector.clone();
        std_sorted.sort();
        // let time_now = SystemTime::now();
        // let elapsed = time_now.elapsed().expect("Error: Failed to get elapsed time.").as_nanos();
        // println!("Elapsed time: {:?} nanoseconds", elapsed);
        quick_sort(&mut quick_sorted);
        assert_eq!(std_sorted, quick_sorted);
        // panic!()
    }
}
