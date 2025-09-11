pub struct Crq {
    pub problem_goal: String,
    pub proposed_solution: String,
    pub justification_impact: String,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum NextStep {
    Develop,
    Refactor,
    Document,
    RespondTo,
    CoderabbitAIReview, // New variant
    Unknown,
}

pub fn determine_next_step(crq_content: &str) -> NextStep {
    let lower_content = crq_content.to_lowercase();
    let content_byte_size = crq_content.as_bytes().len();

    // Rule 1: CoderabbitAI Review based on size
    if content_byte_size < 2500 { // N = 2500 bytes
        return NextStep::CoderabbitAIReview;
    }

    // Rule 2: Respond To / Our Turn (if not already classified for CoderabbitAI Review)
    if lower_content.contains("clarify") || lower_content.contains("question") || lower_content.contains("decision") || lower_content.contains("feedback") || lower_content.contains("input") || lower_content.contains("discuss") || lower_content.contains("confirm") || lower_content.contains("review") || lower_content.contains("audit") || lower_content.contains("verify") || lower_content.contains("assess") || lower_content.contains("reflect") || lower_content.contains("strategic") || lower_content.contains("conceptual") || lower_content.contains("architecture") || lower_content.contains("plan") {
        NextStep::RespondTo
    } else if lower_content.contains("develop") || lower_content.contains("implement") || lower_content.contains("build") {
        NextStep::Develop
    } else if lower_content.contains("refactor") || lower_content.contains("restructure") || lower_content.contains("improve") {
        NextStep::Refactor
    } else if lower_content.contains("document") || lower_content.contains("sop") || lower_content.contains("guide") {
        NextStep::Document
    } else {
        NextStep::Unknown
    }
}

pub fn parse_crq(content: &str) -> Option<Crq> {
    let mut problem_goal = String::new();
    let mut proposed_solution = String::new();
    let mut justification_impact = String::new();

    // Try to extract the main title as the problem_goal
    for line in content.lines() {
        let trimmed_line = line.trim();
        if trimmed_line.starts_with("# CRQ:") || trimmed_line.starts_with("## Change Request:") || trimmed_line.starts_with("**CRQ:") {
            problem_goal = trimmed_line.trim_start_matches("# CRQ:").trim_start_matches("## Change Request:").trim_start_matches("**CRQ:").trim().to_string();
            // Remove markdown formatting from the title
            problem_goal = problem_goal.replace("**", "").trim().to_string();
            break;
        }
    }

    // Fallback if no clear title is found (e.g., use first few lines or filename)
    if problem_goal.is_empty() {
        problem_goal = content.lines().next().unwrap_or("Untitled CRQ").trim().to_string();
        // If it's still a markdown heading, clean it up
        if problem_goal.starts_with('#') {
            problem_goal = problem_goal.trim_start_matches('#').trim().to_string();
        }
    }

    // For now, proposed_solution and justification_impact will remain empty or be filled with a placeholder
    // if no specific sections are found. This can be improved later if needed.
    proposed_solution = "N/A".to_string();
    justification_impact = "N/A".to_string();

    Some(Crq {
        problem_goal,
        proposed_solution,
        justification_impact,
    })
}
