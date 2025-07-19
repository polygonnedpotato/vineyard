//use serde::Serialize;
//use serde::Deserialize;
use toml::Table

//#[derive(Serialize)]
struct config {
    data_dir: String::from(
      print!("{}/.local/share/vineyard/",env!("HOME"))
    ),
}