mod common;

use adsb_parser::adsb_json::{ADSBJsonMessage, NewADSBJsonMessage};
use std::error::Error;

use crate::common::{combine_files_of_message_type, compare_errors, MessageType};

/// This test will ingest contents from the adsb json sample files as a message per line to a `Vec<String>`.
/// It combines the two files together into a single `Vec<String>` for iterating through.
/// Then it will cycle them into `Vec<ADSBMessage>` and back to `String`.
/// It validates that there are no errors going `String` -> `ADSBMessage` and `ADSBMessage` -> `String`.
#[test]
fn test_adsb_parsing() -> Result<(), Box<dyn Error>> {
    match combine_files_of_message_type(MessageType::ADSBJson) {
        Err(load_failed) => Err(load_failed),
        Ok(adsb_messages) => {
            let mut valid_adsb_messages: Vec<ADSBJsonMessage> = Vec::new();
            let mut failed_decodes: Vec<String> = Vec::new();
            for line in adsb_messages {
                match line.to_adsb() {
                    Err(_) => failed_decodes.push(line),
                    Ok(adsb_message) => valid_adsb_messages.push(adsb_message),
                }
            }
            println!("Size of bad messages: {}", failed_decodes.len());
            for message in valid_adsb_messages {
                assert!(message.to_string().as_ref().err().is_none());
                assert!(message.to_bytes().as_ref().err().is_none());
            }
            for line in failed_decodes {
                compare_errors(line.to_adsb().err(), serde_json::from_str(&line), &line);
            }
            Ok(())
        }
    }
}
