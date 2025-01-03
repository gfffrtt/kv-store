use std::string::FromUtf8Error;

/// This module contains the protocol definition for the KV-Store.
///
/// The protocol is defined as follows:
///
/// ```plaintext
/// +-----------------+----------------+----------------+
/// |     Command     |      Key       |     Value      |
/// +-----------------+----------------+----------------+
/// ```
///
/// * **command**: The command is a single byte that represents the operation to be performed.
/// It can be one of the following:
///     * *GET*: Retrieves the value of a key from the KV store.
///     * *SET*: Sets the value of a key in the KV store.
///     * *DELETE*: Deletes a key from the KV store.
///     * *EXISTS*: Checks if a key exists in the KV store.
///     * *KEYS*: Retrieves all keys from the KV store.
/// * **key**: The key is a string that represents the key of the KV pair.
/// * **value**: The value is a string that represents the value of the KV pair.
///
/// Caveats:
/// - The value should ALWAYS be a string representation of the data
/// - The value should only be specified for the SET command
/// - The key should be specified for all commands except KEYS
pub fn decode(buffer: [u8; 4096], size: usize) -> Command {
    let command = buffer[0];

    assert!(
        is_valid_command(command),
        "Trying to decode an invalid command"
    );

    match command {
        GET => {
            let key = String::from_utf8(buffer[1..size].to_vec());
            assert!(
                key.is_ok(),
                "Tried to call GET without a key, or with an invalid key"
            );
            Command::Get(key.unwrap())
        }
        SET => {
            let key_value = &buffer[1..size];
            let cmd = String::from_utf8(key_value.to_vec());
            assert!(
                cmd.is_ok(),
                "Tried to call SET without a key, or with an invalid key"
            );
            let parts = cmd
                .unwrap()
                .split(" ")
                .take(2)
                .map(|part| part.to_string())
                .collect::<Vec<String>>();
            Command::Set(parts[0].clone(), parts[1].clone())
        }
        DELETE => {
            let key = String::from_utf8(buffer[1..size].to_vec());
            assert!(
                key.is_ok(),
                "Tried to call DELETE without a key, or with an invalid key"
            );
            Command::Delete(key.unwrap())
        }
        EXISTS => {
            let key = String::from_utf8(buffer[1..size].to_vec());
            assert!(
                key.is_ok(),
                "Tried to call EXISTS without a key, or with an invalid key"
            );
            Command::Exists(key.unwrap())
        }
        KEYS => {
            assert!(size == 1, "Can't call EXISTS with a key or value");
            Command::Keys
        }
        _ => unreachable!(
            "Assert failed when validating that the command is valid and entered match statement"
        ),
    }
}

pub enum DecodeError {
    InvalidUtf8(FromUtf8Error),
}

pub enum Command {
    Get(String),
    Set(String, String),
    Delete(String),
    Exists(String),
    Keys,
}

pub const GET: u8 = b'0';
pub const SET: u8 = b'1';
pub const DELETE: u8 = b'2';
pub const EXISTS: u8 = b'3';
pub const KEYS: u8 = b'4';

pub fn is_valid_command(command: u8) -> bool {
    command >= GET && command <= KEYS
}

/// Encodes a response to be sent over the network.
/// The response is encoded as follows:
/// ```plaintext
/// +------------+------------------+
/// |    Code    |      Value       |
/// +------------+------------------+
/// ```
pub enum CommandResponse {
    Success(Vec<u8>),
    InternalError(Vec<u8>),
    NotFound,
}

const SUCCESS: u8 = b'0';
const INTERNAL_ERROR: u8 = b'1';
const NOT_FOUND: u8 = b'2';

impl CommandResponse {
    pub fn into_bytes(&self) -> Vec<u8> {
        match self {
            CommandResponse::Success(value) => {
                let mut bytes = vec![SUCCESS];
                bytes.extend(value);
                bytes
            }
            CommandResponse::InternalError(message) => {
                let mut bytes = vec![INTERNAL_ERROR];
                bytes.extend(message);
                bytes
            }
            CommandResponse::NotFound => vec![NOT_FOUND],
        }
    }
}
