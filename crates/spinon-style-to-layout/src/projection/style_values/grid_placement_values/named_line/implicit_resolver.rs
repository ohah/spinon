use spinon_core::NodeId;

use crate::StyleLayoutError;

use super::{LineMetadata, area, unsupported_named_line, validate_line_metadata};

#[derive(Clone, Copy)]
pub(super) enum SearchDirection {
    Forward,
    Backward,
}

pub(super) struct ImplicitNamedLineResolver<'a, 'b> {
    node: NodeId,
    line_metadata: &'a LineMetadata<'b>,
    allow_implicit_named_lines: bool,
}

impl<'a, 'b> ImplicitNamedLineResolver<'a, 'b> {
    pub(super) fn new(
        node: NodeId,
        line_metadata: &'a LineMetadata<'b>,
        allow_implicit_named_lines: bool,
    ) -> Self {
        Self {
            node,
            line_metadata,
            allow_implicit_named_lines,
        }
    }

    pub(super) fn resolve_span_line(
        &self,
        property: &'static str,
        name: &str,
        count: u32,
        opposite_line: i32,
        direction: SearchDirection,
    ) -> Result<(i32, bool), StyleLayoutError> {
        let node = self.node;
        let line_metadata = self.line_metadata;
        validate_line_metadata(node, property, name, line_metadata)?;
        let explicit_last_boundary = i64::try_from(line_metadata.track_count).map_err(|_| {
            unsupported_named_line(
                node,
                property,
                name,
                "track count exceeds the supported range",
            )
        })?;
        let opposite_boundary = css_line_to_boundary(opposite_line, line_metadata.track_count)
            .map_err(|reason| unsupported_named_line(node, property, name, reason))?;
        if !self.allow_implicit_named_lines
            && (opposite_boundary < 0 || opposite_boundary > explicit_last_boundary)
        {
            return Err(unsupported_named_line(
                node,
                property,
                name,
                "opposite definite line requires an implicit grid line",
            ));
        }
        let mut remaining = count;
        match direction {
            SearchDirection::Forward => {
                let mut boundary = opposite_boundary
                    .checked_add(1)
                    .ok_or_else(|| {
                        unsupported_named_line(
                            node,
                            property,
                            name,
                            "named span search exceeds the supported range",
                        )
                    })?
                    .max(0);
                while boundary <= explicit_last_boundary && remaining > 0 {
                    let index = usize::try_from(boundary).map_err(|_| {
                        unsupported_named_line(
                            node,
                            property,
                            name,
                            "named span search exceeds the supported range",
                        )
                    })?;
                    if area::line_has_name(line_metadata, index, name) {
                        remaining -= 1;
                        if remaining == 0 {
                            return css_line_for_boundary(boundary, line_metadata.track_count)
                                .map(|line| (line, false))
                                .map_err(|reason| {
                                    unsupported_named_line(node, property, name, reason)
                                });
                        }
                    }
                    boundary += 1;
                }
            }
            SearchDirection::Backward => {
                let mut boundary = opposite_boundary
                    .checked_sub(1)
                    .ok_or_else(|| {
                        unsupported_named_line(
                            node,
                            property,
                            name,
                            "named span search exceeds the supported range",
                        )
                    })?
                    .min(explicit_last_boundary);
                while boundary >= 0 && remaining > 0 {
                    let index = usize::try_from(boundary).map_err(|_| {
                        unsupported_named_line(
                            node,
                            property,
                            name,
                            "named span search exceeds the supported range",
                        )
                    })?;
                    if area::line_has_name(line_metadata, index, name) {
                        remaining -= 1;
                        if remaining == 0 {
                            return css_line_for_boundary(boundary, line_metadata.track_count)
                                .map(|line| (line, false))
                                .map_err(|reason| {
                                    unsupported_named_line(node, property, name, reason)
                                });
                        }
                    }
                    boundary -= 1;
                }
            }
        }
        if self.allow_implicit_named_lines {
            let missing = i64::from(remaining);
            let generated_boundary = match direction {
                SearchDirection::Backward if opposite_boundary <= 0 => {
                    opposite_boundary.checked_sub(missing)
                }
                SearchDirection::Backward => missing.checked_neg(),
                SearchDirection::Forward if opposite_boundary >= explicit_last_boundary => {
                    opposite_boundary.checked_add(missing)
                }
                SearchDirection::Forward => explicit_last_boundary.checked_add(missing),
            }
            .ok_or_else(|| {
                unsupported_named_line(
                    node,
                    property,
                    name,
                    "generated named span line exceeds the supported range",
                )
            })?;
            let implicit_line =
                css_line_for_boundary(generated_boundary, line_metadata.track_count)
                    .map_err(|reason| unsupported_named_line(node, property, name, reason))?;
            return Ok((implicit_line, true));
        }
        Err(unsupported_named_line(
            node,
            property,
            name,
            "named span requires an implicit grid line outside the explicit grid",
        ))
    }
}

pub(super) fn find_named_occurrence(
    node: NodeId,
    property: &'static str,
    name: &str,
    occurrence: i32,
    line_metadata: &LineMetadata<'_>,
) -> Result<i32, StyleLayoutError> {
    let (selected, _) = locate_named_occurrence(node, property, name, occurrence, line_metadata)?;
    let selected = selected.ok_or_else(|| {
        unsupported_named_line(
            node,
            property,
            name,
            "named line occurrence requires an implicit grid line outside the explicit grid",
        )
    })?;
    let boundary = i64::try_from(selected).map_err(|_| {
        unsupported_named_line(
            node,
            property,
            name,
            "resolved line exceeds the supported range",
        )
    })?;
    css_line_for_boundary(boundary, line_metadata.track_count)
        .map_err(|reason| unsupported_named_line(node, property, name, reason))
}

pub(super) fn resolve_named_line_occurrence(
    node: NodeId,
    property: &'static str,
    name: &str,
    occurrence: i32,
    line_metadata: &LineMetadata<'_>,
    allow_implicit_named_lines: bool,
) -> Result<(i32, bool), StyleLayoutError> {
    let (selected, matched_count) =
        locate_named_occurrence(node, property, name, occurrence, line_metadata)?;
    if let Some(selected) = selected {
        let boundary = i64::try_from(selected).map_err(|_| {
            unsupported_named_line(
                node,
                property,
                name,
                "resolved line exceeds the supported range",
            )
        })?;
        return css_line_for_boundary(boundary, line_metadata.track_count)
            .map(|line| (line, false))
            .map_err(|reason| unsupported_named_line(node, property, name, reason));
    }
    if !allow_implicit_named_lines {
        return Err(unsupported_named_line(
            node,
            property,
            name,
            "named line occurrence requires an implicit grid line outside the explicit grid",
        ));
    }
    let requested = usize::try_from(occurrence.unsigned_abs()).map_err(|_| {
        unsupported_named_line(
            node,
            property,
            name,
            "named line occurrence exceeds the supported range",
        )
    })?;
    let missing = requested.checked_sub(matched_count).ok_or_else(|| {
        unsupported_named_line(
            node,
            property,
            name,
            "named line occurrence count is inconsistent",
        )
    })?;
    let missing = i64::try_from(missing).map_err(|_| {
        unsupported_named_line(
            node,
            property,
            name,
            "named line occurrence exceeds the supported range",
        )
    })?;
    let boundary = if occurrence > 0 {
        i64::try_from(line_metadata.track_count)
            .ok()
            .and_then(|count| count.checked_add(missing))
    } else {
        missing.checked_neg()
    }
    .ok_or_else(|| {
        unsupported_named_line(
            node,
            property,
            name,
            "generated named line exceeds the supported range",
        )
    })?;
    css_line_for_boundary(boundary, line_metadata.track_count)
        .map(|line| (line, true))
        .map_err(|reason| unsupported_named_line(node, property, name, reason))
}

fn locate_named_occurrence(
    node: NodeId,
    property: &'static str,
    name: &str,
    occurrence: i32,
    line_metadata: &LineMetadata<'_>,
) -> Result<(Option<usize>, usize), StyleLayoutError> {
    validate_line_metadata(node, property, name, line_metadata)?;
    if occurrence == 0 {
        return Err(unsupported_named_line(
            node,
            property,
            name,
            "named line occurrence cannot be zero",
        ));
    }
    let matched_count = line_metadata
        .names
        .iter()
        .enumerate()
        .filter(|(index, _)| area::line_has_name(line_metadata, *index, name))
        .count();
    let mut matches = line_metadata
        .names
        .iter()
        .enumerate()
        .filter(|(index, _)| area::line_has_name(line_metadata, *index, name));
    let ordinal = usize::try_from(occurrence.unsigned_abs() - 1).map_err(|_| {
        unsupported_named_line(
            node,
            property,
            name,
            "named line occurrence exceeds the supported range",
        )
    })?;
    let selected = if occurrence > 0 {
        matches.nth(ordinal).map(|(index, _)| index)
    } else {
        matches.rev().nth(ordinal).map(|(index, _)| index)
    };
    Ok((selected, matched_count))
}

fn css_line_to_boundary(line: i32, track_count: usize) -> Result<i64, &'static str> {
    if line == 0 {
        return Err("opposite definite line cannot be zero");
    }
    if line > 0 {
        return Ok(i64::from(line) - 1);
    }
    i64::try_from(track_count)
        .ok()
        .and_then(|count| count.checked_add(1))
        .and_then(|last_boundary| last_boundary.checked_add(i64::from(line)))
        .ok_or("opposite definite line exceeds the supported range")
}

fn css_line_for_boundary(boundary: i64, track_count: usize) -> Result<i32, &'static str> {
    let track_count = i64::try_from(track_count).map_err(|_| "track count exceeds range")?;
    let line = if boundary >= 0 {
        boundary.checked_add(1)
    } else {
        boundary.checked_sub(
            track_count
                .checked_add(1)
                .ok_or("track count exceeds range")?,
        )
    }
    .ok_or("resolved line exceeds the supported range")?;
    i32::try_from(line).map_err(|_| "resolved line exceeds the supported range")
}
