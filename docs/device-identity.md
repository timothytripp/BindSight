# Identical joystick models

SDL GUIDs and Star Citizen Product GUIDs identify a model, not an individual
unit. Two vJoy collections can share both GUIDs and their displayed name while
having separate SDL instance IDs and HID paths. Capabilities such as hat counts
help recognize a device during verification, but are not identity keys.

BindSight uses GUID plus SDL instance ID for live events, highlighting, axis
throttling, held inputs, and editor recording. Joystick slot selection is shared
by monitoring, binding recording, and binding previews. Image-map compatibility
and existing image-map choices still use model IDs, so identical units can use
the same visualization.

## Choosing slots

For identical models, use **Choose slot** on each Input Devices tile. Verify in
Star Citizen which `jsN` belongs to each device, then select that slot in
BindSight. This changes only BindSight's local GUI state. Selecting a slot
already assigned to another connected device clears the other assignment.
Until a unit has a valid choice, its joystick inputs do not resolve to bindings.
Unique models continue to resolve automatically.

Choices use SDL's OS device path, or its serial when no path is supplied. A
missing identity or an identity shared by several connected units permits only
a session-local slot choice. Choices also include the game's current slot map
and the identities of the connected units of that model. A changed set or slot
map requires verification again. Returning to the same set restores its choices.
Once a model has required manual selection, it continues to require selection
even when only one unit remains.

Paths are not a universal permanent hardware ID: Windows driver reinstalls,
USB port changes, or Linux evdev enumeration may change them. Moving a device
to another slot within an otherwise identical Product-only game map cannot be
detected. Recheck the choices after changing controller configuration or the
game's order. The log and actionmaps file do not establish a physical unit's
identity, so BindSight does not infer it from SDL enumeration order.

The order analyzer consumes repeated Product GUID occurrences separately.
Automatic resort suggestions are disabled when the saved or current map
contains repeated Product GUIDs; a warning explains the ambiguity. HID report
descriptor lookup prefers the exact SDL collection path. When that cannot be
matched and several HID joystick interfaces share a VID/PID, axis diagnostics
report the ambiguity instead of reading another unit's descriptor.

## Regression and hardware verification

Run `pnpm test`, `pnpm build`, and `cargo test --lib` in `src-tauri`.
`cargo run --example enum_joysticks -- --json` emits the full enumeration for
local comparison. Device paths and serials may identify the machine; redact
them before sharing diagnostics.

Windows vJoy verification performed during development:

- Two devices with identical names, SDL GUIDs, and Product GUIDs were enumerated
  separately. Both had eight axes and 128 buttons; one had four hats and the
  other one hat. Their HID paths identified separate collections; SDL supplied
  no serial for either unit. Axis descriptor lookup succeeded for both.
- A second diagnostic process preserved the two collection paths and counts.
- The game's log and saved joystick options contained two occurrences of the
  same Product GUID, at js1 and js2, without a per-unit identifier.
- Synthetic regression tests reverse enumeration and replace SDL instance IDs
  while preserving path-based choices; they also cover conflicting choices,
  disconnected peers, changed slot maps, and ambiguous serials.

Still requiring physical/game verification:

1. Confirm which of the four-hat and one-hat units Star Citizen uses as js1 and
   js2. Make those choices in BindSight.
2. Press and release the same button number on both devices, including holding
   one while releasing the other. Verify independent highlights and the correct
   bindings. Repeat with axes, hats, image-map recording, and binding recording
   (cancel the dialog to leave game bindings untouched).
3. Restart BindSight and confirm the choices and input ownership remain correct.
4. Reconnect or re-enable one virtual unit, then change enumeration order.
   Verify distinct live event ownership and either preserved path-based choices
   or a request to verify again when identities or the slot map change.

The development diagnostics did not change vJoy, HidHide, or game configuration.
