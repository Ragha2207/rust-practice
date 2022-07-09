pub enum FlashMessage {
    Success,                                    // A unit variant
    Warning { category: i32, message: String }, // A struct variant
    Error(String),                              // A tuple variant
}

pub fn print_flash_message(m: FlashMessage) {
    // Pattern matching with enum
    match m {
        FlashMessage::Success => println!("Form Submitted correctly"),
        FlashMessage::Warning { category, message } =>
        // Destructure, should use same field names
        {
            println!("Warning : {} - {}", category, message)
        }
        FlashMessage::Error(msg) => println!("Error : {}", msg),
    }
}
