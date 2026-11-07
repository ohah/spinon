# PR #1245 변경분 독립 검토 30개

대상은 PR #1245의 Tailnet 제공본 검증 증거, PR #1243 병합 후 링크, 그리고 C11.4 상위 계획의 최신 상태·다음 작업 표기다. 각 항목은 다른 문서 불변 조건이나 실패 경계를 점검한다.

| # | 실패 관점 | 결과 |
|---:|---|---|
| 1 | PR #1243을 미병합 후보처럼 표현하는가 | merge 시각과 SHA `50d171d69b730af821076faf4ffcb02a4a5e44a8`를 기록한다. 통과 |
| 2 | 상태·로드맵 PR #1244의 merge SHA를 혼동하는가 | `4a2c7cf97b9f64bdddcda158648075629fa2a0cf`를 PR #1244와 연결한다. 통과 |
| 3 | bounded implementation을 C11.4 전체 완료로 승격하는가 | C11.4/C11 미완료와 `78/214` 유지 문구가 남는다. 통과 |
| 4 | 내부 계약 버전을 slice 증거만으로 올리는가 | README는 `0.1.0`을 유지한다고 명시한다. 통과 |
| 5 | Tailnet 서버를 별도 제공본이 아닌 repository source로 취급하는가 | README가 별도 제공 경로와 server PID를 식별한다. 통과 |
| 6 | sync 전에 기존 Tailnet bytes가 보존되지 않는가 | 기존 bytes/hash와 backup 경로를 기록한다. 통과 |
| 7 | 새 repository source와 복사본의 크기·SHA-256이 다른가 | 양쪽 613,422 bytes 및 동일 SHA-256을 기록한다. 통과 |
| 8 | 복사 도중 읽는 클라이언트가 부분 파일을 볼 수 있는가 | same-directory temp, fsync, atomic rename 순서를 기록한다. 통과 |
| 9 | 기존 preview server가 불필요하게 재시작되는가 | PID 28040을 유지하고 재시작하지 않았다고 기록한다. 통과 |
| 10 | local preview 응답 실패를 숨기는가 | HTTP 200, bytes, served hash가 evidence JSON에 기록된다. 통과 |
| 11 | Tailnet HTTPS 응답이 stale copy인데 성공으로 처리되는가 | HTTPS 200의 hash를 source와 대조하고 marker 목록도 검사한다. 통과 |
| 12 | PR #1243 merge SHA가 served marker에서 누락되는가 | required marker와 served-verification 결과에 포함한다. 통과 |
| 13 | 공개 완료 집계가 이전 수치로 잘못 기록되는가 | Tailnet served copy와 본문 모두 `78/214`를 확인한다. 통과 |
| 14 | C11 상태가 완료로 보이도록 잘못 표기되는가 | served marker는 C11.4/C11 미완료를 요구한다. 통과 |
| 15 | 계획의 #1234 상태가 이미 merged인데 후보로 남는가 | stale 후보 문단을 제거하고 실제 PR #1234 merge 기록으로 바꾼다. 통과 |
| 16 | #1234 구현 commit SHA나 merge 시간이 잘못 연결되는가 | PR 원격 metadata와 기록된 `0983a94...` 및 UTC 시각을 대조한다. 통과 |
| 17 | #1237이 #1234보다 앞선 작업처럼 나열되는가 | dependency 순서와 merge 시각 순서대로 기록한다. 통과 |
| 18 | #1237의 ten alignment 값이 fixed-gap slice와 혼합되는가 | computed alignment profile과 later gap profile을 별도 문단으로 둔다. 통과 |
| 19 | #1240 fixed px row-gap을 percentage slice로 오해하는가 | #1240은 px 범위, #1243은 percentage basis로 구분한다. 통과 |
| 20 | #1243이 indefinite percentage basis까지 지원한다고 확대 해석되는가 | definite 40px block-size basis로 scope를 제한한다. 통과 |
| 21 | Chrome 관측을 Spinon WPT 적합성으로 표현하는가 | Chrome observation과 독립 expected를 나란히 쓰고 WPT/일반 admission 주장은 없다. 통과 |
| 22 | Simulator screenshot을 surface readback 증거로 부르는가 | OS screenshot과 PixelCopy/Metal drawable readback을 구분한다. 통과 |
| 23 | Android/iOS 기기 종류·버전을 뒤바꾸는가 | API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator를 분리 표기한다. 통과 |
| 24 | percentage row-gap evidence가 PR implementation evidence와 분리되는가 | 별도 Tailnet README 경로로 두며 runtime report와 역할을 혼합하지 않는다. 통과 |
| 25 | desktop viewport 측정이 screenshot과 연결되는가 | 1440×1000 viewport, DOM bounds, screenshot hash와 파일을 연결한다. 통과 |
| 26 | mobile viewport에서 문서 가로 overflow가 숨겨지는가 | 390×844 viewport와 document/body width 및 card bounds를 기록한다. 통과 |
| 27 | 카드 순서가 최신 PR과 어긋나는가 | Agent Browser 기록에서 #1243 → #1240 → #1237 → #1220 순서를 확인한다. 통과 |
| 28 | browser automation이 열린 채 남는가 | lifecycle JSON은 close exit 0 및 잔여 browser process 없음으로 기록한다. 통과 |
| 29 | 금지된 Terminal Browser나 GitHub Pages 배포를 수행한 것으로 보이는가 | README는 Agent Browser 사용과 Pages 미배포를 분명히 한다. 통과 |
| 30 | parent plan이 완료 표시 또는 자동 successor를 약속하는가 | 계획은 C11.4/C11 미완료를 보존하고 다음 slice는 별도 범위 확정이 필요하다고 적는다. 통과 |

**판정:** 30/30 통과. PR #1245는 문서·실행 증거와 계획 큐 설명만 갱신하며 제품 코드나 공식 완료 체크를 변경하지 않는다.
