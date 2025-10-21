#import "@preview/codly:1.3.0": codly, codly-init

#show: codly-init.with()

#let _base_config() = {
  codly(
    highlight-stroke: it => none,
    highlight-radius: 0pt,
    zebra-fill: none,
  )
}

#let _render_block(lines, lang, color, prefix) = {
  let code = lines.map(line => line.text).join("\n")
  let highlight_lines = lines.filter(line => line.spans.len() > 0).map(line => line.line)

  let light_fill = color.lighten(60%)

  {
    _base_config()
    codly(
      fill: light_fill,
      zebra-fill: light_fill,
      highlighted-default-color: color,
      highlight-fill: it => it.mix(light_fill),
      highlighted-lines: highlight_lines,
      highlights: (),
    )
    raw(code, lang: lang)
  }
}

#let inline_diff(plugin, before, after, lang: "rust", colors: (
  before: red,
  after: green,
)) = {
  let payload = json(plugin.inline_diff_segments(
    bytes(lang),
    bytes(before),
    bytes(after),
  ))

  let left = _render_block(payload.before, lang, colors.before, "-")
  let right = _render_block(payload.after, lang, colors.after, "+")

  {
    left
    v(1em)
    right
  }
}
