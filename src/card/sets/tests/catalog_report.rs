use std::fmt::Write;
use std::path::Path;

use super::source_organization::{AuditStatus, source_audits_for_format};
use super::*;
use crate::card::CardDefinition;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct CatalogCoverage {
    complete: usize,
    partial: usize,
    metadata_only: usize,
}

impl CatalogCoverage {
    fn from_definitions<'a>(definitions: impl IntoIterator<Item = &'a CardDefinition>) -> Self {
        let mut coverage = Self::default();
        for definition in definitions {
            match definition.implementation_status() {
                ImplementationStatus::Complete => coverage.complete += 1,
                ImplementationStatus::Partial => coverage.partial += 1,
                ImplementationStatus::MetadataOnly => coverage.metadata_only += 1,
            }
        }
        coverage
    }

    const fn total(self) -> usize {
        self.complete + self.partial + self.metadata_only
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct FormatCoverage {
    catalog: CatalogCoverage,
    blocked: usize,
}

impl FormatCoverage {
    fn from_repository(root: &Path, catalog: &crate::card::CardCatalog, format: Format) -> Self {
        let catalog_coverage = CatalogCoverage::from_definitions(
            catalog
                .definitions()
                .into_iter()
                .filter(|definition| catalog.is_allowed_in(definition.id, format)),
        );
        let blocked = source_audits_for_format(root, catalog, format)
            .into_iter()
            .filter(|audit| audit.status == AuditStatus::Blocked)
            .count();
        Self {
            catalog: catalog_coverage,
            blocked,
        }
    }

    const fn total(self) -> usize {
        self.catalog.total() + self.blocked
    }
}

fn write_catalog_coverage(report: &mut String, coverage: CatalogCoverage) {
    writeln!(report, "    complete       {:>6}", coverage.complete)
        .expect("writing to a String cannot fail");
    writeln!(report, "    partial        {:>6}", coverage.partial)
        .expect("writing to a String cannot fail");
    writeln!(report, "    metadata-only  {:>6}", coverage.metadata_only)
        .expect("writing to a String cannot fail");
}

fn write_format_coverage(
    report: &mut String,
    format: Format,
    qualifier: Option<&str>,
    coverage: FormatCoverage,
) {
    write!(report, "  {}", format.display_name()).expect("writing to a String cannot fail");
    if let Some(qualifier) = qualifier {
        write!(report, " ({qualifier})").expect("writing to a String cannot fail");
    }
    writeln!(report).expect("writing to a String cannot fail");
    write_catalog_coverage(report, coverage.catalog);
    writeln!(report, "    blocked        {:>6}", coverage.blocked)
        .expect("writing to a String cannot fail");
    writeln!(report, "    total          {:>6}", coverage.total())
        .expect("writing to a String cannot fail");
}

fn render_report(
    repository: CatalogCoverage,
    old_school: FormatCoverage,
    standard: FormatCoverage,
) -> String {
    let mut report = String::new();
    writeln!(report, "Catalog coverage").expect("writing to a String cannot fail");
    writeln!(report, "================").expect("writing to a String cannot fail");
    writeln!(report).expect("writing to a String cannot fail");
    writeln!(
        report,
        "Repository catalog definitions (all sets and synthetic objects)"
    )
    .expect("writing to a String cannot fail");
    write_catalog_coverage(&mut report, repository);
    writeln!(report, "    total          {:>6}", repository.total())
        .expect("writing to a String cannot fail");
    writeln!(report).expect("writing to a String cannot fail");
    writeln!(report, "Audited set identity corpora").expect("writing to a String cannot fail");
    write_format_coverage(
        &mut report,
        Format::OldSchool9394,
        Some("including banned identities"),
        old_school,
    );
    write_format_coverage(&mut report, Format::IsdRtrStandard, None, standard);
    report
}

fn repository_report() -> String {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let catalog = crate::card::catalog().expect("built-in catalog");
    render_report(
        CatalogCoverage::from_definitions(catalog.definitions()),
        FormatCoverage::from_repository(root, &catalog, Format::OldSchool9394),
        FormatCoverage::from_repository(root, &catalog, Format::IsdRtrStandard),
    )
}

#[test]
fn report_layout_names_repository_and_format_coverage() {
    let report = render_report(
        CatalogCoverage {
            complete: 12,
            partial: 3,
            metadata_only: 1,
        },
        FormatCoverage {
            catalog: CatalogCoverage {
                complete: 7,
                partial: 2,
                metadata_only: 1,
            },
            blocked: 20,
        },
        FormatCoverage {
            catalog: CatalogCoverage {
                complete: 5,
                partial: 1,
                metadata_only: 0,
            },
            blocked: 10,
        },
    );

    assert_eq!(
        report,
        concat!(
            "Catalog coverage\n",
            "================\n",
            "\n",
            "Repository catalog definitions (all sets and synthetic objects)\n",
            "    complete           12\n",
            "    partial             3\n",
            "    metadata-only       1\n",
            "    total              16\n",
            "\n",
            "Audited set identity corpora\n",
            "  Old School 93/94 (including banned identities)\n",
            "    complete            7\n",
            "    partial             2\n",
            "    metadata-only       1\n",
            "    blocked            20\n",
            "    total              30\n",
            "  ISD-RTR Standard\n",
            "    complete            5\n",
            "    partial             1\n",
            "    metadata-only       0\n",
            "    blocked            10\n",
            "    total              16\n",
        )
    );
}

#[test]
fn print_catalog_report() {
    print!("{}", repository_report());
}
