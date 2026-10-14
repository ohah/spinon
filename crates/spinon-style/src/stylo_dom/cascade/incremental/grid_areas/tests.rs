use style::{
    Atom, OwnedStr,
    servo_arc::Arc,
    values::{
        computed::GridTemplateAreas,
        specified::position::{NamedArea, TemplateAreas, TemplateAreasArc, UnsignedRange},
    },
};

use super::computed_grid_template_areas;

fn injected_template_area(rows: UnsignedRange, columns: UnsignedRange) -> GridTemplateAreas {
    GridTemplateAreas::Areas(TemplateAreasArc(Arc::new(TemplateAreas {
        areas: vec![NamedArea {
            name: Atom::from("area"),
            rows,
            columns,
        }]
        .into(),
        strings: vec![OwnedStr::from("area".to_owned())].into(),
        width: 1,
    })))
}

fn range(start: u32, end: u32) -> UnsignedRange {
    UnsignedRange { start, end }
}

#[test]
fn malformed_computed_ranges_fail_closed_with_node_and_property_context() {
    let node = spinon_core::NodeId::new(733).unwrap();
    let malformed = [
        ("row start zero", range(0, 1), range(1, 2)),
        ("row end out of bounds", range(1, 3), range(1, 2)),
        ("row range reversed", range(2, 1), range(1, 2)),
        ("row range empty", range(1, 1), range(1, 2)),
        ("column start zero", range(1, 2), range(0, 1)),
        ("column end out of bounds", range(1, 2), range(1, 3)),
        ("column range reversed", range(1, 2), range(2, 1)),
        ("column range empty", range(1, 2), range(1, 1)),
    ];

    for (label, rows, columns) in malformed {
        let error = computed_grid_template_areas(
            injected_template_area(rows, columns),
            node,
            "direct typed injection",
        )
        .unwrap_err();

        assert!(
            matches!(
                error,
                crate::stylo_dom::cascade::CssCascadeError::UnsupportedComputedGridValue {
                    node: actual_node,
                    property: "grid-template-areas",
                    ref value,
                } if actual_node == node && value == "direct typed injection"
            ),
            "{label}: {error:?}"
        );
    }
}
