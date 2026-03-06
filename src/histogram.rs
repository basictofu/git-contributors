use std::vec::Vec;

pub fn hist(data: &Vec<u32>, range: (u32, u32), num_bins: usize) -> Vec<usize> {
    let mut histogram = vec![0; num_bins];
    let (min, max) = range;
    let bin_size = ((max - min) as f64) / (num_bins as f64);

    for &value in data {
        if value < min {
            continue; // Before window => ignore
        }
        if value > max {
            continue; // After window => ignore
        }

        let i = (((value - min) as f64) / bin_size).floor() as usize;
        histogram[i] += 1;
    }

    histogram
}

#[test]
fn test_num_bins() {
    let data = vec![1, 1];
    let num_bins: usize = 1;
    let bins = hist(&data, (0, 2), num_bins);
    assert_eq!(bins.len(), num_bins);
}

#[test]
fn test_simple() {
    let h = hist(&vec![1, 1], (0, 3), 2);
    assert_eq!(h, vec![2, 0]);
}
