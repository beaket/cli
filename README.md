# beaket

`beaket` reads the `specs/` directory of a repository and answers, for each topic that names a code
site, whether the code still agrees with the answer written down.

It runs on a checkout — in CI, or on your own machine — and holds no credential.

## Status

Nothing is released yet. This repository is being built against a contract that already exists.

## Where the contract is

**The records this program implements are held privately, in `beaket/bk`.** They are cited here by
their stable names — `Spec (line)`, `Spec (drift)`, `Spec (index)`, `ADR-0068` — and never by URL, so
a reader without access still learns that a record exists and what it is called, rather than
following a link that does not open.
