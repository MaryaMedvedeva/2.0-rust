fn main() {
    // array


    let mut array: [i8; 5] = [1, 2, 3, 4, 5];
    //                         0  1  2  3  4

    array[2] = 10;
    array[0] = 2;
    array[1] = 13;
    array[3] = 43;
    array[4] = 1;

    println!("{}", array[2]);
    println!("{:?}", array);
}
