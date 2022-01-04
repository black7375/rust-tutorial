#[macro_export]
macro_rules! vector {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
                temp_vec
        }
    };
}

fn main() {
    let vec = vector![3, 6, 5];
    println("{:?}", vec);
}
