use std::fmt::Write;
use std::path::Path;

use super::source_organization::{AuditStatus, source_audits_for_format};
use super::*;
use crate::card::CardDefinition;
use crate::format::vintage_cube::VINTAGE_CUBE_POOL;

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

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct PoolCoverage {
    complete: Vec<String>,
    partial: Vec<String>,
    metadata_only: Vec<String>,
    uncataloged: Vec<String>,
}

impl PoolCoverage {
    fn from_catalog(catalog: &crate::card::CardCatalog, pool: &[&str]) -> Self {
        let mut coverage = Self::default();
        for &name in pool {
            let Some(definition) = catalog
                .find_by_name(name)
                .and_then(|definition| catalog.get(definition))
            else {
                coverage.uncataloged.push(name.to_owned());
                continue;
            };
            match definition.implementation_status() {
                ImplementationStatus::Complete => coverage.complete.push(name.to_owned()),
                ImplementationStatus::Partial => coverage.partial.push(name.to_owned()),
                ImplementationStatus::MetadataOnly => {
                    coverage.metadata_only.push(name.to_owned());
                }
            }
        }
        coverage
    }

    const fn catalog_coverage(&self) -> CatalogCoverage {
        CatalogCoverage {
            complete: self.complete.len(),
            partial: self.partial.len(),
            metadata_only: self.metadata_only.len(),
        }
    }

    const fn total(&self) -> usize {
        self.complete.len() + self.partial.len() + self.metadata_only.len() + self.uncataloged.len()
    }
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

fn write_pool_names(report: &mut String, status: &str, names: &[String]) {
    writeln!(report, "    {status} cards").expect("writing to a String cannot fail");
    for name in names {
        writeln!(report, "      - {name}").expect("writing to a String cannot fail");
    }
}

fn write_pool_coverage(report: &mut String, coverage: &PoolCoverage, verbose: bool) {
    writeln!(report, "  Vintage Cube").expect("writing to a String cannot fail");
    write_catalog_coverage(report, coverage.catalog_coverage());
    writeln!(
        report,
        "    uncataloged    {:>6}",
        coverage.uncataloged.len()
    )
    .expect("writing to a String cannot fail");
    writeln!(report, "    total          {:>6}", coverage.total())
        .expect("writing to a String cannot fail");

    if verbose {
        write_pool_names(report, "complete", &coverage.complete);
        write_pool_names(report, "partial", &coverage.partial);
        write_pool_names(report, "metadata-only", &coverage.metadata_only);
        write_pool_names(report, "uncataloged", &coverage.uncataloged);
    }
}

fn render_report(
    repository: CatalogCoverage,
    old_school: FormatCoverage,
    standard: FormatCoverage,
    vintage_cube: &PoolCoverage,
    verbose: bool,
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
    write_format_coverage(&mut report, Format::IsdDgmStandard, None, standard);
    writeln!(report).expect("writing to a String cannot fail");
    writeln!(report, "Fixed card pools").expect("writing to a String cannot fail");
    write_pool_coverage(&mut report, vintage_cube, verbose);
    report
}

fn repository_report(verbose: bool) -> String {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let catalog = crate::card::catalog().expect("built-in catalog");
    render_report(
        CatalogCoverage::from_definitions(catalog.definitions()),
        FormatCoverage::from_repository(root, &catalog, Format::OldSchool9394),
        FormatCoverage::from_repository(root, &catalog, Format::IsdDgmStandard),
        &PoolCoverage::from_catalog(&catalog, VINTAGE_CUBE_POOL),
        verbose,
    )
}

#[test]
fn report_layout_includes_repository_formats_and_fixed_pools() {
    let vintage_cube = PoolCoverage {
        complete: vec!["Complete Card".to_owned()],
        partial: vec!["Partial Card".to_owned()],
        metadata_only: Vec::new(),
        uncataloged: vec!["Missing Card".to_owned()],
    };
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
        &vintage_cube,
        false,
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
            "  ISD-DGM Standard\n",
            "    complete            5\n",
            "    partial             1\n",
            "    metadata-only       0\n",
            "    blocked            10\n",
            "    total              16\n",
            "\n",
            "Fixed card pools\n",
            "  Vintage Cube\n",
            "    complete            1\n",
            "    partial             1\n",
            "    metadata-only       0\n",
            "    uncataloged         1\n",
            "    total               3\n",
        )
    );
}

#[test]
fn print_catalog_report() {
    let verbose = std::env::var("PENTA_CATALOG_REPORT_VERBOSE")
        .is_ok_and(|value| value == "1" || value.eq_ignore_ascii_case("true"));
    print!("{}", repository_report(verbose));
}
