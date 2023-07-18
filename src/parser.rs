use std::{fs::File, io::Read};
use regex::Regex;
use walkdir::WalkDir;
use serde::{Serialize, Deserialize};

use crate::writer::Writer;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsedApi {
    pub endpoint: String,
    pub http_method: String,
    pub params: Vec<String>,
    pub action: String,
    pub file: Option<String>,
	pub return_type: String
}

impl UsedApi {
    pub fn new(endpoint: String, http_method: String, params: Vec<String>, action: String, file: Option<String>, return_type: String) -> Self {
        UsedApi {
            endpoint,
            http_method,
            params,
            action,
            file,
			return_type
        }
    }

    pub fn to_string(&self) -> String {
        let par = if self.params.len() > 0 {
            self.params.join(", ")
        } else {
            "-".to_string()
        };

        format!(
            "| {} | {} | {} | {} | {}",
            self.endpoint,
            self.http_method,
            par,
            self.action,
            self.file.as_ref().map_or("", |f| f.as_str())
        )
    }
}

#[derive(Debug)]
pub struct ApiParser {
	out: Writer,
}

impl ApiParser {
	pub fn new(writer: Writer) -> Self {
		ApiParser {
			out: writer
		}
	}

	pub fn parse_file(&mut self, filename: String) -> Vec<UsedApi> {
		let mut file:File;
		match File::open(&filename) {
			Ok(f) => file = f,
			Err(_) => {
				self.out.println(format!("File {} does not exist", filename).as_str());
				std::process::exit(1);
			}
		};
		let mut text = String::new();
		file.read_to_string(&mut text).unwrap();

		self.parse_text(text, Some(filename.as_str()))
	}

	pub fn parse_text(&mut self, text: String, file: Option<&str>) -> Vec<UsedApi> {
		let mut apis: Vec<UsedApi> = Vec::new();
		let re = Regex::new(r#"@(\w+)\("{0,1}([^\n()]+)"{0,1}\)\s+[\w_]+(<[^"{}]+>){0,1}\s*([\w_]+)\(([^\n{}]*)\);"#).unwrap();

		for capture in re.captures_iter(text.as_str()) {
			let http_method = capture.get(1).unwrap().as_str().to_owned();
			let endpoint = capture.get(2).unwrap().as_str().to_owned();
			// If it ends with ", then remove it
			let endpoint = if endpoint.ends_with("\"") {
				endpoint[..endpoint.len()-1].to_owned()
			} else {
				endpoint
			};
			// If it does not start with /, then add it
			let endpoint = if !endpoint.starts_with("/") {
				format!("/{}", endpoint)
			} else {
				endpoint
			};
			
			if capture.get(5).is_some() {
				let return_type = match capture.get(3) {
					Some(t) => t.as_str().to_owned(),
					None => String::from("-")
				};
				let method_name = match capture.get(4) {
					Some(m) => m.as_str().to_owned(),
					None => String::new()
				};
				let parameters_str = match capture.get(5) {
					Some(p) => p.as_str(),
					None => ""
				};

				// let method_name = capture.get(4).unwrap().as_str().to_owned();
				// let parameters_str = capture.get(5).unwrap().as_str();
				let parameters: Vec<String> = if parameters_str.is_empty() {
					Vec::new()
				} else {
					parameters_str.split(',').map(|p| p.trim().to_owned()).collect()
				};

				apis.push(UsedApi::new(
					endpoint,
					http_method,
					parameters,
					camel_case_to_text(method_name),
					Some(file.unwrap().clone().to_owned()),
					return_type
				));
			} else {
				let method_name = match capture.get(3) {
					Some(t) => t.as_str().to_owned(),
					None => String::new()
				};
				let parameters_str = match capture.get(4) {
					Some(m) => m.as_str().to_owned(),
					None => String::new()
				};

				// let method_name = capture.get(3).unwrap().as_str().to_owned();
				// let parameters_str = capture.get(4).unwrap().as_str();
				let parameters: Vec<String> = if parameters_str.is_empty() {
					Vec::new()
				} else {
					parameters_str.split(',')
						.map(|p| p.trim().to_owned())
						.map(// Split on ' ' and remove last element
							|p| p.split(' ')
								.map(|s| s.trim().to_owned())
								.collect::<Vec<String>>()
								.into_iter()
								.take_while(|s| !s.is_empty())
								.collect::<Vec<String>>()
								.join(" ")
							)
						.collect()
				};

				apis.push(UsedApi::new(endpoint, http_method, parameters,
					camel_case_to_text(method_name), Some(file.unwrap().clone().to_owned()), String::from("-")
				));
			}
		}
		
		apis
	}

	pub fn parse_dir(&mut self, dir: String) -> Vec<UsedApi> {
		let blacklist: Vec<&str> = vec!["resources", "assets", "static", "kotlinx"];
		let mut apis: Vec<UsedApi> = Vec::new();
		let mut i = 0;

		let entries = WalkDir::new(dir)
			.into_iter()
			.filter(|e| /*No element of blacklist is in e */ {
				let path = e.as_ref().unwrap().path();
				!blacklist.iter().any(|b| path.to_string_lossy().contains(b))
			});
		
		for entry in entries {
			let entry = entry.unwrap();
			let path = entry.path();
			
			if path.is_file() && path.extension().map_or(false, |ext| ext == "java") {
				let t = self.parse_file(path.to_string_lossy().to_string());
				i += t.len();
				apis.extend(t);
			}
		}
		println!("Found {} APIs", i);
		apis
	}

	pub fn print_md_table(&mut self, apis: Vec<UsedApi>) {
		let mut apis = apis;
		apis.sort_by(|a, b| a.endpoint.cmp(&b.endpoint));

		self.out.println("| Endpoint | Method | Request Body | Action | File |");
		self.out.println("|:---------|:-------|:-------------|:-------|:-----|");
		apis.iter().for_each(|api| self.out.println(api.to_string().as_str()));
	}

	pub fn print_json(&mut self, apis: Vec<UsedApi>) {
		let mut apis = apis;
		apis.sort_by(|a, b| a.endpoint.cmp(&b.endpoint));

		self.out.println(serde_json::to_string_pretty(&apis).unwrap().as_str());
	}
}

fn camel_case_to_text(s: String) -> String {
	let mut result = String::new();
	let mut last = ' ';
	for c in s.chars() {
		if c.is_uppercase() && !last.is_uppercase() {
			result.push(' ');
		}
		result.push(c.to_ascii_lowercase());
		last = c;
	}
	// First char to uppercase
	let mut chars = result.chars();
	let first = chars.next().unwrap();
	result = first.to_uppercase().collect::<String>() + chars.as_str();
	result
}