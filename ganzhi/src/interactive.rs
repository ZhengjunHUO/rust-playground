use inquire::autocompletion::{Autocomplete, Replacement};
use inquire::error::CustomUserError;

#[derive(Clone)]
pub struct HourCompleter {
    pub hours: Vec<String>,
}

impl HourCompleter {
    fn filter_candidates(&self, input: &str) -> Vec<String> {
        let pattern = input.to_lowercase();

        self.hours
            .clone()
            .into_iter()
            .filter(|s| s.starts_with(&pattern))
            .collect()
    }
}

impl Autocomplete for HourCompleter {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, CustomUserError> {
        Ok(self.filter_candidates(input))
    }

    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<Replacement, CustomUserError> {
        Ok(match highlighted_suggestion {
            Some(suggestion) => Replacement::Some(suggestion),
            None => {
                let list = self.filter_candidates(input);
                if list.len() == 0 {
                    Replacement::None
                } else {
                    Replacement::Some(list[0].clone())
                }
            }
        })
    }
}
