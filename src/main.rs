mod vector;

use vector::VecF;

fn main() {
    let a = VecF::as2(3.0, 1.4);
    let b = VecF::as2(0.0, 2.0);
    let c = VecF::as2(1., 1.);

    let d = &a + &b;
    println!("d = {}", d);

    println!("{}\n\n{}\n\n{}", a, b, c);
    println!("Len:\n\n{}\n\n{}\n\n{}", a.len(), b.len(), c.len());

    let dot_product = VecF::dot(&a, &b);
    println!("Dot product: {}", dot_product);

    let na = a.normalize();
    let nb = b.normalize();

    let dot_product = VecF::dot(&na, &nb);
    println!("Dot product after normalization: {}", dot_product);
}
