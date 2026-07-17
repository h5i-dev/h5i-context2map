---
name: c2m
description: Survey a whole repository in ~2k tokens as a semantic map image (Repository Atlas), then zoom to exact source via stable handles. Use at the START of any task in an unfamiliar or large repo — before grepping around — to learn where the relevant code lives, what depends on what, and where the trust hazards are.
---

# c2m — Repository Atlas

`c2m` compiles the repository into a **query-conditioned map**: an image where
position = module topology, cell area = code size, **elevation (▲1–▲5) =
relevance to your current task**, hatched red = trust hazards, arrows =
dependencies. Every region/file/symbol carries a stable handle (`R3`, `F103`,
`S12`) that resolves back to exact source. The image is an *index*, never the
source of truth — exact code always comes from `c2m read` as text.

## Workflow

1. **Map the repo against your task** (auto-builds the index on first run):

   ```bash
   c2m map "<one line describing your task>" --provider claude --budget 2000
   ```

   stdout is the legend (region roster with handles + elevation). If it prints
   an `# atlas: <path>` line, **Read that PNG file now** — the image carries
   the geography the legend can't. On small repos it may print a text-only
   roster instead (`representation: text`) — that alone is a complete map; no
   image to read.

2. **Pick the summit.** Start from the highest-elevation regions (▲5/▲4) and
   their top files. `⚠net/exec/secrets/eval` tags mark files that touch the
   outside world — relevant for anything security-adjacent.

3. **Zoom one level** when a region looks right:

   ```bash
   c2m zoom R3            # writes a region tile image + prints a file/symbol roster
   c2m zoom R3 --text     # roster only, no image
   c2m zoom F103          # file detail: symbols with S-handles, imports, hazards
   ```

   Read the tile image the same way if one is written.

4. **Get exact source as text** (never trust pixels for code):

   ```bash
   c2m read F103                  # whole file, numbered
   c2m read S12                   # just that symbol's line range
   c2m read F103 --lines 40:120
   c2m locate "session|expiry"    # find handles by substring
   ```

## Rules

- The atlas is for *navigation*. Quote, edit, and reason only over `c2m read`
  output (Layer 3 text), never over what you saw rendered in the image.
- Handles are stable across runs and queries — safe to mention in commits,
  notes, and follow-up commands.
- Re-run `c2m map "<new task>"` whenever your task changes; it re-elevates the
  same geography in well under a second (warm cache), so map early and often.
- `--json` on `map`/`zoom` gives `{atlas_path, legend, ...}` when you need to
  script it.
- For a human-facing map (README, PR description):
  `c2m render --out map.svg` (parchment theme) or `c2m badge`.
