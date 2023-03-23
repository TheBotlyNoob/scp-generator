mod craiyon;
mod scp;

use anyhow::Result;
use rust_bert::pipelines::question_answering::{QaInput, QuestionAnsweringModel};

use crate::{craiyon::Craiyon, scp::Scp};

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let qa_model = QuestionAnsweringModel::new(Default::default())?;

    let Scp { description, .. } = Scp::scrape(2)?;

    let answer = qa_model
        .predict(
            &[QaInput {
                question: String::from("what does this SCP resemble?"),
                context: description,
            }],
            1,
            64,
        )
        .pop()
        .unwrap()
        .pop()
        .unwrap()
        .answer;

    tracing::info!(?answer);

    tracing::info!("drawing image (this may take over a minute)...");
    let image = Craiyon::draw(&answer)?.image()?;
    std::fs::write("image.webp", image)?;

    Ok(())
}
