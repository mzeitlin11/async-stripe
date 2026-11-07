use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Write as _;

use petgraph::prelude::DiGraphMap;

use super::{csv, dot};
use crate::codegen::CodeGen;
use crate::item_catalog::RustItem;
use crate::item_layout::EmissionSite;
use crate::module_dependencies::module_dependencies;
use crate::rust_object::RustObject;

/// Counts each canonical declaration once, without revisiting its children or
/// contributing inferred occurrences. Fields count struct fields; variants
/// count the explicitly represented enum variants.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
struct DeclarationCounts {
    items: usize,
    fields: usize,
    variants: usize,
}

impl DeclarationCounts {
    fn add_item(&mut self, item: &RustItem) {
        self.items += 1;
        if let RustItem::Object(object) = item {
            match &object.definition {
                RustObject::Struct(strukt) => self.fields += strukt.fields.len(),
                RustObject::Enum(variants) => self.variants += variants.len(),
                RustObject::FieldlessEnum(variants) => self.variants += variants.len(),
            }
        }
    }

    fn add(&mut self, other: Self) {
        self.items += other.items;
        self.fields += other.fields;
        self.variants += other.variants;
    }
}

/// A sorted report view of represented dependencies between emission modules.
/// This describes type uses, without selecting a feature or placement policy.
pub(super) struct ModuleAnalysis {
    model_modules: BTreeMap<String, DeclarationCounts>,
    dependencies: BTreeSet<(String, String)>,
}

impl ModuleAnalysis {
    pub(super) fn new(codegen: &CodeGen) -> Self {
        let items = codegen.model.items();
        let graph = module_dependencies(items, codegen.model.requests(), &codegen.item_layout)
            .expect("module dependencies were validated when constructing CodeGen");
        let locations = codegen.output_locations();
        let path = |site| locations.emission_path(site, items).display().to_string();
        let mut declarations: HashMap<EmissionSite, (DeclarationCounts, bool)> = HashMap::new();
        for (item, record) in items.iter() {
            let (counts, has_models) = declarations
                .entry(codegen.item_layout.placement(item).emission_site())
                .or_default();
            counts.add_item(&record.item);
            *has_models |= matches!(record.item, RustItem::Object(_));
        }
        let model_modules = declarations
            .into_iter()
            .filter(|(site, (_, has_models))| {
                *has_models
                    && !matches!(site, EmissionSite::ComponentRequests(_) | EmissionSite::Webhooks)
            })
            .map(|(site, (counts, _))| (path(site), counts))
            .collect();
        let dependencies = graph
            .iter()
            .flat_map(|(&source, targets)| {
                targets.iter().map(move |&target| (path(source), path(target)))
            })
            .collect();
        Self { model_modules, dependencies }
    }

    pub(super) fn dependencies_csv(&self) -> String {
        let mut out = String::from("source_emission_path,target_emission_path\n");
        for (source, target) in &self.dependencies {
            let _ = writeln!(out, "{},{}", csv(source), csv(target));
        }
        out
    }

    /// SCCs describe boundaries between model-bearing source modules. Request,
    /// webhook, ID-only, and intra-module references do not constrain that view.
    fn model_graph(&self) -> DiGraphMap<&str, ()> {
        let mut graph = DiGraphMap::new();
        for path in self.model_modules.keys() {
            graph.add_node(path.as_str());
        }
        for (source, target) in &self.dependencies {
            if source != target
                && graph.contains_node(source.as_str())
                && graph.contains_node(target.as_str())
            {
                graph.add_edge(source.as_str(), target.as_str(), ());
            }
        }
        graph
    }

    fn sccs(&self) -> Vec<Vec<&str>> {
        let mut groups = petgraph::algo::kosaraju_scc(&self.model_graph());
        for group in &mut groups {
            group.sort();
        }
        groups.sort();
        groups
    }

    /// Counts refer to the entire SCC and repeat for each membership row.
    pub(super) fn sccs_csv(&self) -> String {
        let mut out = String::from(
            "scc_id,module_count,item_count,field_count,variant_count,cross_module_cycle,emission_path\n",
        );
        for (index, group) in self.sccs().iter().enumerate() {
            let mut counts = DeclarationCounts::default();
            for path in group {
                counts.add(self.model_modules[*path]);
            }
            for path in group {
                let _ = writeln!(
                    out,
                    "{index},{},{},{},{},{},{}",
                    group.len(),
                    counts.items,
                    counts.fields,
                    counts.variants,
                    group.len() > 1,
                    csv(path)
                );
            }
        }
        out
    }

    pub(super) fn graph_dot(&self) -> String {
        let graph = self.model_graph();
        let mut out = String::from(
            "// Represented model modules; requests, webhooks, ID-only modules, and intra-module edges omitted.\n// edge direction: source module depends on target module\ndigraph model_module_dependencies {\n",
        );
        let mut nodes = graph.nodes().collect::<Vec<_>>();
        nodes.sort();
        let mut edges =
            graph.all_edges().map(|(source, target, _)| (source, target)).collect::<Vec<_>>();
        edges.sort();
        for path in nodes {
            let _ = writeln!(out, "  \"{}\";", dot(path));
        }
        for (source, target) in edges {
            let _ = writeln!(out, "  \"{}\" -> \"{}\";", dot(source), dot(target));
        }
        out.push_str("}\n");
        out
    }

    pub(super) fn summary(&self) -> String {
        let graph = self.model_graph();
        let groups = self.sccs();
        let mut counts = DeclarationCounts::default();
        for module in self.model_modules.values() {
            counts.add(*module);
        }
        format!(
            "\n## Represented model modules\n\n- Model modules: {}\n- Canonical declarations in model modules: {}\n- Struct fields in model modules: {}\n- Enum variants in model modules: {}\n- Cross-module model dependencies: {}\n- Strongly connected components: {}\n- Components containing cross-module cycles: {}\n- Largest component: {} modules\n\nThese follow represented declaration and request type uses after placement. The model graph excludes request modules, webhook modules, ID-only modules, and intra-module references; isolated model modules are retained. `model-module-dependencies.csv` also includes the excluded edges.\n\nSCC module/item/field/variant counts cover the entire component and repeat for each membership row. Counts use canonical declarations once, rather than inferred occurrences, and exclude generated helper declarations and implicit unknown variants. The reports do not include facade reexports, handwritten code, or feature activation, and do not measure a consumer's compiled source closure.\n",
            graph.node_count(),
            counts.items,
            counts.fields,
            counts.variants,
            graph.edge_count(),
            groups.len(),
            groups.iter().filter(|group| group.len() > 1).count(),
            groups.iter().map(Vec::len).max().unwrap_or(0)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{DeclarationCounts, ModuleAnalysis};
    use crate::item_catalog::{ObjectItem, RustItem};
    use crate::rust_object::{
        EnumVariant, FieldlessVariant, ObjectUsage, RustObject, Struct, StructField,
    };
    use crate::rust_type::RustType;
    use crate::types::RustIdent;

    fn fixture(reversed: bool) -> ModuleAnalysis {
        let mut model_modules = vec![
            ("a.rs", DeclarationCounts { items: 2, fields: 3, variants: 0 }),
            ("b.rs", DeclarationCounts { items: 3, fields: 0, variants: 5 }),
            ("isolated.rs", DeclarationCounts { items: 1, fields: 0, variants: 0 }),
        ];
        let mut dependencies = vec![
            ("a.rs", "a.rs"),
            ("a.rs", "b.rs"),
            ("b.rs", "a.rs"),
            ("a.rs", "ids.rs"),
            ("requests.rs", "a.rs"),
            ("b.rs", "requests.rs"),
            ("webhooks.rs", "b.rs"),
            ("b.rs", "webhooks.rs"),
        ];
        if reversed {
            model_modules.reverse();
            dependencies.reverse();
        }
        ModuleAnalysis {
            model_modules: model_modules
                .into_iter()
                .map(|(path, counts)| (path.to_string(), counts))
                .collect(),
            dependencies: dependencies
                .into_iter()
                .map(|(source, target)| (source.to_string(), target.to_string()))
                .collect(),
        }
    }

    #[test]
    fn model_cycles_exclude_requests_references_webhooks_and_internal_edges() {
        let report = fixture(false);
        let graph = report.model_graph();
        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 2);
        assert_eq!(report.sccs(), vec![vec!["a.rs", "b.rs"], vec!["isolated.rs"]]);
        assert_eq!(
            report.sccs_csv(),
            "scc_id,module_count,item_count,field_count,variant_count,cross_module_cycle,emission_path\n0,2,5,3,5,true,a.rs\n0,2,5,3,5,true,b.rs\n1,1,1,0,0,false,isolated.rs\n"
        );
        let complete = report.dependencies_csv();
        assert!(complete.contains("a.rs,a.rs\n"));
        assert!(complete.contains("a.rs,ids.rs\n"));
        assert!(complete.contains("b.rs,requests.rs\n"));
        assert!(complete.contains("webhooks.rs,b.rs\n"));
        assert!(report.summary().contains("Canonical declarations in model modules: 6\n"));
        assert!(report.summary().contains("Struct fields in model modules: 3\n"));
    }

    #[test]
    fn reports_are_independent_of_graph_insertion_order() {
        let forward = fixture(false);
        let reversed = fixture(true);
        assert_eq!(forward.dependencies_csv(), reversed.dependencies_csv());
        assert_eq!(forward.sccs_csv(), reversed.sccs_csv());
        assert_eq!(forward.graph_dot(), reversed.graph_dot());
        assert_eq!(forward.summary(), reversed.summary());
    }

    #[test]
    fn declaration_counts_use_canonical_fields_and_variants_once() {
        let object = |definition| {
            RustItem::Object(Box::new(ObjectItem {
                definition,
                usage: ObjectUsage::type_def(),
                enum_of_objects: None,
                identity: None,
            }))
        };
        let declarations = [
            object(RustObject::Struct(Struct::new(vec![
                StructField::new("first", RustType::bool()),
                StructField::new("second", RustType::string()),
            ]))),
            object(RustObject::Enum(vec![
                EnumVariant::new(RustIdent::create("first"), RustType::bool()),
                EnumVariant::fieldless(RustIdent::create("second")),
            ])),
            object(RustObject::FieldlessEnum(vec![
                FieldlessVariant::new("first".to_string(), RustIdent::create("first")),
                FieldlessVariant::new("second".to_string(), RustIdent::create("second")),
            ])),
            RustItem::IdNewtype,
        ];
        let mut counts = DeclarationCounts::default();
        for declaration in &declarations {
            counts.add_item(declaration);
        }
        assert_eq!(counts, DeclarationCounts { items: 4, fields: 2, variants: 4 });
    }
}
