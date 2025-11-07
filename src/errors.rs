pub mod errors {
    pub enum G_ERROR {
        NOT_GOPHER,
        OTHER,
    }

    pub struct Handler {
        code: G_ERROR,
        message: String,
    }

    impl Handler {
        pub fn throw(e: G_ERROR) -> Self {
            match e {
                G_ERROR::NOT_GOPHER => {
                    return Handler {
                        code: e,
                        message: "Can't visit this link: not a gopher hole".to_string(),
                    }
                }
                G_ERROR::OTHER => {
                    return Handler {
                        code: e,
                        message: "".to_string(),
                    }
                }
            }
        }

        pub fn custom_error(self, message: String) -> Self {
            let code = self.code;
            Handler { code, message }
        }

        pub fn get_message(self) -> String {
            self.message
        }
    }
}
