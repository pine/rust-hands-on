extern crate scraper;
use scraper::Html;
use scraper::Selector;

fn main() {
    let html = r#"
<!DOCTYPE html>
<html lang="ja">
  <head>
  </head>
  <body>
    <p>
      Hello, <strong>World!</strong>
    </p>
  </body>
</html>
"#;
    let doc =  Html::parse_document(html);
    let selector = Selector::parse("p strong").unwrap();

    for element in doc.select(&selector) {
        // println!("{:?}", element);
        println!("{:?}", element.value().name());
        println!("{:?}", element.inner_html());
        println!("{:?}", element.text().collect::<Vec<_>>()[0]);
    }
}
