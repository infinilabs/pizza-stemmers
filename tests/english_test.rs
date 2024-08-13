#[cfg(test)]
mod tests {
    use std::fs::File;
    use engine::analysis::Tokenizer;
    use pizza_stemmers::{algorithms, StemmerTokenizer};
    use test_utils;
    use test_utils::analysis_and_check;

    #[test]
    fn it_stems_using_porter() {
        let my_tokenizer: Box<dyn Tokenizer> = Box::new(StemmerTokenizer::new(algorithms::english_porter));
        let input_file = File::open("./tests/assets/en/porter.input.txt").unwrap();
        let output_file = File::open("./tests/assets/en/porter.output.txt").unwrap();
        analysis_and_check(my_tokenizer, input_file, output_file);
    }

    #[test]
    fn it_stems_using_porter_2() {
        let my_tokenizer: Box<dyn Tokenizer> = Box::new(StemmerTokenizer::new(algorithms::english_porter_2));
        let input_file = File::open("./tests/assets/en/porter_2.input.txt").unwrap();
        let output_file = File::open("./tests/assets/en/porter_2.output.txt").unwrap();
        analysis_and_check(my_tokenizer, input_file, output_file);
    }
}
