use rayso_rs::RaysoConfig;

fn main() {
    let config = RaysoConfig::builder()
        .background(false)
        .padding(8)
        .build();

    println!("Generated URL: {}", config.to_url());
}
