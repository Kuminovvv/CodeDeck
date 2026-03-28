# Todo

- [x] Verify the current Linux environment and installed toolchain
- [x] Install required Linux build dependencies with `script/linux`
- [x] Run `cargo build` from the repository root
- [x] Review the result and record blockers or success notes
- [x] Find the borders responsible for the Onboarding pane separators
- [x] Make the Onboarding pane render without the tab/status separator borders
- [x] Run `cargo build` to verify the UI change

## Review

- Host OS: Ubuntu 25.10 x86_64
- `script/linux` completed successfully and installed the required Linux packages plus Rust via `rustup`
- `cargo build` completed successfully from the repository root
- First full debug build finished with `Finished 'dev' profile [unoptimized + debuginfo] target(s) in 7m 00s`
- In the current shell, Rust was made available by sourcing `$HOME/.cargo/env`
- The Onboarding item now disables pane chrome borders for its tab bar and the status bar
- The UI change verified with `cargo build`, which finished successfully in 1m 09s
- The updated app was relaunched with `cargo run`

## Planned UI Parity Work

- [x] Rework the title bar and top toolbar to match the reference layout and spacing
- [x] Restyle the left activity rail and project panel chrome to match the reference density
- [x] Restyle editor tabs, split headers, and editor surface framing to match the reference
- [x] Tune colors, borders, corner radii, and spacing globally so the full window reads as one design
- [x] Verify with `cargo build` and relaunch the app for visual review

## UI Parity Review

- The top chrome now uses a three-zone layout so the run toolbar sits in the center like the reference
- The project panel header is compact and action-driven, with tighter density and a slimmer tree indent
- The workspace shell now uses thinner gaps, softer border contrast, and smaller corner radii
- The activity rail and tab bar use flatter panel styling to match the JetBrains-like frame more closely
- `cargo build` completed successfully after the styling pass in 14.06s
- The left activity rail was further simplified by removing the inner pill background, tightening spacing, and enlarging the icons
- Added a bundled `JetBrains New UI Dark` theme based on the local WebStorm 2025.3 `expUI_dark` and `Darcula` theme resources
- The app was rebuilt and relaunched successfully after switching the default dark theme to `JetBrains New UI Dark`

## JetBrains Icon Parity Work

- [x] Add bundled icon theme loading to `theme_settings`
- [x] Add a `JetBrains New UI Dark` icon theme for directory and chevron assets
- [x] Replace the most visible activity rail SVGs with WebStorm-derived silhouettes
- [x] Run `cargo build`, relaunch the app, and verify there are no icon theme load errors

## JetBrains Icon Review

- `theme_settings` now loads bundled icon themes from `assets/icon_themes/` alongside bundled UI themes
- Added a bundled `JetBrains New UI Dark` icon theme and switched the default `icon_theme` setting to it
- The project tree now uses JetBrains-style folder and chevron assets instead of the default Zed file icon set
- The activity rail now uses WebStorm-derived shapes for the project, search, AI, and structure-facing icons
- `cargo build` completed successfully in 42.88s after the icon theme pass
- The app was relaunched with `cargo run`, and runtime logs showed no theme or icon theme parse/load errors

## Planned Hugeicons Migration

- [x] Audit the current embedded SVG surface and group icons by UI role
- [x] Define the migration strategy for chrome icons, file tree icons, and file-type icons
- [x] Pull hugeicons assets for the mapped set and replace the local SVG files
- [x] Rebuild, relaunch, and verify that the UI still renders correctly after the migration

## Hugeicons Migration Review

- Replaced the main always-visible chrome icons with Hugeicons assets, including the activity rail, folder actions, chevrons, menu, settings, add, close, and primary search glyphs
- Added a bundled `Hugeicons Dark` icon theme for directory and chevron assets and switched the default `icon_theme` setting to it
- Kept the `JetBrains New UI Dark` UI theme so layout, spacing, and color tuning remain aligned with the WebStorm-inspired shell
- `cargo build` completed successfully in 0.57s after the Hugeicons pass
- The app was relaunched with `cargo run`, and runtime logs showed no theme or icon theme load errors

## Planned Rail Button Restyle

- [x] Replace the transparent icon-only rail buttons with square web-style buttons
- [x] Add clear hover and selected states while keeping the current Hugeicons glyph set
- [x] Verify spacing, rebuild, and relaunch the app for visual review

## Rail Button Review

- The left rail now uses square `IconButton` controls instead of transparent icon-only buttons
- Buttons use `Subtle` styling for normal and hover states, with `Filled` styling for the selected state
- The current Hugeicons set and the `20px` icon size were preserved while restoring a normal button hit-area
- `cargo build` completed successfully in 19.59s after the rail button restyle
- The app was relaunched with `cargo run`, and the first frame rendered successfully on the updated binary

## Planned Top Toolbar Restyle

- [x] Move the `Add Configuration` toolbar block out of the centered slot and anchor it to the right side
- [x] Reduce the horizontal stretch so the top panel reads like a compact JetBrains toolbar instead of a centered strip
- [x] Apply JetBrains-like hover and pressed button chrome to the top toolbar actions
- [x] Rebuild, relaunch, and verify the updated title bar layout

## Top Toolbar Review

- The `Add Configuration` toolbar block was removed from the centered title bar slot and moved into the right-aligned control cluster
- The top toolbar now renders as one compact right-anchored group instead of stretching visual attention across the full title bar
- The toolbar actions continue to use the JetBrains-like `Subtle` button chrome, which maps closest to the extracted WebStorm `ActionButton` hover and pressed tokens
- `cargo build` completed successfully in 8.35s after the title bar layout pass
- The app was relaunched with `cargo run`, and the updated binary started successfully

## Planned Tool Window Stripe Correction

- [x] Replace the current web-style rail buttons with a JetBrains-like tool window stripe model
- [x] Match the stripe width, icon size, separators, inset selected background, and muted inactive icon treatment to the local WebStorm reference
- [x] Rebuild, relaunch, and verify the stripe against the reference screenshot

## Tool Window Stripe Review

- The left rail no longer renders as generic square web buttons; it now uses transparent icon buttons inside a narrower stripe capsule that better matches WebStorm's tool window stripe
- Inactive icons are muted, the active item uses an inset selected background, and vertical separators are inserted before the secondary tool-window groups
- `cargo build` completed successfully in 55.61s after the stripe correction
- The app was relaunched with `cargo run`, and startup reached `Rendered first frame` on the updated binary

## Rail Container Background Review

- Removed the dedicated background fill from the `workspace-activity-strip` container so the rail now sits directly on the window surface instead of reading as a separate slab
- Kept the right-side divider and the inner stripe button styling intact, so only the extra container backdrop was removed
- `cargo build` completed successfully in 54.44s after the container background adjustment
- The app was relaunched with `cargo run` on the updated binary

## Status Bar Background Review

- Removed the dedicated background fill from the bottom `StatusBar` container so the lower strip also sits on the shared window surface
- Kept the top border and status content layout intact, so the change only removes the extra slab-like backdrop
- `cargo build` completed successfully in 21.73s after the status bar background adjustment
- The app was relaunched with `cargo run` on the updated binary

## Status Bar Surface Correction

- Corrected the bottom `StatusBar` treatment after the follow-up clarification: it should not be transparent
- The bar now uses the shared window `background` token instead of the darker left-panel-like slab, and its rounded-corner filler border plus sidebar-toggle indicator border were aligned to the same surface
- `cargo build` completed successfully in 21.44s after the status bar surface correction
- The app was relaunched with `cargo run` on the updated binary

## Rail Cursor Hit-Area Review

- Moved the pointer cursor and left-click handling to the outer `30x30` stripe hit-area instead of leaving them only on the inner icon element
- Kept the existing tooltip on the inner `IconButton`, while the whole button capsule now responds to hover cursor and click
- `cargo build` completed successfully in 22.94s after the rail cursor hit-area fix
- The app was relaunched with `cargo run` on the updated binary

## Rail Inner Cursor Review

- Removed the explicit `Arrow` cursor override from the inner `IconButton`, so the outer pointer cursor now stays active over the glyph itself as well
- The tooltip behavior remains unchanged, but the cursor no longer flips back when the pointer moves from the button chrome onto the icon
- `cargo build` completed successfully in 19.16s after the inner cursor correction
- The app was relaunched with `cargo run` on the updated binary

## Planned JetBrains Selection Pass

- [x] Rework editor tabs to match local WebStorm `expUI_dark` geometry and selected-state treatment
- [x] Rework selected project-tree rows to match WebStorm row height, insets, and highlight style
- [x] Rebuild, relaunch, and verify the updated selection styling against the local reference

## JetBrains Selection Review

- Editor tabs now use a flatter JetBrains-like active treatment with a top accent strip, top-only rounding, and quieter inactive text instead of the previous rounded pill style
- Project-tree selection now uses a lighter inset row highlight with a `24px` row height and no heavy focus border box, which reads closer to WebStorm `expUI_dark`
- `cargo build` completed successfully in 35.48s for the main pass, then again cleanly in 10.99s after removing follow-up warnings
- The app was relaunched with `cargo run` on the updated binary

## Planned WebStorm Tab Correction

- [x] Replace the current top-accent active tab with a WebStorm-like rounded selected chip
- [x] Match the tab bar height, active border treatment, inactive density, and spacing to the local WebStorm reference
- [x] Rebuild, relaunch, and verify the corrected tab styling against the reference screenshot

## WebStorm Tab Correction Review

- Replaced the previous top-accent active tab model with a compact rounded selected chip that uses a full outline and inset sizing closer to the local WebStorm reference
- Increased the tab geometry to a `32px` container with a `28px` inner chip and softened inactive tabs so the selected tab reads as the primary surface
- `cargo build` completed successfully in 40.16s after the tab correction pass
- The app was relaunched with `cargo run` on the updated binary

## Tab Radius Correction Review

- Increased the active tab chip radius from `rounded_sm` to `rounded_md` so the selected tab reads as fully rounded instead of slightly squared off
- `cargo build` completed successfully in 27.94s after the tab radius correction
- The app was relaunched with `cargo run` on the updated binary

## Tab Bottom Inset Review

- Restored the full WebStorm-like vertical geometry for editor tabs by returning the tab container to `40px` and giving the inner chip explicit `6px` top and bottom insets
- `cargo build` completed successfully in 35.42s after the tab bottom inset correction
- The app was relaunched with `cargo run` on the updated binary

## Tab Alignment Review

- Removed the extra vertical padding from `TabBar` once the tab itself owned the full `40px` geometry, which prevents the selected chip from reading as slightly shifted within the bar
- `cargo build` completed successfully in 28.77s after the tab alignment correction
- The app was relaunched with `cargo run` on the updated binary

## Planned Active Tab End-Slot Pass

- [x] Change editor tab end-slot logic so the active tab always shows the close icon
- [x] Add explicit right-side breathing room for the active tab so the close icon does not sit flush against the chip edge
- [x] Rebuild, relaunch, and verify the updated active-tab layout

## Active Tab End-Slot Review

- Active editor tabs now always render the close icon even when the global setting would otherwise hide it until hover
- Added explicit right-side spacing for the active tab end-slot so the close icon does not sit flush against the chip edge
- `cargo build` completed successfully in 55.08s after the active-tab end-slot pass
- The app was relaunched with `cargo run` on the updated binary

## Tab End-Slot Width Review

- The missing right-side breathing room turned out to be a generic `Tab` geometry issue rather than a `pane.rs` policy issue
- Increased the shared tab end-slot width from `12px` to `16px` and only reserve that slot when the tab actually renders trailing content, so the active close icon no longer sits flush against the chip edge
- `cargo build` completed successfully in 27.22s after the end-slot width correction

## Planned Tab Content Alignment Pass

- [x] Inspect the active tab inner flex alignment and slot wrappers
- [x] Center the filename label and close icon on the same visual axis without disturbing the WebStorm geometry
- [x] Rebuild, relaunch, and verify the corrected active-tab alignment

## Tab Content Alignment Review

- The selected tab content read as off-center because the chip reserved less width on the left than on the right, and the active end-slot also had an extra one-off right padding wrapper
- Balanced the chip geometry by widening the shared start slot to `16px` to match the end slot and removing the active-only `pr_1()` wrapper from `pane.rs`, so the filename and close icon now read centered as a group
- `cargo build` completed successfully in 26.90s after the tab content alignment correction
- The app was relaunched with `cargo run` on the updated binary

## Tab Label Line-Height Review

- The remaining visual misalignment came from the tab title using the default text label line-height instead of the compact UI label line-height expected in editor tabs
- Switched tab labels to `LineHeightStyle::UiLabel` in the shared item tab-content path and in the editor-specific tab-content renderer, so the filename now sits on the same compact visual center as the close glyph
- `cargo build` completed successfully in 53.61s after the tab label line-height correction
- The app was relaunched with `cargo run` on the updated binary

## Planned Tab Optical Alignment Pass

- [x] Inspect whether the remaining mismatch is the close glyph optical center rather than layout centering
- [x] Apply the minimal selected-tab close-slot optical adjustment needed to match the local WebStorm look
- [x] Rebuild, relaunch, and record the correction

## Tab Optical Alignment Review

- The remaining mismatch was no longer structural layout centering; it was the close glyph reading slightly low relative to the compact tab title
- Applied a minimal `1px` upward optical correction to the non-pinned tab end-slot in `pane.rs` instead of disturbing the whole tab geometry
- `cargo build` completed successfully in 19.15s after the optical alignment correction
- The app was relaunched with `cargo run` on the updated binary

## Tab Inner Padding Review

- Split the tab insets into separate left text inset and right close-button inset instead of treating them as one symmetric padding value
- The chip now uses an explicit `8px` left inset, while tabs with a close button reserve a `24px` end slot so the right breathing room is measured from the button rather than from the text
- Empty trailing tabs still keep an explicit `8px` right inset when no close button is present
- `cargo build` completed successfully in 37.78s after the tab inner padding correction
- The app was relaunched with `cargo run` on the updated binary

## Tab Outline Removal Review

- Removed the tab-chip outline so the selected tab now relies only on its filled surface instead of a separate border stroke
- Dropped the generic chip `border_1()` and the selected border color override while preserving the existing tab geometry, insets, and background treatment
- `cargo build` completed successfully in 26.60s after the tab outline removal
- The app was relaunched with `cargo run` on the updated binary

## Tab Bar Surface Review

- The mismatch the user pointed out was the tab-bar strip itself, not the tab-chip background
- Switched the tab-bar container from `tab_bar_background` to `title_bar_background` so the whole strip now matches the adjacent top chrome surface
- `cargo build` completed successfully in 29.35s after the tab-bar surface correction
- The app was relaunched with `cargo run` on the updated binary

## Tab Bar Editor Surface Review

- The follow-up clarified that the tab strip should match the editor surface, not the top chrome surface
- Switched the tab-bar container from `title_bar_background` to `editor_background` so the strip now uses the same dark surface as the code editor area
- `cargo build` completed successfully in 27.66s after the editor-surface correction
- The app was relaunched with `cargo run` on the updated binary

## Planned Panel Chrome Pass

- [x] Find every workspace panel container that currently owns the dock/editor outer radius
- [x] Set the left/right/bottom/center panel containers to a uniform outer radius
- [x] Remove their outer borders and shadows without disturbing inner separators
- [x] Rebuild, relaunch, and verify the unified panel chrome

## Panel Chrome Review

- Unified the workspace panel chrome by updating the shared dock shell in `dock.rs` and all editor-shell layout branches in `workspace.rs`
- Removed the outer panel borders while preserving existing inner separators and pane dividers
- The final agreed outer radius is `16px` after follow-up corrections from the initial `32px`
- No separate shadows were present on those panel containers; the only shadows in the area belong to the zoom overlay and window decorations, which were intentionally left alone
- `cargo build` completed successfully in 19.46s after the final panel chrome correction
- The app was relaunched with `cargo run` on the updated binary

## Planned Panel Spacing Pass

- [x] Inspect the workspace layout wrappers that currently place panel shells flush against each other
- [x] Add a consistent outer gap between all large panel shells without changing their internal dividers
- [x] Rebuild, relaunch, and verify the spacing across left, center, right, and bottom panels

## Panel Spacing Review

- Added a shared `8px` outer gap around the large workspace panel shells by updating the layout wrappers in `workspace.rs`
- Applied the new spacing consistently across the `Full`, `LeftAligned`, `RightAligned`, and `Contained` bottom-dock layouts, covering the left dock, center editor shell, right dock, and bottom panel spacing
- Kept all internal pane dividers and inner separators unchanged; only the outer shell-to-shell spacing was increased
- `cargo build` completed successfully in 57.67s after the panel spacing pass
- The app was relaunched with `cargo run` on the updated binary

## Planned Split Panel Shell Pass

- [x] Add shell-level spacing between split panes inside PaneGroup
- [x] Give individual split panes the same rounded clipping as the large outer shells
- [x] Keep resize handles working while removing the visual pressed-together look
- [x] Rebuild, relaunch, and verify split-pane spacing and rounding

## Split Panel Shell Review

- Added a shared `8px` gap between split-pane members inside `PaneGroup`, so editor/chat-style splits are no longer pressed directly against each other
- Wrapped each leaf pane member in its own rounded, clipped shell so split panes inherit the same visible corner treatment as the larger outer panel shells
- Kept resize handles active on the invisible gap and removed the old painted divider line, which was the source of the pressed-together look
- `cargo build` completed successfully in 38.70s after the split panel shell pass
- The app was relaunched with `cargo run` on the updated binary

## Planned Commit And Push

- [ ] Verify the final file set to be committed
- [ ] Create a single commit for the workspace/UI restyling pass
- [ ] Push the commit to `origin/main`
