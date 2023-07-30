fn main() {
    // array


    let array = [1, 2, 2, 4, 5, 6, 7, 7, 8, 9, 9];

    let mut i = 0;
    while i < array.len() {
        let mut j = i + 1;
        while j < array.len() {
            if array[i] == array[j] {
                println!("{}", array[i]);
            }
            j += 1;
        }
        i += 1;
    }
}
