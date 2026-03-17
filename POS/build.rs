use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
struct MediaRule {
    bp: &'static str,
    selector: String,
    prop: String,
    value: String,
}

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");

    let ui_dir = PathBuf::from(&manifest_dir).join("src/ui");
    println!("cargo:rerun-if-changed={}", ui_dir.display());
    let mut css_files = collect_css_files(&ui_dir, Path::new(&manifest_dir));
    css_files.sort();

    let mut output = String::new();

    for rel in css_files.iter() {
        println!("cargo:rerun-if-changed={}", rel);
        let path = PathBuf::from(&manifest_dir).join(rel);
        match fs::read_to_string(&path) {
            Ok(css) => {
                let transformed = transform_css(&css);
                output.push_str(&transformed);
                output.push('\n');
            }
            Err(e) => {
                eprintln!("cargo:warning=Skipping {} ({}).", path.display(), e);
                continue;
            }
        }
    }

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let out_path = PathBuf::from(out_dir).join("styles.css");
    fs::write(&out_path, output).unwrap_or_else(|e| {
        panic!("Failed to write {}: {}", out_path.display(), e)
    });
}

fn collect_css_files(root: &Path, manifest_dir: &Path) -> Vec<String> {
    let mut out = Vec::new();
    let Ok(entries) = fs::read_dir(root) else {
        return out;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            out.extend(collect_css_files(&path, manifest_dir));
            continue;
        }
        let is_css = path
            .extension()
            .and_then(|s| s.to_str())
            .map(|ext| ext.eq_ignore_ascii_case("css"))
            .unwrap_or(false);
        if !is_css {
            continue;
        }
        let Ok(rel) = path.strip_prefix(manifest_dir) else {
            continue;
        };
        if let Some(rel_str) = rel.to_str() {
            out.push(rel_str.to_string());
        }
    }
    out
}

// Note: this is a line-based transformer and does not rewrite nested @media blocks.
fn transform_css(css: &str) -> String {
    let mut out = String::new();

    let mut depth: i32 = 0;
    let mut in_block = false;
    let mut block_header = String::new();
    let mut block_lines: Vec<String> = Vec::new();
    let mut pending: Vec<MediaRule> = Vec::new();

    for line in css.lines() {
        let open = line.matches('{').count() as i32;
        let close = line.matches('}').count() as i32;

        if depth == 0 && open > 0 {
            // Start of a block (including single-line rules)
            let mut parts = line.splitn(2, '{');
            let header = parts.next().unwrap_or("").trim().to_string();
            let rest = parts.next().unwrap_or("");

            // Handle single-line rule: "selector { prop: val; prop2: val2; }"
            if rest.contains('}') && !header.trim_start().starts_with('@') {
                let body = rest.splitn(2, '}').next().unwrap_or("");
                block_lines.push(format!("{} {{", header));
                for decl in body.split(';') {
                    let decl = decl.trim();
                    if decl.is_empty() {
                        continue;
                    }
                    let line = format!("  {};", decl);
                    if let Some(new_line) = process_decl_line(&line, &header, &mut pending) {
                        if !new_line.is_empty() {
                            block_lines.push(new_line);
                        }
                    } else {
                        block_lines.push(line);
                    }
                }
                block_lines.push("}".to_string());
                flush_block(&mut out, &mut block_lines, &mut pending);
                continue;
            }

            in_block = true;
            block_header = header;
            block_lines.push(format!("{}{{{}", block_header, rest));
            depth += open - close;

            if depth == 0 {
                flush_block(&mut out, &mut block_lines, &mut pending);
                in_block = false;
                block_header.clear();
            }
            continue;
        }

        if in_block {
            let mut handled = false;

            if !block_header.trim_start().starts_with('@') && depth == 1 {
                if let Some(new_line) = process_decl_line(line, &block_header, &mut pending) {
                    if !new_line.is_empty() {
                        block_lines.push(new_line);
                    }
                    handled = true;
                }
            }

            if !handled {
                block_lines.push(line.to_string());
            } else if close > 0 {
                // If we consumed a line that contained a closing brace, preserve it.
                block_lines.push("}".to_string());
            }

            depth += open - close;

            if depth == 0 {
                flush_block(&mut out, &mut block_lines, &mut pending);
                in_block = false;
                block_header.clear();
            }
        } else {
            out.push_str(line);
            out.push('\n');
        }
    }

    // If file ends while still in block, flush what we have.
    if in_block {
        flush_block(&mut out, &mut block_lines, &mut pending);
    }

    out
}

fn flush_block(out: &mut String, block_lines: &mut Vec<String>, pending: &mut Vec<MediaRule>) {
    if !block_lines.is_empty() {
        out.push_str(&block_lines.join("\n"));
        out.push('\n');
    }

    if !pending.is_empty() {
        let mut order: Vec<&'static str> = Vec::new();
        let mut grouped: Vec<(&'static str, Vec<MediaRule>)> = Vec::new();

        for rule in pending.drain(..) {
            if let Some(idx) = order.iter().position(|bp| *bp == rule.bp) {
                grouped[idx].1.push(rule);
            } else {
                order.push(rule.bp);
                grouped.push((rule.bp, vec![rule]));
            }
        }

        for (bp, rules) in grouped {
            if rules.is_empty() {
                continue;
            }
            let selector = rules[0].selector.clone();
            out.push_str(&format!("@media (max-width: {}) {{\n", bp));
            out.push_str(&format!("  {} {{\n", selector));
            for r in rules {
                out.push_str(&format!("    {}: {};\n", r.prop, r.value));
            }
            out.push_str("  }\n}\n");
        }
    }

    block_lines.clear();
}

fn process_decl_line(line: &str, selector: &str, pending: &mut Vec<MediaRule>) -> Option<String> {
    let colon = line.find(':')?;
    let semi = line.rfind(';')?;
    if colon >= semi {
        return None;
    }

    let indent: String = line.chars().take_while(|c| c.is_whitespace()).collect();
    let prop = line[indent.len()..colon].trim().to_string();
    let value = line[colon + 1..semi].trim();

    if !value.contains("-(") {
        return None;
    }

    let tokens = extract_tokens(value);
    if tokens.is_empty() {
        return None;
    }

    let mut base_value: Option<String> = None;
    for (key, val) in tokens {
        if key == "base" {
            base_value = Some(val);
            continue;
        }
        if let Some(bp) = map_breakpoint(&key) {
            pending.push(MediaRule {
                bp,
                selector: selector.to_string(),
                prop: prop.clone(),
                value: val,
            });
        }
    }

    if let Some(base) = base_value {
        return Some(format!("{}{}: {};", indent, prop, base));
    }

    // No base provided: drop the line and keep only breakpoint rules.
    Some(String::new())
}

fn map_breakpoint(key: &str) -> Option<&'static str> {
    match key {
        "xs" => Some("480px"),
        "sm" => Some("600px"),
        "md" => Some("768px"),
        "lg" => Some("900px"),
        "xl" => Some("1200px"),
        _ => None,
    }
}

fn extract_tokens(value: &str) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    let chars: Vec<char> = value.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        // find "<key>-(" where key is alphanumeric
        if chars[i].is_ascii_alphanumeric() {
            let start = i;
            while i < chars.len() && chars[i].is_ascii_alphanumeric() {
                i += 1;
            }
            if i + 1 < chars.len() && chars[i] == '-' && chars[i + 1] == '(' {
                let key = chars[start..i].iter().collect::<String>();
                i += 2; // skip "-("
                let val_start = i;
                let mut depth = 1;
                while i < chars.len() {
                    if chars[i] == '(' {
                        depth += 1;
                    } else if chars[i] == ')' {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                    }
                    i += 1;
                }
                if i < chars.len() && chars[i] == ')' {
                    let val = chars[val_start..i].iter().collect::<String>().trim().to_string();
                    out.push((key, val));
                    i += 1;
                    continue;
                }
            }
        }
        i += 1;
    }

    out
}
