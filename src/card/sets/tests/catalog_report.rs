use std::fmt::Write;
use std::path::Path;

use super::source_organization::{AuditStatus, source_audits_for_format};
use super::*;
use crate::card::{CardComposition, CardDefinition, CardRules};
use crate::{AbilityCoverageDef, CardBehavior, FormatCategory, FormatDefinition};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct CatalogCoverage {
    declarative: usize,
    custom: usize,
    partial: usize,
    metadata_only: usize,
}

impl CatalogCoverage {
    fn from_definitions<'a>(definitions: impl IntoIterator<Item = &'a CardDefinition>) -> Self {
        let mut coverage = Self::default();
        for definition in definitions {
            match definition.implementation_status() {
                ImplementationStatus::Complete if definition_uses_custom_execution(definition) => {
                    coverage.custom += 1;
                }
                ImplementationStatus::Complete => coverage.declarative += 1,
                ImplementationStatus::Partial => coverage.partial += 1,
                ImplementationStatus::MetadataOnly => coverage.metadata_only += 1,
            }
        }
        coverage
    }

    const fn total(self) -> usize {
        self.declarative + self.custom + self.partial + self.metadata_only
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct SetCoverage {
    catalog: CatalogCoverage,
    blocked: usize,
}

impl SetCoverage {
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

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct PoolCoverage {
    declarative: Vec<String>,
    custom: Vec<String>,
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
                ImplementationStatus::Complete if definition_uses_custom_execution(definition) => {
                    coverage.custom.push(name.to_owned());
                }
                ImplementationStatus::Complete => coverage.declarative.push(name.to_owned()),
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
            declarative: self.declarative.len(),
            custom: self.custom.len(),
            partial: self.partial.len(),
            metadata_only: self.metadata_only.len(),
        }
    }

    const fn total(&self) -> usize {
        self.declarative.len()
            + self.custom.len()
            + self.partial.len()
            + self.metadata_only.len()
            + self.uncataloged.len()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum FormatCoverage {
    Sets(SetCoverage),
    Cube(PoolCoverage),
}

fn coverage_for(root: &Path, catalog: &crate::card::CardCatalog, format: Format) -> FormatCoverage {
    match format.definition() {
        FormatDefinition::Sets(_) => {
            FormatCoverage::Sets(SetCoverage::from_repository(root, catalog, format))
        }
        FormatDefinition::Cube(definition) => {
            FormatCoverage::Cube(PoolCoverage::from_catalog(catalog, definition.cards))
        }
    }
}

fn write_catalog_coverage(report: &mut String, coverage: CatalogCoverage) {
    writeln!(report, "    declarative    {:>6}", coverage.declarative)
        .expect("writing to a String cannot fail");
    writeln!(report, "    custom         {:>6}", coverage.custom)
        .expect("writing to a String cannot fail");
    writeln!(report, "    partial        {:>6}", coverage.partial)
        .expect("writing to a String cannot fail");
    writeln!(report, "    metadata-only  {:>6}", coverage.metadata_only)
        .expect("writing to a String cannot fail");
}

fn write_pool_names(report: &mut String, status: &str, names: &[String]) {
    writeln!(report, "    {status} cards").expect("writing to a String cannot fail");
    for name in names {
        writeln!(report, "      - {name}").expect("writing to a String cannot fail");
    }
}

fn write_format_coverage(
    report: &mut String,
    format: Format,
    coverage: &FormatCoverage,
    verbose: bool,
) {
    writeln!(report, "  {}", format.display_name()).expect("writing to a String cannot fail");
    match coverage {
        FormatCoverage::Sets(coverage) => {
            write_catalog_coverage(report, coverage.catalog);
            writeln!(report, "    blocked        {:>6}", coverage.blocked)
                .expect("writing to a String cannot fail");
            writeln!(report, "    total          {:>6}", coverage.total())
                .expect("writing to a String cannot fail");
        }
        FormatCoverage::Cube(coverage) => {
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
                write_pool_names(report, "declarative", &coverage.declarative);
                write_pool_names(report, "custom", &coverage.custom);
                write_pool_names(report, "partial", &coverage.partial);
                write_pool_names(report, "metadata-only", &coverage.metadata_only);
                write_pool_names(report, "uncataloged", &coverage.uncataloged);
            }
        }
    }
}

fn render_report(
    repository: CatalogCoverage,
    categories: &[(FormatCategory, Vec<(Format, FormatCoverage)>)],
    verbose: bool,
) -> String {
    let mut report = String::new();
    writeln!(report, "Catalog coverage").expect("writing to a String cannot fail");
    writeln!(report, "================").expect("writing to a String cannot fail");
    writeln!(report).expect("writing to a String cannot fail");
    writeln!(report, "Repository catalog definitions").expect("writing to a String cannot fail");
    write_catalog_coverage(&mut report, repository);
    writeln!(report, "    total          {:>6}", repository.total())
        .expect("writing to a String cannot fail");

    for (category, formats) in categories {
        writeln!(report).expect("writing to a String cannot fail");
        writeln!(report, "{}", category.display_name()).expect("writing to a String cannot fail");
        for (format, coverage) in formats {
            write_format_coverage(&mut report, *format, coverage, verbose);
        }
    }
    report
}

fn repository_report(verbose: bool) -> String {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let catalog = crate::card::catalog().expect("built-in catalog");
    let categories = FormatCategory::ALL
        .iter()
        .map(|category| {
            (
                *category,
                category
                    .formats()
                    .iter()
                    .map(|format| (*format, coverage_for(root, &catalog, *format)))
                    .collect(),
            )
        })
        .collect::<Vec<_>>();
    render_report(
        CatalogCoverage::from_definitions(catalog.definitions()),
        &categories,
        verbose,
    )
}

static CUSTOM_MODE: [AbilityDef; 1] = [AbilityDef::custom_full(
    "A custom mode.",
    CardBehavior::Fireball,
    "The test exercises a custom modal branch.",
)];

#[test]
fn complete_definitions_are_split_by_execution_kind() {
    fn definition(id: u64, name: &str, rules: &CardRules) -> CardDefinition {
        let rules = *rules;
        let composition = CardComposition::single(name, rules);
        CardDefinition {
            id: CardDefinitionId::new(id),
            name: name.to_owned(),
            art: None,
            debut_set: CardSet::Alpha,
            printings: Vec::new(),
            rules,
            parts: composition.parts,
            structure: composition.structure,
            play_options: composition.play_options,
        }
    }

    let declarative = definition(
        10_001,
        "Declarative",
        &CardRules::new_creature(ManaCost::default(), &["Test"], 1, 1),
    );
    let keyed_custom = definition(
        10_002,
        "Keyed custom",
        &CardRules::new_instant(ManaCost::default()).with_ability(AbilityDef::custom_full(
            "A custom effect.",
            CardBehavior::Fireball,
            "The test exercises keyed custom execution.",
        )),
    );
    let card_owned = definition(
        10_003,
        "Card-owned",
        &CardRules::new_sorcery(ManaCost::default()).with_ability(
            AbilityDef::spell("A card-owned effect.", EffectDef::None)
                .with_effect_execution(EffectExecutionDef::CardOwned)
                .with_coverage(AbilityCoverageDef::explained_complete(
                    "The test exercises card-owned execution.",
                )),
        ),
    );

    assert_eq!(
        CatalogCoverage::from_definitions([&declarative, &keyed_custom, &card_owned]),
        CatalogCoverage {
            declarative: 1,
            custom: 2,
            partial: 0,
            metadata_only: 0,
        }
    );

    assert!(ability_uses_custom_execution(&AbilityDef::modal_spell(
        "Choose one.",
        &CUSTOM_MODE,
        1,
        1,
        false,
    )));
}

#[test]
fn report_layout_is_derived_from_category_registries() {
    let categories = vec![
        (
            FormatCategory::Standard,
            vec![(
                Format::SomM13Standard,
                FormatCoverage::Sets(SetCoverage {
                    catalog: CatalogCoverage {
                        declarative: 4,
                        custom: 1,
                        partial: 1,
                        metadata_only: 7,
                    },
                    blocked: 0,
                }),
            )],
        ),
        (
            FormatCategory::Cube,
            vec![(
                Format::PauperCube,
                FormatCoverage::Cube(PoolCoverage {
                    declarative: vec!["Declarative Card".to_owned()],
                    custom: vec!["Custom Card".to_owned()],
                    partial: Vec::new(),
                    metadata_only: vec!["Stub Card".to_owned()],
                    uncataloged: Vec::new(),
                }),
            )],
        ),
    ];
    let report = render_report(
        CatalogCoverage {
            declarative: 10,
            custom: 2,
            partial: 3,
            metadata_only: 8,
        },
        &categories,
        false,
    );

    assert_eq!(
        report,
        concat!(
            "Catalog coverage\n",
            "================\n",
            "\n",
            "Repository catalog definitions\n",
            "    declarative        10\n",
            "    custom              2\n",
            "    partial             3\n",
            "    metadata-only       8\n",
            "    total              23\n",
            "\n",
            "Standard\n",
            "  Standard: SOM-M13\n",
            "    declarative         4\n",
            "    custom              1\n",
            "    partial             1\n",
            "    metadata-only       7\n",
            "    blocked             0\n",
            "    total              13\n",
            "\n",
            "Cubes\n",
            "  Cube: The Pauper Cube\n",
            "    declarative         1\n",
            "    custom              1\n",
            "    partial             0\n",
            "    metadata-only       1\n",
            "    uncataloged         0\n",
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
