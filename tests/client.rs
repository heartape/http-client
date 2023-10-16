use std::io::Read;

#[test]
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut response = String::new();
    reqwest::blocking::get("https://file.heartape.com")?
        .read_to_string(&mut response)?;
    println!("{:#?}", response);
    Ok(())

}
