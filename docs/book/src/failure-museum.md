# Failure museum

`experiments/_wip/` is a deliberately preserved directory for *failed*
work. The convention comes from a working note that crystallised over
several long debugging sessions:

> "Three months from now you will need exactly the lemma you almost had
> on Tuesday. Do not delete the file."

When a phase of work hits its bug-fix budget and is genuinely stuck, the
working tree for that phase is moved (not copied — moved) to
`experiments/_wip/<topic>-<YYYYMMDD>/`. The `.gitkeep` at the top of the
directory exists only to keep the folder around for first-time clones.

## Why this is a feature

- **It curtails sunk-cost rumination.** Once the failing branch is in the
  museum it is no longer "the thing I should make work today". The mental
  budget you previously spent on it returns to other tasks.
- **It keeps regression material.** Many of the same bugs are encountered
  twice, six months apart. Comparing the new symptom against the old
  failure tree is faster than re-doing the original investigation.
- **It documents the path *not* taken.** Future readers can see which
  approaches were tried and why they were abandoned, without having to
  trawl `git log`.

## What should not go in

- Anything secret. The museum is committed.
- Anything generated. Build outputs, target directories, locally-derived
  caches. Those belong in `.gitignore`.
- Anything that is just *unfinished* rather than *abandoned*. Use a
  branch for in-progress work; the museum is for completed-and-given-up.

## Pruning

Once a year the museum is pruned: any directory older than three years
that has not been referenced from a CHANGELOG, a doc, or an issue may be
deleted. Pruning is always a single commit so that the history is
recoverable.
