use serde::Deserialize;
use std::{collections::HashMap, ops::Deref};
use tui::widgets::TableState;

pub struct App {
    pub state: TableState,
    pub sheets: HashMap<String, Vec<Vec<String>>>,
    pub current: Option<Sheet>,
}

#[derive(Deserialize, Debug)]
pub struct Sheet {
    pub name: String,
    pub data: Vec<Vec<String>>,
}

impl Deref for Sheet {
    type Target = Vec<Vec<String>>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[derive(Deserialize, Debug)]
pub struct MyError {}

impl From<reqwest::Error> for MyError {
    fn from(value: reqwest::Error) -> Self {
        MyError {}
    }
}

impl From<serde_json::Error> for MyError {
    fn from(value: serde_json::Error) -> Self {
        MyError {}
    }
}

impl App {
    pub fn new() -> App {
        App {
            state: TableState::default(),
            sheets: HashMap::new(),
            current: None,
        }
    }

    pub fn add_sheet(&mut self, name: Option<String>, sheet: Vec<Vec<String>>) {
        let name = name.unwrap_or_else(|| self.sheets.len().to_string());
        self.sheets.insert(name, sheet);
    }

    pub fn switch_sheet(&mut self, name: impl AsRef<str>) {
        let new = if let Some((name, new)) = self.sheets.remove_entry(name.as_ref()) {
            Sheet { name, data: new }
        } else {
            panic!()
        };

        if let Some(Sheet { name, data }) = self.current.take() {
            self.sheets.insert(name, data);
        }

        self.current = Some(new);
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if let Some(current) = self.current.as_ref() {
                    if i < current.data.len() - 1 {
                        i + 1
                    } else {
                        0
                    }
                } else {
                    0
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if let Some(current) = self.current.as_ref() {
                    if i == 0 {
                        current.len() - 1
                    } else {
                        i - 1
                    }
                } else {
                    0
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub async fn get_sheet() -> Result<Sheet, MyError> {
    let body = reqwest::get("https://sheetdb.io/api/v1/42f48r00fkm6d")
        .await?
        .text()
        .await?;
    let res: Sheet = serde_json::from_str(&body)?;
    Ok(res)
}
