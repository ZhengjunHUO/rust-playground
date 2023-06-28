use inquire::autocompletion::{Autocomplete, Replacement};
use inquire::error::CustomUserError;
use inquire::validator::Validation;
use inquire::Text;

fn main() {
    let validator = |input: &str| {
        if input.chars().count() > 30 {
            Ok(Validation::Invalid("Please keep the answer short.".into()))
        } else {
            Ok(Validation::Valid)
        }
    };

    let help = "Be honest, pls.";
    let ac = EmotionCompleter {
        feelings: vec![
            "accepting",
            "adventurous",
            "affectionate",
            "afraid",
            "aggravated",
            "agitated",
            "aloof",
            "amazed",
            "angry",
            "anguish",
            "annoyed",
            "anxious",
            "appreciative",
            "apprehensive",
            "ashamed",
            "awe",
            "bitter",
            "blessed",
            "bliss",
            "bored",
            "brave",
            "burned out",
            "calm",
            "capable",
            "caring",
            "centered",
            "compassion",
            "concerned",
            "confident",
            "confused",
            "connected",
            "contempt",
            "content",
            "courageous",
            "cranky",
            "curious",
            "cynical",
            "daring",
            "delighted",
            "depleted",
            "depressed",
            "despondent",
            "determined",
            "disappointed",
            "disconnected",
            "discouraged",
            "disdain",
            "disgruntled",
            "dissatisfied",
            "distant",
            "disturbed",
            "doubt",
            "eager",
            "ecstatic",
            "edgy",
            "embarrassed",
            "empathy",
            "empty",
            "enchanted",
            "encouraged",
            "energized",
            "engaged",
            "enthusiastic",
            "exasperated",
            "excited",
            "exhausted",
            "expectant",
            "exploring",
            "fascinated",
            "fear",
            "forlorn",
            "fortunate",
            "fragile",
            "frazzled",
            "free",
            "frightened",
            "frustrated",
            "fulfilled",
            "furious",
            "gloomy",
            "grace",
            "grateful",
            "grief",
            "grouchy",
            "grounded",
            "guilt",
            "happy",
            "heartbroken",
            "helpless",
            "hesitant",
            "hopeful",
            "hopeless",
            "hostile",
            "humbled",
            "humiliated",
            "impatient",
            "impotent",
            "incapable",
            "indifferent",
            "inhibited",
            "inspired",
            "interested",
            "intrigued",
            "invigorated",
            "involved",
            "irate",
            "irritated",
            "isolated",
            "joy",
            "lethargic",
            "listless",
            "lively",
            "lonely",
            "longing",
            "loving",
            "lucky",
            "melancholy",
            "moody",
            "mortified",
            "moved",
            "nervous",
            "on edge",
            "open",
            "optimistic",
            "outraged",
            "overwhelm",
            "panic",
            "paralyzed",
            "passionate",
            "patient",
            "peaceful",
            "perplexed",
            "pissed",
            "playful",
            "powerful",
            "powerless",
            "present",
            "proud",
            "questioning",
            "radiant",
            "rattled",
            "reflective",
            "refreshed",
            "regret",
            "rejecting",
            "rejuvenated",
            "relaxed",
            "reluctant",
            "remorseful",
            "removed",
            "renewed",
            "resentful",
            "resigned",
            "resistant",
            "restless",
            "rusty",
            "sad",
            "safe",
            "satisfied",
            "scared",
            "self-conscious",
            "self-loving",
            "sensitive",
            "serene",
            "shaken",
            "shame",
            "shocked",
            "shut down",
            "skeptical",
            "sorrow",
            "sorry",
            "stimulated",
            "stressed",
            "strong",
            "suspicious",
            "teary",
            "tender",
            "tense",
            "terrified",
            "thankful",
            "thrilled",
            "tight",
            "touched",
            "trapped",
            "trusting",
            "uneasy",
            "ungrounded",
            "unhappy",
            "unsure",
            "upset",
            "useless",
            "valiant",
            "vibrant",
            "victim",
            "vindictive",
            "vulnerable",
            "warm",
            "weak",
            "weary",
            "withdrawn",
            "worn out",
            "worried",
            "worthless",
            "worthy",
            "yearning",
        ], //.iter().map(|&s| s.into()).collect(),
    };

    let resp = Text::new("How do you feel now ?")
        .with_initial_value("rusty")
        .with_help_message(&help)
        .with_validator(validator)
        .with_autocomplete(ac)
        .with_page_size(10)
        .prompt();

    match resp {
        Ok(feeling) => println!("You feel {}", feeling),
        Err(err) => println!("Error retrieving your response: {}", err),
    }
}

#[derive(Clone)]
pub struct EmotionCompleter {
    feelings: Vec<&'static str>,
}

impl EmotionCompleter {
    fn filter_candidates(&self, input: &str) -> Vec<String> {
        let pattern = input.to_lowercase();

        self.feelings
            .iter()
            .filter(|s| s.starts_with(&pattern))
            .map(|s| String::from(*s))
            .collect()
    }
}

impl Autocomplete for EmotionCompleter {
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
