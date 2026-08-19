pub mod error;
pub mod flags;
pub mod args;
pub mod search;
pub mod output;
pub mod file_ops;

use error::Result;
use args::parse_args;
use file_ops::expand_file_patterns;
use search::{SearchConfig, search};
use output::{OutputConfig, OutputMode, format_match, format_count};
use file_ops::read_file_contents;
use std::path::PathBuf;
use rayon::prelude::*;
use std::sync::Mutex;

pub fn run(args: Vec<String>) -> Result<()> {
    let parsed_args = parse_args(&args)?;
    
    let file_paths = expand_file_patterns(&parsed_args.file_patterns)?;
    
    let search_config = SearchConfig::from_flags(&parsed_args.flags);
    let output_config = OutputConfig::new(&parsed_args.flags, file_paths.len() > 1);
    
    if output_config.verbose {
        eprintln!("Processing {} file(s)", file_paths.len());
        eprintln!("---");
    }
    
    // Use a mutex to ensure thread-safe output
    let output_mutex = Mutex::new(());
    
    // Process files in parallel using rayon
    file_paths
        .par_iter()
        .try_for_each(|path| {
            process_file_parallel(path, &parsed_args.pattern, &search_config, &output_config, &output_mutex)
        })?;
    
    Ok(())
}

fn process_file(
    path: &PathBuf,
    pattern: &str,
    search_config: &SearchConfig,
    output_config: &OutputConfig,
) -> Result<()> {
    let filename = path.to_string_lossy();
    
    if output_config.verbose {
        eprintln!("Searching in file: {}", filename);
        eprintln!("Pattern: \"{}\"", pattern);
        eprintln!("Search options: case_insensitive={}, invert_match={}", 
                  search_config.case_insensitive, 
                  search_config.invert_match);
    }
    
    let contents = read_file_contents(path)?;
    
    let search_result = search(&contents, pattern, search_config);
    
    if output_config.verbose {
        eprintln!("Found {} matches", search_result.total_count);
    }
    
    match output_config.mode {
        OutputMode::Count => {
            let output = format_count(
                search_result.total_count,
                Some(&filename),
                output_config.show_filename
            );
            println!("{}", output);
        }
        OutputMode::FullLines => {
            for matched_line in search_result.matches {
                let output = format_match(
                    &matched_line.content,
                    matched_line.line_number,
                    Some(&filename),
                    output_config
                );
                println!("{}", output);
            }
        }
    }
    
    Ok(())
}

fn process_file_parallel(
    path: &PathBuf,
    pattern: &str,
    search_config: &SearchConfig,
    output_config: &OutputConfig,
    output_mutex: &Mutex<()>,
) -> Result<()> {
    let filename = path.to_string_lossy();
    
    // Acquire lock for verbose output
    if output_config.verbose {
        let _lock = output_mutex.lock().unwrap();
        eprintln!("Searching in file: {}", filename);
        eprintln!("Pattern: \"{}\"", pattern);
        eprintln!("Search options: case_insensitive={}, invert_match={}", 
                  search_config.case_insensitive, 
                  search_config.invert_match);
    }
    
    let contents = read_file_contents(path)?;
    
    let search_result = search(&contents, pattern, search_config);
    
    // Acquire lock for verbose output
    if output_config.verbose {
        let _lock = output_mutex.lock().unwrap();
        eprintln!("Found {} matches", search_result.total_count);
    }
    
    // Acquire lock for actual output to ensure thread-safe printing
    let _lock = output_mutex.lock().unwrap();
    
    match output_config.mode {
        OutputMode::Count => {
            let output = format_count(
                search_result.total_count,
                Some(&filename),
                output_config.show_filename
            );
            println!("{}", output);
        }
        OutputMode::FullLines => {
            for matched_line in search_result.matches {
                let output = format_match(
                    &matched_line.content,
                    matched_line.line_number,
                    Some(&filename),
                    output_config
                );
                println!("{}", output);
            }
        }
    }
    
    Ok(())
} 