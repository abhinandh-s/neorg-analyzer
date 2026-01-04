pub fn offset_to_position(
    offset: usize,
    rope: &ropey::Rope,
) -> Option<tower_lsp::lsp_types::Position> {
    let line = rope.try_char_to_line(offset).ok()?;
    let first_char_of_line = rope.try_line_to_char(line).ok()?;
    let column = offset - first_char_of_line;
    Some(tower_lsp::lsp_types::Position::new(
        line as u32,
        column as u32,
    ))
}

pub fn position_to_offset(
    position: tower_lsp::lsp_types::Position,
    rope: &ropey::Rope,
) -> Option<usize> {
    let line_char_offset = rope.try_line_to_char(position.line as usize).ok()?;
    let slice = rope.slice(0..line_char_offset + position.character as usize);
    Some(slice.len_bytes())
}
