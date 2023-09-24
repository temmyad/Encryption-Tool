use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use std::str;
use std::env;
use std::io::{stdin, stdout, Read, Write};

//Applying a pause to program
fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue to Decrypt...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn main() {

	let mut string = String::new();
        println!("What data do you wanna save? :");
        let _data = std::io::stdin().read_line(&mut string).unwrap();

	let args: Vec<String> = env::args().collect();
    if args.len() >1 { string = args[1].clone();}
	

    let mut rng = rand::thread_rng();
let bits = 1024;
let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
let pub_key = RsaPublicKey::from(&priv_key);

//println!("Message:\t{}",string);
println!("Number of bits:\t{}",bits);

let data = string.as_bytes();

//Encrypting the data and printing the progress

let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..]).expect("failed to encrypt");
assert_ne!(&data[..], &enc_data[..]);

    let hex_string = hex::encode(enc_data.clone());
    println!("Encrypting, please wait....");
    println!("\n\nEncrypted Data:\t{}",hex_string);
    println!("Encrypting complete");

    pause();

//Decrypting the data and printing the progress
let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &enc_data).expect("failed to decrypt");
assert_eq!(&data[..], &dec_data[..]);
    let mystr = str::from_utf8(&dec_data).unwrap();

    println!("Decrypting, please wait....");
    println!("\nDecrypted Data:\t{}",mystr);
    println!("Data Successfully decrypted!");

}
