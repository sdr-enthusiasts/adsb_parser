mod common;

use crate::common::{
    combine_files_of_message_type, test_enum_serialisation, test_value_serialisation,
    ContentDuplicator, MessageType, RunDurations, SerialisationTarget, SpeedTestComparisons,
    SpeedTestType, Stopwatch, StopwatchType,
};
use adsb_parser::{ADSBMessage, DecodeMessage};
use byte_unit::Byte;
use chrono::Utc;
use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;
use rand::thread_rng;
use rayon::prelude::*;
use serde_json::Value;
use std::error::Error;
use std::mem::size_of_val;
use std::sync::{Arc, Mutex, MutexGuard};
use thousands::Separable;

#[test]
#[ignore]
fn test_speed_large_queue() {
    large_queue_comparison(1.large_queue_library(), 1.large_queue_value());
    large_queue_comparison(1_000.large_queue_library(), 1_000.large_queue_value());
    large_queue_comparison(2_500.large_queue_library(), 2_500.large_queue_value());
    large_queue_comparison(5_000.large_queue_library(), 5_000.large_queue_value());
    large_queue_comparison(7_500.large_queue_library(), 7_500.large_queue_value());
    large_queue_comparison(10_000.large_queue_library(), 10_000.large_queue_value());
}
#[test]
#[ignore]
fn test_library_speed() {
    1.large_queue_library()
        .large_queue_duration(SpeedTestType::LargeQueueLibrary);
    1_000
        .large_queue_library()
        .large_queue_duration(SpeedTestType::LargeQueueLibrary);
    2_500
        .large_queue_library()
        .large_queue_duration(SpeedTestType::LargeQueueLibrary);
    5_000
        .large_queue_library()
        .large_queue_duration(SpeedTestType::LargeQueueLibrary);
    7_500
        .large_queue_library()
        .large_queue_duration(SpeedTestType::LargeQueueLibrary);
    10_000
        .large_queue_library()
        .large_queue_duration(SpeedTestType::LargeQueueLibrary);
    20_000
        .large_queue_library()
        .large_queue_duration(SpeedTestType::LargeQueueLibrary);
    30_000
        .large_queue_library()
        .large_queue_duration(SpeedTestType::LargeQueueLibrary);
    40_000
        .large_queue_library()
        .large_queue_duration(SpeedTestType::LargeQueueLibrary);
}

/// Trait for performing speed tests.
pub(crate) trait SpeedTest {
    fn large_queue_library(&self) -> Result<RunDurations, Box<dyn Error>>;
    fn large_queue_value(&self) -> Result<RunDurations, Box<dyn Error>>;
}

impl SpeedTest for i64 {
    fn large_queue_library(&self) -> Result<RunDurations, Box<dyn Error>> {
        println!(
            "\n{} => Starting a queue processing speed test using the library",
            Utc::now()
        );
        match combine_files_of_message_type(MessageType::All) {
            Err(load_error) => Err(load_error),
            Ok(all_messages) => {
                let mut run_durations: RunDurations = RunDurations::new();
                println!("{} => Loaded data successfully", Utc::now());
                let mut rng: ThreadRng = thread_rng();
                println!(
                    "{} => Duplicating content by {}",
                    Utc::now(),
                    self.separate_with_commas()
                );
                let mut test_message_queue: Vec<String> = all_messages.duplicate_contents(self);
                let queue_memory_size: Byte =
                    Byte::from_bytes(size_of_val(&*test_message_queue) as u128);
                run_durations.queue_memory_size = queue_memory_size;
                println!(
                    "{} => Content duplicated, queue contains {} messages ({})",
                    Utc::now(),
                    test_message_queue.len().separate_with_commas(),
                    queue_memory_size.get_appropriate_unit(false)
                );
                run_durations.run_processed_items = test_message_queue.len();
                let successfully_decoded_items: Arc<Mutex<Vec<ADSBMessage>>> =
                    Arc::new(Mutex::new(Vec::new()));
                println!("{} => Shuffling data order", Utc::now());
                test_message_queue.shuffle(&mut rng);
                println!("{} => Shuffling done, starting to process data", Utc::now());
                let mut total_run_stopwatch: Stopwatch = Stopwatch::start(StopwatchType::TotalRun);
                let mut deserialisation_run_stopwatch: Stopwatch =
                    Stopwatch::start(StopwatchType::LargeQueueDeser);
                test_message_queue.par_iter().for_each(|entry| {
                    if let Ok(decoded_message) = entry.decode_message() {
                        successfully_decoded_items
                            .lock()
                            .unwrap()
                            .push(decoded_message);
                    }
                });
                deserialisation_run_stopwatch.stop();
                let mut successfully_decoded_items_lock: MutexGuard<Vec<ADSBMessage>> =
                    successfully_decoded_items.lock().unwrap();
                run_durations.update_run_durations(&deserialisation_run_stopwatch);
                successfully_decoded_items_lock.shuffle(&mut rng);
                let mut serialisation_run_stopwatch: Stopwatch =
                    Stopwatch::start(StopwatchType::LargeQueueSer);
                successfully_decoded_items_lock
                    .par_iter()
                    .for_each(|message| {
                        test_enum_serialisation(message, SerialisationTarget::Both);
                    });
                serialisation_run_stopwatch.stop();
                total_run_stopwatch.stop();
                run_durations.update_run_durations(&serialisation_run_stopwatch);
                run_durations.update_run_durations(&total_run_stopwatch);
                println!(
                    "{} => Processing complete, building output content",
                    Utc::now()
                );
                Ok(run_durations)
            }
        }
    }

    fn large_queue_value(&self) -> Result<RunDurations, Box<dyn Error>> {
        println!(
            "{} => Starting a queue processing speed test using serde Value",
            Utc::now()
        );
        match combine_files_of_message_type(MessageType::All) {
            Err(load_error) => Err(load_error),
            Ok(all_messages) => {
                let mut run_durations: RunDurations = RunDurations::new();
                println!(
                    "{} => Loaded data successfully, retrieved {} items",
                    Utc::now(),
                    all_messages.len()
                );
                let mut rng: ThreadRng = thread_rng();
                println!(
                    "{} => Duplicating content by {}",
                    Utc::now(),
                    self.separate_with_commas()
                );
                let mut test_message_queue: Vec<String> = all_messages.duplicate_contents(self);
                let queue_memory_size: Byte =
                    Byte::from_bytes(size_of_val(&*test_message_queue) as u128);
                run_durations.queue_memory_size = queue_memory_size;
                println!(
                    "{} => Content duplicated, queue contains {} messages ({})",
                    Utc::now(),
                    test_message_queue.len().separate_with_commas(),
                    queue_memory_size.get_appropriate_unit(false)
                );
                run_durations.run_processed_items = test_message_queue.len();
                let successfully_decoded_items: Arc<Mutex<Vec<Value>>> =
                    Arc::new(Mutex::new(Vec::new()));
                println!("{} => Shuffling data order", Utc::now());
                test_message_queue.shuffle(&mut rng);
                println!("{} => Shuffling done, starting to process data", Utc::now());
                let mut total_run_stopwatch: Stopwatch = Stopwatch::start(StopwatchType::TotalRun);
                let mut deserialisation_run_stopwatch: Stopwatch =
                    Stopwatch::start(StopwatchType::LargeQueueDeser);
                test_message_queue.par_iter().for_each(|entry| {
                    if let Ok(decoded_message) = serde_json::from_str::<Value>(entry) {
                        successfully_decoded_items
                            .lock()
                            .unwrap()
                            .push(decoded_message);
                    }
                });
                deserialisation_run_stopwatch.stop();
                let mut successfully_decoded_items_lock: MutexGuard<Vec<Value>> =
                    successfully_decoded_items.lock().unwrap();
                run_durations.update_run_durations(&deserialisation_run_stopwatch);
                successfully_decoded_items_lock.shuffle(&mut rng);
                let mut serialisation_run_stopwatch: Stopwatch =
                    Stopwatch::start(StopwatchType::LargeQueueSer);
                successfully_decoded_items_lock
                    .par_iter()
                    .for_each(|message| {
                        test_value_serialisation(message, SerialisationTarget::Both);
                    });
                serialisation_run_stopwatch.stop();
                total_run_stopwatch.stop();
                run_durations.update_run_durations(&serialisation_run_stopwatch);
                run_durations.update_run_durations(&total_run_stopwatch);
                println!(
                    "{} => Processing complete, building output content",
                    Utc::now()
                );
                Ok(run_durations)
            }
        }
    }
}

fn large_queue_comparison(
    result_one: Result<RunDurations, Box<dyn Error>>,
    result_two: Result<RunDurations, Box<dyn Error>>,
) {
    match (result_one, result_two) {
        (Err(library_error), _) => println!("Library test had an error: {}", library_error),
        (_, Err(value_error)) => println!("Value test had an error: {}", value_error),
        (Ok(library), Ok(value)) => {
            let comparison: SpeedTestComparisons = SpeedTestComparisons {
                test_one_type: SpeedTestType::LargeQueueLibrary,
                test_one_results: library,
                test_two_type: SpeedTestType::LargeQueueValue,
                test_two_results: value,
            };
            comparison.compare_large_queue();
        }
    }
}

pub(crate) trait ProcessQueueResults {
    fn large_queue_duration(self, speed_test_type: SpeedTestType);
}

impl ProcessQueueResults for Result<RunDurations, Box<dyn Error>> {
    fn large_queue_duration(self, speed_test_type: SpeedTestType) {
        match self {
            Err(test_error) => println!("Library test had an error: {}", test_error),
            Ok(valid_test) => valid_test.display_run_duration(speed_test_type),
        }
    }
}
