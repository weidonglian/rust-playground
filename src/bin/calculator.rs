macro_rules! calculate {
    (eval $e: expr) => {
        let val: usize = $e;
        println!("{} = {}", stringify!($e), val);
    };
}

fn main() {
    calculate! {
        eval 1*2+5
    }
}
