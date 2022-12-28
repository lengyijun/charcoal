use crate::{Cache, ExactQuery, Question};
use anyhow::anyhow;
use rodio::{Decoder, OutputStream, Sink};
use whatlang::Lang;
use std::{
    fs::File,
    io::{BufReader, Write},
};

pub struct Speech;

impl Speech {
    pub async fn query(
        word_query: &ExactQuery, cache: &Cache, is_speak: bool,
    ) -> anyhow::Result<()> {
        if is_speak {
            let file = Speech::store(word_query, cache).await?;
            Speech::speak(file).await
        } else {
            Ok(())
        }
    }

    fn url(word_query: &ExactQuery) -> anyhow::Result<String> {
        let code = match word_query.assumed_lang() {
            Lang::Eng => "en",
            Lang::Fra => "fr",
            Lang::Cmn => "zh_cn",
            _ => Err(anyhow!("Language inferred not supported"))?
        };
        Ok(format!(
            "https://translate.google.com/translate_tts?ie=UTF-8&client=tw-ob&tl={}&q={}",
            code,
            word_query.word()
        ))
    }

    async fn store(word_query: &ExactQuery, cache: &Cache) -> anyhow::Result<File> {
        let word = word_query.word();
        let file = (cache.query(&word, "mp3")).or_else(|_| -> anyhow::Result<File> {
            let url = Speech::url(word_query);
            futures::executor::block_on(async {
                // request
                let res = reqwest::get(url?).await?;

                // write
                let mut file = cache.store(&word, "mp3")?;
                let bytes = res.bytes().await?;
                file.write_all(&bytes)?;

                // read again to avoid overflow
                let file = cache.query(word, "mp3")?;
                Ok(file)
            })
        })?;
        Ok(file)
    }

    async fn speak(file: File) -> anyhow::Result<()> {
        // rodio
        // Get a output stream handle to the default physical sound device
        let (_stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;

        let file = BufReader::new(file);
        let source = Decoder::new(file)?;
        sink.append(source);
        sink.sleep_until_end();

        Ok(())
    }
}
