pub fn choose_policy_language(language: &String) -> Result<String, String> {
    match language.as_str() {
        "ALFA" => Ok(String::from("I have this policy. Can you translate it to ALFA (Abbreviated Language For Authorization)?")),
        "ODRL" => Ok(String::from("I have this policy. Can you translate it to ODRL (Open Digital Rights Language)?")),
        "MY DATA" => Ok(String::from("I have this policy. Can you translate it to MY DATA Control Technologies policy language?")),
        "XACML" => Ok(String::from("I have this policy. Can you translate it to XACML (eXtensible Access Control Markup Language)?")),
        _ => Err(String::from("Unrecognized policy language."))
    }
}