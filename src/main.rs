
use std::io;                // For handling input/output operations 
use rand::Rng;              // For generating random numbers 
use qrcode::QrCode;         // For creating QR codes
use image::Luma;            // For handling the image format (black and white in this case)

fn main() {
    println!("Enter your Network Name: ");

    // Create a mutable (changeable) empty string to store the network name
    let mut network_name = String::new();
    
    // Read user input into the network_name variable
    io::stdin().read_line(&mut network_name).expect("Error reading line");

    let network_name = network_name.trim(); //delete the /n at the end of the SSID 

    println!("Password length: ");

    let mut password_length = String::new();
    io::stdin().read_line(&mut password_length).expect("Error reading line");
    
    // Convert the input string to a number
    // trim() removes whitespace
    // parse() converts the string to a number
    let password_length: usize = password_length.trim().parse().expect("Enter a valid input.");

    // Create a vector (like a Python list) of characters to use in the password
    let characters: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_=+".chars().collect();
    let characters_length = characters.len();

    // Create a random number generator (RNG) a machine that creates a random number each time you call it
    let mut rng = rand::thread_rng();
    
    // Create a string to hold the password with pre-allocated capacity
    let mut password = String::with_capacity(password_length);

    // Loop to generate each character of the password
    for _ in 0..password_length {
        // Generate a random index between 0 and characters_length-1
        let index = rng.gen_range(0..characters_length);
        
        // Get the character at that index
        let character = characters[index];
        
        // Add the character to our password string
        password.push(character);
    }
    
    // Print the generated password
    println!("Wifi password: {}", password);
    
    // Create the WiFi QR code data string in the standard format
    let wifi_qr_data = format!("WIFI:S:{};T:WPA;P:{};H:false;;",network_name, password);
    
    // Create and save the QR code image
    // Rust uses match (similar to Python's match or switch) for error handling
    // This is like a try/except block in Python
    match QrCode::new(wifi_qr_data.as_bytes()) {
        // If QR code creation is successful
        Ok(code) => {
            // Render the QR code as a black and white image
            let image = code.render::<Luma<u8>>().build();
            
            // Try to save the image and handle potential errors
            match image.save("wifi_qr.png") {
                Ok(_) => println!("QR code saved"),
                Err(e) => println!("Error saving the QR code: {}", e),
            }
        },
        // If QR code creation fails
        Err(e) => {
            println!("Error creating QR code: {}", e);
        }
    }
}