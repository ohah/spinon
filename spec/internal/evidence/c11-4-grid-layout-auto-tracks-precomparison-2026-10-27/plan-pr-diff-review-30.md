# 계획 PR 변경 적대 검토 30개

검토 범위는 `plan/c11-4-grid-layout-auto-tracks-wpt.md`, `plan/c11-4-grid-layout-auto-tracks-plan-review-20.md`, `spec/STATUS.md`의 계획-only 기록, 사전 비교 README/JSON/screenshots와 `tests/fixtures/css/c11/grid-layout-auto-tracks-c114-precomparison.html`이다. 이 검토는 구현/runtime 적대 검토를 대신하지 않는다.

| # | 실패 관점 | 확인 결과 | 판정 |
|---:|---|---|---|
| 1 | WPT revision 착오 | 계획·expected JSON이 동일한 `c999c58338ee1d223df5ad62e48be1202ced0d37`을 가리킨다 | PASS |
| 2 | source SHA 불일치 | 저장소의 고정 inventory와 browser로 읽은 upstream source SHA가 일치한다 | PASS |
| 3 | reference 경로 오류 | `../reference/grid-layout-auto-tracks-ref.html`이 계획의 pinned WPT tree 내 경로와 일치한다 | PASS |
| 4 | reference hash 누락/오류 | expected JSON과 README에 reference hash가 기록돼 있다 | PASS |
| 5 | reftest를 testharness로 오기한다 | README/계획 모두 화면 비교 관찰로 한정한다 | PASS |
| 6 | Chrome source 화면만 보고 reference 통과를 주장한다 | source/reference 각각 캡처하고 byte hash 일치를 보고한다 | PASS |
| 7 | Chrome viewport·DPR 유실 | browser와 JSON에 viewport 및 DPR이 기록돼 있다 | PASS |
| 8 | default font line-height를 고정 oracle로 승격한다 | 19px 관찰값과 Ahem 10px normative 입력을 분리했다 | PASS |
| 9 | Ahem font 경로 오타 | fixture 경로에서 `tests/fixtures/resources/c15/Ahem.ttf`를 실제 요청한다 | PASS |
| 10 | font loading 경합으로 잘못 측정한다 | font status와 `document.fonts.check()` 결과가 true다 | PASS |
| 11 | Ahem fixture hash drift | normalized fixture SHA가 expected JSON에 고정됐다 | PASS |
| 12 | source markup에서 item 하나 누락 | 세 item과 B child가 upstream 구조를 보존한다 | PASS |
| 13 | explicit 100px column을 auto로 오인 | fixture 및 expected 값이 첫 column을 100px로 분리한다 | PASS |
| 14 | implicit column을 생략 | B의 column 2 placement와 50px used track을 함께 고정한다 | PASS |
| 15 | B child width contribution 손실 | child width 50px 및 auto column 결과 50px을 기록한다 | PASS |
| 16 | row 1 경쟁 contribution을 합산 | 계획은 row 1의 track contribution을 max 10px로 계산한다 | PASS |
| 17 | implicit row 2를 explicit template에 잘못 포함 | 계획은 C가 만드는 positive implicit row를 별도 기술한다 | PASS |
| 18 | root auto height를 definite height로 가정 | root height auto 및 content-sized 20px 결과를 명시한다 | PASS |
| 19 | grid width와 expanded columns 합계를 혼동 | root 100px와 tracks 150px의 범위를 따로 고정한다 | PASS |
| 20 | item의 B overflow를 root 내부 표본으로만 검사 | root 밖 content 좌표 두 점을 고정했다 | PASS |
| 21 | item 끝 경계 off-by-one | x=149 포함과 x=150 외부 대조를 모두 기록한다 | PASS |
| 22 | frame만 검사하고 sizing stage를 빠뜨린다 | layout trace에서 각 축 tracks와 contributions를 보도록 계획한다 | PASS |
| 23 | Ahem adaptation을 upstream CSS와 동일하다고 과장 | normalization 목적과 차이를 명시한다 | PASS |
| 24 | generic CSS admission이 우연히 열린다 | 정확한 root/style/topology allowlist 및 pre-layout 거부를 요구한다 | PASS |
| 25 | 미등록 scene이 성공 fallback을 얻는다 | 고정 ID 외 route는 계산 전 거부한다 | PASS |
| 26 | HostDocument/style revision이 섞인다 | runtime matcher의 exact revision 확인을 요구한다 | PASS |
| 27 | fixture payload가 기본 제품 archive에 노출된다 | 기본 negative와 opt-in positive control을 분리한다 | PASS |
| 28 | Android AVD를 실기기 GPU 증거로 부른다 | Android API 37 ARM64 AVD 한계 표기를 요구한다 | PASS |
| 29 | iOS drawable을 compositor 이후 결과로 부른다 | compositor 제출 전 표본으로 제한해 보고한다 | PASS |
| 30 | 부분 fixture 결과로 C11.4 완료 수를 올린다 | 계획과 status 기록이 checkbox 및 `72/213`을 유지한다 | PASS |

## 판정

계획 PR 변경 검토 PASS 30/30, PARTIAL 0, FAIL 0. 계획 및 사전 비교 변경만 검토했으며 제품 구현 검증은 남아 있다.
