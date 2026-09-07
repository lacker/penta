//! Mechanic names exist only at declaration/diagnostic time. Validate their
//! numeric identities without maintaining a second central list of names.

use crate::ids::MechanicId;
use std::collections::BTreeMap;
use std::path::Path;
use syn::visit::Visit;

#[derive(Default)]
struct Declarations(BTreeMap<MechanicId, String>);

impl<'ast> Visit<'ast> for Declarations {
    fn visit_expr_call(&mut self, call: &'ast syn::ExprCall) {
        if let syn::Expr::Path(path) = call.func.as_ref() {
            let segments = path
                .path
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect::<Vec<_>>();
            if segments.ends_with(&["MechanicId".into(), "from_name".into()]) {
                let arguments = call.args.iter().collect::<Vec<_>>();
                let [
                    syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(name),
                        ..
                    }),
                ] = arguments.as_slice()
                else {
                    panic!("declare mechanic IDs from literal names, then import the constant");
                };
                let name = name.value();
                let id = MechanicId::from_name(&name);
                assert!(
                    self.0.insert(id, name.clone()).is_none(),
                    "duplicate or colliding mechanic ID: {name}"
                );
            }
        }
        syn::visit::visit_expr_call(self, call);
    }
}

fn visit_sources(path: &Path, declarations: &mut Declarations) {
    for entry in std::fs::read_dir(path).unwrap() {
        let path = entry.unwrap().path();
        if path.file_stem().is_some_and(|name| name == "tests") {
            continue;
        }
        if path.is_dir() {
            visit_sources(&path, declarations);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            declarations
                .visit_file(&syn::parse_file(&std::fs::read_to_string(&path).unwrap()).unwrap());
        }
    }
}

#[test]
fn named_mechanics_have_unique_globally_stable_numeric_identities() {
    let mut declarations = Declarations::default();
    visit_sources(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("src/card"),
        &mut declarations,
    );
    assert!(!declarations.0.is_empty());
    assert_eq!(
        std::mem::size_of::<MechanicId>(),
        std::mem::size_of::<u64>()
    );
    // Frozen FNV-1a test vector; moving a declaration must not alter it.
    assert_eq!(
        MechanicId::from_name("mtg:forage").get(),
        0xa7c1_4960_3568_481f
    );
}
