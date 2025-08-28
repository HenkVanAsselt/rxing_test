use rxing::oned::Code128Writer;
use rxing::BarcodeFormat::CODE_128;
use rxing::helpers::save_image;
use rxing::Writer;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Create a Code128 barcode
    let data = "1234567890123";
    let width=200;
    let height = 50;
    let format = CODE_128;
    let bitmatrix = Code128Writer.encode(data, &format, width, height).unwrap();
    // println!("{:?}",bitmatrix);

    // Write the barcode to a PNG file
    let filename = "code128_barcode.png";
    let _result = save_image(filename, &bitmatrix);
    println!("Barcode generated successfully and saved to {}", filename);

    Ok(())
}