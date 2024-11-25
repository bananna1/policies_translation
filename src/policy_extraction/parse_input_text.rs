use std::fs;
use regex::Regex;
use crate::policy_extraction::policy::Policy;

pub fn parse_input_text(file_path: &String) -> Result<Vec<Policy>, String> {

    let text = fs::read_to_string(file_path)
        .expect("Could not read file.");

    // parse text in lines
    let lines = text.split("\n");

    let chapter_title_re;
    match Regex::new(r"^\s*CHAPTER (2|3|4)\.\s+[A-Z\- ]+\s*$") {
        Ok(re) => {
            chapter_title_re = re;
        }
        Err(_) => return Err(String::from("Could not elaborate regular expression"))
    }
    let subsection_re;
    match Regex::new(r"^\s*\d+\.\s+[A-Z\-].*\s*$") {
        Ok(re) => {
            subsection_re = re;
        }
        Err(_) => return Err(String::from("Could not elaborate regular expression"))
    }
    let paragraph_re;
    match Regex::new(r"^\s*[a-z]\.\s+[A-Z\- ].*\s*$") {
        Ok(re) => {
            paragraph_re = re;
        }
        Err(_) => return Err(String::from("Could not elaborate regular expression"))
    }

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
            //println!("{}. Exiting now", trimmed_line);
            continue;
        }

        // Check for a new subsection if in target chapter
        if in_target_chapter && subsection_re.is_match(trimmed_line) {
            if !current_paragraph.is_empty() {
                //println!("CURRENT SUBSECTION NUMBER JUST BEFORE METHOD CALLING: {}", current_subsection_number);
                //println!("CURRENT PARAGRAPH NUMBER JUST BEFORE METHOD CALLING: {}", current_paragraph_number);
                //println!("CURRENT PARAGRAPH:\n{}", current_paragraph);
                let policy;
                match build_policy_from_paragraph(&current_subsection_number, &current_paragraph_number, &current_subsection, &current_paragraph) {
                    Ok(p) => {
                        policy = p;
                    },
                    Err(s) => {
                        return Err(s)
                    }
                }
                policies.push(policy);
                //println!("Paragraph: {}{}\n", current_subsection_number.as_str(), current_paragraph);
            }
            if !current_subsection.is_empty() {
                //println!("Subsection: {}\n", current_subsection);
                let policy;
                if current_paragraph.is_empty() {
                    match build_policy_from_subsection(&current_subsection_number, &current_subsection) {
                        Ok(p) => {
                            policy = p;
                        },
                        Err(s) => {
                            return Err(s);
                        }
                    }
                    policies.push(policy);
                }
            }
            current_subsection = String::from(trimmed_line);
            let subsection_re_num;
            match Regex::new(r"^\s*(\d+)\.\s+.+$") {
                Ok(re) => {
                    subsection_re_num = re;
                }
                Err(_) => return Err(String::from("Could not elaborate regular expression"))
            }


            match subsection_re_num.captures(line) {
                Some(pre_num) => {
                    match pre_num.get(1) {
                        Some(num) => {
                            current_subsection_number = String::from(num.as_str());
                        },
                        None => return Err(String::from("Couldn't extract subsection number"))
                    }
                },
                None => return Err(String::from("Couldn't extract subsection number"))
            }
            //println!("CURRENT SUBSECTION NUMBER: {}", current_subsection_number);

            current_paragraph.clear(); // Reset paragraph when subsection changes
            continue;
        }

        if in_target_chapter && !current_subsection.is_empty() {
            // Check for a new paragraph if in target chapter
            if in_target_chapter && paragraph_re.is_match(trimmed_line) {
                if !current_paragraph.is_empty() {
                    //println!("CURRENT SUBSECTION NUMBER JUST BEFORE METHOD CALLING: {}", current_subsection_number);
                    //println!("CURRENT PARAGRAPH NUMBER JUST BEFORE METHOD CALLING: {}", current_paragraph_number);
                    //println!("CURRENT PARAGRAPH:\n{}", current_paragraph);
                    let policy;
                    match build_policy_from_paragraph(&current_subsection_number, &current_paragraph_number, &current_subsection, &current_paragraph) {
                        Ok(p) => {
                            policy = p;
                        },
                        Err(s) => {
                            return Err(s);
                        }
                    }
                    policies.push(policy);
                    //println!("Paragraph: {}{}\n", current_subsection_number.as_str(), current_paragraph);
                }
                current_paragraph = String::from(trimmed_line);
                let paragraph_re_num;
                match Regex::new(r"^\s*([a-z])\.\s+.+$") {
                    Ok(re) => {
                        paragraph_re_num = re;
                    }
                    Err(_) => return Err(String::from("Could not elaborate regular expression"))
                }
                match paragraph_re_num.captures(line) {
                    Some(pre_num) => {
                        match pre_num.get(1) {
                            Some(num) => {
                                current_paragraph_number = String::from(num.as_str());
                            },
                            None => return Err(String::from("Could not extract paragraph number"))
                        }
                    },
                    None => return Err(String::from("Couldn't extract paragraph number"))
                }
                //println!("CURRENT PARAGRAPH NUMBER: {}", current_paragraph_number);
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
            let policy;
            match build_policy_from_subsection(&current_subsection_number, &current_subsection) {
                Ok(p) => {
                    policy = p;
                },
                Err(s) => return Err(s)

            }
            policies.push(policy);
        }
    }
    if !current_paragraph.is_empty() {
        if !current_paragraph.is_empty() {
            //println!("CURRENT SUBSECTION NUMBER JUST BEFORE METHOD CALLING: {}", current_subsection_number);
            //println!("CURRENT PARAGRAPH NUMBER JUST BEFORE METHOD CALLING: {}", current_paragraph_number);
            //println!("CURRENT PARAGRAPH:\n{}", current_paragraph);
            let policy;
            match build_policy_from_paragraph(&current_subsection_number, &current_paragraph_number, &current_subsection, &current_paragraph) {
                Ok(p) => {
                    policy = p;
                },
                Err(s) => return Err(s)
            }
            policies.push(policy);
            //println!("Paragraph: {}{}\n", current_subsection_number.as_str(), current_paragraph);
        }
    }
    Ok(policies)

}

fn build_policy_from_paragraph(subsection_number: &String, paragraph_number: &String, subsection: &String, paragraph: &String) -> Result<Policy, String> {
    //println!("Subsection number: {}; paragraph number: {}", subsection_number, paragraph_number);
    let id = format!("{}{}", subsection_number, paragraph_number);

    let first_line_re;
    match Regex::new(r"^\s*\d+\.\s+(.+)$") {
        Ok(re) => {
            first_line_re = re;
        },
        Err(_) => return Err(String::from("Could not elaborate regular expression"))
    }

    let mut lines = subsection.lines();
    let mut label = String::new();
    let mut context = String::new();

    if let Some(first_line) = lines.next() {
        match extract_label(first_line, first_line_re) {
            Ok(l) => {
                label = l;
            }
            Err(s) => return Err(s)
        }
        context = lines.collect::<Vec<&str>>().join(" ");
        //println!("Context:\n{}", body);
    }

    // Exclude the policy id from the body
    let paragraph_re_trim;
    match Regex::new(r"(?s)^\s*[a-z]\.\s+(.+)$") {
        Ok(re) => {
            paragraph_re_trim = re;
        }
        Err(_) => return Err(String::from("Could not elaborate regular expressions"))
    }

    let mut body: String;
    match paragraph_re_trim.captures(paragraph) {
        Some(pre_body) => {
            match pre_body.get(1) {
                Some(b) => {
                    body = String::from(b.as_str());
                },
                None => return Err(String::from("Could not extract paragraph body"))
            }
        },
        None => return Err(String::from("Could not extract paragraph body"))
    }


    // split the context into sentences
    let sentences: Vec<&str> = context.split(". ").collect();
    if sentences.len() > 0 {
        let last_sentence = sentences[sentences.len() - 1];

        // Check if the last sentence of the context contains the words "Component(s)" or "shall"
        let components_re;
        match Regex::new(r".*Component.*") {
            Ok(re) => {
                components_re = re;
            }
            Err(_) => return Err(String::from("Could not elaborate regular expression"))
        }
        let shall_re;
        match Regex::new(r".*shall.*") {
            Ok(re) => {
                shall_re = re;
            }
            Err(_) => return Err(String::from("Could not elaborate regular expression"))
        }

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
    Ok(policy)
}

fn build_policy_from_subsection(subsection_number: &String, subsection: &String, ) -> Result<Policy, String> {
    //println!("Subsection number: {}", subsection_number);
    let id = String::from(subsection_number);


    let first_line_re;
    match Regex::new(r"^\s*\d+\.\s+(.+)$") {
        Ok(re) => {
            first_line_re = re;
        }
        Err(_) => return Err(String::from("Could not elaborate regular expression"))
    }

    let mut lines = subsection.lines();
    let mut label = String::new();
    let mut body = String::new();

    if let Some(first_line) = lines.next() {
        match extract_label(first_line, first_line_re) {
            Ok(l) => {
                label = l;
            }
            Err(s) => return Err(s)
        }
        body = lines.collect::<Vec<&str>>().join(" ");
        //println!("Context:\n{}", body);
    }

    let policy = Policy {
        id,
        label,
        context: String::new(),
        body,
    };
    Ok(policy)
}

fn extract_label(line: &str, regular_expr: Regex) -> Result<String, String> {
    match regular_expr.captures(line) {
        Some(pre_label) => {
            match pre_label.get(1) {
                Some(l) => Ok(String::from(l.as_str())),
                None => Err(String::from("Could not extract label"))
            }
        },
        None => Err(String::from("Could not extract label"))
    }
}