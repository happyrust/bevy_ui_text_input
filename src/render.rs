use crate::TextInputBuffer;
use crate::TextInputGlyph;
use crate::TextInputLayoutInfo;
use crate::TextInputNode;
use crate::TextInputPrompt;
use crate::TextInputPromptLayoutInfo;
use crate::TextInputStyle;
use crate::edit::is_buffer_empty;
use bevy::asset::AssetId;
use bevy::asset::Assets;
use bevy::color::Alpha;
use bevy::color::LinearRgba;
use bevy::ecs::entity::Entity;
use bevy::ecs::system::Commands;
use bevy::ecs::system::Query;
use bevy::ecs::system::Res;
use bevy::ecs::system::ResMut;
use bevy::image::TextureAtlasLayout;
use bevy::input_focus::InputFocus;

use bevy::math::Rect;
use bevy::math::Vec2;

use bevy::render::Extract;
use bevy::render::sync_world::TemporaryRenderEntity;
use bevy::render::view::InheritedVisibility;
use bevy::sprite::BorderRect;
use bevy::text::TextColor;
use cosmic_text::Edit;

use bevy::ui::CalculatedClip;
use bevy::ui::ComputedNode;
use bevy::ui::ComputedNodeTarget;
use bevy::ui::ExtractedGlyph;
use bevy::ui::ExtractedUiItem;
use bevy::ui::ExtractedUiNode;
use bevy::ui::ExtractedUiNodes;
use bevy::ui::NodeType;
use bevy::ui::ResolvedBorderRadius;
use bevy::ui::UiCameraMap;
use bevy::ui::UiGlobalTransform;


pub fn extract_text_input_nodes(
    mut commands: Commands,
    mut extracted_uinodes: ResMut<ExtractedUiNodes>,
    texture_atlases: Extract<Res<Assets<TextureAtlasLayout>>>,
    active_text_input: Extract<Res<InputFocus>>,
    uinode_query: Extract<
        Query<(
            Entity,
            &ComputedNode,
            &UiGlobalTransform,
            &InheritedVisibility,
            Option<&CalculatedClip>,
            &ComputedNodeTarget,
            &TextInputLayoutInfo,
            &TextColor,
            &TextInputStyle,
            &TextInputNode,
            &TextInputBuffer,
        )>,
    >,
    camera_map: Extract<UiCameraMap>,
) {
    let mut camera_mapper = camera_map.get_mapper();

    let mut start = extracted_uinodes.glyphs.len();
    let mut end = start + 1;

    for (
        entity,
        uinode,
        ui_global_transform,
        inherited_visibility,
        clip,
        target,
        text_layout_info,
        text_color,
        style,
        input,
        input_buffer,
    ) in &uinode_query
    {
        // Skip if not visible or if size is set to zero (e.g. when a parent is set to `Display::None`)
        if !inherited_visibility.get() || uinode.is_empty() {
            continue;
        }

        let Some(extracted_camera_entity) = camera_mapper.map(target) else {
            continue;
        };

        let color = text_color.0.to_linear();
        let selection_color = style
            .selected_text_color
            .map(|selection_color| selection_color.to_linear())
            .unwrap_or(color);

        let scroll = input_buffer
            .editor
            .with_buffer(|buffer| Vec2::new(buffer.scroll().horizontal, 0.)); // buffer.scroll().vertical));

        // UI 使用 2D 变换
        let ui_transform = **ui_global_transform;
        let transform = ui_transform * bevy::math::Affine2::from_translation(-0.5 * uinode.size() - scroll);

        let node_rect = Rect::from_center_size(
            ui_transform.translation,
            uinode.size(),
        );

        let clip = Some(
            clip.map(|clip| clip.clip.intersect(node_rect))
                .unwrap_or(node_rect),
        );

        let line_height = input_buffer
            .editor
            .with_buffer(|buffer| buffer.metrics().line_height);

        for (i, rect) in input_buffer.selection_rects.iter().enumerate() {
            let size = if (1..input_buffer.selection_rects.len()).contains(&i) {
                rect.size() + Vec2::Y
            } else {
                rect.size()
            } + 2. * Vec2::X;
            extracted_uinodes.uinodes.push(ExtractedUiNode {
                stack_index: uinode.stack_index(),
                color: LinearRgba::from(style.selection_color),
                image: AssetId::default(),
                clip,
                extracted_camera_entity,
                rect: Rect {
                    min: Vec2::ZERO,
                    max: size,
                },
                item: ExtractedUiItem::Node {
                    atlas_scaling: None,
                    flip_x: false,
                    flip_y: false,
                    border_radius: ResolvedBorderRadius::ZERO,
                    border: BorderRect::ZERO,
                    node_type: NodeType::Rect,
                    transform: transform * bevy::math::Affine2::from_translation(rect.center()),
                },
                main_entity: entity.into(),
                render_entity: commands.spawn(TemporaryRenderEntity).id(),
            });
        }

        let is_active = active_text_input.0.is_some_and(|active| active == entity);
        let is_enabled = input.is_enabled;
        let blink_visible = input_buffer.cursor_blink_time < style.blink_interval;
        let color_visible = !style.cursor_color.is_fully_transparent();

        let cursor_visable = is_active && is_enabled && blink_visible && color_visible;

        // Debug logging for active text inputs (commented out to reduce noise)
        // if is_active {
        //     bevy::log::info!(
        //         "Entity {:?}: active={}, enabled={}, blink_visible={} (time={:.3}, interval={:.3}), color_visible={}, final={}",
        //         entity, is_active, is_enabled, blink_visible, input_buffer.cursor_blink_time, style.blink_interval, color_visible, cursor_visable
        //     );
        // }

        let cursor_position = input_buffer
            .editor
            .cursor_position()
            .filter(|_| cursor_visable);

        let selection = input_buffer.editor.selection_bounds();

        for TextInputGlyph {
            position,
            atlas_info,

            line_index,
            byte_index,
            ..
        } in text_layout_info.glyphs.iter()
        {
            let color_out = if let Some((s0, s1)) = selection {
                if (s0.line < *line_index || (*line_index == s0.line && s0.index <= *byte_index))
                    && (*line_index < s1.line || (*line_index == s1.line && *byte_index < s1.index))
                {
                    selection_color
                } else {
                    color
                }
            } else {
                color
            };

            let Some(rect) = texture_atlases
                .get(&atlas_info.texture_atlas)
                .map(|atlas| atlas.textures[atlas_info.location.glyph_index].as_rect())
            else {
                continue;
            };

            extracted_uinodes.glyphs.push(ExtractedGlyph {
                transform: transform * bevy::math::Affine2::from_translation(*position),
                rect,
            });

            extracted_uinodes.uinodes.push(ExtractedUiNode {
                stack_index: uinode.stack_index(),
                color: color_out,
                image: atlas_info.texture.id(),
                clip,
                rect,
                extracted_camera_entity,
                item: ExtractedUiItem::Glyphs { range: start..end },
                main_entity: entity.into(),
                render_entity: commands.spawn(TemporaryRenderEntity).id(),
            });

            start = end;
            end += 1;
        }

        if let Some((x, y)) = cursor_position {
            let cursor_height = line_height * style.cursor_height;

            let x = x as f32;
            let y = y as f32;

            let scale_factor = uinode.inverse_scale_factor().recip();
            let width = style.cursor_width * scale_factor;

            extracted_uinodes.uinodes.push(ExtractedUiNode {
                stack_index: uinode.stack_index(),
                color: style.cursor_color.to_linear(),
                image: AssetId::default(),
                clip,
                extracted_camera_entity,
                rect: Rect {
                    min: Vec2::ZERO,
                    max: Vec2::new(width, cursor_height),
                },
                item: ExtractedUiItem::Node {
                    atlas_scaling: None,
                    flip_x: false,
                    flip_y: false,
                    border_radius: ResolvedBorderRadius::ZERO,
                    border: BorderRect::ZERO,
                    node_type: NodeType::Rect,
                    transform: transform * bevy::math::Affine2::from_translation(Vec2::new(
                        x + 0.5 * width,
                        y + 0.5 * line_height,
                    )),
                },
                main_entity: entity.into(),
                render_entity: commands.spawn(TemporaryRenderEntity).id(),
            });
        }
    }
}

pub fn extract_text_input_prompts(
    mut commands: Commands,
    mut extracted_uinodes: ResMut<ExtractedUiNodes>,
    texture_atlases: Extract<Res<Assets<TextureAtlasLayout>>>,
    // 手动提取：分别查询各个组件
    prompt_query: Extract<Query<(Entity, &TextInputPrompt)>>,
    layout_query: Extract<Query<&TextInputPromptLayoutInfo>>,
    buffer_query: Extract<Query<&TextInputBuffer>>,
    node_query: Extract<Query<(&ComputedNode, &UiGlobalTransform, &InheritedVisibility, Option<&CalculatedClip>, Option<&ComputedNodeTarget>)>>,
    color_query: Extract<Query<&TextColor>>,
    camera_map: Extract<UiCameraMap>,
) {
    let mut camera_mapper = camera_map.get_mapper();

    bevy::log::trace!("extract_text_input_prompts: start manual extraction");

    let prompt_count = prompt_query.iter().count();
    bevy::log::trace!("extract_text_input_prompts: found {} prompt entities", prompt_count);

    // 如果没有任何文字输入提示实体，直接返回，避免不必要的处理
    if prompt_count == 0 {
        return;
    }

    let mut start = extracted_uinodes.glyphs.len();

    let mut end = start + 1;

    // 手动提取数据：遍历所有有 TextInputPrompt 的实体
    for (entity, prompt) in prompt_query.iter() {
        // 检查是否有所需的其他组件
        let Ok(text_layout_info) = layout_query.get(entity) else {
            bevy::log::trace!("prompt skip: entity {:?} missing TextInputPromptLayoutInfo", entity);
            continue;
        };

        let Ok(input) = buffer_query.get(entity) else {
            bevy::log::trace!("prompt skip: entity {:?} missing TextInputBuffer", entity);
            continue;
        };

        let Ok((uinode, ui_global_transform, inherited_visibility, clip, target)) = node_query.get(entity) else {
            bevy::log::trace!("prompt skip: entity {:?} missing node components", entity);
            continue;
        };

        let Ok(text_color) = color_query.get(entity) else {
            bevy::log::trace!("prompt skip: entity {:?} missing TextColor", entity);
            continue;
        };
        // only display the prompt if the text input is empty, including whitespace
        let empty = input.editor.with_buffer(is_buffer_empty);
        if !empty {
            bevy::log::trace!("prompt skip: buffer not empty for entity={:?}", entity);
            continue;
        }

        // visibility and non-empty node
        let vis = inherited_visibility.get();
        let node_empty = uinode.is_empty();
        if !vis || node_empty {
            bevy::log::warn!(
                "prompt skip: visibility/node (visible={}, node_empty={}) entity={:?} size={:?}",
                vis, node_empty, entity, uinode.size()
            );
            continue;
        }

        let extracted_camera_entity = if let Some(target) = target {
            let Some(extracted_camera_entity) = camera_mapper.map(target) else {
                bevy::log::warn!("prompt skip: no camera mapping for entity={:?}", entity);
                continue;
            };
            extracted_camera_entity
        } else {
            // 如果没有 ComputedNodeTarget，使用默认相机
            bevy::log::trace!("prompt: no ComputedNodeTarget, using default camera for entity={:?}", entity);
            continue; // 暂时跳过，需要更好的默认相机处理
        };

        let color = prompt.color.unwrap_or(text_color.0).to_linear();

        // bevy::log::info!(
        //     "prompt extract: entity={:?}, glyphs_len={}, size={:?}",
        //     entity,
        //     text_layout_info.glyphs.len(),
        //     uinode.size()
        // );

        // UI 使用 2D 变换，不需要 3D 变换
        let ui_transform = **ui_global_transform;
        let transform = ui_transform * bevy::math::Affine2::from_translation(-0.5 * uinode.size());

        let node_rect = Rect::from_center_size(
            ui_transform.translation,
            uinode.size(),
        );

        let clip = Some(
            clip.map(|clip| clip.clip.intersect(node_rect))
                .unwrap_or(node_rect),
        );

        if text_layout_info.glyphs.is_empty() {
            bevy::log::warn!(
                "prompt skip: no glyphs generated for entity={:?} (size={:?})",
                entity,
                uinode.size()
            );
        }
        for TextInputGlyph {
            position,
            atlas_info,
            ..
        } in text_layout_info.glyphs.iter()
        {
            let rect = texture_atlases
                .get(&atlas_info.texture_atlas)
                .unwrap()
                .textures[atlas_info.location.glyph_index]
                .as_rect();
            extracted_uinodes.glyphs.push(ExtractedGlyph {
                transform: transform * bevy::math::Affine2::from_translation(*position),
                rect,
            });
            extracted_uinodes.uinodes.push(ExtractedUiNode {
                stack_index: uinode.stack_index(),
                color,
                image: atlas_info.texture.id(),
                clip,
                rect,
                item: ExtractedUiItem::Glyphs { range: start..end },
                main_entity: entity.into(),
                render_entity: commands.spawn(TemporaryRenderEntity).id(),
                extracted_camera_entity,
            });

            start = end;
            end += 1;
        }
    }
}
