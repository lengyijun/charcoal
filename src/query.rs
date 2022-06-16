mod authority;
mod sentence;

use scraper::{ElementRef, Html, Selector};

use self::sentence::Sentence;

pub trait Select {
    type Target;
    fn select(elem: ElementRef) -> anyhow::Result<Self::Target>;
}

#[derive(Clone, Debug)]
pub struct WordQuery {
    pub pronunciation: Vec<(String, String)>,
    pub brief: Vec<String>,
    pub variants: Vec<String>,
    pub authority: Vec<String>,
    pub sentence: Vec<(String, String)>,
}

impl WordQuery {
    pub fn is_empty(&self) -> bool {
        return self.pronunciation.is_empty()
            && self.brief.is_empty()
            && self.variants.is_empty()
            && self.authority.is_empty()
            && self.sentence.is_empty();
    }
}

async fn get_html(url: impl AsRef<str> + reqwest::IntoUrl) -> anyhow::Result<String> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}

fn trim_str(t: &str) -> Option<String> {
    let t = t.trim();
    if t.is_empty() {
        None
    } else {
        Some(t.to_owned())
    }
}

impl Select for WordQuery {
    type Target = Self;

    fn select(elem: ElementRef) -> anyhow::Result<Self::Target> {
        let doc = elem;
        let pronunciation = {
            let sel = Selector::parse("span.pronounce").unwrap();
            doc.select(&sel)
                .map(|child| {
                    let pron = child.text().filter_map(trim_str).collect::<Vec<String>>();
                    (pron[0].to_owned(), pron[1].to_owned())
                })
                .collect()
        };

        let brief = {
            let sel = Selector::parse("#phrsListTab .trans-container ul li").unwrap();
            doc.select(&sel)
                .map(|child| {
                    child
                        .text()
                        .filter_map(trim_str)
                        .collect::<Vec<String>>()
                        .join("")
                })
                .collect()
        };

        let variants = {
            let sel = Selector::parse("#phrsListTab .trans-container p").unwrap();
            doc.select(&sel)
                .map(|child| {
                    child.text().map(|t| {
                        t.split("\n")
                            .filter_map(trim_str)
                            .collect::<Vec<String>>()
                            .join(" ")
                    })
                })
                .flatten()
                .collect()
        };

        let sentence = Sentence::select(elem)?;

        Ok(WordQuery {
            pronunciation,
            brief,
            variants,
            authority: Vec::new(),
            sentence,
        })
    }
}

impl WordQuery {
    pub async fn query(query_word: impl AsRef<str>) -> anyhow::Result<WordQuery> {
        let youdao_dict_url = url::Url::parse(&format!(
            "http://dict.youdao.com/search?q={}",
            query_word.as_ref()
        ))?;

        let xml = get_html(youdao_dict_url).await?;
        let doc = Html::parse_document(&xml);

        Self::select(doc.root_element())
    }
}
