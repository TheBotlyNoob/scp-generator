use rust_bert::pipelines::question_answering::{QaInput, QuestionAnsweringModel};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let qa_model = QuestionAnsweringModel::new(Default::default())?;

    let question = String::from("what does this SCP resemble?");
    let context = String::from(SCP);

    let answer = qa_model
        .predict(&[QaInput { question, context }], 1, 64)
        .pop()
        .unwrap()
        .pop()
        .unwrap()
        .answer;

    println!("{}", answer);

    Ok(())
}

const SCP: &str = r#"SCP-096 is a humanoid creature measuring approximately 2.38 meters in height. Subject shows very little muscle mass, with preliminary analysis of body mass suggesting mild malnutrition. Arms are grossly out of proportion with the rest of the subject's body, with an approximate length of 1.5 meters each. Skin is mostly devoid of pigmentation, with no sign of any body hair."#;
