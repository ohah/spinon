#[path = "../../../../../tests/fixtures/dom/s03/node-lifecycle-limits-v1.rs"]
mod limits;

#[test]
fn s032_resource_budgets_match_the_s033_contract_fixture() {
    assert_eq!(
        limits::EXPECTED_EXTERNAL_ROOT_LIMIT,
        16_384,
        "S03.3 외부 root lease 한도 계약"
    );
    assert_eq!(
        super::MAX_DOCUMENT_STRING_UNITS,
        limits::EXPECTED_STRING_QUOTA_UTF16
    );
}
