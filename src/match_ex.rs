pub fn determine_shirt_size(x: i32) {
    let tshirt_width = x;
    let tshirt_size = match tshirt_width {
        16 => "S",      // check 16
        17 | 18 => "M", // check 17 and 18
        19..=21 => "L", // check from 19 to 21 (19,20,21)
        22 => "XL",
        _ => "Not Available",
    };

    println!("{}", tshirt_size);
}

pub fn give_mark_feedback(x: u8, y: u8) {
    let marks_paper_a: u8 = x;
    let marks_paper_b: u8 = y;

    let output = match (marks_paper_a, marks_paper_b) {
        (50, 50) => "Full marks for both papers",
        (50, _) => "Full marks for paper A",
        (_, 50) => "Full marks for paper B",
        (x, y) if x > 25 && y > 25 => "Good",
        (_, _) => "Work hard",
    };

    println!("{}", output); // Work hard
}
