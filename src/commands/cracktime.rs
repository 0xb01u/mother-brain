/*
 *  Mother Brain: Discord bot for kinda securely generating kinda secure
 *  passwords.
 *  Copyright (C) 2023-2024  Bolu <bolu@tuta.io>
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Affero General Public License as published
 *  by the Free Software Foundation, either version of the License, or
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

use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};
use std::fs::read_to_string;

// Some interesting pages:
//  * https://pmdn.org/password-strength/
//  * https://alecmccutcheon.github.io/Password-Entropy-Calculator/
const TWO_BILLION: f64 = 2e9;
const ONE_TRILLION: f64 = 1e12;
const POWERFUL_MINER: f64 = 390e12; // MicroBT Whatsminer MS Hydro90T ($13,699).
const MILLION_DOLLAR_MINER: f64 = 132.0 * 335e12; // Bitmain Antminer S21 Hydro ($7,599).
const SHA256_TO_BCRYPT5_TIME: f64 = 8.81e6 / (1.0 / (0.0024385929107666 / 4.0));
// Sources for the previous estimation ^^^:
//  * https://wildlyinaccurate.com/bcrypt-choosing-a-work-factor/ (Intel i3-2120 (Quad Core, 3.30GHz))
//    (assumed only one core used).
//  * https://en.bitcoin.it/wiki/Non-specialized_hardware_comparison#CPUs.2FAPUs (Core i3-2100
//    @ 3.10 GHz, extrapolated to0 GHz).
const HOURS_TO_SECONDS: f64 = 3600.0;
const DAYS_TO_SECONDS: f64 = 3600.0 * 24.0;
const YEARS_TO_SECONDS: f64 = 3600.0 * 24.0 * 365.0;

pub fn run(options: &[ResolvedOption]) -> String {
    let mut response = "".to_owned();

    // Get the number of words (strength) used for the password:
    let num_words: i32;
    if let Some(ResolvedOption {
        value: ResolvedValue::Integer(nw),
        ..
    }) = options.first()
    {
        num_words = *nw as i32;
    } else {
        // Default to 6 words for the password:
        num_words = 6;
    }

    // Load list of words to compose the password:
    let dict_size = read_to_string("wordlist.txt")
        .expect("Could not open word-list file.")
        .lines()
        .count() as f64;

    // Compute number of possible passwords:
    let num_options = dict_size.powi(num_words);

    // Average attempts to find the password:
    let avg_crack_attempts = num_options / 2.0;

    // Estimate crack time:
    response.push_str(&format!(
        "Number of possible options/combinations: {:.3e}\n",
        num_options
    ));
    response.push_str(&format!(
        "Password entropy: {:.3} bits\n",
        num_options.log(2.0)
    ));
    response.push_str("\n**Assuming SHA-256 hashes:**\n");
    let guesses_two_billion_gps = avg_crack_attempts / TWO_BILLION;
    // https://www.password-depot.de/en/know-how/brute-force-attacks.htm VVV
    response.push_str(&format!("With 2 billion guesses/s (\"a very strong single computer\" in 2012): {:.3e} seconds, {:.3e} hours, {:.3e} days, {:.3e} years.\n",
                              guesses_two_billion_gps, guesses_two_billion_gps / HOURS_TO_SECONDS,
                              guesses_two_billion_gps / DAYS_TO_SECONDS, guesses_two_billion_gps / YEARS_TO_SECONDS));
    let guesses_one_trillion_gps = avg_crack_attempts / ONE_TRILLION;
    response.push_str(&format!(
        "With 1 trillion guesses/s: {:.3e} seconds, {:.3e} hours, {:.3e} days, {:.3e} years.\n",
        guesses_one_trillion_gps,
        guesses_one_trillion_gps / HOURS_TO_SECONDS,
        guesses_one_trillion_gps / DAYS_TO_SECONDS,
        guesses_one_trillion_gps / YEARS_TO_SECONDS
    ));
    let guesses_powerful_miner_gps = avg_crack_attempts / POWERFUL_MINER;
    response.push_str(&format!("With the equivalent of a powerful bitcoin miner: {:.3e} seconds, {:.3e} hours, {:.3e} days, {:.3e} years.\n",
                              guesses_powerful_miner_gps, guesses_powerful_miner_gps / HOURS_TO_SECONDS,
                              guesses_powerful_miner_gps / DAYS_TO_SECONDS, guesses_powerful_miner_gps / YEARS_TO_SECONDS));
    let guesses_million_dollar_miner_gps = avg_crack_attempts / MILLION_DOLLAR_MINER;
    response.push_str(&format!("With the equivalent of $1M worth of bitcoin miners (cost efficient): {:.3e} seconds, {:.3e} hours, {:.3e} days, {:.3e} years.\n",
                              guesses_million_dollar_miner_gps, guesses_million_dollar_miner_gps / HOURS_TO_SECONDS,
                              guesses_million_dollar_miner_gps / DAYS_TO_SECONDS, guesses_million_dollar_miner_gps / YEARS_TO_SECONDS));
    response.push_str("\n**Assuming Bcypt hashes of work factor 5 (the minimum recommended):**\n");
    let guesses_two_billion_gps = avg_crack_attempts / (TWO_BILLION / SHA256_TO_BCRYPT5_TIME);
    response.push_str(&format!("With 2 billion guesses/s (\"a very strong single computer\" in 2012): {:.3e} seconds, {:.3e} hours, {:.3e} days, {:.3e} years.\n",
                              guesses_two_billion_gps, guesses_two_billion_gps / HOURS_TO_SECONDS,
                              guesses_two_billion_gps / DAYS_TO_SECONDS, guesses_two_billion_gps / YEARS_TO_SECONDS));
    let guesses_one_trillion_gps = avg_crack_attempts / (ONE_TRILLION / SHA256_TO_BCRYPT5_TIME);
    response.push_str(&format!(
        "With 1 trillion guesses/s: {:.3e} seconds, {:.3e} hours, {:.3e} days, {:.3e} years.\n",
        guesses_one_trillion_gps,
        guesses_one_trillion_gps / HOURS_TO_SECONDS,
        guesses_one_trillion_gps / DAYS_TO_SECONDS,
        guesses_one_trillion_gps / YEARS_TO_SECONDS
    ));
    let guesses_powerful_miner_gps = avg_crack_attempts / (POWERFUL_MINER / SHA256_TO_BCRYPT5_TIME);
    response.push_str(&format!("With the equivalent of a powerful bitcoin miner: {:.3e} seconds, {:.3e} hours, {:.3e} days, {:.3e} years.\n",
                              guesses_powerful_miner_gps, guesses_powerful_miner_gps / HOURS_TO_SECONDS,
                              guesses_powerful_miner_gps / DAYS_TO_SECONDS, guesses_powerful_miner_gps / YEARS_TO_SECONDS));
    let guesses_million_dollar_miner_gps =
        avg_crack_attempts / (MILLION_DOLLAR_MINER / SHA256_TO_BCRYPT5_TIME);
    response.push_str(&format!("With the equivalent of $1M worth of bitcoin miners (cost efficient): {:.3e} seconds, {:.3e} hours, {:.3e} days, {:.3e} years.\n",
                              guesses_million_dollar_miner_gps, guesses_million_dollar_miner_gps / HOURS_TO_SECONDS,
                              guesses_million_dollar_miner_gps / DAYS_TO_SECONDS, guesses_million_dollar_miner_gps / YEARS_TO_SECONDS));

    response
}

pub fn register() -> CreateCommand {
    CreateCommand::new("cracktime").description("Give an estimate for the average time needed to crack a generated password.")
        .add_option(
            CreateCommandOption::new(CommandOptionType::Integer, "strength", "Strength of the password to crack. Valid values are between 1 and 50 (included). Default is 6.")
                .min_int_value(1).max_int_value(50)
                .required(false)
        )
}
