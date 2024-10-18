use std::env;
use std::fs;
use regex::Regex;
use crate::policy::Policy;

pub fn parse_input_text(file_path: &String) -> Vec<Policy> {
    let current_dir = env::current_dir().unwrap();
    println!("Current directory: {:?}", current_dir);

    let text = fs::read_to_string(file_path)
        .expect("Couldn't read file.");

    // parse text in lines
    let lines = text.split("\n");

    let chapter_title_re = Regex::new(r"^\s*CHAPTER (2|3|4)\.\s+[A-Z\- ]+\s*$").unwrap();
    let subsection_re = Regex::new(r"^\s*\d+\.\s+[A-Z\-].*\s*$").unwrap();
    let paragraph_re = Regex::new(r"^\s*[a-z]\.\s+[A-Z\- ].*\s*$").unwrap();

    let mut policies: Vec<Policy> = Vec::new();

    let mut in_target_chapter = false;

    let mut current_subsection = String::new();
    let mut current_subsection_number = String::new();
    let mut current_paragraph = String::new();
    let mut current_paragraph_number = String::new();

    for line in lines {
        let trimmed_line = line.trim();

        // Check if it's a chapter title
        if chapter_title_re.is_match(trimmed_line) {
            //println!("Chapter found: {}", trimmed_line);
            in_target_chapter = true; // We are in chapters 2, 3 or 4
            continue;
        }

        // Exiting target chapter
        if trimmed_line.starts_with("CHAPTER") && !chapter_title_re.is_match(trimmed_line) {
            in_target_chapter = false;
            println!("{}. Exiting now", trimmed_line);
            continue;
        }

        // Check for a new subsection if in target chapter
        if in_target_chapter && subsection_re.is_match(trimmed_line) {
            if !current_paragraph.is_empty() {
                let policy = build_policy_from_paragraph(&current_subsection_number, &current_paragraph_number, &current_subsection, &current_paragraph);
                policies.push(policy);
                //println!("Paragraph: {}{}\n", current_subsection_number.as_str(), current_paragraph);
            }
            if !current_subsection.is_empty() {
                //println!("Subsection: {}\n", current_subsection);
                if current_paragraph.is_empty() {
                    let policy = build_policy_from_subsection(&current_subsection_number, &current_subsection);
                    policies.push(policy);
                }
            }
            current_subsection = String::from(trimmed_line);
            let subsection_re_num = Regex::new(r"^\s*(\d+)\.\s+.+$").unwrap();
            current_subsection_number = String::from(subsection_re_num.captures(line).unwrap().get(1).unwrap().as_str());

            current_paragraph.clear(); // Reset paragraph when subsection changes
            continue;
        }

        if in_target_chapter && !current_subsection.is_empty() {
            // Check for a new paragraph if in target chapter
            if in_target_chapter && paragraph_re.is_match(trimmed_line) {
                if !current_paragraph.is_empty() {
                    let policy = build_policy_from_paragraph(&current_subsection_number, &current_paragraph_number, &current_subsection, &current_paragraph);
                    policies.push(policy);
                    //println!("Paragraph: {}{}\n", current_subsection_number.as_str(), current_paragraph);
                }
                current_paragraph = String::from(trimmed_line);
                let paragraph_re_num = Regex::new(r"^\s*([a-z])\.\s+.+$").unwrap();
                current_paragraph_number = String::from(paragraph_re_num.captures(line).unwrap().get(1).unwrap().as_str());
                continue;
            }

            // If the line is part of an ongoing paragraph, append it
            if in_target_chapter && !current_paragraph.is_empty() {
                current_paragraph.push(' '); // Add a space to separate the lines
                current_paragraph.push_str(trimmed_line);
            }
            else {
                // There are additional lines to the subsection text
                current_subsection.push_str("\n");
                current_subsection.push_str(trimmed_line);
            }
        }

    }
    // Output the last collected paragraph and subsection, if any
    if !current_subsection.is_empty() {
        //println!("Subsection: {}\n\n", current_subsection);
        if current_paragraph.is_empty() {
            let policy = build_policy_from_subsection(&current_subsection_number, &current_subsection);
            policies.push(policy);
        }
    }
    if !current_paragraph.is_empty() {
        if !current_paragraph.is_empty() {
            let policy = build_policy_from_paragraph(&current_subsection_number, &current_paragraph_number, &current_subsection, &current_paragraph);
            policies.push(policy);
            //println!("Paragraph: {}{}\n", current_subsection_number.as_str(), current_paragraph);
        }
    }

    /*
    for policy in &policies {
        println!("{}\n", policy)
    }
    */

    policies

}

fn build_policy_from_paragraph(subsection_number: &String, paragraph_number: &String, subsection: &String, paragraph: &String) -> Policy {
    let id = format!("{}{}", subsection_number, paragraph_number);

    let first_line_re = Regex::new(r"^\s*\d+\.\s+(.+)$").unwrap();

    let mut lines = subsection.lines();
    let mut label = "";
    let mut context = String::new();

    if let Some(first_line) = lines.next() {
        label = first_line_re.captures(first_line).unwrap().get(1).unwrap().as_str();
        //println!("label: {}", label);

        context = lines.collect::<Vec<&str>>().join(" ");
        //println!("Context:\n{}", context);
    }

    // Exclude the policy id from the body
    let paragraph_re_trim = Regex::new(r"(?s)^\s*[a-z]\.\s+(.+)$").unwrap();
    let mut body = String::from(paragraph_re_trim.captures(paragraph).unwrap().get(1).unwrap().as_str());

    // split the context into sentences
    let sentences: Vec<&str> = context.split(". ").collect();
    if sentences.len() > 0 {
        let last_sentence = sentences[sentences.len() - 1];

        // Check if the last sentence of the context contains the words "Component(s)" or "shall"
        let components_re = Regex::new(r".*Component.*").unwrap();
        let shall_re = Regex::new(r".*shall.*").unwrap();

        // If so, remove the last sentence from the context and add it to the body of the policy
        if components_re.is_match(&last_sentence) || shall_re.is_match(&last_sentence) {
            body = format!("{last_sentence} {body}");
            context = sentences[..sentences.len() - 1].join(". ");
        }
    }

    let policy = Policy {
        id,
        label: String::from(label),
        context,
        body,
    };
    policy
}

fn build_policy_from_subsection(subsection_number: &String, subsection: &String, ) -> Policy {
    let mut id = String::from(subsection_number);

    let first_line_re = Regex::new(r"^\s*\d+\.\s+(.+)$").unwrap();

    let mut lines = subsection.lines();
    let mut label = "";
    let mut body = String::new();

    if let Some(first_line) = lines.next() {
        label = first_line_re.captures(first_line).unwrap().get(1).unwrap().as_str();
        //println!("label: {}", label);

        body = lines.collect::<Vec<&str>>().join(" ");
        //println!("Context:\n{}", body);
    }

    let policy = Policy {
        id,
        label: String::from(label),
        context: String::new(),
        body,
    };
    policy
}
