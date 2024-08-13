use std::fs::File;
use std::io::{BufRead, BufReader};
use engine::analysis::{Analyzer, Normalizer, SimpleTokenCollector, Token, TokenFilter, Tokenizer};
use pizza_stemmers::{algorithms, StemmerTokenizer};

pub fn reduce_token_stream_to_string(token_stream: &[Token]) -> String {
    // Estimate the total length to minimize reallocations
    let total_len: usize = token_stream.iter().map(|token| token.term.len()).sum();

    // Create a `String` with a pre-allocated capacity
    let mut result = String::with_capacity(total_len + token_stream.len() - 1);

    for (i, token) in token_stream.iter().enumerate() {
        if i > 0 {
            result.push(' '); // Add space between tokens
        }
        result.push_str(&token.term);
    }

    result
}

pub fn analysis_and_check(my_tokenizer: Box<dyn Tokenizer>, input_file: File, output_file: File){
    let my_normalizers: Vec<Box<dyn Normalizer>> = vec![];
    let tokenizer = StemmerTokenizer::new(algorithms::english_porter);
    let my_token_filters: Vec<Box<dyn TokenFilter>> = vec![];
    let my_analyzer = Analyzer::new(my_normalizers, my_tokenizer, my_token_filters);

    let input_reader = BufReader::new(input_file);
    let output_reader = BufReader::new(output_file);

    let lines = input_reader.lines().zip(output_reader.lines());

    for (input, output) in lines {
        let input_str = input.unwrap();
        let expected_output = output.unwrap();


        // Use a `String` to accumulate results
        let mut results = String::new();

        // Define the closure to update `results` correctly
        let mut collector = SimpleTokenCollector::new(|tokens1: Vec<Token>| {
            let token_strings: Vec<String> = tokens1.into_iter()
                .map(|token| token.term.into())
                .collect();

            // Join the token strings with a space and append to `results`
            results.push_str(&token_strings.join(" "));
            results.push(' '); // Add space to separate tokens
        });

        // Perform analysis
        my_analyzer.analyze(&mut collector, input_str.clone().into());

        // Trim trailing space
        let trimmed_results = results.trim_end().to_string();

        // Print and assert
        // println!("input:{}, expect:{}, actual:{}",input_str,expected_output,trimmed_results);

        assert_eq!(trimmed_results, expected_output);
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use super::*;

    #[test]
    fn test_reduce_token_stream_to_string() {
        // Define some sample tokens
        let tokens = vec![
            Token {
                term: Cow::Borrowed("hello"),
                start_offset: 0,
                end_offset: 5,
                position: 0,
            },
            Token {
                term: Cow::Borrowed("world"),
                start_offset: 6,
                end_offset: 11,
                position: 1,
            },
            Token {
                term: Cow::Borrowed("rust"),
                start_offset: 12,
                end_offset: 16,
                position: 2,
            },
        ];

        // Expected result after reducing the token stream to a single string
        let expected = "hello world rust";

        // Call the function
        let result = reduce_token_stream_to_string(&tokens);

        // Assert that the result matches the expected output
        assert_eq!(result, expected);
    }
}
