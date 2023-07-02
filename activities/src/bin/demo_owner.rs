enum Light {
    Bright,
    Dull,
}

fn main() {
    let light = Light::Bright;
    display_light(&light);
    display_light(&light);
}

fn display_light(light: &Light) {
    match light {
        Light::Bright => println!("its bright"),
        Light::Dull => println!("its dull"),
    }
}
