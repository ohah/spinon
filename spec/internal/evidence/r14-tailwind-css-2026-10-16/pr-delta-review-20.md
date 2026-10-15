# PR #208 변경분 적대 검토 · 2026-10-16

계획 검토와 구현·실행 검토에서 쓰지 않은 관점으로 PR의 코드, 공식 상태, PR 페이지와 문서 경계를 다시 살폈다.

| # | 실패 관점 | 확인한 근거 | 결과 |
|---:|---|---|---|
| 1 | R14 완료 표기가 Tailwind 제품 지원으로 확대 해석 | `spec/STATUS.md`가 조사 완료 및 미지원 범위를 같은 문장에 둠 | 통과 |
| 2 | API가 없는 실험을 공개 API 또는 Android/iOS 런타임 완료로 표시 | API 해당 없음, 제품 pipeline·simulator 미검증을 명시 | 통과 |
| 3 | 계획 링크가 PR에서 404 | Agent Browser에서 d72606f7 permalink의 계획 페이지를 열어 제목과 본문 확인 | 통과 |
| 4 | generated CSS permalink가 없는 파일을 가리킴 | Agent Browser에서 d72606f7 CSS 페이지를 열고 `@layer theme` 출력 확인 | 통과 |
| 5 | converter JSON 링크가 잘못되거나 비어 있음 | Agent Browser에서 d72606f7 JSON 페이지를 열고 `tailwind-flex-positive` 확인 | 통과 |
| 6 | Rspress가 상대 screenshot 경로를 resolve하지 못함 | 최종 docs build 로그에서 R14 경로 오류·module error 없음 | 통과 |
| 7 | fixture가 제품 CSS 경로에 붙어 기본 앱에 포함 | 별도 `spikes/tailwind-generated-css` package; 앱/runtime 코드는 수정하지 않음 | 통과 |
| 8 | RSPress·기존 lockfile 의존성을 바꿔 unrelated toolchain drift | 변경 파일에 Tailwind spike 전용 `bun.lock`만 추가 | 통과 |
| 9 | Tailwind core와 CLI의 버전 불일치 | package와 frozen lock 모두 4.3.3, 실행 banner도 같은 버전 | 통과 |
| 10 | fixture 외 README·script class가 generated utility에 섞임 | `source(none)`·단일 `@source`; analyzer가 utility selector 10개 전체를 exact 비교 | 통과 |
| 11 | theme var가 선언됐지만 utility가 다른 값을 사용 | theme token과 `.bg-brand-*` var 참조를 analyzer와 raw CSS에서 대조 | 통과 |
| 12 | cascade layer 순서 또는 block을 누락 | raw CSS와 JSON에 statements·theme/base/utilities/properties block 기록 | 통과 |
| 13 | Preflight global reset을 UA stylesheet 일부로 취급 | README가 author base layer와 내장 UA origin을 분리; 6개 reset sentinel 존재 | 통과 |
| 14 | generated CSS에서 일부 rule만 조용히 drop한 뒤 전체 통과 | 전체 CSS converter는 첫 `layer-statement`에서 실패 | 통과 |
| 15 | 거부된 변환 뒤 partial JSON을 정상 산출물로 남김 | analyzer가 negative control마다 nonzero와 output 부재를 요구 | 통과 |
| 16 | converter가 모든 입력을 거부해도 positive control 없이 성공 주장 | Tailwind의 `.flex`에서 정확히 한 `display:flex` style을 확인 | 통과 |
| 17 | CSS variables/calc/media/selector boundary를 누락 | spacing·color `unparsed`, hover selector/media, responsive media, global selector를 따로 거부 | 통과 |
| 18 | generated output이 달라졌는데 README metrics가 예전 hash | analyzer의 pinned SHA-256 guard와 evidence CSS SHA-256 일치 | 통과 |
| 19 | PR 본문 이미지 URL이 404이거나 화면 경계 설명이 빠짐 | Agent Browser에서 두 이미지 complete/visible, natural size 1680×1200·390×844; 본문에 Chromium-only 경계 표시 | 통과 |
| 20 | 전체 docs build 실패를 새 R14 링크 탓으로 숨김 | 최종 실행 exit 1, repository 기존 dead-link 보고 432건, R14 경로 보고 0건 | 통과 |

검토 결과: 변경 범위에서 별도 수정이 필요한 결함을 찾지 못했다. docs build와 Android/iOS 제품 runtime은 완료로 주장하지 않는다.
