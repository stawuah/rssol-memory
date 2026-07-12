#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Set { key: String, value: String },
    Delete { key: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    InvalidShape,
    UnknownCommand,
}

pub fn parse_command(input: &str) -> Result<Command, ParseError> {
    todo!("parse `set <key> <value>` and `delete <key>`")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_set() {
        assert_eq!(
            parse_command("set user:1 online"),
            Ok(Command::Set {
                key: "user:1".into(),
                value: "online".into(),
            })
        );
    }

    #[test]
    fn parses_delete() {
        assert_eq!(
            parse_command("delete user:1"),
            Ok(Command::Delete {
                key: "user:1".into(),
            })
        );
    }

    #[test]
    fn rejects_unknown_command() {
        assert_eq!(parse_command("ping node-a"), Err(ParseError::UnknownCommand));
    }
}
