// check std array

pub fn iterate() {
    println!("Array iteration here");
    let mut arr: [i32; 5] = [0; 5];
    arr[0] = 1;
    arr[1] = 4;
    println!("{:#?}", &arr);

    for item in arr.into_iter().enumerate() {
        let (index, x): (usize, i32) = item;
        println!("idx: {} - item: {}", index, x);
    }
}
