use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}

fn main() {
    let article: Article = Article {
        article: String::from("how to write json"),
        author: String::from("mrk1tty"),
        paragraph: vec![
            Paragraph {
                name: String::from("first sentence")
            },
            Paragraph {
                name: String::from("body of the para")
            },
            Paragraph {
                name: String::from("end of the para")
            },
        ]
    };

    let serialized = serde_json::to_string(&article).unwrap();
    println!("{}", serialized);
}