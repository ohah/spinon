# C13.4 iOS Safari 관찰 PR 변경 실패 관점 검토 30개 · 2026-10-26

**대상 변경:** Android first-run blocker에 추가된 iOS Safari 26.2 geometry 관찰, fixture·before/result/after screenshot·hash 기록과 계획/계약/index의 상태 설명
**범위:** 문서와 관찰 산출물만 추가했다. Rust/runtime 구현, 공식 C13 상태 및 완료 수는 변경하지 않았다.

| # | PR 변경에서 확인한 실패 경로 | 검토 결과 |
| --- | --- | --- |
| 1 | 다른 iOS Simulator의 결과를 지정 기기 결과로 기록 | UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`를 캡처·실행 인자로 고정했다. |
| 2 | 물리 기기나 Android AVD를 함께 건드림 | iOS Safari 관찰에는 지정 Simulator만 사용했다. |
| 3 | Safari UA의 `iPhone OS 18_7`을 Simulator OS 버전으로 오해 | Simulator runtime iOS 26.2와 Safari UA를 별도 필드로 기록했다. |
| 4 | Safari build version을 모호하게 기재 | `Version/26.2 Mobile/15E148 Safari/604.1`과 전체 UA를 남겼다. |
| 5 | 약관·로그인 또는 첫 실행 확인을 동의 없이 통과 | URL을 Safari에 직접 열었고 동의·로그인 prompt가 없어 우회/승인이 없었다. |
| 6 | localhost fixture server가 페이지 파일을 제공하지 않음 | `GET /mobile-mode-probe.html` HTTP 200과 Safari 화면 렌더를 확인했다. |
| 7 | HTTP server에서 읽은 파일과 manifest 입력 해시가 다름 | fixture HTML SHA-256을 JSON에 기록해 다시 대조했다. |
| 8 | 첫 1초 blank 화면을 결과로 오해 | 초기 loading screenshot을 별도 파일로 두고 결과는 4초 캡처로 분리했다. |
| 9 | 결과 캡처 시간/표본이 불명확 | page content와 metrics가 보이는 capture를 result image로 지정했다. |
| 10 | before screenshot이 이번 실행 전 상태를 나타내지 않음 | 이번 preflight가 Home screen이었음을 기록하고, 이전 C11.4 image와 구분했다. |
| 11 | after screenshot이 앱 복귀를 증명하지 못함 | 마지막에 `dev.spinon.bootstrap`을 foreground로 열고 화면을 캡처했다. |
| 12 | fixture viewport/dimensions를 DPR 보정 없이 혼합 | 모든 CSS geometry 값을 CSS px로 JSON에 기록하고 screenshot dimensions를 그대로 따로 보존했다. |
| 13 | `CSS.supports`가 false인데 gutter 결과로 계산 | 지원 결과 `true`를 직접 기록했다. |
| 14 | `auto` test에 두 축 overflow가 없음 | fixture의 `overflow:scroll/scroll`, scroll `380×280`을 기록했다. |
| 15 | border-box와 client geometry를 혼동 | outer `offset=220×130`과 inner `client=216×126`을 따로 기록했다. |
| 16 | scroll extent를 client extent로 잘못 읽음 | scroll size `380×280`을 별도 항목으로 기록했다. |
| 17 | `stable`이 `auto`와 달라졌는데 무시 | 세 style별 client/offset/scroll 값을 JSON에 나란히 기록했다. |
| 18 | `stable both-edges`의 inline 위치 변화를 놓침 | 세 box의 x=91px, width=220px가 같아 시작 gutter 이동이 없음을 기록했다. |
| 19 | frame y 차이를 gutter 변화로 오인 | frame의 x/y를 각각 보존하고 box별 배치 y 차이를 style 차이와 구분했다. |
| 20 | `overflow:clip`을 scroll owner positive로 사용 | clip negative control의 overflow, client, scroll, frame을 별도 기록했다. |
| 21 | 화면에 scrollbar가 안 보여 CSS geometry도 미관찰로 처리 | computed style과 DOM geometry는 읽었으며, visual indicator는 별도 미관찰로 적었다. |
| 22 | 캡처에 indicator가 없는데 visible/overlay mode를 단정 | overlay/no-gutter와 일치한다고만 해석하고 active transient UI mode 증거는 아니라고 제한했다. |
| 23 | iOS Safari 결과를 Spinon runtime parity로 확대 | 관찰은 Safari browser 환경 한정이며 Spinon 구현 증거가 아니라고 명시했다. |
| 24 | desktop Chrome 156 값을 iOS width로 재사용 | Chrome 156 자료는 별도 desktop positive control이며 platform 폭 기준이 아니라고 적었다. |
| 25 | Android Chrome first-run blocker를 해결된 것으로 표시 | Chrome `FirstRunActivity` blocker는 그대로 unresolved다. |
| 26 | Safari를 종료해 다른 기존 Safari state를 잃음 | Safari는 preflight에 실행 중이지 않아 이번 실행의 process만 종료했다. |
| 27 | Safari process가 종료되지 않아 브라우저가 전면에 남음 | 종료 후 `launchctl`에서 Safari UI process가 없고 Spinon이 active임을 확인했다. |
| 28 | local HTTP server를 계속 남김 | Python server를 종료하고 interrupt exit를 확인했다. |
| 29 | iOS 관찰을 공식 C13 완료 또는 72/213 증가로 반영 | 공식 상태 파일을 수정하지 않았고 PR을 계획/근거 변경으로 한정했다. |
| 30 | iOS 산출물만 추가해도 Android mode와 구현 gate가 해소된 듯 표시 | Android 미확인, iOS transient UI mode 미확인으로 구현 gate가 여전히 닫혀 있다. |

**결론:** Safari에서 stable/both-edges zero-gutter geometry는 측정됐으나 UI mode를 직접 식별하지 못했다. 이 PR은 추가 관찰 기록이며 scrollbar 구현이나 완료 판정이 아니다.
