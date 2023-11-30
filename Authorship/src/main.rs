use Authorship;

use std::env;


fn main() {
    let mut data_set:Authorship::Dataset = Authorship::Dataset {
        features: Vec::new(),
        labels: Vec::new(),
    };

    let num_of_paragraphs = 10;
    let mut file_name = "./austen-northanger-abbey.txt";
    let mut file_content = Authorship::read_file("./austen-northanger-abbey.txt");
    match file_content {
        Ok(result) => {
            let tokenized_paragraphs = Authorship::tokenize_paragraphs(&result);
            let mut n = 0;
            for paragraph in tokenized_paragraphs {
                if n < num_of_paragraphs {
                    data_set.features.push(Authorship::extract_features(&paragraph));
                    data_set.labels.push(Authorship::label_paragraph(&file_name));
                }
                n = n+1;
            }
        }
        Err(err) => {
            println!("{}",err);
        }
    }
    file_name = "./shelley-the-last-man.txt";
    file_content = Authorship::read_file("./shelley-frankenstein.txt");
    match file_content {
        Ok(result) => {
            let tokenized_paragraphs = Authorship::tokenize_paragraphs(&result);
            let mut n = 0;
            for paragraph in tokenized_paragraphs {
                if n < num_of_paragraphs {
                    data_set.features.push(Authorship::extract_features(&paragraph));
                    data_set.labels.push(Authorship::label_paragraph(&file_name));
                }
                n = n+1;
            }
        }
        Err(err) => {
            println!("{}",err);
        }
    }

    let mut shelley_num = 0;
    let mut austen_num = 0;
    for label in &data_set.labels {
        if *label == Authorship::Author::Austen {
            austen_num += 1;
        } else {
            shelley_num += 1;
        }
    }
    println!("shelly: {}, austen: {}", shelley_num, austen_num);

    let old_data = data_set.clone();
    let folds = 3;
    let mut overall_accuracy = 0.0;
    for fold in 0..folds {
        //let mut attributes = Authorship::get_attributes(&data_set);
        let (train_set, val_set) = Authorship::split_dataset(&data_set, 0.5);
        //println!("{:?}",train_set);
        let mut attributes = Authorship::get_attributes(&train_set);
        println!("{}",attributes.len());
        // for att in &attributes {
        //     println!("{}", att);
        // }
        // for feature in &train_set.features {
        //     for (word,value) in feature {
        //         println!("{}: {}", word, value);
        //     }
        // }
        let decision_tree = Authorship::build_decision_tree(&train_set, &mut attributes, 40);
        println!("{:?}",decision_tree);
        let accuracy = Authorship::validate_tree(&decision_tree, &val_set);
        println!("Fold {} Accuracy: {:.2}%", fold+1, accuracy * 100.0);
        data_set = old_data.clone();
        overall_accuracy += accuracy;
    }
    println!("Overall Accuracy: {:.2}%", (overall_accuracy/folds as f64) * 100.0);
}
