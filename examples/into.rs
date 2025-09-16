use equal_parts::IntoEqualParts;

fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let parts = data.into_equal_parts(3);

    for part in parts {
        println!("{:?}", part);
    }
}
