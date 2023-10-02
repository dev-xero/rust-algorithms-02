use core::panic;

pub fn sort<T>(arr: &mut Vec<T>, low: usize, high: usize)
where
    T: Eq + PartialEq + PartialOrd + Clone
{
    if low <= high {
        let p = partition(arr, low, high, &|a, b| a <= b);
        sort(arr, low, p-1);
        sort(arr, p+1, high);
    }
}

fn partition<T, F>(arr: &mut Vec<T>, low: usize, high: usize, f: &F) -> usize
where
    T: Clone,
    F: Fn(&T, &T) -> bool
{
    let pivot = match arr.get(high) {
        Some(v) => v.clone(),
        _ => panic!("Array index is out of bounds.")
    };

    let mut i = low;

    for j in low..high-1 {
        match arr.to_vec().get(j) {
            Some(v) => {
                if f(v, &pivot) {
                    arr.swap(i, j);
                    i += 1;
                }
            },
            _ => panic!("Array index is out of bounds.")
        }
    }

    arr.swap(i, high);
    i
}