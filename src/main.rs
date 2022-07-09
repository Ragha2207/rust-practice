//! mod used for importing library class
mod combine_two_strings;
mod enum_ex;
mod generics_ex;
mod get_square_root;
mod impl_trait_ex;
mod match_ex;
mod print_com;
mod structs_ex;
mod vector_game;

// use is used for importing the methods and structs
use enum_ex::print_flash_message;
use enum_ex::FlashMessage;
use generics_ex::Point;
use impl_trait_ex::FullName;
use impl_trait_ex::LastName;
use impl_trait_ex::PlayNumber;
use impl_trait_ex::Player;
use structs_ex::get_midnightblue_color;
use structs_ex::Color;

fn main() {
    print_com::run();
    println!(
        "The square root of 6 is {}",
        get_square_root::get_square_root(6)
    );
    println!(
        "Combination of a and b is  {}",
        combine_two_strings::combine("Ragha", " Mutluru")
    );
    match_ex::determine_shirt_size(25);
    match_ex::give_mark_feedback(40, 40);
    vector_game::vector_game();

    // structs example

    // Creating an instance
    let black = Color {
        red: 0,
        green: 0,
        blue: 0,
    };

    // Accessing its fields using dot notation
    println!(
        "Black = rgb({}, {}, {})",
        black.red, black.green, black.blue
    ); //Black = rgb(0, 0, 0)

    // Structs are immutable by default, use `mut` to make it mutable but doesn't support field level mutability
    let mut link_color = Color {
        red: 0,
        green: 0,
        blue: 255,
    };
    link_color.blue = 238;
    println!(
        "Link Color = rgb({}, {}, {})",
        link_color.red, link_color.green, link_color.blue
    ); //Link Color = rgb(0, 0, 238)

    // Copy elements from another instance
    let blue = Color {
        blue: 255,
        ..link_color
    };
    println!("Blue = rgb({}, {}, {})", blue.red, blue.green, blue.blue); //Blue = rgb(0, 0, 255)

    // Destructure the instance using a `let` binding, this will not destruct blue instance
    let Color {
        red: r,
        green: g,
        blue: b,
    } = blue;
    println!("Blue = rgb({}, {}, {})", r, g, b); //Blue = rgb(0, 0, 255)

    // Creating an instance via functions & accessing its fields
    let midnightblue = get_midnightblue_color();
    println!(
        "Midnight Blue = rgb({}, {}, {})",
        midnightblue.red, midnightblue.green, midnightblue.blue
    ); //Midnight Blue = rgb(25, 25, 112)

    // Destructure the instance using a `let` binding
    let Color {
        red: r,
        green: g,
        blue: b,
    } = get_midnightblue_color();
    println!("Midnight Blue = rgb({}, {}, {})", r, g, b); //Midnight Blue = rgb(25, 25, 112)

    let mut form_status = FlashMessage::Success;
    print_flash_message(form_status);

    form_status = FlashMessage::Warning {
        category: 2,
        message: String::from("Field X is required"),
    };
    print_flash_message(form_status);

    form_status = FlashMessage::Error(String::from("Connection Error"));
    print_flash_message(form_status);

    let x_int = Point { x: 0, y: 0 };

    println!("Point in int {:?}", x_int);

    let x_float = Point { x: 0.1, y: 0.2 };

    println!("Point in float {:?}", x_float);

    let player_1 = Player {
        first_name: "Ragha".to_string(),
        last_name: "Mutluru".to_string(),
    };

    let player_2 = Player {
        first_name: "Roger".to_string(),
        last_name: "Federer".to_string(),
    };

    let player_3 = Player {
        first_name: "Rafael".to_string(),
        last_name: "Nadal".to_string(),
    };

    println!("Player 02: {}", player_2.full_name());
    println!("Player 02 Lastname: {}", player_2.last_name());
    println!("Player Default shirt number: {}", player_2.getDefaultPlayerNumber());
    
    println!(
        "Player 01 its {}, get {} shirt number. Shirt number is {}",
        player_1.last_name(),
        player_1.full_name(),
        player_1.getPlayerShirtNumberByName()
    );
    println!(
        "Player 02 its {}, get {} shirt number. Shirt number is {}",
        player_2.last_name(),
        player_2.full_name(),
        player_2.getPlayerShirtNumberByName()
    );
    println!(
        "Player 03 its {}, get {} shirt number. Shirt number is {}",
        player_3.last_name(),
        player_3.full_name(),
        player_3.getPlayerShirtNumberByName()
    );
}
