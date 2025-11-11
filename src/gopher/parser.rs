use crate::utils::string::get_from_vec;

pub struct Item {
    pub code: char,
    pub title: String,
    pub path: String,
    pub host: String,
    pub port: u16,
}

pub struct GopherParser {
    pub page: Vec<Item>,
}

impl GopherParser {
    fn parse_title(source: &Vec<&str>) -> String {
        let split_source = get_from_vec(source, 0).chars();
        let split_count = split_source.clone().count();

        Self::pop_return(&mut split_source.skip(1).take(split_count).collect())
    }

    fn parse_port(source: &Vec<&str>) -> u16 {
        let mut port = get_from_vec(source, 3).to_string();
        Self::pop_return(&mut port).parse::<u16>().unwrap_or(0)
    }

    fn parse_code(source: &Vec<&str>) -> char {
        get_from_vec(source, 0).chars().next().unwrap_or('e')
    }

    fn parse_path(source: &Vec<&str>) -> String {
        get_from_vec(source, 1).to_string()
    }

    fn parse_host(source: &Vec<&str>) -> String {
        get_from_vec(source, 2).to_string()
    }

    pub fn new(tcp_string: &str) -> Self {
        let collect_string = tcp_string.split("\n");
        GopherParser {
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
