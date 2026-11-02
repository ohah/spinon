# C11.1 PR #1015 병합 후 상태·로드맵 변경 실패 관점 검토 30개

**검토 범위:** `spec/STATUS.md`, 내부 계약 0059, C11 부모/집중 계획, `spec/public/roadmap.html`, Agent Browser local-source observation. 이 검토는 구현/runtime 검토를 대체하지 않는다.

| # | 실패 관점 | 판정 및 근거 |
|---:|---|---|
| 1 | PR 상태가 미병합으로 남거나 날짜가 다를 수 있음 | **PASS** — GitHub PR #1015 상태는 merged, `mergedAt=2026-11-02T00:39:29Z`다. |
| 2 | 문서에 잘못된 merge SHA가 들어갈 수 있음 | **PASS** — 문서는 `015ff0e9ac959a269b60728a1ac3a6ab962791ea`를 사용하며 `origin/main` HEAD와 같다. |
| 3 | 빈 check rollup이 CI 성공으로 표현될 수 있음 | **PASS** — 상태·계약·계획은 GitHub check run이 없었고 CI 통과로 주장하지 않는다고 기록한다. |
| 4 | C11 checkbox가 제한 slice 하나 때문에 닫힐 수 있음 | **PASS** — `spec/STATUS.md`의 C11 checkbox는 unchecked로 유지한다. |
| 5 | Root HTML slice가 C11.1 전체 완료로 오해될 수 있음 | **PASS** — status와 계획은 C11.1 전체 미완료를 명시하며 별도 C11.1 checkbox가 있다고 주장하지 않는다. |
| 6 | 공식 완료 집계가 업데이트 없이 임의 증가할 수 있음 | **PASS** — 변경 전후 official `72/213`을 유지한다. |
| 7 | 제한된 profile이 일반 CSS support로 표현될 수 있음 | **PASS** — `RuntimeRootHtmlTextPaintV1` 경계와 일반 CSS admission 미완료를 명시한다. |
| 8 | supported track 형식이 지나치게 넓게 적힐 수 있음 | **PASS** — direct finite non-negative px/fr explicit tracks만 설명한다. |
| 9 | `minmax()`와 fraction 조합이 허용된다고 오해할 수 있음 | **PASS** — `minmax()` + `fr`를 미지원으로 남긴다. |
| 10 | percentage 또는 intrinsic flex sizing을 지원한다고 오해할 수 있음 | **PASS** — percentage, auto/intrinsic 및 indefinite flex sizing을 남은 범위로 표시한다. |
| 11 | min/max-size와 aspect-ratio guards가 생략될 수 있음 | **PASS** — focused plan, internal contract와 public card가 제한을 가리킨다. |
| 12 | named-area + fr가 fixed named-area 예외를 우회할 수 있음 | **PASS** — named area와 `fr` 결합은 거부한다고 기록한다. |
| 13 | Chrome 156의 큰-factor 결과를 표준 기대값으로 혼동할 수 있음 | **PASS** — divergence는 internal plan/evidence로 분리되고 public roadmap은 규범 확장으로 주장하지 않는다. |
| 14 | runtime marker screenshot이 CSS geometry 증거로 과장될 수 있음 | **PASS** — screenshot을 visual smoke로 분리하고 수치 geometry는 Rust matrix에 귀속한다. |
| 15 | Android simulator가 실기기/hardware GPU 근거로 보일 수 있음 | **PASS** — API 37 ARM64 AVD 범위를 명시하며 hardware renderer 주장이나 PixelCopy는 하지 않는다. |
| 16 | iOS simulator 캡처가 compositor pixel equality로 오해될 수 있음 | **PASS** — iOS 26.2 Simulator draw/present만 쓰고 Metal drawable/compositor readback을 주장하지 않는다. |
| 17 | Root HTML regression count가 현재 run과 맞지 않을 수 있음 | **PASS** — evidence log는 `10 passed`를 기록하고 status/plan은 10/10을 쓴다. |
| 18 | Grid projection count가 변경 전 범위와 섞일 수 있음 | **PASS** — focused projection log의 `47 passed`와 docs의 47/47이 같다. |
| 19 | Taffy regression count가 부분 filter와 섞일 수 있음 | **PASS** — vendored library 전체 `176 passed` log와 docs의 176/176이 일치한다. |
| 20 | implementation PR diff review 결과를 상태 문서에서 누락할 수 있음 | **PASS** — status와 evidence README가 20-angle runtime 및 30-angle PR review 파일을 연결한다. |
| 21 | 집중 계획이 계속 미병합 후보로 표시될 수 있음 | **PASS** — PR #1015 병합 결과와 구현 evidence를 새 결과 절에 기록한다. |
| 22 | internal contract와 public roadmap의 지원 범위가 서로 다를 수 있음 | **PASS** — 둘 다 existing parsed Root HTML bounded profile, direct px/fr 및 남은 C11 범위를 구분한다. |
| 23 | roadmap progress card가 merge SHA 대신 branch tip을 표시할 수 있음 | **PASS** — card는 PR merge SHA를 포함한다. |
| 24 | public card에서 C11 완료 여부를 혼동할 수 있음 | **PASS** — card는 C11/C11.1 미완료와 `72/213`을 바로 명시한다. |
| 25 | 화면·근거 링크가 임의 저장 경로를 가리킬 수 있음 | **PASS** — public roadmap 링크는 `/spinon/` 아래의 plan, evidence, status route를 사용한다. |
| 26 | 새 local-source screenshot이 Tailnet 게시 완료를 암시할 수 있음 | **PASS** — README는 source capture와 Tailnet serving proof를 분리한다. |
| 27 | Agent Browser 결과와 실제 브라우저 화면이 다를 수 있음 | **PASS** — observation은 PR #1015 progress paragraph text와 1440px viewport scroll width를 기록하고 screenshot을 보존한다. |
| 28 | desktop viewport에서 horizontal overflow를 숨길 수 있음 | **PASS** — local-source observation의 `scrollWidth`는 viewport와 같은 1440px이다. Mobile Tailnet layout은 게시 후 별도 확인 대상으로 둔다. |
| 29 | 사용자에게 원래 요청한 Tailnet URL이 갱신되었다고 조기 보고할 수 있음 | **PASS** — 이 PR 문서는 Tailnet sync 전 상태를 밝히고 후속 HTTP/hash 검증을 분리한다. |
| 30 | 문서 sync가 사용자 데이터나 미리보기 기존 route를 삭제할 수 있음 | **PASS** — 문서 변경은 repo route만 추가하고 Tailnet sync는 별도 preflight/backup/no-delete 절차 이후 진행한다. |

Local source screenshot은 이 문서 PR의 시각 확인용이다. 실제 Tailnet serving 검증, 모바일 viewport 확인과 서버 process/hash 확인은 후속 sync evidence에서만 판정한다.
