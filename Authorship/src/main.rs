fn main() {
    let mut data_set:Authorship::Dataset = Authorship::Dataset {
        features: Vec::new(),
        labels: Vec::new(),
    };

    let num_of_paragraphs = 50;
    let mut file_name = "./austen-northanger-abbey.txt";
    let mut file_content = Authorship::read_file("./austen-northanger-abbey.txt");
    match file_content {
        Ok(result) => {
            let tokenized_paragraphs = Authorship::tokenize_paragraphs(&result);
            let mut n = 0;
            for paragraph in tokenized_paragraphs {
                if n < num_of_paragraphs {
                    data_set.features.push(Authorship::extract_features(paragraph));
                    data_set.labels.push(Authorship::label_paragraph(file_name));
                }
                n+=1;
            }
        }
        Err(err) => {
            println!("{}",err);
        }
    }
    file_name = "./austen-pride-and-prejudice.txt";
    file_content = Authorship::read_file("./austen-pride-and-prejudice.txt");
    match file_content {
        Ok(result) => {
            let tokenized_paragraphs = Authorship::tokenize_paragraphs(&result);
            let mut n = 0;
            for paragraph in tokenized_paragraphs {
                if n < num_of_paragraphs {
                    data_set.features.push(Authorship::extract_features(paragraph));
                    data_set.labels.push(Authorship::label_paragraph(file_name));
                }
                n+=1;
            }
        }
        Err(err) => {
            println!("{}",err);
        }
    }
    file_name = "./shelley-the-last-man.txt";
    file_content = Authorship::read_file("./shelley-the-last-man.txt");
    match file_content {
        Ok(result) => {
            let tokenized_paragraphs = Authorship::tokenize_paragraphs(&result);
            let mut n = 0;
            for paragraph in tokenized_paragraphs {
                if n < num_of_paragraphs {
                    data_set.features.push(Authorship::extract_features(paragraph));
                    data_set.labels.push(Authorship::label_paragraph(file_name));
                }
                n += 1;
            }
        }
        Err(err) => {
            println!("{}",err);
        }
    }
    file_name = "./shelley-frankenstein.txt";
    file_content = Authorship::read_file("./shelley-frankenstein.txt");
    match file_content {
        Ok(result) => {
            let tokenized_paragraphs = Authorship::tokenize_paragraphs(&result);
            let mut n = 0;
            for paragraph in tokenized_paragraphs {
                if n < num_of_paragraphs {
                    data_set.features.push(Authorship::extract_features(paragraph));
                    data_set.labels.push(Authorship::label_paragraph(file_name));
                }
                n += 1;
            }
        }
        Err(err) => {
            println!("{}",err);
        }
    }

    let old_data = data_set.clone();
    let folds = 10;
    let mut overall_accuracy = 0.0;
    for fold in 0..folds {
        let (train_set, val_set) = Authorship::split_dataset(&data_set, 0.9);
        let mut attributes = Authorship::get_attributes(&train_set);

        let decision_tree = Authorship::build_decision_tree(&train_set, &mut attributes, 5);
        let accuracy = Authorship::validate_tree(&decision_tree, &val_set);
        println!("Fold {} Accuracy: {:.2}%", fold+1, accuracy * 100.0);
        data_set = old_data.clone();
        overall_accuracy += accuracy;
    }
    println!("Overall Accuracy: {:.2}%", (overall_accuracy/folds as f64) * 100.0);
}
