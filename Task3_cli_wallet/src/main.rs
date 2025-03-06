use secp256k1::{Secp256k1, SecretKey, PublicKey, Message, ecdsa::Signature};
use rand::RngCore;
use hex;
use std::io; // For reading user input

struct Wallet {
    secret_key: SecretKey,
    public_key: PublicKey,
}

impl Wallet {
    fn new() -> Self {
        let secp = Secp256k1::new();
        let mut rng = rand::thread_rng();
        let mut secret_bytes = [0u8; 32];
        rng.fill_bytes(&mut secret_bytes);

        let secret_key = SecretKey::from_slice(&secret_bytes).expect("Invalid private key");
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);

        Wallet {
            secret_key,
            public_key,
        }
    }

    fn public_key(&self) -> String {
        hex::encode(self.public_key.serialize_uncompressed())
    }

    fn sign_message(&self, message: &str) -> String {
        let secp = Secp256k1::new();
        let mut message_bytes = [0u8; 32];
        let input_bytes = message.as_bytes();
        for i in 0..input_bytes.len().min(32) {
            message_bytes[i] = input_bytes[i];
        }
        let msg = Message::from_slice(&message_bytes).expect("32 bytes");
        let signature = secp.sign_ecdsa(&msg, &self.secret_key);
        hex::encode(signature.serialize_der())
    }

    fn verify_message(&self, message: &str, signature_hex: &str) -> bool {
        let secp = Secp256k1::new();
        let signature_bytes = match hex::decode(signature_hex) {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };
        let signature = match Signature::from_der(&signature_bytes) {
            Ok(sig) => sig,
            Err(_) => return false,
        };
        let mut message_bytes = [0u8; 32];
        let input_bytes = message.as_bytes();
        for i in 0..input_bytes.len().min(32) {
            message_bytes[i] = input_bytes[i];
        }
        let msg = Message::from_slice(&message_bytes).expect("32 bytes");
        secp.verify_ecdsa(&msg, &signature, &self.public_key).is_ok()
    }
}

fn main() {
    println!("Welcome to the CLI Wallet!");
    println!("Commands: create, public, sign <message>, verify <message> <signature>, exit");

    let mut wallet: Option<Wallet> = None; // Store wallet, starts as None

    loop {
        println!("\nEnter a command:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim(); // Remove trailing newline

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            println!("Empty command, try again.");
            continue;
        }

        match parts[0].to_lowercase().as_str() {
            "create" => {
                wallet = Some(Wallet::new());
                println!("New wallet created!");
            }
            "public" => {
                if let Some(ref w) = wallet {
                    println!("Public Key: {}", w.public_key());
                } else {
                    println!("No wallet created yet. Use 'create' first.");
                }
            }
            "sign" => {
                if let Some(ref w) = wallet {
                    if parts.len() > 1 {
                        let message = parts[1..].join(" ");
                        let signature = w.sign_message(&message);
                        println!("Signature: {}", signature);
                    } else {
                        println!("Please provide a message to sign (e.g., 'sign Hello world').");
                    }
                } else {
                    println!("No wallet created yet. Use 'create' first.");
                }
            }
            "verify" => {
                if let Some(ref w) = wallet {
                    if parts.len() >= 3 {
                        let message = parts[1..parts.len() - 1].join(" ");
                        let signature = parts[parts.len() - 1];
                        let is_valid = w.verify_message(&message, signature);
                        println!("Signature Valid: {}", is_valid);
                    } else {
                        println!("Please provide a message and signature (e.g., 'verify Hello world <signature>').");
                    }
                } else {
                    println!("No wallet created yet. Use 'create' first.");
                }
            }
            "exit" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Unknown command. Use: create, public, sign <message>, verify <message> <signature>, exit"),
        }
    }
}