use rand::Rng;              // For generating random numbers 
use qrcode::QrCode;         // For creating QR codes
use image::Luma;            // For handling the image format (black and white in this case)
use colored::Colorize;
use eframe::egui;

struct VentanaSimple {
    texto_ingresado: String,
    password_length: String,
    network_name: String,
    password: String,
    generated_password: String,

}

impl VentanaSimple {
    fn new() -> Self {
        Self {
            texto_ingresado: String::new(),
            password_length: String::new(),
            network_name: String::new(),
            password: String::new(),
            generated_password: String::new(),

        }
    }

    fn link_qr_code(&self) {
    let link = self.texto_ingresado.trim();

        match QrCode::new(link.as_bytes()) {
            Ok(code) => {
                let image = code.render::<Luma<u8>>().build();
                match image.save("link_qr.png") {
                    Ok(_) => println!("{}","QR code saved".green()),
                    Err(e) =>{ 
                        let error = format!("Error saving the QR code: {}", e);
                        println!("{}",error.red());
                    } 
                }
            },
            Err(e) => {
                let error = format!("Error creating QR code: {}", e);
                println!("{}",error.red());
            }
        }
    }

    fn wifi_qr_code(&self) {
    let wifi_qr_data = format!("WIFI:S:{};T:WPA;P:{};H:false;;",self.network_name.trim(), self.password.trim());

        match QrCode::new(wifi_qr_data.as_bytes()) {
            Ok(code) => {
                let image = code.render::<Luma<u8>>().build();
                match image.save("wifi_qr.png") {
                    Ok(_) => println!("{}","QR code saved".green()),
                    Err(e) =>{ 
                        let error = format!("Error saving the QR code: {}", e);
                        println!("{}",error.red());
                    } 
                }
            },
            Err(e) => {
                let error = format!("Error creating QR code: {}", e);
                println!("{}",error.red());
            }
        }
    }
    fn password_generator(&mut self){
        let password_length: usize = self.password_length.trim().parse().expect("Enter a valid input.");
        // Create a vector (like a Python list) of characters to use in the password
        let characters: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_=+".chars().collect();
        let characters_length = characters.len();

        // Create a random number generator (RNG) a machine that creates a random number each time you call it
        #[allow(deprecated)]
        let mut rng = rand::thread_rng();
        
        // Create a string to hold the password with pre-allocated capacity
        let mut password = String::with_capacity(password_length);

        // Loop to generate each character of the password
        for _ in 0..password_length {
            // Generate a random index between 0 and characters_length-1
            let index = rng.random_range(0..characters_length);
            
            // Get the character at that index
            let character = characters[index];
            
            // Add the character to our password string
            password.push(character);
        }
        self.generated_password = password;
    }
}

impl eframe::App for VentanaSimple {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Panel izquierdo (todo tu contenido aquí)
        egui::SidePanel::left("left_panel")
            .resizable(false) // Para que no se pueda redimensionar
            .default_width(300.0) // Ancho fijo
            .show(ctx, |ui| {   
                ui.heading("Enter your text here");
                ui.text_edit_singleline(&mut self.texto_ingresado);

                if ui.button("Create QR Code").clicked() {
                    self.link_qr_code();
                }

                ui.separator();
                ui.add_space(20.0);

                ui.heading("Enter your network name");
                ui.text_edit_singleline(&mut self.network_name);

                ui.heading("Enter your password");
                ui.text_edit_singleline(&mut self.password);

                // Botón 2: Genera wifi Qr code
                if ui.button("Create QR Code").clicked() {
                    self.wifi_qr_code();
                }

                ui.separator();  // Línea divisoria
                ui.add_space(20.0); // 20 píxeles de separación

                ui.heading("Enter your password length");
                ui.text_edit_singleline(&mut self.password_length);

                // Botón 3: Genera password random
                if ui.button("Generate password").clicked() {
                    self.password_generator();
                }

                // Mostrar el texto actual debajo
                ui.label("Password generated:");
                ui.label(&self.generated_password);
            });
            
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "RUSTYBITE",
        options,
        Box::new(|_cc| Ok(Box::new(VentanaSimple::new()))),
    )
}