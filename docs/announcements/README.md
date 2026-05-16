# Announcement drafts

Public-launch templates for v0.1.0. **None of these have been posted.**
They are drafts; the actual posting happens only after the public-push
go/no-go check at the top of `docs/book/src/release-process.md`.

| File | Channel | Status |
|---|---|---|
| [`r-rust.md`](./r-rust.md) | reddit.com/r/rust | draft only |
| [`mathstodon.md`](./mathstodon.md) | mathstodon.xyz (Mastodon) | draft only |

## Why these two

These two channels were chosen, in order of priority, when the project
was scoped (see `architecture/user-decisions.md` Q3). Other channels
(HN Show, arXiv, lobste.rs, X) are deliberately deferred:

- **r/rust** — primary, large but technically critical audience; if the
  library has API gaffes, this is where we want to learn it. Low
  bouncing-off-the-walls risk.
- **mathstodon.xyz** — secondary, smaller but extremely well-informed
  about the math; the right place to advertise the signature-parametric
  angle.

## Cooling-off rule

We will not post both on the same day. The r/rust post goes first; we
wait at least 24 h, read the comments, and only then publish to
mathstodon. This makes it easier to iterate on the post body if a
common criticism surfaces.
