#[derive(Debug, Clone)]
pub struct UsedApi {
    endpoint: String,
    method: String,
    params: Vec<String>,
    action: String,
    file: Option<String>,
}

impl UsedApi {
    pub fn new(endpoint: String, method: String, params: Vec<String>, action: String, file: Option<String>) -> Self {
        UsedApi {
            endpoint,
            method,
            params,
            action,
            file,
        }
    }

    pub fn to_string(&self) -> String {
        let par = if self.params.len() > 0 {
            self.params.join(", ")
        } else {
            "-".to_string()
        };

        format!(
            "| {}{} | {} | {} | {} | {}",
            if self.endpoint.chars().next() != Some('/') { "/" } else { "" },
            self.endpoint,
            self.method,
            par,
            self.action,
            self.file.as_ref().map_or("", |f| f.as_str())
        )
    }
}
