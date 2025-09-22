fn main() {

    let str1: String = "hello".to_owned();

    let str2: &str = "hello again";

    let a = 13;

    let x1 = &a;

    let x2 = &a;

    let mut a = 13;
    let mut y1 = &mut a;
    *y1 = 90;
}

