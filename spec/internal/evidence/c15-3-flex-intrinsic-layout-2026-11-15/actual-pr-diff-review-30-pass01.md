# C15.3 Flex intrinsic test-only · 실제 PR diff 적대 검토 30관점 · pass01

## 검토 경계

검토 대상은 GitHub PR [#1390](https://github.com/ohah/spinon/pull/1390)의 실제 diff다. 최신 소스 head `e193fbadf3de9d939e99b750d3cad440a67e8857`; 이후 head `4b1eb273c9b94266ff68c9dc61b0d38ef2389ba0`는 README/README.html 두 문서 동기화만 포함한다. 코드 reviewer는 e193 head에서 source patch를 확인했고, evidence reviewer는 4b1 head에서 실제 PR patch·본문·라벨을 확인했다. 마지막 body에는 이 검토 뒤 Rust toolchain 차이를 덧붙였고 root가 다시 대조했다. 파일 수정·빌드·테스트·브라우저·GitHub review/comment·merge는 검토 단계에서 실행하지 않았다.

이 review report와 README 링크는 검토를 마친 뒤 기록 보존을 위해 추가한 문서 전용 변경이다. test/source/fixture는 변경되지 않았고 root가 보고서와 문서 parity를 다시 확인했다.

## 코드·테스트 관점 1–10

| # | 적대 관점과 판정 | 확인 근거 |
|---:|---|---|
| 1 | 제품 산출물에 테스트 코드나 font bytes가 포함되는가 — **PASS** | 부모 `c15_text_layout_tests`가 `#[cfg(test)]` 아래다. |
| 2 | fixture path가 다른 폰트나 경로 이탈을 가리키는가 — **PASS** | `include_bytes!`는 저장소의 C15.3 `NotoSansKR-VF.ttf`다. |
| 3 | host system font fallback이 결과를 바꾸는가 — **PASS** | `new_without_system_fonts()` 뒤 고정 family를 등록한다. font hash도 README/manifest와 일치한다. |
| 4 | text 입력·paragraph revision이 oracle과 무관하게 바뀌는가 — **PASS, 제한 있음** | `한글字`, 20px, line-height 24px, LTR, no-wrap이 맞는다. helper의 strict line-break 차이는 이 한 줄 사례에서 분기를 바꾸지 않는다. |
| 5 | 실제 Flex intrinsic 소비자 대신 고정 frame을 입력하는가 — **PASS, bounded** | Taffy에 max-content Flex 및 auto width/height/basis text item을 전달한다. |
| 6 | CSS fixture와 다른 tree/style을 같다고 주장하는가 — **PARTIAL, blocker 아님** | Chrome은 `inline-flex`/`span`, `height:auto`, 기본 stretch다. Rust는 direct `Flex`/Paragraph, `height:max-content`, `FlexStart`다. 한 줄 입력의 일치로 CSS projection을 주장하지 않는다. |
| 7 | child oracle rect가 없는데 parent 정답을 child에도 강제하는가 — **PARTIAL, blocker 아님** | Chrome JSON에는 Flex parent rect가 있으며 자식 frame의 별도 정답은 없다. 단일 장식 없는 item의 frame 비교로 범위를 제한한다. |
| 8 | Canvas advance를 DOM rect처럼 비교하는가 — **PASS** | 기대값은 Chrome `#flex-sizing` DOM rect `56.8125×24px`이고 Canvas metric과 구분한다. |
| 9 | layout snapshot의 owner/source/style/revision/line 결과가 잘못 연결될 수 있는가 — **PASS, 일부 미검증** | snapshot node, source/style, output revision, line count/height, available width를 확인한다. glyph advance와 snapshot width는 별도 oracle 비교가 아니다. |
| 10 | no-wrap에서 max-content callback 귀속을 확정하는가 — **PARTIAL, blocker 아님** | min/max-content 폭이 같을 수 있어 callback provenance는 보장하지 않는다. 이름·본문·근거는 bounded Taffy consumer로 한정한다. |

## 증거·PR package 관점 11–20

| # | 적대 관점과 판정 | 확인 근거 |
|---:|---|---|
| 11 | Chrome 관찰의 버전·viewport·DPR·font readiness가 빠지는가 — **PASS** | Chrome 155.0.8059.85, Agent Browser 0.31.1, macOS ARM64, 640×760, DPR 1, loaded font가 사전 비교 README에 있다. |
| 12 | Noto font 파일이 oracle 근거와 다른가 — **PASS** | asset SHA/라이선스가 기록돼 있고 현재 bytes가 기록 hash와 일치한다. |
| 13 | `56.8125`가 잘못된 metric에서 온 값인가 — **PASS** | 기대값은 DOM rect이며 Canvas width가 아니다. |
| 14 | 실행 로그가 README의 pass/hash와 불일치하는가 — **PASS, 경미한 주의** | pass01·pass02 각각 1/1 및 hash 일치. filtered 수 555/557은 서로 다른 보존 실행이다. raw log의 끝 빈 줄은 hash 보존 때문에 그대로 두어 `git diff --check` 경고가 난다. |
| 15 | C11 pass07의 일부 성공이 전체 matrix green으로 읽히는가 — **PASS** | 기록은 36/43, 실패 7개라고 명시하고 입력 hash mismatch 0과 matrix pass를 구분한다. |
| 16 | source hash closure를 PR head에서 다시 실행 가능한 검증이라고 과장하는가 — **PASS** | closure는 과거 HEAD `401911e…`과 기존 artifact를 고정하므로 역사적 검증으로만 사용한다. 재실행 가능한 일반 verifier라고 주장하지 않는다. |
| 17 | README.html이 Markdown의 오래된 내용이나 다른 결과를 보이는가 — **PASS** | 최신 `<pre>` 본문이 README.md와 일치하며 pass02 scope review와 입력 차이를 포함한다. |
| 18 | PR 본문 라벨이 저장소의 실제 라벨과 다른가 — **PASS** | `enhancement`, `area: layout`, `area: testing`이 실제로 적용됐다. |
| 19 | pass02 toolchain을 PR의 `mise.toml` 변경으로 오인하는가 — **PASS** | 본문은 pass02 Rust 1.100.0, 별도 toolchain 변경 제외, base `mise.toml` Rust 1.96.1을 명시한다. |
| 20 | 제품 화면 부재를 숨기거나 Chrome oracle을 앱 화면이라고 부르는가 — **PASS** | 본문은 순수 Rust test-only 변경이며 앱·simulator 화면이 없다고 밝힌다. |

## PR 격리·상태 관점 21–30

| # | 적대 관점과 판정 | 확인 근거 |
|---:|---|---|
| 21 | 현재 checkout의 C11.4 WIP가 C15 PR에 섞였는가 — **PASS** | GitHub가 반환한 변경 파일 16개는 C15 test/evidence뿐이며 C11.4 source는 없다. |
| 22 | Cargo.lock의 C11.4 `sha2` 의존성이나 mise bump가 따라왔는가 — **PASS** | 실제 PR diff에는 `Cargo.lock`과 `mise.toml`이 없다. |
| 23 | C15 test-only PR이 공식 상태를 조용히 바꾸는가 — **PASS** | `spec/STATUS.md`와 `spec/public/roadmap.html`은 PR 변경 파일이 아니며 본문은 `78/214`/C15.3 미완료를 유지한다. |
| 24 | 오래된 pass01 PR 초안이 최신 본문과 모순되는가 — **PASS** | `pr-body-draft-pass01.md`는 branch commit에서 제거돼 실제 PR diff에 없다. 로컬 원본은 보존했다. |
| 25 | PR head/base 및 mergeability가 불명확하거나 충돌 상태인가 — **PASS, 시점 한정** | GitHub는 base `5ab794c…`, head `4b1eb27…`, `mergeable=true`, `CLEAN`을 반환했다. merge 전 재확인한다. |
| 26 | 리뷰 전 PR을 ready로 바꾸거나 병합했다고 오해하는가 — **PASS** | 30관점 review 완료 뒤에만 PR #1390을 ready로 바꿨다. 최신 상태는 open/unmerged다. |
| 27 | 빈 status check 목록을 CI green으로 해석하는가 — **PASS** | `gh pr checks`는 no checks reported이며 main required status-check endpoint는 enabled가 아니다. 이는 green CI가 아니다. |
| 28 | 기록된 pass02를 이번 turn에서 다시 실행했다고 주장하는가 — **PASS** | 본문은 기록된 실행이라고 표현한다. 이번 PR review에서는 build/test를 실행하지 않았다. |
| 29 | 이 test-only 변경에 동작 화면을 억지로 붙여 오해를 만드는가 — **PASS** | 앱 UI가 없는 범위라 screenshot 미첨부 사유를 본문에 썼다. |
| 30 | C15.3 또는 C11.4를 완료 처리하는가 — **PASS** | PR은 체크리스트·official completion을 바꾸지 않는다. C11.4 matrix도 7개 실패가 남는다. |

## 결론

세 검토자의 30관점에서 이 제한 test-only diff의 blocker는 발견되지 않았다. 주요 **PARTIAL**은 CSS/HTML/runtime 동등성, child frame의 독립 Chrome oracle, exact intrinsic callback attribution, glyph advance 비교와 toolchain 차이다. PR 본문과 evidence는 이를 범위 밖 또는 기록 환경으로 표시한다. 검토 snapshot은 source/evidence patch head `4b1eb273c`다. 이후 `8a2abf1e0`에는 이 기록과 README 링크만 추가됐고 test/source/fixture는 변경되지 않았다. PR #1390은 30관점 검토 뒤 ready로 전환했으며 현재 checks는 보고되지 않았다. 30관점 검토만으로 CI, C15.3 완료 또는 공식 집계 변경을 주장하지 않는다.
