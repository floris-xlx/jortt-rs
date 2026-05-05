use jortt::api::endpoints::GENERATED_METHOD_COUNT;
use jortt::api::operations::{ALL_OPERATION_SPECS, OPERATION_COUNT};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Ord, PartialOrd)]
struct SnapshotOperation {
    tag: String,
    method: String,
    path: String,
    operation_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
struct ComparableOperation {
    tag: String,
    method: String,
    path: String,
    operation_id: String,
}

#[test]
fn generated_operation_count_matches_snapshot_and_methods() {
    assert_eq!(OPERATION_COUNT, 126);
    assert_eq!(ALL_OPERATION_SPECS.len(), OPERATION_COUNT);
    assert_eq!(GENERATED_METHOD_COUNT, OPERATION_COUNT);
}

#[test]
fn generated_endpoints_file_exposes_one_method_per_operation() {
    let method_count = include_str!("../src/api/endpoints.rs")
        .lines()
        .filter(|line| line.trim_start().starts_with("pub async fn "))
        .count();

    assert_eq!(method_count, OPERATION_COUNT);
}

#[test]
fn openapi_inventory_matches_generated_catalog_exactly() {
    let snapshot: Vec<SnapshotOperation> =
        serde_json::from_str(include_str!("../docs/openapi/operations-2026-05-05.json"))
            .expect("snapshot JSON should parse");

    let mut snapshot_ops = snapshot
        .into_iter()
        .map(|op| ComparableOperation {
            tag: op.tag,
            method: op.method,
            path: op.path,
            operation_id: op.operation_id,
        })
        .collect::<Vec<_>>();

    let mut generated_ops = ALL_OPERATION_SPECS
        .iter()
        .map(|spec| ComparableOperation {
            tag: spec.tag.to_string(),
            method: spec.method.as_str().to_string(),
            path: spec.path.to_string(),
            operation_id: spec.operation_id.to_string(),
        })
        .collect::<Vec<_>>();

    snapshot_ops.sort();
    generated_ops.sort();

    assert_eq!(snapshot_ops.len(), OPERATION_COUNT);
    assert_eq!(generated_ops.len(), OPERATION_COUNT);
    assert_eq!(snapshot_ops, generated_ops);
}
