# Spec: ActionSheet

Source: `ely-core/src/buttons/sheet.rs`.

- Props: id, on_close, title (optional), message (optional), actions (label, destructive, handler).
- On a Backdrop placed at the bottom; the panel 380 px (`sheet_size`) wide, centered, 16 px off the bottom.
- Panel: a focus trap (role dialog: named by the title, else the message, else "Actions"; described by the message when titled), two cards 8 px apart.
- Card: column, clipped, radius xl, overlay fill, 1 px border, modal shadow.
- Heading (with a title or message): centered column, 4 px apart, 24 px sides, 16 px top and bottom; title sm semibold fg-muted, message sm fg-subtle.
- Row: lg + sm/2 control heights tall (44 px), 1 px top border, md text centered, normal weight; danger ink when destructive, fg otherwise; pointer; Tab stop; active fill while focused, hover fill on hover. A click runs the handler (logs), then closes.
- Cancel: its own card, a row without the top border, semibold "Cancel"; closes.
- Escape (stops propagation) and a press on the scrim close. Closing hands focus back.
- Entrance: over BASE, ease-out-cubic, opacity t and a bottom margin of −24 px × (1 − t).

## Slint mapping
- `ActionSheet inherits Backdrop { title; message; actions: [SheetAction]; activated(index); close() }`.
- It focuses its trap on showing; the owner refocuses in `close` (G19); Tab wraps through `FocusSteps` (G18).
- The panel is drawn lower inside a fixed box while it rises (G49). Role groupbox for dialog (G51).
