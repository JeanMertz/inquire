use super::{Text, TextAnswer};
use crate::ui::{Key, KeyModifiers};
use crate::validator::{ErrorMessage, Validation};

fn default<'a, 'b>() -> Text<'a, 'b> {
    Text::new("Question?")
}

macro_rules! text_to_events {
    ($text:expr) => {{
        $text
            .chars()
            .map(|c| Key::Char(c, KeyModifiers::NONE))
            .collect::<Vec<Key>>()
    }};
}

macro_rules! text_test {
    ($name:ident,$input:expr,$output:expr) => {
        text_test! {$name, $input, $output, default()}
    };

    ($name:ident,$input:expr,$output:expr,$prompt:expr) => {
        #[test]
        fn $name() {
            let mut buf = Vec::new();
            let mut backend = crate::prompts::test::fake_backend(&mut buf, $input);

            let ans = $prompt.prompt_with_backend(&mut backend).unwrap();

            assert_eq!($output, ans);
        }
    };
}

macro_rules! raw_text_test {
    ($name:ident,$input:expr,$output:expr,$prompt:expr) => {
        #[test]
        fn $name() {
            let mut buf = Vec::new();
            let mut backend = crate::prompts::test::fake_backend(&mut buf, $input);

            let ans = $prompt.raw_prompt_with_backend(&mut backend).unwrap();

            assert_eq!($output, ans);
        }
    };
}

text_test!(empty, vec![Key::Enter], "");

text_test!(
    single_letter,
    vec![Key::Char('b', KeyModifiers::NONE), Key::Enter],
    "b"
);

text_test!(
    letters_and_enter,
    text_to_events!("normal input\n"),
    "normal input"
);

text_test!(
    letters_and_enter_with_emoji,
    text_to_events!("with emoji 🧘🏻‍♂️, 🌍, 🍞, 🚗, 📞\n"),
    "with emoji 🧘🏻‍♂️, 🌍, 🍞, 🚗, 📞"
);

text_test!(
    input_and_correction,
    {
        let mut events = vec![];
        events.append(&mut text_to_events!("anor"));
        events.push(Key::Backspace);
        events.push(Key::Backspace);
        events.push(Key::Backspace);
        events.push(Key::Backspace);
        events.append(&mut text_to_events!("normal input"));
        events.push(Key::Enter);
        events
    },
    "normal input"
);

text_test!(
    input_and_excessive_correction,
    {
        let mut events = vec![];
        events.append(&mut text_to_events!("anor"));
        events.push(Key::Backspace);
        events.push(Key::Backspace);
        events.push(Key::Backspace);
        events.push(Key::Backspace);
        events.push(Key::Backspace);
        events.push(Key::Backspace);
        events.push(Key::Backspace);
        events.push(Key::Backspace);
        events.push(Key::Backspace);
        events.push(Key::Backspace);
        events.append(&mut text_to_events!("normal input"));
        events.push(Key::Enter);
        events
    },
    "normal input"
);

text_test!(
    input_correction_after_validation,
    {
        let mut events = vec![];
        events.append(&mut text_to_events!("1234567890"));
        events.push(Key::Enter);
        events.push(Key::Backspace);
        events.push(Key::Backspace);
        events.push(Key::Backspace);
        events.push(Key::Backspace);
        events.push(Key::Backspace);
        events.append(&mut text_to_events!("yes"));
        events.push(Key::Enter);
        events
    },
    "12345yes",
    Text::new("").with_validator(|ans: &str| match ans.len() {
        len if len > 5 && len < 10 => Ok(Validation::Valid),
        _ => Ok(Validation::Invalid(ErrorMessage::Default)),
    })
);

raw_text_test!(
    an_empty_submission_answers_with_the_default,
    vec![Key::Enter],
    TextAnswer {
        value: "main".to_owned(),
        is_default: true,
    },
    Text::new("Branch?").with_default("main")
);

raw_text_test!(
    typing_the_default_is_not_accepting_it,
    text_to_events!("main\n"),
    TextAnswer {
        value: "main".to_owned(),
        is_default: false,
    },
    Text::new("Branch?").with_default("main")
);

raw_text_test!(
    an_empty_submission_without_a_default_is_not_a_default,
    vec![Key::Enter],
    TextAnswer {
        value: String::new(),
        is_default: false,
    },
    Text::new("Branch?")
);

raw_text_test!(
    input_cleared_back_to_empty_answers_with_the_default,
    {
        let mut events = text_to_events!("x");
        events.push(Key::Backspace);
        events.push(Key::Enter);
        events
    },
    TextAnswer {
        value: "main".to_owned(),
        is_default: true,
    },
    Text::new("Branch?").with_default("main")
);
