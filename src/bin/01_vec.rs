fn demo1() {
    let v = vec![1, 2, 3, 4];

    // map: transform each item -> iterator -> collect back to Vec
    let doubled = v.iter().map(|x| x * 2).collect::<Vec<i32>>();
    //Looks at each item in the vector without taking it. Think “read-only view.”
    assert_eq!(doubled, vec![2, 4, 6, 8]);

    // filter: keep only even
    let evens = v
        .iter()
        .copied()
        .filter(|x| x % 2 == 0)
        .collect::<Vec<i32>>();
    assert_eq!(evens, vec![2, 4]);

    // filter: keep only even - no copied
    let evens = v.iter().filter(|x| **x % 2 == 0).collect::<Vec<_>>();
    assert_eq!(evens, vec![&2, &4]);

    // chained = map + filter
    let chained_double_squre_root = v
        .iter()
        .map(|x: &i32| x * 2)
        .filter(|x: &i32| x % 2 == 0)
        .collect::<Vec<i32>>();
    assert_eq!(chained_double_squre_root, vec![2, 4, 6, 8]);
}

fn demo2() {
    let v = vec![1, 2, 3];

    // immutably borrowed items: &i32
    let sum= v.iter().sum::<i32>();
    assert_eq!(sum, 6);

    // mutably borrowed items: &mut i32 (modify in-place)
    let mut w = vec![1, 2, 3];
    for x in w.iter_mut() { *x = *x *2 }
    // Looks at each item with permission to change it. Think “editable view.”
    assert_eq!(w, vec![2, 4, 6]);


    // by value: moves items out (consumes the Vec)
    let v = vec![1, 2, 3];

    let plus_one = v.into_iter().map(|x| x + 1).collect::<Vec::<i32>>();
    // Takes ownership of each item, consuming the vector. Think “I’m dismantling this Vec and using its parts.”
    // println!("{}", v[0]); error -> ownership moved in above line
    assert_eq!(plus_one, vec![2,3,4]);
}

fn demo3() {
     // enumerate: index + value
    let v = vec!['a', 'b', 'c'];
    v.iter().enumerate().for_each(|(index, c)| {
        println!("{}: {}", index, c);
    });

    let uppercased = v.iter().enumerate().map(|(_, el)| el.to_ascii_uppercase() ).collect::<Vec<char>>();
    assert_eq!(uppercased, vec!['A', 'B', 'C']);

    // zip: combine two sequences

    let numbers = vec![1, 2, 3];
    let letters = vec!['a', 'b', 'c'];

    let zipped: Vec<(i32, String)> = numbers.iter().zip(letters.iter()).map(|(num, ch)| (*num, ch.to_string())).collect::<Vec<(i32, String)>>();
    assert_eq!(zipped, vec![(1, "a".to_string()), (2, "b".to_string()), (3, "c".to_string())]);

    // flat_map: 1->many
    let words = vec!["hello", "world"];
    let flat_mapped = words.iter().flat_map(|word| word.chars()).collect::<Vec<char>>();
}
fn main() { demo3(); }
