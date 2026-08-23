use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::PathBuf;

use super::source_organization::{AuditStatus, SourceAudit, source_audits_for_format};
use super::*;

const SET_IDENTITY_COUNT: usize = 988;
const SET_IDENTITY_FINGERPRINT: u64 = 587_596_519_181_014_230;

fn identity_fingerprint(names: &BTreeSet<String>) -> u64 {
    const FNV_OFFSET_BASIS: u64 = 14_695_981_039_346_656_037;
    const FNV_PRIME: u64 = 1_099_511_628_211;

    let mut fingerprint = FNV_OFFSET_BASIS;
    for name in names {
        for byte in name.bytes().chain(std::iter::once(b'\n')) {
            fingerprint ^= u64::from(byte);
            fingerprint = fingerprint.wrapping_mul(FNV_PRIME);
        }
    }
    fingerprint
}

fn assert_exact_set_identity_inventory(names: &BTreeSet<String>) {
    assert_eq!(
        identity_fingerprint(names),
        SET_IDENTITY_FINGERPRINT,
        "the exact 93/94 set identity inventory changed"
    );
}

#[test]
fn every_incomplete_old_school_set_identity_has_one_audited_capability_gap() {
    let catalog = crate::card::catalog().expect("built-in catalog");
    let mut audited = HashMap::new();
    let mut set_names = BTreeSet::new();

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
        set_names.insert(name.clone());
    }

    let mut complete = 0;
    let mut cataloged_incomplete = HashSet::new();
    for definition in catalog.definitions() {
        if !catalog.is_allowed_in(definition.id, Format::OldSchool9394) {
            continue;
        }
        let key = definition.name.to_lowercase();
        set_names.insert(definition.name.clone());
        match definition.implementation_status() {
            ImplementationStatus::Complete => {
                complete += 1;
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

    assert_eq!(
        complete + audited.len(),
        SET_IDENTITY_COUNT,
        "the completed catalog and incomplete audit must partition every identity in the 93/94 sets"
    );
    assert_exact_set_identity_inventory(&set_names);
}
