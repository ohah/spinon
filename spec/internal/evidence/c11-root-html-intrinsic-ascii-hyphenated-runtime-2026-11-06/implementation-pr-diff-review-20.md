# C11.4 ASCII 하이픈 단어 구현 변경 적대 검토 20개

구현 변경의 경계와 실제 diff를 계획 문서와 대조했다. 이는 구현/runtime 실행 검토와 구별되는 소스 diff 검토다. Android/iOS app run이 끝나면 해당 근거와 현재 결과를 함께 반영한다.

| # | diff 공격 관점 | 확인 위치 | 판정 |
|---:|---|---|---|
| 1 | `split('-')`가 하이픈 외 구두점을 허용 | `root_html_intrinsic_auto_rows.rs` token predicate | 허용 기호는 ASCII U+002D뿐 |
| 2 | split 결과에 빈 segment가 남아 `-A`, `A-`, `A--B` 허용 | 같은 predicate 및 grammar positive/negative unit test | `!segment.is_empty()`로 거부 |
| 3 | Unicode alphanumeric을 ASCII로 오인 | `segment.bytes().all(is_ascii_alphanumeric)` | ASCII만 허용 |
| 4 | collapsible whitespace를 임의로 넓히거나 줄임 | 기존 token split·whitespace controls | 기존 U+0020/TAB/LF/CR 유지 |
| 5 | expected count를 JSON 길이에 종속시켜 scene 누락 은폐 | Style-to-layout/runtime letter_words helpers | 9개를 명시해 확인 |
| 6 | precomparison expected를 새로 쓰면서 browser oracle과 source를 섞음 | `expected.json`·Chrome observation·derive manifest | independent expected를 별도 유지 |
| 7 | RuntimeSession matrix가 expected CSS track을 frame과 혼동 | runtime frame·row trace assertions | 실제 frame 및 track contribution을 별도로 검사 |
| 8 | original source identity를 normalized marker text로 대체 | `input.sources`, node id, revision assertions | source 및 owner identity 유지 |
| 9 | positive hyphen cases만 추가하고 invalid cases 제거 | style/runtime negative controls | leading/trailing/repeated·Unicode controls 유지 |
| 10 | `B-1` 기존 reject control을 positive grammar로 바꿔도 구별력 없음 | stale controls를 `B--1`로 치환 | 새 reject class 유지 |
| 11 | marker가 non-digit successor 앞에도 생성 | `add_ascii_hyphen_digit_break_markers` | 다음 byte ASCII digit일 때만 삽입 |
| 12 | marker가 leading minus 앞에 생성 | previous-byte alphanumeric check 및 `-13` case | leading minus에는 삽입 안 함 |
| 13 | normalized buffer만 바꿔야 하는 경계에서 HostDocument를 수정 | `normalize.rs`와 line-break unit test | input source 유지, buffer-only marker |
| 14 | raster가 zero-length UTF-16 mapping을 거부 | authored hyphen source range에 marker를 병합 | runtime raster 경로 통과 |
| 15 | 다중 삽입에서 후속 UTF-8 ranges가 stale | 역순 삽입 및 range shift | 복수 하이픈 matrix 통과 |
| 16 | line-break internal `Strict`가 CSS `auto` 기본 경로를 막음 | enum gate 제거 및 line-break Normal/Strict tests | CSS default route 통과 |
| 17 | `hyphens:auto` fixture에 U+200B를 끼워 넣음 | support gate와 auto-hyphenation fixture tests | auto profile에서 skip; 3/3 |
| 18 | simulator run이 기존 실행 app이나 asset을 덮어씀 | `run-simulators.sh` preflight/asset backup | 별도 Android ID, fresh iOS sim, restore trap |
| 19 | Android test build가 기존 application ID를 덮어쓰거나 임시 변경을 남김 | clean diff 검사, temporary Gradle backup, trap restore | byte-for-byte restoration 검사 포함 |
| 20 | screenshot/sample을 geometry 전체 검증으로 과장 | README 및 plan scope | Rust matrix는 geometry, surface 표본은 특정 좌표만 주장 |

이 검토에서 unresolved source-code 결함은 남지 않았다. Simulator 실행 결과를 반영한 뒤 실제 PR 전체 diff를 새 30관점으로 검토한다.
