
#[cfg(test)]
mod tests {
    use std::{fs::File, io::{BufRead, BufReader}};
    use engine::analysis::Tokenizer;
    use pizza_stemmers::{algorithms, StemmerTokenizer};

    use test_utils;
    use test_utils::analysis_and_check;

    #[test]
    fn it_stems_using_german() {

        let my_tokenizer: Box<dyn Tokenizer> = Box::new(StemmerTokenizer::new(algorithms::german));
        let input_file = File::open("./tests/assets/de/german.input.txt").unwrap();
        let output_file = File::open("./tests/assets/de/german.output.txt").unwrap();
        analysis_and_check(my_tokenizer, input_file, output_file);

    }
}
