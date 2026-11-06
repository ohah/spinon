# PR #1225 Tailnet roadmap sync evidence diff review · 30 points

This review covers the external preview update and its evidence. It checks the target boundary, source/served identity, browser screenshots, and status claims.

| # | Adversarial check | Evidence inspected | Result |
|---:|---|---|---|
| 1 | Sync source is from an unmerged branch | `preflight.json` source HEAD | Main merge SHA `b9e38dac…` |
| 2 | Latest card links the wrong implementation PR | Source marker and browser text | PR #1224 |
| 3 | Card cites an incorrect implementation merge SHA | Source, local HTTP, Tailnet HTTPS markers | `4039ed603…` |
| 4 | Roadmap implies full C11 completion | Card source and desktop/mobile DOM checks | C11.4/C11 remain incomplete |
| 5 | Overall completion count changes or disagrees | Source and served body marker | `78/214` preserved |
| 6 | Tailnet update targets a generated directory instead of its real source | Sync script and README | `spec/public/roadmap.html` is the standalone source |
| 7 | A stale or unexpected target path is overwritten | Preflight target path | Only `SpinonPreview/roadmap.html` |
| 8 | Preview root resolves through a symlink | Preflight realpath assertion | Rejected unless exact regular directory |
| 9 | Unexpected listener or server process is used | `lsof`, `ps`, preflight report | Expected Node listener PID `28040` |
| 10 | Tailscale is disconnected while reporting a successful deploy | `tailscale status --json`, HTTPS fetch | Backend Running; HTTPS 200 |
| 11 | Existing target is open during replacement | `lsof` target check | Zero open handles |
| 12 | Existing content is overwritten without a backup | Backup JSON and hash | Old target copied byte-identically |
| 13 | Backup path collides with previous evidence | Unique backup root and exclusive create | New isolated pass directory |
| 14 | Copy can expose a partially written document | Same-directory temp and atomic rename | Atomic replace recorded |
| 15 | Other preview files are deleted or changed | Script `filesToReplace` and report | One file; no deletes |
| 16 | Preview service is restarted unnecessarily | Copy and verification reports | `serverRestarted=false` |
| 17 | Local server serves a different body than source | `served-verification.json` | HTTP 200, exact size/hash |
| 18 | Tailnet HTTPS serves stale content | `served-verification.json` | HTTP 200, exact size/hash |
| 19 | Existing backup is corrupted or differs from old target | Backup re-hash | Verified against previous target hash |
| 20 | Browser visited a local file or another hostname | Agent Browser URL report | Exact user Tailnet HTTPS route |
| 21 | Browser page title or route is an error page | Desktop/mobile DOM report | “스피논 구현 로드맵” |
| 22 | New PR and merge SHA are absent in rendered card | DOM checks and screenshots | Both present |
| 23 | `78/214` or incomplete status is absent in rendered card | DOM checks and screenshots | Both present |
| 24 | Desktop content overflows its viewport | Desktop geometry | 1440px document/body; no overflow |
| 25 | Mobile content overflows its viewport | Mobile geometry | 390px document/body; no overflow |
| 26 | Desktop card unexpectedly narrows or clips | Desktop card bounds | x=90, width=928, right=1018 |
| 27 | Mobile card unexpectedly narrows or clips | Mobile card bounds | x=14, width=362, right=376 |
| 28 | Browser session remains open after capture | lifecycle JSON and `session list` | Closed; no active session |
| 29 | Screenshots are mislabeled or stale | PNG dimensions and visible card content | Desktop/mobile captures match the live page |
| 30 | Deployment evidence is presented as a code completion increase | README and status ledger | No checkbox or counter promotion |

The live preview update matched source bytes at both HTTP endpoints. No site-wide deployment or implementation-state promotion is included.
