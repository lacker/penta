use std::collections::HashMap;
use std::fs;
use std::path::Path;

use syn::visit::Visit;

use super::{parse_header, printed_set_files};

#[test]
fn card_local_definition_helpers_are_reused_or_recursive() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut violations = Vec::new();

    for path in printed_set_files(&root.join("src/card/sets")) {
        let source = fs::read_to_string(&path).expect("a printed set source file is readable");
        let declarations = card_local_helper_declarations(&source);
        if declarations.is_empty() {
            continue;
        }

        let syntax = syn::parse_file(&source).unwrap_or_else(|error| {
            panic!("{} must parse as Rust: {error}", path.display());
        });
        let mut uses = declarations
            .iter()
            .map(|declaration| (declaration.name.clone(), 0))
            .collect::<HashMap<_, _>>();
        PathUseCounter { uses: &mut uses }.visit_file(&syntax);

        for declaration in declarations {
            if uses[&declaration.name] <= 1 {
                violations.push(format!(
                    "{}:{}: {} is referenced only once",
                    path.display(),
                    declaration.line,
                    declaration.name
                ));
            }
        }
    }

    let shown = violations
        .iter()
        .take(100)
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join("\n");
    let omitted = violations.len().saturating_sub(100);
    assert!(
        violations.is_empty(),
        "inline card-local helpers unless they are reused or recursive ({} violations; showing up to 100):\n{shown}{}",
        violations.len(),
        if omitted == 0 {
            String::new()
        } else {
            format!("\n... and {omitted} more")
        }
    );
}

struct ValueDeclaration {
    name: String,
    line: usize,
}

fn card_local_helper_declarations(source: &str) -> Vec<ValueDeclaration> {
    let mut inside_card_blocks = false;
    let mut declarations = Vec::new();

    for (index, line) in source.lines().enumerate() {
        if let Some(header) = parse_header(line) {
            inside_card_blocks = header.printing_kind.is_none();
        }
        if !inside_card_blocks {
            continue;
        }

        let value = line
            .strip_prefix("static ")
            .or_else(|| line.strip_prefix("const "));
        let function = line
            .strip_prefix("const fn ")
            .and_then(|body| body.split_once('(').map(|(name, _)| name));
        let name = value
            .and_then(|body| body.split_once(':').map(|(name, _)| name))
            .or(function);
        let Some(name) = name else {
            continue;
        };
        if name
            .bytes()
            .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'_')
        {
            declarations.push(ValueDeclaration {
                name: name.to_string(),
                line: index + 1,
            });
        }
    }

    declarations
}

struct PathUseCounter<'a> {
    uses: &'a mut HashMap<String, usize>,
}

impl<'ast> Visit<'ast> for PathUseCounter<'_> {
    fn visit_path(&mut self, path: &'ast syn::Path) {
        if path.leading_colon.is_none()
            && path.segments.len() == 1
            && let Some(count) = self.uses.get_mut(&path.segments[0].ident.to_string())
        {
            *count += 1;
        }
        syn::visit::visit_path(self, path);
    }
}
