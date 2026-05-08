# HTML Guidelines

## Document structure

- Use semantic elements that match the content purpose before reaching for generic containers.
- Keep document structure shallow and readable; avoid unnecessary wrapper nesting.
- Use headings in order so the content outline remains understandable to people and tools.
- Give interactive regions, navigation, and complementary content explicit landmarks.
- Keep templates focused on content structure, not styling workarounds.

## Naming and composition

- Use stable, intention-revealing class and data attribute names.
- Prefer small reusable partials or templates over large pages with repeated markup blocks.
- Keep components cohesive; split sections when markup serves multiple unrelated purposes.
- Treat large repeated fragments as a signal to extract a shared template or component.
- Keep embedded behavior hooks obvious and localized.

## Accessibility and content quality

- Write markup that works without JavaScript enhancement wherever practical.
- Provide meaningful text for buttons, links, labels, and form instructions.
- Use `alt` text for informative images and empty `alt` for purely decorative images.
- Associate form controls with labels and expose errors near the relevant field.
- Do not use structure, color, or placeholder text alone to convey required information.

## State and behavior boundaries

- Keep HTML responsible for structure and meaning; keep styling in CSS and behavior in scripts.
- Avoid inline event handlers and inline styles except in tightly justified cases.
- Use `data-*` attributes for script hooks when no semantic attribute fits.
- Keep DOM structure predictable so behavior and tests can target stable elements.
- Minimize hidden coupling between markup structure and script assumptions.

## Performance and maintainability

- Avoid excessive DOM depth and repeated heavy fragments when simpler markup will do.
- Load media and embeds intentionally with attributes that match the user experience goal.
- Remove dead elements, commented-out blocks, and stale fallback markup quickly.
- Keep templates easy to diff by using consistent attribute order and formatting.
- When a page becomes hard to scan or explain briefly, split it into smaller components.

## Testing and validation

- Validate markup and fix structural errors instead of relying on browser recovery behavior.
- Test rendered output with keyboard navigation, screen readers, and narrow viewports as needed.
- Keep selectors for tests resilient by targeting semantics or stable hooks instead of brittle depth.
- Exercise forms, empty states, and error states, not just happy paths.
- Review generated HTML for accessibility regressions whenever templates change.
