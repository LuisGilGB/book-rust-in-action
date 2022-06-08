fn greet_world() {
    println!("Hello, world!");
    let spanish = "¡Hola, mundo!";
    let southern_germany = "Grüß Gott!";
    let japan = "ハロー・ワールド";
    let regions = [spanish, southern_germany, japan];
    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn main() {
    greet_world();
}
