use std::collections::BTreeMap;

use spinon_core::NodeId;
use spinon_render::{
    OpaqueCssSrgb, RuntimePaint, RuntimePaintCommand, RuntimeRenderBox, RuntimeSrgbColor,
};
use spinon_style::ComputedBackgroundPaint;

use super::{reserve::FallibleReserve, types::StackingContext};
use crate::StyleRenderError;

pub(super) fn runtime_paint(
    paint: ComputedBackgroundPaint,
) -> Result<RuntimePaint, StyleRenderError> {
    match paint {
        ComputedBackgroundPaint::Transparent => Ok(RuntimePaint::None),
        ComputedBackgroundPaint::Opaque(color) => Ok(RuntimePaint::Opaque(OpaqueCssSrgb::new(
            color.red,
            color.green,
            color.blue,
        ))),
        ComputedBackgroundPaint::Translucent(color) => Ok(RuntimePaint::Color(
            RuntimeSrgbColor::new(color.red, color.green, color.blue, color.alpha)?,
        )),
    }
}

pub(super) fn paint_commands_for_boxes<R: FallibleReserve>(
    boxes: &[RuntimeRenderBox],
    context_by_node: &BTreeMap<NodeId, usize>,
    contexts: &[StackingContext],
    reserve: &mut R,
) -> Result<Vec<RuntimePaintCommand>, StyleRenderError> {
    let mut commands = Vec::new();
    let mut active_groups = Vec::new();
    let mut desired_groups = Vec::new();
    reserve.reserve(&mut active_groups, contexts.len(), "opacity group stack")?;
    reserve.reserve(&mut desired_groups, contexts.len(), "opacity context path")?;

    for (index, render_box) in boxes.iter().enumerate() {
        desired_groups.clear();
        let mut context = Some(
            *context_by_node
                .get(&render_box.node_id())
                .ok_or(StyleRenderError::InvalidRoot)?,
        );
        let mut traversed = 0_usize;
        while let Some(context_index) = context {
            traversed = traversed
                .checked_add(1)
                .ok_or(StyleRenderError::InvalidRoot)?;
            if traversed > contexts.len() {
                return Err(StyleRenderError::InvalidRoot);
            }
            let current = contexts
                .get(context_index)
                .ok_or(StyleRenderError::InvalidRoot)?;
            if current.opacity.is_some() {
                desired_groups.push(context_index);
            }
            context = current.parent;
        }
        desired_groups.reverse();

        let common_prefix = active_groups
            .iter()
            .zip(&desired_groups)
            .take_while(|(active, desired)| active == desired)
            .count();
        while active_groups.len() > common_prefix {
            active_groups.pop();
            append_paint_command(&mut commands, RuntimePaintCommand::EndGroup, reserve)?;
        }
        for context_index in desired_groups.iter().skip(common_prefix).copied() {
            let opacity = contexts
                .get(context_index)
                .and_then(|context| context.opacity)
                .ok_or(StyleRenderError::InvalidRoot)?;
            append_paint_command(
                &mut commands,
                RuntimePaintCommand::BeginGroup { opacity },
                reserve,
            )?;
            active_groups.push(context_index);
        }
        append_paint_command(
            &mut commands,
            RuntimePaintCommand::DrawBox { index },
            reserve,
        )?;
    }

    while active_groups.pop().is_some() {
        append_paint_command(&mut commands, RuntimePaintCommand::EndGroup, reserve)?;
    }
    Ok(commands)
}

fn append_paint_command<R: FallibleReserve>(
    commands: &mut Vec<RuntimePaintCommand>,
    command: RuntimePaintCommand,
    reserve: &mut R,
) -> Result<(), StyleRenderError> {
    reserve.reserve(commands, 1, "opacity paint commands")?;
    commands.push(command);
    Ok(())
}
