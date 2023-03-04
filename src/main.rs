use scr_to_svg::get_scramble_svg;

fn main() {
    let svg = get_scramble_svg("333", "");

    println!("{svg}");
}
