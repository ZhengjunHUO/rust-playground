pub trait Inspect {
    //fn show_name(&self) -> String;
    fn show_name(&self) -> String {
        String::from("test")
    }

    fn info(&self) -> String {
        // default implementation
        format!("Show something for {}", self.show_name())
    }
}
