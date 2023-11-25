

fn main() {
    let input = include_str!("../input/day24.txt");
    let nums: Vec<u64> = input.lines().map(|x| x.parse().unwrap()).collect();

    let num_groups = 3;
    let mut group_size = min_group_size(&nums, num_groups);
    let mut ans = u64::MAX;
    while ans == u64::MAX {
        get_min_prod(&nums, 0, group_size, nums.iter().sum::<u64>()/num_groups, 1, &mut ans);
        group_size += 1;
    }
    println!("Part 1: Product of gifts in smallest of 3 groups: {ans}");

    let num_groups = 4;
    let mut group_size = min_group_size(&nums, num_groups);
    let mut ans = u64::MAX;
    while ans == u64::MAX {
        get_min_prod(&nums, 0, group_size, nums.iter().sum::<u64>()/num_groups, 1, &mut ans);
        group_size += 1;
    }
    println!("Part 2: Product of gifts in smallest of 4 groups: {ans}");
}

fn min_group_size(nums: &Vec<u64>, num_groups: u64) -> usize {
    let total = &nums.iter().sum();
    let mut i = nums.len();
    let mut sum: u64 = 0;
    while sum < total / num_groups {
        i -= 1;
        sum += nums[i];
    }
    nums.len() - i
}

fn get_min_prod(nums: &Vec<u64>, idx: usize, count: usize, partial_sum: u64, mut partial_prod: u64, ans: &mut u64) {
    if count == 0 {
        if partial_sum == 0 {
            *ans = *ans.min(&mut partial_prod)
        }
        return
    }
    for i in idx..nums.len() {
        if nums[i] > partial_sum {
            continue
        }
        get_min_prod(nums, i + 1, count - 1, partial_sum - nums[i], partial_prod * nums[i], ans);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_size_2() {
        let nums: Vec<u64> = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        let group_size = 2;
        let num_groups = 3;
        assert_eq!(min_group_size(&nums, num_groups), group_size);
        let mut ans = u64::MAX;
        get_min_prod(&nums, 0, group_size, nums.iter().sum::<u64>()/num_groups, 1, &mut ans);
        assert_eq!(ans, 99);
    }

    #[test]
    fn group_size_3() {
        let nums: Vec<u64> = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        let group_size = 2;
        let num_groups = 4;
        assert_eq!(min_group_size(&nums, num_groups), group_size);
        let mut ans = u64::MAX;
        get_min_prod(&nums, 0, group_size, nums.iter().sum::<u64>()/num_groups, 1, &mut ans);
        assert_eq!(ans, 44);
    }

}