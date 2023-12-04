# AIFinalProject
Pedro Gonzalez

Professor Bart Massey

CS 441 : AI

# Authorship

Jane Austen was an early 19th-century author famous for her English romance novels. 

Mary Wollstonecraft Shelley was an early 19th-century author famous for her SF novels, especially Frankenstein. 

Their writing styles are quite distinct.

Cleaned data from class:

    austen-northanger-abbey.txt
    austen-pride-and-prejudice.txt
    shelley-frankenstein.txt
    shelley-the-last-man.txt

Construct a machine learner that can classify paragraphs as Austen vs Shelley. 

Achieve at least 70% accuracy against a an independent validation set you construct from the text, or as a mean in 10-way cross-validation.

## Project Notes

This project took way longer than it probably should have for me. 

I feel like I learned a lot but I never got my machine leaner to achieve 70% accuracy. 

I have tried tuning the depth of the decision tree, changing the attribute decision making, changing the tree itself, changing the prediction logic... 

All this and I think the highest I achieved was 68%. 

It was a fun project but after so many hour I am ready to throw in the towel and start the other part of the project.

In order to run this project you must have Rust and Crate installed.

You can install them [here](https://www.rust-lang.org/)

Then the command to start the program is cargo run.