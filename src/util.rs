use serenity::{
    model::id::{ChannelId, MessageId},
    utils::parse_message_id_pair,
};

pub fn parse_message_with_emotes(message: String) -> String {
    let mut result = String::new();

    for c in message.chars() {
        //
    }

    result
}

// custom_id: "some-id=<channel>-<message>"
pub fn get_message_ref(custom_id: &str) -> Option<(ChannelId, MessageId)> {
    let tokens = custom_id.split("=").collect::<Vec<&str>>();

    if tokens.len() == 2 {
        let parsed = parse_message_id_pair(tokens[1]);

        if let Some(parsed) = parsed {
            Some(parsed)
        } else {
            None
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_slash_input() {
        let result = parse_message_with_emotes(String::from("Hello //world"));
        assert_eq!(result, String::from("Hello /world"))
    }

    #[test]
    fn newline_input() {
        let result = parse_message_with_emotes(String::from(r#"Hello \world"#));
        assert_eq!(result, String::from("Hello \nworld"))
    }

    #[test]
    fn backslash_input() {
        let result = parse_message_with_emotes(String::from(r#"Hello \\world"#));
        assert_eq!(result, String::from(r#"Hello \world"#))
    }
}
