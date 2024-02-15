use tray_item::{IconSource, TrayItem};
use std::io::Cursor;
use std::thread::sleep;
use std::time::Duration;
fn main() {
    // Gray Image
    let cursor_gray_image = Cursor::new(include_bytes!("../images/gray.png"));
    // Blank Image
    let cursor_blank_image = Cursor::new(include_bytes!("../images/blank.png"));
    // Define Gray Image Decoder
    let decoder_gray = png::Decoder::new(cursor_gray_image);
    // Define Blank Image Decoder
    let decoder_blank = png::Decoder::new(cursor_blank_image);
    // Define Gray Image Reader
    let mut reader_gray = decoder_gray.read_info().unwrap();
    // Define Blank Image Reader
    let mut reader_blank = decoder_blank.read_info().unwrap();
    // Define Gray Image Buffer
    let mut buf_gray = vec![0; reader_gray.output_buffer_size()];
    // Define Blank Image Buffer
    let mut buf_blank = vec![0; reader_blank.output_buffer_size()];
    // Read data into buffer
    reader_blank.next_frame(&mut buf_gray).unwrap();
    reader_gray.next_frame(&mut buf_blank).unwrap();
    // Define icon data
    let icon_blank = IconSource::Data {
        data: buf_blank,
        height: 32,
        width: 32,
    };
    let icon_gray = IconSource::Data {
        data: buf_gray,
        height: 32,
        width: 32,
    };
    // Define tray item
    let mut tray = TrayItem::new("Flicker Fix", icon_blank.clone()).unwrap();
    // Add quit button
    tray.add_menu_item("Quit", move || {
        println!("Quiting!");
        std::process::exit(0);
    }).unwrap();
    // Spawn color change thread
    println!("Starting Loop");
    loop {
        tray.set_icon(icon_blank.clone()).unwrap();
        sleep(Duration::from_millis(200));
        tray.set_icon(icon_gray.clone()).unwrap();
        sleep(Duration::from_millis(200));
    }
}