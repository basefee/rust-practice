use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}

fn main() {
    let json = r#"
    {
        "article": "how to work with json",
        "author": "garv",
        "paragraph": [
            {
                "name": "start of the paragraph"
            },
            {
                "name": "body of the para"
            },
            {
                "name": "end of the para"
            }
        ]
    }"#;

    let parsed: Article = read_json_typed(json);
    println!("{}", parsed.paragraph[0].name);
}

fn read_json_typed(json: &str) -> Article {
    let parsed: Article = serde_json::from_str(json).unwrap();
    return parsed
}
