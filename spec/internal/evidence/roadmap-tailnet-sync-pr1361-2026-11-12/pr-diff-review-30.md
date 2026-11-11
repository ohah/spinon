# PR #1361 Tailnet 미리보기 동기화 · 적대적 diff 검토 30개 관점

검토 대상은 PR #1361 source에서 만든 선택 문서 빌드, Tailnet 사전목록/백업/복사/HTTP 해시 영수증, Agent Browser DOM 및 화면 캡처다. 앱 runtime 기능이나 GitHub Pages 배포를 검증한 것으로 확대하지 않는다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | source revision mismatch | All selected-root and publication receipts pin source `459783407f043265b350030e9ebc49bfb4651a1d` and tree `754e4fe183f37a1ef2dc27da434d5359e4fa397f`. |
| 2 | checkout tree drift | The selected checkout tree is `754e4fe183f37a1ef2dc27da434d5359e4fa397f`, equal to the source tree. |
| 3 | unexpected selected files | The selected root contains 67 tracked files and 7,796,436 bytes; it records zero symlinks. |
| 4 | full docs build overstated | The full docs build failed on pre-existing dead links; README explicitly preserves this as a limitation. |
| 5 | restricted build validation overstated | The selected-root build passes with generic dead-link/image checks disabled, and all receipts keep fullDocsLinkValidation=false. |
| 6 | RSPress version drift | The selected build used the repository-pinned RSPress 2.0.22; package and lockfile were not edited. |
| 7 | build config scope leak | The temporary config is scoped to the selected spec root and is outside tracked source. |
| 8 | publication route omissions | The publication manifest contains 138 routes and lists 11 required roadmap/status/plan/evidence routes. |
| 9 | unreviewed publication bytes | The publication manifest accounts for all 22,763,944 bytes with per-route hashes. |
| 10 | deletion of existing preview files | Preflight, copy, and publication receipts each report no deletes. |
| 11 | wrong source route selection | Roadmap aliases, STATUS routes, the five-fit plan, contract, and post-merge evidence pages are listed among required routes. |
| 12 | stale roadmap aliases | The selected publication routes roadmap.html, public/roadmap.html, and spec/public/roadmap.html from the same source HTML hash. |
| 13 | unexpected source collision | The staged root is built from one source commit; route mapping is captured explicitly instead of inferred at copy time. |
| 14 | existing files overwritten without record | Preflight enumerates 31 replacements and 31 backups before copy. |
| 15 | backup content drift | All 31 backup hashes passed verification; aggregate size is 10,283,318 bytes. |
| 16 | created files mislabeled | Preflight counts 94 create, 31 replace, and 13 same; sum is 138. |
| 17 | copy partial failure hidden | The first attempt stopped before target writes because the backup parent did not exist; the successful second attempt is separately receipted. |
| 18 | target bytes differ from selected build | All 138 target files were re-read and hash-verified after copying. |
| 19 | wrong preview root | Preflight names the selected root as `SpinonPreview` under the existing preview server. |
| 20 | wrong preview server lifecycle | Listener PID 90711 remained on 127.0.0.1:8768; it was neither replaced nor restarted. |
| 21 | Tailnet endpoint not actually reachable | All 138 HTTPS routes returned HTTP 200. |
| 22 | served response is stale or mismatched | The served-verification receipt compares the digest of every response body with the selected publication digest; all match. |
| 23 | browser checks only the local build | Agent Browser opened the public Tailnet HTTPS URL `https://macstudio.tailed42f2.ts.net/spinon/roadmap.html`, not a local build path. |
| 24 | merged card or evidence is missing from rendered page | Rendered DOM contains PR #1360, its merge SHA, and the linked evidence labels. |
| 25 | official completion count inflated | Rendered page shows 78/214; the sync did not edit STATUS or completion checkboxes. |
| 26 | C11.4 mislabeled complete or active | Rendered page states C11.4/C11 incomplete and current Kanban progress 0/2; C11.4 appears as latest merged summary. |
| 27 | runtime result represented as CI | The page states 34,560/34,560 RuntimeSession scenes and explicitly says GitHub checks/workflows are zero, not CI green. |
| 28 | iOS evidence generalized to feature parity | Roadmap copy qualifies iOS/Android checks as basic Root HTML smoke and says five-fit geometry was not verified. |
| 29 | screenshot inaccurate or unverifiable | Agent Browser screenshot is 1280x633, SHA-256 `85ec6f6063c17cc5789aba66c0eec6f5128bf209be930cc807f5f2fd56928b68`, and visibly shows the updated top-of-page status. |
| 30 | agent browser or unrelated browser left open | The dedicated Agent Browser session was closed and its executable process was checked; user-owned Terminal Browser was not touched. |

**검토 결과:** Tailnet 제공본은 PR #1361 merge source와 138개 경로 모두 일치한다. 전체 문서 dead-link 검증은 기존 오류로 통과하지 않았으며, 독립 외부 reviewer의 승인을 뜻하지 않는다.
