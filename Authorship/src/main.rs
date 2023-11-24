use Authorship;


fn main() {
    println!("Hello, world!");
    let fileContent = Authorship::read_file("./austen-northanger-abbey.txt");

    match fileContent {
        Ok(result) => {
            let tokenized_paragraphs = Authorship::tokenize_paragraphs(&result);
            let mut data_set:Authorship::Dataset = Authorship::Dataset {
                features: Vec::new(),
                labels: Vec::new(),
            };
            for paragraph in tokenized_paragraphs {
                data_set.features.push(Authorship::extract_features(&paragraph));
                data_set.labels.push(Authorship::label_paragraph(&paragraph));
                //println!("{:?}", Authorship::label_paragraph(&paragraph));
            }
            let attributes = Authorship::get_attributes(&data_set);
            let (train_set, val_set) = Authorship::split_dataset(&data_set, 0.7);
            let decision_tree = Authorship::build_decision_tree(&train_set, &attributes);

            // Make predictions on the validation set
            let predictions = val_set
            .features
            .iter()
            .map(|example| Authorship::predict_tree(&decision_tree, example))
            .collect::<Vec<_>>();

            // Evaluate and report accuracy
            let correct_predictions = predictions
            .iter()
            .zip(&val_set.labels)
            .filter(|(&pred, &actual)| pred == actual)
            .count();

            //println!("{}", correct_predictions);

            let accuracy = correct_predictions as f64 / val_set.labels.len() as f64;
            println!("Accuracy: {:.2}%", accuracy * 100.0);

        }
        Err(err) => {
            println!("{}", err);
        }
    }
}
