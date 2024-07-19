struct RGBColor(u8, u8, u8);
struct Point(u8, u8, u8);

fn main() {
    let black = RGBColor(0, 0, 0);
    let RGBColor(r, g, b) = black;

    let origin = Point(0, 0, 0);
    let Point(x,y,z) = origin;

    println!("The point ({x}, {y}, {z}) is black, which has the RGB-value ({r}, {g}, {b})");
}