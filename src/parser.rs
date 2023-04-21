pub trait FromFile<T> {
    fn from_file(filename: String) -> Result<T, Box<dyn std::error::Error>>;
}