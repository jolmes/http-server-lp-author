#[derive(Debug, PartialEq)]
enum Lang {
  English,
  Spanish,
  Chinese,
  Texan,
  German,
  Portuguese,
  Swahili,
  Japanese
}

#[derive(Debug, PartialEq)]
struct Greeting {
    message: String,
    lang: Lang,
}

fn main() {
  let mut v :Vec<Greeting> = Vec::new();

  let g : Greeting = Greeting { lang: Lang::English, message: String::from("Hello WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Spanish, message: String::from("Hola WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Spanish, message: String::from("Howdy WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Chinese, message: String::from("WasmEdge 你好!") };
  v.push(g);

  let spanish_only: Vec<_> = v.iter().clone().filter(|x| matches!(x.lang, Lang::Spanish)).collect();
  println!("{:?}", spanish_only);

  for e in v {
    println!("{:?} {}", e.lang, e.message);
  }
}
