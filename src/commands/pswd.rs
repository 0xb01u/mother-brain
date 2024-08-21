/*
 *  Mother Brain: Discord bot for kinda securely generating kinda secure
 *  passwords.
 *  Copyright (C) 2023-2024  Bolu <bolu@tuta.io>
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Affero General Public License as published
 *  by the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 *  GNU Affero General Public License for more details.
 *
 *  You should have received a copy of the GNU Affero General Public License
 *  along with this program. If not, see <https://www.gnu.org/licenses/>.
 */
extern crate rand;
extern crate rand_chacha;
extern crate rs_sha512;

use openssl::symm::{decrypt, encrypt, Cipher};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rs_sha512::Sha512State;
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};
use std::{
    fs::{read_to_string, File},
    hash::{BuildHasher, Hash, Hasher},
    io::Read,
};

pub fn run(options: &[ResolvedOption]) -> String {
    // Default to 6 words for the password:
    let num_words: u32;
    if let Some(ResolvedOption {
        value: ResolvedValue::Integer(nw),
        ..
    }) = options.get(2)
    {
        num_words = *nw as u32;
    } else {
        num_words = 6;
    }

    // Extract password hint (used to generate the password):
    let ResolvedOption {
        value: ResolvedValue::String(w),
        ..
    } = options.first().unwrap()
    else {
        panic!("pswd command's hint is not a string (this message should never be printed).");
    };
    let what = (*w).to_string();

    // Extract secret key (used to aes256-decrypt initial data):
    let ResolvedOption {
        value: ResolvedValue::String(sk),
        ..
    } = options.get(1).unwrap()
    else {
        panic!(
            "pswd command's secret words are not a string (this message should never be printed)."
        );
    };
    let secret_key = &[(*sk).as_bytes(), &[0x62; 32]].concat()[0..32]; // Pad with 'b's until length 32
                                                                       // (arbitrary character that can be typed as text).

    // Load list of words to compose the password:
    let words: Vec<String> = read_to_string("wordlist.txt")
        .expect("Could not open word-list file.")
        .lines()
        .map(String::from)
        .collect();

    // Retrieve key used to encrypt the password, aes256-encryted:
    let mut secret_file = File::open("secret256.dat").expect("Could not open secret256.dat.");
    let mut secret = Vec::new();
    secret_file
        .read_to_end(&mut secret)
        .expect("Could not read secret256.dat.");

    // Decrypt the key used to encrypt the password, using the secret key provided by the user:
    let pswd_key = match decrypt(Cipher::aes_256_cbc(), secret_key, None, &*secret) {
        Ok(data) => data,
        // On error (incorrect secret_key provided) use a fake key not to reveal the
        // provided key was incorrect:
        Err(_err) => [b"Nice try", secret_key, &[0; 24]].concat()[0..32].to_vec(),
    };

    // Generate RNG seed from user-provided data:
    let mut hasher = Sha512State::default().build_hasher();
    // FIXME: The slice hotfix is for "legacy" compatibility reasons:
    pswd_key[4..20].hash(&mut hasher); // Feed the password key as PRNG seed.
    what.hash(&mut hasher); // Feed password hint.
    num_words.hash(&mut hasher); // Feed password strength
                                 // (avoids lower-strength passwords being prefixes
                                 // of higher-strength ones for the same hint).
    let seed = hasher.finish();

    // Get pseudo-random password from the list of words:
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let mut pswd = "".to_owned();

    for _ in 0..num_words {
        pswd.push_str(&format!("{} ", words[rng.gen_range(0..words.len())]));
    }
    // Pad result with spaces to 512 characters
    // (so the ciphertext provides no information is given on the length of the decoded password's words):
    pswd.push_str(&String::from_iter([' '; 512]));
    pswd = pswd[0..512].to_string();

    // Generate random initiation vector:
    let iv_str = format!("{:016x}", rng.gen::<u64>());
    let iv = iv_str.as_bytes();

    // Encrypt the resposne (list of words composing the password) using the pswd_key as key,
    // and the pseudo-randomly-generated initiation vector.
    let mut encrypted_data = encrypt(Cipher::aes_256_cbc(), &pswd_key, Some(iv), pswd.as_bytes())
        .expect("AES encryption failed for the password.")
        .into_iter()
        .map(|b| format!("{:02x}", b)) // Format as hex string.
        .collect::<String>();
    encrypted_data.push_str(&iv_str); // Send IV together with encrypted data.
    encrypted_data
}

pub fn register() -> CreateCommand {
    CreateCommand::new("pswd").description("Generate/retrieve password.")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "tip", "A tip about the password you want to generate/retrieve.")
                .required(true))
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "secret_words", "The secret words to make me do work.")
                .required(true))
        .add_option(
            CreateCommandOption::new(CommandOptionType::Integer, "strength", "Strength of the generated password. Valid values are between 1 and 10 (included). Default is 6.")
                .min_int_value(1).max_int_value(10)
                .required(false)
        )
}
