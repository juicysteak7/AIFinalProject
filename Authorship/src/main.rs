use Authorship;

use std::env;


fn main() {
    let mut data_set:Authorship::Dataset = Authorship::Dataset {
        features: Vec::new(),
        labels: Vec::new(),
    };

    let mut file_name = "./austen-northanger-abbey.txt";
    let mut file_content = Authorship::read_file("./austen-northanger-abbey.txt");
    match file_content {
        Ok(result) => {
            let tokenized_paragraphs = Authorship::tokenize_paragraphs(&result);
            let mut n = 0;
            for paragraph in tokenized_paragraphs {
                if n <= 1 {
                    data_set.features.push(Authorship::extract_features(&paragraph));
                    data_set.labels.push(Authorship::label_paragraph(&file_name));
                }
                n = n+1;
                //println!("{}",paragraph);
                //println!("{:?}", Authorship::label_paragraph(&file_name));
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
                if n <= 1 {
                    data_set.features.push(Authorship::extract_features(&paragraph));
                    data_set.labels.push(Authorship::label_paragraph(&file_name));
                }
                n = n+1;
                //println!("{}",paragraph);
                //println!("{:?}", Authorship::label_paragraph(&file_name));
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


    let attributes = Authorship::get_attributes(&data_set);
    println!("{}",attributes.len());
    let (train_set, val_set) = Authorship::split_dataset(&data_set, 0.5);
    let decision_tree = Authorship::build_decision_tree(&train_set, &attributes);
    let accuracy = Authorship::validate_tree(&decision_tree, &val_set);
    println!("Accuracy: {:.2}%", accuracy * 100.0);

}
