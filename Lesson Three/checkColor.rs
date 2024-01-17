struct Color(u64, u64, u64);

fn main() {
    let color_one = Color(1, 3, 5);
    println!(
        "the colors are matching: {}",
        check_color(1_u64, 3_u64, 5_u64, color_one)
    );
}

fn check_color(r: u64, g: u64, b: u64, sample: Color) -> bool {
    r == sample.0 && g == sample.1 && b == sample.2
}
