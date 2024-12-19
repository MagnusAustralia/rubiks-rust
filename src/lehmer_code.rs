fn generate_corner_indexes() {
    state = [[0u8; 4], [1u8; 4], [2u8; 4], [3u8; 4], [4u8; 4], [5u8; 4]];

}

fn lexicogrpahic_order(arr: Vec<u8>) {
    let n = arr.len();
    let mut factorial = [u32; n + 1];
    let mut order = 0;

    factorial[0] = 1;
    for i in 1..n {
        factorial[i] = factorial[i - 1] * i;
    }

    for i in 0..n-1 {
        let current_element = arr[i];

        let mut smaller_count = 0;
        for j in i+1..n-1 {
            if arr[j] < current_element {
                smaller_count += 1;
            }
        }

        order += smaller_count * factorial[n - 1 - i]
    }

    order + 1
}


fn return_index() {
    return (self.findCornerPerm() * 3**7) + self.rankOrientations()
}