use clap::{Parser, Subcommand};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process;

/// Markdown Node Editor - Parse and edit markdown files by node path
#[derive(Parser)]
#[command(name = "md-cli")]
#[command(about = "Markdown Node Editor", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Parse {
        #[arg(value_name = "FILE")]
        path: PathBuf,
    },
    Show {
        #[arg(value_name = "FILE")]
        path: PathBuf,
        node: String,
    },
    Replace {
        #[arg(value_name = "FILE")]
        path: PathBuf,
        node: String,
        content: String,
    },
    Insert {
        #[arg(value_name = "FILE")]
        path: PathBuf,
        node: String,
        content: String,
    },
    Delete {
        #[arg(value_name = "FILE")]
        path: PathBuf,
        node: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkdownNode {
    pub path: String,
    #[serde(rename = "type")]
    pub node_type: NodeType,
    pub level: Option<u8>,
    pub heading: Option<String>,
    pub content: String,
    pub start_line: usize,
    pub end_line: usize,
    #[serde(default)]
    pub children: Vec<MarkdownNode>,
    pub parent_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum NodeType {
    Heading,
    Paragraph,
    CodeBlock,
    List,
    Blockquote,
    Table,
    Hr,
    Empty,
}

impl std::fmt::Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeType::Heading => write!(f, "heading"),
            NodeType::Paragraph => write!(f, "paragraph"),
            NodeType::CodeBlock => write!(f, "code_block"),
            NodeType::List => write!(f, "list"),
            NodeType::Blockquote => write!(f, "blockquote"),
            NodeType::Table => write!(f, "table"),
            NodeType::Hr => write!(f, "hr"),
            NodeType::Empty => write!(f, "empty"),
        }
    }
}

struct MarkdownParser {
    heading_re: Regex,
    code_block_re: Regex,
    list_re: Regex,
    blockquote_re: Regex,
    table_re: Regex,
    hr_re: Regex,
    empty_re: Regex,
}

impl MarkdownParser {
    fn new() -> Self {
        Self {
            heading_re: Regex::new(r"^(#{1,6})\s+(.+)$").unwrap(),
            code_block_re: Regex::new(r"^```").unwrap(),
            list_re: Regex::new(r"^\s*([-*+]|\d+\.)\s+").unwrap(),
            blockquote_re: Regex::new(r"^>\s?").unwrap(),
            table_re: Regex::new(r"^\|").unwrap(),
            hr_re: Regex::new(r"^[-*_]{3,}\s*$").unwrap(),
            empty_re: Regex::new(r"^\s*$").unwrap(),
        }
    }

    fn detect_node_type(&self, line: &str) -> NodeType {
        if self.heading_re.is_match(line) {
            NodeType::Heading
        } else if self.code_block_re.is_match(line) {
            NodeType::CodeBlock
        } else if self.list_re.is_match(line) {
            NodeType::List
        } else if self.blockquote_re.is_match(line) {
            NodeType::Blockquote
        } else if self.table_re.is_match(line) {
            NodeType::Table
        } else if self.hr_re.is_match(line) {
            NodeType::Hr
        } else if self.empty_re.is_match(line) {
            NodeType::Empty
        } else {
            NodeType::Paragraph
        }
    }

    fn parse_heading(&self, line: &str) -> Option<(u8, String)> {
        self.heading_re.captures(line).map(|caps| {
            let level = caps.get(1).unwrap().as_str().len() as u8;
            let heading = caps.get(2).unwrap().as_str().trim().to_string();
            (level, heading)
        })
    }

    fn is_code_block_start(&self, line: &str) -> bool {
        self.code_block_re.is_match(line) && !line.trim().ends_with("```")
    }

    fn parse_nodes(&self, content: &str) -> Vec<MarkdownNode> {
        let mut nodes = Vec::new();
        let mut node_index: Vec<usize> = vec![0; 7];
        let mut current_path = String::new();
        let mut current_level: u8 = 0;

        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i];
            let node_type = self.detect_node_type(line);

            if self.is_code_block_start(line) {
                let start_line = i + 1;
                let mut code_content = String::new();
                i += 1;

                while i < lines.len() {
                    if self.code_block_re.is_match(lines[i]) && lines[i].trim() == "```" {
                        i += 1;
                        break;
                    }
                    code_content.push_str(lines[i]);
                    code_content.push('\n');
                    i += 1;
                }

                node_index[0] += 1;
                let path = if current_path.is_empty() {
                    node_index[0].to_string()
                } else {
                    format!("{}.{}", current_path, node_index[current_level as usize + 1])
                };

                nodes.push(MarkdownNode {
                    path,
                    node_type: NodeType::CodeBlock,
                    level: None,
                    heading: None,
                    content: code_content.trim_end().to_string(),
                    start_line,
                    end_line: i,
                    children: vec![],
                    parent_path: if current_path.is_empty() { None } else { Some(current_path.clone()) },
                });
                continue;
            }

            if node_type == NodeType::Empty {
                i += 1;
                continue;
            }

            if node_type == NodeType::Heading {
                let (level, heading) = self.parse_heading(line).unwrap();
                let start_line = i + 1;

                // Find parent path
                let parent_path = if level == 1 {
                    None
                } else {
                    let mut found = false;
                    for node in nodes.iter().rev() {
                        if node.node_type == NodeType::Heading {
                            if let Some(nl) = node.level {
                                if nl < level {
                                    current_path = node.path.clone();
                                    found = true;
                                    break;
                                }
                            }
                        }
                    }
                    if !found {
                        current_path = String::new();
                    }
                    if current_path.is_empty() { None } else { Some(current_path.clone()) }
                };

                node_index[level as usize] += 1;
                for l in (level as usize + 1)..=6 {
                    node_index[l] = 0;
                }

                let path = if level == 1 {
                    node_index[1].to_string()
                } else if current_path.is_empty() {
                    format!("{}", node_index[1])
                } else {
                    format!("{}.{}", current_path, node_index[level as usize])
                };

                current_level = level;
                current_path = path.clone();

                nodes.push(MarkdownNode {
                    path,
                    node_type: NodeType::Heading,
                    level: Some(level),
                    heading: Some(heading),
                    content: line.to_string(),
                    start_line,
                    end_line: start_line,
                    children: vec![],
                    parent_path,
                });
                i += 1;
                continue;
            }

            // Paragraph or other content
            let start_line = i + 1;
            let mut paragraph_content = String::new();

            while i < lines.len() {
                let curr_line = lines[i];
                let curr_type = self.detect_node_type(curr_line);

                if curr_type == NodeType::Heading || self.is_code_block_start(curr_line) {
                    break;
                }
                if curr_type == NodeType::Empty && paragraph_content.is_empty() {
                    i += 1;
                    continue;
                }
                if curr_type == NodeType::Empty {
                    i += 1;
                    break;
                }

                paragraph_content.push_str(curr_line);
                paragraph_content.push('\n');
                i += 1;
            }

            if !paragraph_content.trim().is_empty() {
                node_index[0] += 1;
                let path = if current_path.is_empty() {
                    node_index[0].to_string()
                } else {
                    format!("{}.{}", current_path, node_index[current_level as usize + 1])
                };

                let ntype = self.detect_node_type(paragraph_content.trim());

                nodes.push(MarkdownNode {
                    path,
                    node_type: ntype,
                    level: None,
                    heading: None,
                    content: paragraph_content.trim_end().to_string(),
                    start_line,
                    end_line: i,
                    children: vec![],
                    parent_path: if current_path.is_empty() { None } else { Some(current_path.clone()) },
                });
            }
        }

        nodes
    }

    fn find_node<'a>(&self, nodes: &'a [MarkdownNode], path: &str) -> Option<&'a MarkdownNode> {
        nodes.iter().find(|n| n.path == path)
    }

    fn render_tree(&self, nodes: &[MarkdownNode]) -> String {
        let mut output = String::new();
        output.push_str("# Markdown Node Tree\n\n");

        for node in nodes {
            match node.node_type {
                NodeType::Heading => {
                    let indent = "  ".repeat(node.level.unwrap_or(1) as usize - 1);
                    let h = node.heading.as_deref().unwrap_or("");
                    output.push_str(&format!("{}{} [heading] {}\n", indent, node.path, h));
                }
                NodeType::Paragraph => {
                    let preview: String = node.content.chars().take(50).collect();
                    let preview = if node.content.len() > 50 {
                        format!("{}...", preview)
                    } else {
                        preview
                    };
                    output.push_str(&format!("  {} [paragraph] \"{}\"\n", node.path, preview));
                }
                NodeType::CodeBlock => {
                    let preview: String = node.content.chars().take(30).collect();
                    let preview = preview.replace('\n', " ");
                    let preview = if node.content.len() > 30 {
                        format!("{}...", preview)
                    } else {
                        preview
                    };
                    output.push_str(&format!("  {} [code_block] \"{}\"\n", node.path, preview));
                }
                _ => {
                    output.push_str(&format!("  {} [{}]\n", node.path, node.node_type));
                }
            }
        }

        output
    }
}

fn read_file(path: &PathBuf) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))
}

fn write_file(path: &PathBuf, content: &str) -> Result<(), String> {
    fs::write(path, content).map_err(|e| format!("Failed to write file: {}", e))
}

fn parse_range(content: &str, start_line: usize, end_line: usize) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let start = start_line.saturating_sub(1);
    let end = end_line.min(lines.len());
    lines[start..end].join("\n")
}

fn delete_range(content: &str, start_line: usize, end_line: usize) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let start = start_line.saturating_sub(1);
    let end = end_line.min(lines.len());

    let mut result = String::new();
    for (i, line) in lines.iter().enumerate() {
        if i < start || i >= end {
            result.push_str(line);
            result.push('\n');
        }
    }
    result.trim_end().to_string() + "\n"
}

fn replace_range(content: &str, start_line: usize, end_line: usize, new_content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let start = start_line.saturating_sub(1);
    let end = end_line.min(lines.len());

    let mut result = String::new();
    for line in &lines[..start] {
        result.push_str(line);
        result.push('\n');
    }
    result.push_str(new_content);
    if !new_content.is_empty() && !new_content.ends_with('\n') {
        result.push('\n');
    }
    for line in &lines[end..] {
        result.push_str(line);
        result.push('\n');
    }
    result.trim_end().to_string()
}

fn insert_after(content: &str, after_line: usize, new_content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let insert_pos = after_line.min(lines.len());

    let mut result = String::new();
    for (i, line) in lines.iter().enumerate() {
        result.push_str(line);
        result.push('\n');
        if i == insert_pos - 1 {
            result.push_str(new_content);
            if !new_content.ends_with('\n') {
                result.push('\n');
            }
        }
    }
    result.trim_end().to_string()
}

fn main() {
    let cli = Cli::parse();
    let parser = MarkdownParser::new();

    match &cli.command {
        Commands::Parse { path } => {
            let content = match read_file(path) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            };
            let nodes = parser.parse_nodes(&content);
            println!("{}", parser.render_tree(&nodes));
        }

        Commands::Show { path, node } => {
            let content = match read_file(path) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            };
            let nodes = parser.parse_nodes(&content);
            match parser.find_node(&nodes, node) {
                Some(n) => {
                    let node_content = parse_range(&content, n.start_line, n.end_line);
                    println!("Node: {}", n.path);
                    println!("Type: {}", n.node_type);
                    if let Some(l) = n.level {
                        println!("Level: {}", l);
                    }
                    if let Some(h) = &n.heading {
                        println!("Heading: {}", h);
                    }
                    println!("Lines: {}-{}", n.start_line, n.end_line);
                    println!("\n--- Content ---\n{}", node_content);
                }
                None => {
                    eprintln!("Node not found: {}", node);
                    process::exit(1);
                }
            }
        }

        Commands::Replace { path, node, content: new_content } => {
            let content = match read_file(path) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            };
            let nodes = parser.parse_nodes(&content);
            match parser.find_node(&nodes, node) {
                Some(n) => {
                    let new_file_content = replace_range(&content, n.start_line, n.end_line, new_content);
                    if let Err(e) = write_file(path, &new_file_content) {
                        eprintln!("Error writing file: {}", e);
                        process::exit(1);
                    }
                    println!("Replaced node {} (lines {}-{})", node, n.start_line, n.end_line);
                }
                None => {
                    eprintln!("Node not found: {}", node);
                    process::exit(1);
                }
            }
        }

        Commands::Insert { path, node, content: new_content } => {
            let content = match read_file(path) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            };
            let nodes = parser.parse_nodes(&content);
            match parser.find_node(&nodes, node) {
                Some(n) => {
                    let new_file_content = insert_after(&content, n.end_line, new_content);
                    if let Err(e) = write_file(path, &new_file_content) {
                        eprintln!("Error writing file: {}", e);
                        process::exit(1);
                    }
                    println!("Inserted after node {} (line {})", node, n.end_line);
                }
                None => {
                    eprintln!("Node not found: {}", node);
                    process::exit(1);
                }
            }
        }

        Commands::Delete { path, node } => {
            let content = match read_file(path) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            };
            let nodes = parser.parse_nodes(&content);
            match parser.find_node(&nodes, node) {
                Some(n) => {
                    let new_file_content = delete_range(&content, n.start_line, n.end_line);
                    if let Err(e) = write_file(path, &new_file_content) {
                        eprintln!("Error writing file: {}", e);
                        process::exit(1);
                    }
                    println!("Deleted node {} (lines {}-{})", node, n.start_line, n.end_line);
                }
                None => {
                    eprintln!("Node not found: {}", node);
                    process::exit(1);
                }
            }
        }
    }
}
