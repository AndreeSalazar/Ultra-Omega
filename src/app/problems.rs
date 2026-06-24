#[derive(Clone, Debug)]
pub struct ProblemEntry {
    pub message: String,
    pub line: Option<usize>,
    pub is_error: bool,
}

#[derive(Clone, Debug, Default)]
pub struct ProblemsPanel {
    pub problems: Vec<ProblemEntry>,
}

impl ProblemsPanel {
    pub fn from_output(output: &crate::vulkan::renderer::OutputPanel) -> Self {
        let mut problems = Vec::new();
        if !output.has_run {
            return Self { problems };
        }
        for line in &output.lines {
            if line.starts_with(">>>") || line.starts_with("<<<") || line.starts_with("[stderr]") {
                continue;
            }
            if !line.trim().is_empty() {
                let line_num = output.error_line;
                problems.push(ProblemEntry {
                    message: line.clone(),
                    line: line_num,
                    is_error: output.is_error,
                });
            }
        }
        Self { problems }
    }

    pub fn error_count(&self) -> usize {
        self.problems.iter().filter(|p| p.is_error).count()
    }

    pub fn warning_count(&self) -> usize {
        self.problems.iter().filter(|p| !p.is_error).count()
    }
}
