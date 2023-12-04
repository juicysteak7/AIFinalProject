use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use rand::seq::SliceRandom;
use std::cmp::Ordering;

#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)]
pub enum Author {
    Austen,
    Shelley,
}

#[derive(Debug)]
pub enum DecisionTreeNode {
    Internal {
        attribute: String,
        children: HashMap<u32,DecisionTreeNode>,
    },
    Leaf {
        class_label: Author,
    },
}

#[derive(Debug, Clone)]
pub struct Dataset {
    pub features: Vec<HashMap<String, u32>>,
    pub labels: Vec<Author>,
}

pub fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}

pub fn tokenize_paragraphs(text: &str) -> Vec<&str> {
    text.split("\n\n").collect()
}

pub fn extract_features(paragraph: &str) -> HashMap<String, u32> {
    let mut features = HashMap::new();

    for word in paragraph.split_whitespace() {
        if word.contains("-") {
            let words = word.split("-");
            for word in words {
                if word.len() > 0 {
                    if word != "." && word != "—" && word != "," {
                        *features.entry(word.to_string()).or_insert(0) += 1;
                    }
                }
            }
        } else if word.contains("—") {
            let words = word.split("—");
            for word in words {
                if word.len() > 0 {
                    if word != "." && word != "—" && word != "," {
                        *features.entry(word.to_string()).or_insert(0) += 1;
                    }
                }
            }
        }
        let word = word.to_lowercase().replace('.',"").replace(',',"").replace(":","").replace("?","").replace(";","").replace("!","").replace("”","").replace("“","").replace("_","").replace("’","").replace("\"","");
        *features.entry(word.to_string()).or_insert(0) += 1;
    }

    features
}

pub fn label_paragraph(file_name: &str) -> Author {
    if file_name.contains("austen") {
        Author::Austen
    } else {
        Author::Shelley
    }
}

// Need to loop on features and check word frequences based on number of total features
fn calculate_information_gain(data: &Dataset, attribute: &str, att_val: u32) -> f64 {
    let mut shelley_val = 0;
    let mut austen_val = 0;
    for i in 0..data.features.len() { 
        if let Some(result) = data.features[i].get(attribute) {
            if data.labels[i] == Author::Austen {
                austen_val+=result;
            } else {
                shelley_val+=result;
            }
        }
    }
    let result = shelley_val as f64 - austen_val as f64;
    if result < 0.0 {
        return -result;
    }
    result
}

// Might attach word frequence with attribute to cross refrence when predicting tree
fn choose_best_attribute(data: &Dataset, attributes: &HashMap<String, u32>) -> String {
    attributes.iter().max_by(|&(a_word, a_val), &(b_word, b_val)| {
        calculate_information_gain(data, a_word, *a_val).partial_cmp(&calculate_information_gain(data, b_word, *b_val)).unwrap_or(Ordering::Equal)
    }).map(|(word, _)| word.clone()).unwrap_or_default()
}

pub fn build_decision_tree(data: &Dataset, attributes: &mut HashMap<String,u32>, depth: u32) -> DecisionTreeNode {
    // If all examples in the subset have the same class label, return a leaf node
    if data.labels.iter().all(|&label| label == Author::Austen) {
        return DecisionTreeNode::Leaf { class_label: Author::Austen };
    } else if data.labels.iter().all(|&label| label == Author::Shelley) {
        return DecisionTreeNode::Leaf { class_label: Author::Shelley };
    }

    // If there are no more attributes to split on, return a leaf node with the majority class label
    if attributes.is_empty() || depth == 0 {
        let majority_class = data.labels.iter().cloned().fold(None, |acc, label| {
            match acc {
                None => Some(label),
                Some(prev) if prev == label => Some(prev),
                _ => None,
            }
        }).unwrap_or(Author::Austen); // Default to Austen in case of a tie
        return DecisionTreeNode::Leaf { class_label: majority_class };
    }

    // Choose the best attribute to split on
    let mut best_attribute = choose_best_attribute(data, attributes);

    attributes.remove(&best_attribute);


    // Split the dataset based on the chosen attribute
    let mut subsets: HashMap<u32, Dataset> = HashMap::new();
    for (i, feature) in data.features.iter().enumerate() {
        if let Some(value) = feature.get(&best_attribute) {
            subsets.entry(value.clone()).or_insert_with(|| Dataset {
                features: Vec::new(),
                labels: Vec::new(),
            }).features.push(feature.clone());
            subsets.get_mut(&value).unwrap().labels.push(data.labels[i]);
        } else {
            subsets.entry(0).or_insert_with(|| Dataset {
                features: Vec::new(),
                labels: Vec::new(),
            }).features.push(feature.clone());
            subsets.get_mut(&0).unwrap().labels.push(data.labels[i]);
        }
    }

    // Recursively build child nodes
    let mut children: HashMap<u32, DecisionTreeNode> = HashMap::new();
    for (value, subset) in subsets {
        children.insert(value, build_decision_tree(&subset, attributes, depth - 1));
    }

    DecisionTreeNode::Internal {
        attribute: best_attribute,
        children,
    }
}

pub fn predict_tree(node: &DecisionTreeNode, example: &HashMap<String, u32>, word_occurances: u32) -> Author {
    match node {
        DecisionTreeNode::Internal { attribute, children } => {
            if let Some(value) = example.get(attribute) {
                if let Some(child) = children.get(&value) {
                    predict_tree(child, example, word_occurances)
                } else {
                    // If the attribute value is not found, return a default class label (Austen in this case)
                    if let Some(child) = children.get(&word_occurances) {
                        return predict_tree(child, example, word_occurances+1);
                    } else {
                        Author::Austen
                    }
                }
            } else {
                if let Some(child) = children.get(&word_occurances) {
                    return predict_tree(child, example, word_occurances+1);
                } else {
                    return Author::Austen;
                }
            }
        }
        DecisionTreeNode::Leaf { class_label } => *class_label,
    }
}

pub fn validate_tree(tree: &DecisionTreeNode, validation_data: &Dataset) -> f64 {
    let mut correct_predictions = 0;

    for (example, true_label) in validation_data.features.iter().zip(&validation_data.labels) {
        let predicted_label = predict_tree(tree, example, 1);

        if predicted_label == *true_label {
            correct_predictions += 1;
        }
    }

    let accuracy = correct_predictions as f64 / validation_data.labels.len() as f64;
    accuracy
}

pub fn get_attributes(data: &Dataset) -> HashMap<String,u32> {
    // Collect all unique words in the dataset
    let mut unique_words: HashMap<String,u32> = HashMap::new();

    for feature in &data.features {
        for (word, value) in feature {
            if word != "the" && word != "and" && word != "of" && word != "my" && word != "to" && word != "i" && word != "his" && word != "a" && word != "that" && word != "—" {
                if let Some(result) = unique_words.get(word) {
                    unique_words.insert(word.clone(), result + value);
                } else {
                    unique_words.insert(word.clone(), *value);
                }

            }
        }
    }

    unique_words
}

pub fn split_dataset(dataset: &Dataset, train_ratio: f64) -> (Dataset, Dataset) {
    // Ensure the ratio is within bounds (0.0 to 1.0)
    let train_ratio = train_ratio.max(0.0).min(1.0);

    // Calculate the number of examples for the training set
    let num_train_examples = (dataset.features.len() as f64 * train_ratio) as usize;

    // Shuffle the indices to randomly select examples
    let mut indices: Vec<usize> = (0..dataset.features.len()).collect();
    indices.shuffle(&mut rand::thread_rng());

    // Split the indices into training and validation
    let train_indices = &indices[0..num_train_examples];
    let val_indices = &indices[num_train_examples..];

    // Create training and validation datasets
    let train_data = Dataset {
        features: train_indices.iter().map(|&i| dataset.features[i].clone()).collect(),
        labels: train_indices.iter().map(|&i| dataset.labels[i]).collect(),
    };

    let val_data = Dataset {
        features: val_indices.iter().map(|&i| dataset.features[i].clone()).collect(),
        labels: val_indices.iter().map(|&i| dataset.labels[i]).collect(),
    };

    (train_data, val_data)
}