# E08 시작 시간 최적화 계획 · 적대 검토 20개

- **대상:** [E08 계획](./e08-startup-optimization.md)
- **검토 기준:** main `04947b4a4055ce315a221194feb7f6c2460c9f16` 기반 계획 전용 worktree
- **검토 범위:** 계획의 입력·oracle·계측·호환성·통계·자원 격리 실패 경로만 검토. 구현·빌드·측정의 통과 기록이 아니다.
- **수행 방식:** 서로 다른 3개 관점이 7·7·6개 실패 경로를 맡아 읽기 전용으로 검토했다. 초안에서 발견된 부분 판정을 계획에 반영한 뒤 해당 관점을 재검토했다.

| # | 적대 관점 | 발견 사항과 보정 | 최종 판정 |
|---:|---|---|---|
| 1 | “bundle pre-transform”을 임의로 하나의 변환으로 정함 | transpile·bundle·minify 의미를 미정으로 유지하고, 실행 전 구체 정의 또는 근거 있는 미적용을 요구한다. | PASS |
| 2 | 잘못된 JS/HTML bytes를 baseline으로 선택 | 실제 Android/iOS route와 산출물 bytes의 일치를 확인한 뒤 복사본·원본 hash를 고정하며 shared output을 재생성하지 않는다. | PASS |
| 3 | 빠진 동작이나 hash 하나로 의미 동등성 오판 | 기본 `index.html`/`root-app.js` route, field별 DOM assertion과 expected JSON을 baseline 전에 고정한다. | PASS |
| 4 | V8/platform/compile duration의 중첩 이중 계수 | 각 stage의 시작·끝, monotonic clock, nested interval 및 전체 합산 금지를 기록한다. compile/evaluate와 worker/session ready 이벤트를 분리했다. | PASS · 재검토 |
| 5 | GPU submit 또는 callback을 첫 픽셀로 오인 | submit과 presentation callback을 별도 event로 두며 둘 다 optical display 증거로 쓰지 않는다. | PASS |
| 6 | 청크 graph/OTA/X01 완료를 E08에 전제 | 단일 bundled path만 제한적으로 다루며 loader·OTA·Vite/Rspack parity는 별도 항목으로 둔다. | PASS |
| 7 | A를 측정하지 않고 B/C만으로 E08 전체 완료 | A/B/C 각각을 측정·근거 있는 미적용·미해결로 분류하고 미해결이면 완료 후보를 막는다. | PASS |
| 8 | 다른 V8 library 또는 adapter의 artifact를 재사용 | full revision, static library 및 adapter source hash, build flags, ABI, OS/profile, JIT mode와 artifact format을 identity에 포함한다. | PASS |
| 9 | compile cache를 다른 mode/origin/options로 소비 | default `Script::Compile` baseline을 기록하고 실제 API overload, `ScriptOrigin`, script/module mode와 compile options를 pinned V8 source에서 확인하도록 한다. | PASS · fail-closed |
| 10 | V8 snapshot에서 이전 session pointer/callback 복원 | snapshot seed에서 session-specific pointer/state를 금지하고 새 Isolate마다 callback·user data를 새 포인터로 재결합한 뒤 evaluate하도록 한다. | PASS |
| 11 | corrupt/truncated 또는 빠진 metadata를 유효 cache로 처리 | byte flip, truncate, absent/conflicting metadata, cache miss를 별도 negative control로 둔다. | PASS |
| 12 | file hash만으로 code-cache hit/snapshot restore 주장 | hash 외에 pinned API의 consume/rejection/restore signal과 stage trace를 요구한다. signal을 입증하지 못하면 implementation/GN 변경과 사용 성능 주장을 중단한다. | PASS · fail-closed |
| 13 | miss/reject/corrupt를 같은 fallback으로 합치거나 다른 파일 삭제 | miss·stale·reject·corrupt를 다른 result code로 남기고 task-owned artifact만 invalidate; baseline compile/rebuild 경로를 요구한다. | PASS |
| 14 | V8 15.6 후보 pin을 cross-version 보장으로 확대 | V8 15.6.0은 실험 pin이며 stable promise가 아니다. 정확히 일치하지 않는 V8/build/platform은 default reuse 대상이 아니다. | PASS, 명시 범위 한정 |
| 15 | cold process와 warm Isolate를 한 cohort로 혼합 | fresh process/no artifact, fresh process/valid artifact, same process/new Isolate, same Isolate/re-evaluate를 분리한다. OS page cache cold 주장은 금지한다. | PASS |
| 16 | 순서·표본·CI·제외 기준을 결과를 본 뒤 선택 | randomized AB/BA blocks, 최소 표본, p95 별도 수량, paired bootstrap 절차 및 사전 고정 제외 규칙을 정한다. threshold/메모리/artifact 예산이 후보 결과 전에 정해지지 않으면 결과는 탐색값이고 E08 미완료다. | PASS · 측정 전 관문 |
| 17 | 계측 overhead 또는 의미 변화가 성능 이득으로 보임 | B0 계측 on/off 대조, baseline noise floor, root-app field별 oracle 및 보정 불가 시 중단 기준을 둔다. | PASS |
| 18 | simulator JIT 결과를 iOS device/JITless로 일반화 | Android emulator·iOS Simulator·실기기를 별도 cohort로 나누고 target ABI, OS/profile, JIT/JITless와 제한된 주장 범위를 기록한다. | PASS |
| 19 | C11 matrix와 Cargo/V8/bootstrap output을 공유 | 계획 작성 시 C11 HEAD/PID/log를 동시성 기록으로 특정하되 E08 결과와 섞지 않는다. 해당 run의 terminal HEAD/log hash 확인 전 build·measure를 금지하고 variant output/Cargo target을 분리한다. | PASS |
| 20 | 실패 증거를 지우거나 다른 process를 cleanup | 실패·손상 artifact와 raw logs를 보존하고 정확한 run-owned directory만 정리한다. 삭제 전 process/file handle을 확인하고 task-owned process만 종료한다. | PASS |

## 최종 판정

위 20개 계획 실패 경로를 검토했고 발견한 부분 사항을 보정한 뒤 해당 관점을 재검토했다. 계획 문서 기준으로 추가 blocker는 없다. 이는 실험 실행이나 E08 구현의 통과가 아니다. C11 terminal 기록, A/B/C 후보별 pinned API·oracle·threshold 준비, 이후 구현 PR 검토가 별도 관문이다.

검토자는 Rust/V8 build, test, benchmark, app launch, browser, simulator를 실행하지 않았다. 계획 검토가 완료되기 전에도 그러한 실행은 하지 않았다.
