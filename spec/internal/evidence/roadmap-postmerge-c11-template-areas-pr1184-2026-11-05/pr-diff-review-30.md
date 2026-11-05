# PR #1184 병합 후 C11 문서·Tailnet 변경 적대 검토 30개

**검토 범위:** `spec/STATUS.md`, 내부 계약 0059, C11.1 계획, 공개 roadmap 카드, Tailnet 동기화 스크립트 및 브라우저 근거. 구현 PR의 코드 diff 검토와 별개다.

| # | 별도 실패 관점 | 실제 변경 대조 결과 |
|---:|---|---|
| 1 | 제한 slice가 C11 상위 항목을 완료로 표시함 | STATUS와 공개 roadmap은 C11 미완료를 유지한다. 통과. |
| 2 | C11.3.3 또는 C11.4가 이번 merge로 닫힘 | 양 항목과 C11 전체를 미승격으로 명시한다. 통과. |
| 3 | 공식 완료 수에 제한 후보 수를 더함 | STATUS, roadmap 및 실행 evidence가 `78/214`로 일치한다. 통과. |
| 4 | PR 번호·merge 시간·SHA가 GitHub와 다름 | #1184, `2026-11-05T14:41:41Z`, `5803547796d7039f591ef979175a36617ce86636`을 merge 결과와 대조했다. 통과. |
| 5 | Rebase and merge 결과 SHA를 PR head SHA와 혼동함 | 문서는 merge SHA를 기록하고 PR head `b94b34f2a394830eba44b1724705a1677a5782ca`는 별도다. 통과. |
| 6 | 빈 check rollup을 CI 통과로 표시함 | check rollup 비어 있음과 main required status checks 부재를 기록하고 CI green을 주장하지 않는다. 통과. |
| 7 | 내부 계약 버전을 구현 범위마다 올림 | 내부 계약 0059 숫자 버전 `0.1.0`을 유지한다. 통과. |
| 8 | named-area 조합을 새 일반 profile로 기술함 | 기존 기본 `RuntimeRootHtmlTextPaintV1`의 `RootHtmlIntrinsicTrackSizingV1` 제한 경로라고 고정한다. 통과. |
| 9 | template-area와 intrinsic track 수가 다르게 허용됨 | 계약에서 area 행·열 수가 explicit track 목록과 같아야 한다. 통과. |
| 10 | non-positive 또는 indefinite axis가 가능하다고 읽힘 | positive finite CSS px definite width/height 제한이 유지된다. 통과. |
| 11 | named map의 비직사각형·겹침·중복 이름이 유효해짐 | unique non-empty rectangular map과 bounded metadata 조건을 문서화했다. 통과. |
| 12 | unknown area가 named-line fallback으로 성공함 | unresolved/cascade fallback placement와 잘못된 area 선언을 fail-closed한다고 적었다. 통과. |
| 13 | area item owner가 임의 descendant로 확대됨 | visible Grid의 직접 HTML `div` 한 개만 연결한다고 적었다. 통과. |
| 14 | nested element·hidden item·anonymous text가 지원됨 | item 수와 text-only child 제한 및 거부 동작을 명시한다. 통과. |
| 15 | named area의 spanning 기여가 사라짐 | span을 intrinsic sizing 단계까지 보존하고 Chrome 사전 비교의 spanning scene을 연결한다. 통과. |
| 16 | `minmax(auto, px|%)`가 새 경로에서 허용된 것처럼 표기됨 | 실행 evidence와 계약에서 지원하지 않는 조합으로 남긴다. 통과. |
| 17 | auto-repeat·implicit track·named line·dense/column flow 지원을 과장함 | 이 조합을 제한 범위에서 제외하고 대장과 계약 표현을 맞췄다. 통과. |
| 18 | public CSS Grid/WPT 적합성을 선언함 | bounded Root HTML slice라고 표시하고 WPT/general CSS Grid는 미완료다. 통과. |
| 19 | focused test count가 원본 로그와 다름 | 5/5 projection, 15/15 runtime, 8/8 intrinsic layout, 14/14 area layout 로그와 일치한다. 통과. |
| 20 | broad suite 실패를 숨기거나 green으로 요약함 | style-to-layout 131/140 및 runtime 87/95를 실패와 함께 유지한다. 통과. |
| 21 | fixture boundary·Clippy·FFI 결과가 누락되거나 확대 해석됨 | scoped Clippy, rustfmt, 기본 FFI check, fixture boundary 33/33만 기록한다. 통과. |
| 22 | Android AVD를 실기기 또는 hardware GPU로 표현함 | API 37 ARM64 AVD screenshot은 visual smoke이며 PixelCopy가 아니다. 통과. |
| 23 | iOS simulator screenshot을 compositor 이후 readback으로 표현함 | iPhone 17 Pro/iOS 26.2 Simulator OS screenshot이며 Metal drawable readback이 아님을 유지한다. 통과. |
| 24 | Chrome 관측값을 독립 area 기대값으로 오인함 | Chrome 156 four-scene 관측과 area/span independent invariants를 구분한다. 통과. |
| 25 | status/contract/plan의 상대 evidence 링크 경로가 잘못됨 | 변경 후 각 relative link의 대상 폴더와 Git 경로를 대조했다. 통과. |
| 26 | 공개 카드에 PR은 있으나 merge SHA 또는 완료 수가 없음 | 카드에 PR #1184, merge 시간/SHA, 공식 `78/214`와 미완료 경계를 적었다. 통과. |
| 27 | 공개 카드의 GitHub links가 계획·근거 외 다른 파일을 가리킴 | 링크 대상이 PR branch에 포함된 plan, contract, precomparison, runtime 및 follow-up evidence다. 통과. |
| 28 | Tailnet server가 내려갔거나 예상 외 process가 port 8768을 소유함 | Tailscale `Running`, PID 28040의 `tailnet-preview-server.mjs`, `127.0.0.1:8768` listener를 확인했다. 통과. |
| 29 | 이전 roadmap 손실·열린 파일 덮어쓰기·부분 파일 공개·불필요 삭제 | 열린 handle은 없고 이전 파일을 검증된 backup에 보존한 뒤 temp+atomic rename으로 한 route만 바꿨다. 통과. |
| 30 | local/remote 응답 또는 desktop/mobile 화면이 갱신본과 다름 | source/local/HTTPS 모두 HTTP 200·576785 bytes·동일 SHA다. Agent Browser desktop 1440px/card 928px, mobile 390px/card 362px·14px margins에서 PR/SHA/count가 보이고 가로 넘침이 없다. Browser 프로세스 종료도 확인했다. 통과. |

**결과:** 이 문서·preview 경계 검토에서 수정 결함은 남지 않았다. Tailnet preview는 갱신했지만 GitHub Pages는 배포하지 않았다.
