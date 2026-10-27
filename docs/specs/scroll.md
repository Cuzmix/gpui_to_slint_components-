# Spec: Scrollbar, ScrollArea, ScrollShadow, ScrollToTop, StickyHeader

Source: `ely-core/src/layout/scroll.rs`, `scroll/source.rs`, `scroll_aids.rs`; stories in `ely-gallery` `pages/layout/scrolling.rs`.

## Scrollbar
- Tokens: thickness 6 px, shortest thumb 24 px.
- Track: along the right edge (vertical) or bottom (horizontal), the box's full length, 12 px thick, the thumb inset 3 px from the inner side. The track takes presses so nothing beneath sees them; the wheel passes.
- Place: span = the box's length on the axis; reach = content − span. No bar when reach ≤ 0. Thumb length = clamp(span × span / content, 24, span); travel = span − length; no bar when travel ≤ 0. Thumb start = travel × clamp(scrolled / reach, 0, 1).
- Thumb: fully round, `fg` at 22 % opacity, 42 % while hovered.
- Drag: the thumb drags; the first pointer position of the drag is the anchor, and each move sets the scroll so `clamp((start at press + along − anchor) / travel, 0, 1)` of the reach lies before the view.
- Presence: given (0..1, opacity; at 0 nothing is there) or auto: fully shown while hovered, while dragged, and for 900 ms after the offset last changed, then linear fade over `motion::SLOW` (320 ms; reduced motion shortens it). Never scrolled: hidden.

## ScrollArea
- A box that clips; its body fills it and scrolls both ways, or only down with `vertical` (children fill its width and never widen it). Wheel events the area can use along their axis stop there.
- Its bars take the area's presence (the auto rule above, held while the pointer is on the area or on either bar). `shadows` adds a ScrollShadow. `track_scroll` lets the owner hold the handle.

## ScrollShadow
- Over the box, clipped to it: a zero-height edge with the `Floating` elevation shadow at the top while scrolled > 0, and at the bottom while scrolled < reach.

## ScrollToTop
- Shown while the view has height and scrolled ≥ half the view height. Floats 16 px from the bottom right: fully round, `surface` fill, `Floating` shadow, a secondary IconButton with `arrow-up`.
- Click: every 8 ms the offset becomes `from × (1 − ease_out_cubic(t))`, t over `motion::SLOW`, until t = 1.

## StickyHeader
- A section: its header in flow (invisible), then the body, then the header again, absolutely placed at `clamp(scrolled − section top, 0, max(0, section height − header height))` on the `bg` fill, drawn over the body.

## Slint
- The scroll handle is a Flickable. Each aid takes Flickable's own terms: `content-length` (`content-height`/`-width`) and `offset` (`content-y`/`-x`: 0 at the start, negative once scrolled), so an owner binds them straight, two-way where the aid moves the scroll. Aids fill their parent; put them beside the Flickable.
- Scrollbar's `auto` and ScrollArea's bars share one presence rule (an idle Timer of 900 ms and an animated linear fade). `on-bar` reports the pointer on the track.
- ScrollArea is a TouchArea (hover) around a Flickable whose body is a VerticalLayout (top aligned), as gpui's block children stack. Mouse-drag panning is off, since gpui scrolls by wheel only. It fills its parent by default (`preferred-width/height: 100%`), and exposes `content-x/y` for the owner (gpui `track_scroll`) and `presence`.
- ScrollShadow draws each edge as a zero-height Elevated; `top-shaded` and `bottom-shaded` report them.
- ScrollToTop runs gpui's 8 ms loop with a Timer and `animation-tick()`.
- StickyHeader is the header itself, placed first in its section; it is told the box's top (`absolute-position.y` of the Flickable) and the section's height, keeps its place in the layout, and draws its content moved down by `pinned`, above later siblings (`z: 1`) on `bg`. Slint cannot render one child twice, so the in-flow copy is the element's own box.
- Gaps: G31 (wheel smoothing and edge hand-off).
