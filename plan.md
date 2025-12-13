# PCAP Viewer TUI Code Review & Improvement Plan

## Issues & Fixes

| Issue | Priority | Location | Fix | Status |
|-------|----------|----------|-----|--------|
| Hardcoded `PacketHeaderFlags` | High | lines 693-714 in pcap.rs | Codegen from protocol.xml | ✅ DONE |
| Duplicate tree state reset | Medium | lines 219, 228, 238, 247 | Extract `reset_tree_state()` method | |
| Detail lines computed twice | Medium | lines 321-334 vs 534-558 | Compute once, store in `App` | |
| Magic numbers (42, 2, 15, etc.) | Medium | throughout | Named constants or calculate from layout | |
| Duplicate styling logic | Medium | lines 484-490, 564-576, 592-596 | Extract `get_style()` helper | |
| Unused `extra_info` field | Low | line 101 in `PacketInfo` | Remove or use | |
| `TreeNode` generic tree | Low | lines 110-178 | Consider own module or dependency | |
| # column too narrow | Low | line 499 | Constraint::Length(4) | ✅ DONE |

## Code Duplication Details

### 1. Flag Formatting Constants ✅ COMPLETED
The flag constants were hardcoded in `format_packet_flags()` and duplicated `PacketHeaderFlags` enum from protocol.xml.
- **Source**: `ACProtocol/protocol.xml` lines 4-28
- **Solution**: 
  - `PacketHeaderFlags` already generated as bitflags in `crates/acprotocol/src/generated/enums/mod.rs`
  - Created `crates/acprotocol/src/packet_flags.rs` with `format_packet_flags()` helper that uses the generated enum
  - Updated TUI to import and use the generated `PacketHeaderFlags` enum instead of hardcoded constants
  - Removed 95 lines of duplicate flag definitions from pcap.rs

### 2. Scroll Reset Logic
Every navigation function resets tree state identically:
```rust
self.tree_scroll_offset = 0;
self.tree_focused_line = 0;
```
- **Locations**: `next()`, `prev()`, `page_down()`, `page_up()` methods
- **Action**: Extract to `App::reset_tree_state()` method

### 3. Detail Line Computation
Computed twice in the same render loop:
- **First**: lines 321-334 for key handling
- **Second**: lines 534-558 for rendering
- **Action**: Compute once per frame, cache in `App` state

### 4. Focus Styling Pattern
Repeated 3+ times with identical logic:
```rust
if focused {
    Style::default().bg(Color::Black).fg(Color::White)
} else if !has_focus {
    Style::default().add_modifier(Modifier::DIM)
} else {
    Style::default()
}
```
- **Locations**: lines 484-490, 564-576, 592-596
- **Action**: Extract `get_focused_style(is_focused: bool, is_pane_focused: bool) -> Style`

## Magic Numbers to Extract

| Value | Context | Suggested Name |
|-------|---------|-----------------|
| `42` | Ethernet + IP + UDP header size | `ETHERNET_IP_UDP_HEADER_SIZE` |
| `2` | Border height | `BORDER_HEIGHT` |
| `15` | Visible rows | Calculate from layout |
| `[36]`, `[37]` | UDP port offset in packet | `UDP_DEST_PORT_OFFSET` |

## Codegen Candidates

### 1. PacketHeaderFlags Enum
- **Source**: `ACProtocol/protocol.xml` (lines 4-28)
- **Type**: Enum mask
- **Generated Output**: 
  - Rust enum with all flag variants
  - `format_packet_flags(flags: u32) -> String` function
  - Mapping table of values to display names

### 2. GameMessageGroup (future)
- **Source**: `ACProtocol/protocol.xml` (lines 50-63)
- **Type**: Enum
- **Status**: Currently hardcoded in JSON extraction, consider for codegen

## Implementation Order

1. **High Priority**: Generate `PacketHeaderFlags` from codegen
2. **Medium Priority**: Extract helper methods (reset_tree_state, get_style)
3. **Medium Priority**: Cache detail_lines in App state
4. **Medium Priority**: Define named constants for magic numbers
5. **Low Priority**: Remove unused `PacketInfo.extra_info` field
6. **Low Priority**: Consider `TreeNode` module extraction
