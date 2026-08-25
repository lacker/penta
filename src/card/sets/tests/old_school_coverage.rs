use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use super::source_organization::{AuditStatus, SourceAudit, source_audits_for_format};
use super::*;

#[test]
fn old_school_catalog_and_implementation_audits_are_consistent() {
    let catalog = crate::card::catalog().expect("built-in catalog");
    let mut audited = HashMap::new();

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for SourceAudit {
        name, status, gap, ..
    } in source_audits_for_format(&root, &catalog, Format::OldSchool9394)
    {
        if status == AuditStatus::Custom {
            continue;
        }
        assert!(!gap.is_empty(), "{name} has no capability-gap explanation");
        assert!(
            audited
                .insert(name.to_lowercase(), (name.clone(), status))
                .is_none(),
            "{name} appears more than once in the Old School audit"
        );
    }

    let mut cataloged_incomplete = HashSet::new();
    for definition in catalog.definitions() {
        if !catalog.is_allowed_in(definition.id, Format::OldSchool9394) {
            continue;
        }
        let key = definition.name.to_lowercase();
        match definition.implementation_status() {
            ImplementationStatus::Complete => {
                assert!(
                    !audited.contains_key(&key),
                    "completed card {} still appears in the incomplete audit",
                    definition.name
                );
            }
            ImplementationStatus::Partial => {
                cataloged_incomplete.insert(key.clone());
                assert_eq!(
                    audited.get(&key).map(|entry| entry.1),
                    Some(AuditStatus::Partial),
                    "partial card {} needs a matching audit row",
                    definition.name
                );
            }
            ImplementationStatus::MetadataOnly => {
                cataloged_incomplete.insert(key.clone());
                assert_eq!(
                    audited.get(&key).map(|entry| entry.1),
                    Some(AuditStatus::MetadataOnly),
                    "metadata-only card {} needs a matching audit row",
                    definition.name
                );
            }
        }
    }

    for (key, (name, status)) in &audited {
        if *status == AuditStatus::Blocked {
            assert!(
                catalog.find_by_name(name).is_none(),
                "blocked card {name} is now cataloged; implement it or update the audit"
            );
        } else {
            assert!(
                cataloged_incomplete.contains(key),
                "{name} is marked {status:?} but has no matching incomplete definition"
            );
        }
    }
}
