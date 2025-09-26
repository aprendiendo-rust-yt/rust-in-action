fn greet_world() {
    println!("Hello, world!");
    let southern_germany = "Grüß Gott!";
    let sonora = "¡Quiubo!";
    let regions = [southern_germany, sonora];
    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn main() {
    greet_world();
}
