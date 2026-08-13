use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::PathBuf;

use super::source_organization::{AuditStatus, SourceAudit, all_source_audits};
use super::*;

const SET_IDENTITY_COUNT: usize = 1_686;
// The catalog names transforming identities by their front face, while the
// reference inventory names the same identities with both face names.
const SET_IDENTITY_FINGERPRINT: u64 = 17_770_363_390_521_155_439;
const COMPLETE_IDENTITY_COUNT: usize = 839;
const PARTIAL_IDENTITY_COUNT: usize = 39;
const METADATA_ONLY_IDENTITY_COUNT: usize = 0;
const BLOCKED_IDENTITY_COUNT: usize = 808;
const STANDARD_SET_CODES: &[&str] = &["ISD", "DKA", "AVR", "M13", "RTR", "GTC", "DGM", "M14"];

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

#[test]
#[allow(clippy::too_many_lines)]
fn every_incomplete_isd_rtr_identity_has_one_audited_capability_gap() {
    let catalog = crate::card::catalog().expect("built-in catalog");
    let mut audited = HashMap::new();
    let mut set_names = BTreeSet::new();

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for SourceAudit {
        set_code,
        name,
        status,
        gap,
    } in all_source_audits(&root)
    {
        let cataloged_legal_reprint = catalog
            .find_by_name(&name)
            .is_some_and(|id| catalog.is_allowed_in(id, Format::IsdRtrStandard));
        if !STANDARD_SET_CODES.contains(&set_code.as_str()) && !cataloged_legal_reprint {
            continue;
        }
        assert!(!gap.is_empty(), "{name} has no capability-gap explanation");
        assert!(
            audited
                .insert(name.to_lowercase(), (name.clone(), status))
                .is_none(),
            "{name} appears more than once in the ISD-RTR audit"
        );
        set_names.insert(name.clone());
    }

    let mut complete = 0;
    let mut cataloged_incomplete = HashSet::new();
    for definition in catalog.definitions() {
        if !catalog.is_allowed_in(definition.id, Format::IsdRtrStandard) {
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
                    "partial card {} needs a matching audit entry",
                    definition.name
                );
            }
            ImplementationStatus::MetadataOnly => {
                cataloged_incomplete.insert(key.clone());
                assert_eq!(
                    audited.get(&key).map(|entry| entry.1),
                    Some(AuditStatus::MetadataOnly),
                    "metadata-only card {} needs a matching audit entry",
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

    assert_eq!(complete, COMPLETE_IDENTITY_COUNT);
    assert_eq!(
        audited
            .values()
            .filter(|(_, status)| *status == AuditStatus::Partial)
            .count(),
        PARTIAL_IDENTITY_COUNT
    );
    assert_eq!(
        audited
            .values()
            .filter(|(_, status)| *status == AuditStatus::MetadataOnly)
            .count(),
        METADATA_ONLY_IDENTITY_COUNT
    );
    assert_eq!(
        audited
            .values()
            .filter(|(_, status)| *status == AuditStatus::Blocked)
            .count(),
        BLOCKED_IDENTITY_COUNT
    );
    assert_eq!(
        complete + audited.len(),
        SET_IDENTITY_COUNT,
        "the completed catalog and incomplete audit must partition every ISD-RTR identity"
    );
    assert_eq!(set_names.len(), SET_IDENTITY_COUNT);
    assert_eq!(
        identity_fingerprint(&set_names),
        SET_IDENTITY_FINGERPRINT,
        "the exact ISD-RTR set identity inventory changed"
    );
}
