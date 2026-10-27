# Spec: ActionBar, BulkActionBar

Source: `ely-core/src/buttons/bars.rs`.

## ActionBar
- Children in a row, items centered, 8 px apart; styleable like a div.
- Docked (default): actions at the end, 16 px sides, 12 px top and bottom, surface fill, a 1 px top border.
- Floating: 6 px padding, a pill (radius full), overlay fill, 1 px border, floating shadow.
- Entrance when it first shows: over BASE, ease-out-cubic, opacity t and top margin 8 px × (1 − t).

## BulkActionBar
- Props: id, count, actions, on_clear (optional).
- Nothing (an empty div) at count 0. Otherwise a floating ActionBar of: the count (AnimatedNumber, sm) and "selected", 4 px apart, sm text in fg, 8 px in from the left; a 1 px hairline a small control tall; the actions; with a handler, a small IconButton `x` with tooltip "Clear selection" (logs, then runs the handler).

## Slint mapping
- `ActionBar { floating; shown; @children }`; out `entrance`. `shown: false` hides it and takes no room; showing it again replays the entrance.
- The bar is drawn lower inside a fixed box during the entrance (gap G49).
- `BulkActionBar inherits ActionBar { count; clearable; clear() }`, shown while `count > 0`. `clearable` stands for gpui's optional handler.
