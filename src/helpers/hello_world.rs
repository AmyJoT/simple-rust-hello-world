use crate::Language;

pub fn greeting(language: Language) -> String {
    match language {
        Language::English => "Hello World".to_string(),
        Language::Spanish => "Hola World".to_string(),
        Language::Italian => "Ciao World".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world_returns_string() {
        let greeting_eng = greeting(Language::English);
        let greeting_spa = greeting(Language::Spanish);
        let greeting_ita = greeting(Language::Italian);

        assert_eq!("Hello World".to_string(), greeting_eng);
        assert_eq!("Hola World".to_string(), greeting_spa);
        assert_eq!("Ciao World".to_string(), greeting_ita);
    }
}
