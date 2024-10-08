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
use serenity::builder::CreateCommand;

pub fn run() -> String {
    "Mother Brain: Discord bot for kinda securely generating kinda secure \
    passwords. \
    Copyright (C) 2023-2024  Bolu <bolu@tuta.io>\n \
    \n\
    This program is free software: you can redistribute it and/or modify \
    it under the terms of the GNU Affero General Public License as published \
    by the Free Software Foundation, either version 3 of the License, or \
    (at your option) any later version.\n\
    \n\
    This program is distributed in the hope that it will be useful, \
    but WITHOUT ANY WARRANTY; without even the implied warranty of \
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the \
    GNU Affero General Public License for more details.\n\
    \n\
    You should have received a copy of the GNU Affero General Public License \
    along with this program. If not, see <https://www.gnu.org/licenses/>.\n"
        .to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("license").description("Show the software license for this bot.")
}
