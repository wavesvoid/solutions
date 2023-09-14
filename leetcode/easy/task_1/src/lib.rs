
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut store = std::collections::HashMap::<&i32, i32>::new();

    for (i, n) in nums.iter().enumerate() {
        if let Some(found) = store.get(&(target - n)) {
            return [*found, i as i32].to_vec();
        }
        store.insert(n, i as i32); // store index under number
    }
    vec![]
}


#[cfg(test)]
mod test {
    use super::*;

    
    #[test]
    fn test_two_sum() {
        let result = two_sum(vec![1,2,3,4], 5);

        assert_eq!([1, 2].to_vec(), result);
    }
}
