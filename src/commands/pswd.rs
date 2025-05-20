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

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rs_sha512::Sha512State;
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};
use std::{
    fs::read_to_string,
    hash::{BuildHasher, Hash, Hasher},
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

    // Load list of words to compose the password:
    let words: Vec<String> = read_to_string("wordlist.txt")
        .expect("Could not open word-list file.")
        .lines()
        .map(String::from)
        .collect();

    // Generate RNG seed from user-provided data:
    let mut hasher = Sha512State::default().build_hasher();
    what.hash(&mut hasher); // Feed password hint.
    num_words.hash(&mut hasher); // Feed password strength
                                 // (avoids lower-strength passwords being prefixes
                                 // of higher-strength ones for the same hint).
    let seed = hasher.finish();

    // Get pseudo-random password from the list of words:
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let mut pswd = "".to_owned();

    for _ in 0..num_words {
        pswd.push_str(&format!("{}-", words[rng.gen_range(0..words.len())]));
    }
    pswd.pop();

    pswd
}

pub fn register() -> CreateCommand {
    CreateCommand::new("pswd").description("Generate/retrieve password.")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "tip", "A tip about the password you want to generate/retrieve.")
                .required(true))
        .add_option(
            CreateCommandOption::new(CommandOptionType::Integer, "strength", "Strength of the generated password. Valid values are between 1 and 10 (included). Default is 6.")
                .min_int_value(1).max_int_value(10)
                .required(false)
        )
}
