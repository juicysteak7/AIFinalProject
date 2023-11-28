use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use rand::seq::SliceRandom;

#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)]
pub enum Author {
    Austen,
    Shelley,
}

#[derive(Debug)]
pub enum DecisionTreeNode {
    Internal {
        attribute: String,
        children: HashMap<String,DecisionTreeNode>,
    },
    Leaf {
        class_label: Author,
    },
}

pub struct Dataset {
    pub features: Vec<HashMap<String, u32>>,
    pub labels: Vec<Author>,
}

pub fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}

pub fn tokenize_paragraphs(text: &str) -> Vec<&str> {
    text.split("\r\n\r\n").collect()
}

pub fn extract_features(paragraph: &str) -> HashMap<String, u32> {
    let mut features = HashMap::new();
    for word in paragraph.split_whitespace() {
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

fn calculate_entropy(labels: &[Author]) -> f64 {
    let total_samples = labels.len() as f64;

    // Count the occurrences of each class
    let class_counts: HashMap<Author, usize> = labels.iter().fold(HashMap::new(), |mut counts, &class_label| {
        *counts.entry(class_label).or_insert(0) += 1;
        counts
    });

    // Calculate entropy
    let entropy = class_counts.values().fold(0.0, |acc, &count| {
        let probability = count as f64 / total_samples;
        acc - probability * probability.log2()
    });

    entropy
}

fn calculate_information_gain(data: &Dataset, attribute: &str) -> f64 {
    let total_samples = data.labels.len() as f64;
    let entropy_s = calculate_entropy(&data.labels);

    // Calculate weighted sum of entropies for each value of the attribute
    let mut entropy_sum = 0.0;
    let values: HashSet<&String> = data.features.iter().flat_map(|feature| feature.keys()).collect();

    for value in values {
        // Filter the data for samples where the attribute has the specific value
        let subset_indices: Vec<usize> = data.features.iter()
            .enumerate()
            .filter(|(_, feature)| feature.get(attribute).map(|v| v.to_string()) == Some(value.to_string()))
            .map(|(i, _)| i)
            .collect();

        let subset_labels: Vec<Author> = subset_indices.iter().map(|&i| data.labels[i]).collect();
        let subset_entropy = calculate_entropy(&subset_labels);

        // Weighted sum
        let probability = subset_indices.len() as f64 / total_samples;
        entropy_sum += probability * subset_entropy;
    }

    // Information Gain
    entropy_s - entropy_sum
}

fn choose_best_attribute(data: &Dataset, attributes: &HashSet<String>) -> String {
    attributes.iter().max_by(|&a, &b| {
        //println!("{:?},{:?}",a,b);
        calculate_information_gain(data, &a).partial_cmp(&calculate_information_gain(data, &b)).unwrap()
    }).cloned().unwrap_or_default()
}

pub fn build_decision_tree(data: &Dataset, attributes: &HashSet<String>) -> DecisionTreeNode {
    // If all examples in the subset have the same class label, return a leaf node
    if data.labels.iter().all(|&label| label == Author::Austen) {
        return DecisionTreeNode::Leaf { class_label: Author::Austen };
    } else if data.labels.iter().all(|&label| label == Author::Shelley) {
        return DecisionTreeNode::Leaf { class_label: Author::Shelley };
    }

    // If there are no more attributes to split on, return a leaf node with the majority class label
    if attributes.is_empty() {
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
    let best_attribute = choose_best_attribute(data, attributes);

    //println!("{}",best_attribute);
    // Split the dataset based on the chosen attribute
    let mut subsets: HashMap<String, Dataset> = HashMap::new();
    for (i, feature) in data.features.iter().enumerate() {
        let value = feature.get(&best_attribute).map(|v| v.to_string()).unwrap_or_default();
        subsets.entry(value.clone()).or_insert_with(|| Dataset {
            features: Vec::new(),
            labels: Vec::new(),
        }).features.push(feature.clone());
        subsets.get_mut(&value).unwrap().labels.push(data.labels[i]);
    }

    // Recursively build child nodes
    let mut children: HashMap<String, DecisionTreeNode> = HashMap::new();
    for (value, subset) in subsets {
        children.insert(value, build_decision_tree(&subset, &attributes));
    }

    DecisionTreeNode::Internal {
        attribute: best_attribute,
        children,
    }
}

pub fn predict_tree(node: &DecisionTreeNode, example: &HashMap<String, u32>) -> Author {
    match node {
        DecisionTreeNode::Internal { attribute, children } => {
            let value = example.get(attribute).map(|v| v.to_string()).unwrap_or_default();
            if let Some(child) = children.get(&value) {
                predict_tree(child, example)
            } else {
                // If the attribute value is not found, return a default class label (Austen in this case)
                Author::Austen
            }
        }
        DecisionTreeNode::Leaf { class_label } => *class_label,
    }
}

pub fn validate_tree(tree: &DecisionTreeNode, validation_data: &Dataset) -> f64 {
    let mut correct_predictions = 0;

    for (example, true_label) in validation_data.features.iter().zip(&validation_data.labels) {
        let predicted_label = predict_tree(tree, example);

        if predicted_label == *true_label {
            correct_predictions += 1;
        }
    }

    let accuracy = correct_predictions as f64 / validation_data.labels.len() as f64;
    accuracy
}

pub fn get_attributes(data: &Dataset) -> HashSet<String> {
    // Collect all unique words in the dataset
    let mut unique_words: HashSet<String> = HashSet::new();

    for feature in &data.features {
        for (word, _) in feature {
            unique_words.insert(word.clone());
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