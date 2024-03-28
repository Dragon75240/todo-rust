pub trait Option {
    fn exec(&self, args: &[String]) -> Result<(), Box<dyn std::error::Error>>;
}