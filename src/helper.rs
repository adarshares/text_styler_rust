pub mod helpers {
    use std::fmt::Display;

    pub fn apply_escape_sequence<T: Display>(content: T, converting_integer: i32) -> String {
        const ESCAPE_FRONT: &str = "\x1b[";
        const ESCAPE_SEQUENCE_END: &str = "m";
        const ESCAPE_LAST: &str = "0";
        let original_content = content.to_string();

        return (String::from(
            String::from(
                String::from(
                    String::from(
                        (String::from(
                            String::from(ESCAPE_FRONT.clone())
                                + converting_integer.to_string().as_str(),
                        )
                        .clone()
                            + ESCAPE_SEQUENCE_END)
                            .as_str(),
                    )
                    .clone()
                        + original_content.as_str(),
                )
                .clone()
                    + ESCAPE_FRONT,
            )
            .clone()
                + ESCAPE_LAST,
        )
        .clone()
            + ESCAPE_SEQUENCE_END);
    }

}