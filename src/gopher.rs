pub mod gopher {
    pub struct Item {
        pub code: char,
        pub title: String,
        pub path: String,
        pub host: String,
        pub port: u16,
    }

    pub struct Parser {
        pub page: Vec<Item>,
    }

    impl Parser {
        fn parse_title(source: &Vec<&str>) -> String {
            let split_source = source.get(0).unwrap_or(&"").chars();
            let split_count = split_source.clone().count();

            Self::pop_return(&mut split_source.skip(1).take(split_count).collect())
        }

        fn parse_port(source: &Vec<&str>) -> u16 {
            let mut port = source.get(3).unwrap_or(&"").to_string();
            Self::pop_return(&mut port).parse::<u16>().unwrap_or(0)
        }

        fn parse_code(source: &Vec<&str>) -> char {
            source.get(0).unwrap_or(&"").chars().next().unwrap_or('e')
        }

        fn parse_path(source: &Vec<&str>) -> String {
            source.get(1).unwrap_or(&"").to_string()
        }

        fn parse_host(source: &Vec<&str>) -> String {
            source.get(2).unwrap_or(&"").to_string()
        }

        pub fn new(tcp_string: &str) -> Self {
            let collect_string = tcp_string.split("\n");
            Parser {
                page: collect_string
                    .map(|c| {
                        let split_string = c.split("\t");
                        let split_collect: Vec<&str> = split_string.collect();

                        Item {
                            code: Self::parse_code(&split_collect),
                            title: Self::parse_title(&split_collect),
                            path: Self::parse_path(&split_collect),
                            host: Self::parse_host(&split_collect),
                            port: Self::parse_port(&split_collect),
                        }
                    })
                    .collect(),
            }
        }

        fn pop_return(to_parse: &mut String) -> String {
            if to_parse.ends_with('\r') {
                to_parse.pop();
            }

            to_parse.to_string()
        }
    }
}
