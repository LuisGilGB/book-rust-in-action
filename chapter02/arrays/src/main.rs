fn main() {
    let array1 = [1, 2, 3];
    let array2: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];

    let arrays = [array1, array2, blank1, blank2];

    for arr in &arrays {
        print!("{:?}: ", arr);
        for num in arr.iter() {
            print!("\t{} + 10 = {}", num, num + 10);
        }

        let mut sum = 0;
        for i in 0..arr.len() {
            sum += arr[i];
        }
        println!("\t(∑{:?} = {})", arr, sum);
    }
}
