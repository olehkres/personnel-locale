use std::result::Result;

use fluent::{FluentArgs, FluentBundle, FluentError, FluentResource};

#[derive(Default)]
pub struct Locale {
    bundle: FluentBundle<FluentResource>,
}

impl Locale {
    pub fn new(source: impl Into<String>) -> Result<Locale, String> {
        let mut bundle = FluentBundle::default();

        let Ok(resource) = FluentResource::try_new(source.into()) else {
            return Err("Error! Fluent resource file is corrupted.".to_string());
        };

        if bundle.add_resource(resource).is_err() {
            return Err("Error! Bundle can not accept this resource.".to_string());
        };

        Ok(Locale { bundle })
    }

    /**
    This function will return localized string by message with apllied attribute and Fluent args if they provided.

    # Arguments
    * `message` - Fluent message. Make sure it is in your .ftl file that you provided to Locale struct.
    * `attribute` - Fluent attribute. You can get attributed message if you specified this param else use `None`.
    * `args` - Fluent args. If you don't wont any than pass empty array like this `&[]`.
     */
    pub fn get_message(
        &self,
        message: impl Into<String>,
        attribute: Option<impl Into<String>>,
        args: &[(&str, &str)],
    ) -> Result<String, &str> {
        let Some(ftl_message) = &self.bundle.get_message(&message.into()) else {
            return Err("Error! Message doesn't exist!");
        };

        // Make pattern or return error.
        let ftl_pattern = if let Some(attribute) = attribute {
            // Make pattern with attributes.
            if let Some(attr) = ftl_message.get_attribute(&attribute.into()[..]) {
                attr.value()
            } else {
                return Err("Error! Attribute dosn't exist!");
            }
        } else {
            // Make pattern without attributes.
            if let Some(msg_value) = ftl_message.value() {
                msg_value
            } else {
                return Err("Error! Message value dosn't exist!");
            }
        };

        // Convert string arguments into specific fluent args.
        let mut ftl_args = FluentArgs::new();

        for (key, value) in args {
            ftl_args.set(*key, *value);
        }

        // Make result
        let mut errors: Vec<FluentError> = vec![];
        let result = self
            .bundle
            .format_pattern(ftl_pattern, Some(&ftl_args), &mut errors);

        Ok(result.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_message() {
        let locale = Locale::new(include_str!("../test.ftl")).unwrap();

        assert_eq!(
            "Hello, World!",
            locale
                .get_message("hello-world", None::<String>, &[])
                .unwrap()
        );
    }

    #[test]
    fn attribute_message() {
        let locale = Locale::new(include_str!("../test.ftl")).unwrap();

        assert_eq!(
            "Hello, Special World!",
            locale
                .get_message("hello-world", Some("special"), &[])
                .unwrap()
        );
    }

    #[test]
    fn argument_message() {
        let locale = Locale::new(include_str!("../test.ftl")).unwrap();

        // Note: here we add special characters. They are not printable.
        // Fluent add it in output for some advance formatting.
        assert_eq!(
            "Hello, \u{2068}Custom\u{2069} World!",
            locale
                .get_message("hello-world", Some("arg"), &[("world-type", "Custom")])
                .unwrap()
        );
    }
}
