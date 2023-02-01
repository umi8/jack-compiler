use mockall::automock;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

#[automock]
pub trait LabelCreator {
    fn create(&self, command: &str) -> String;
}

#[derive(Default)]
pub struct RandomLabelCreator {}

impl LabelCreator for RandomLabelCreator {
    fn create(&self, command: &str) -> String {
        let random_label: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();
        format!("{}_{}", command, random_label)
    }
}
