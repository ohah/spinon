# C11.3.3 leading negative named-line pairs on both axes · 사전 비교

2026-11-01에 Agent Browser `0.31.1`의 HeadlessChrome `156.0.0.0`에서 CSS Grid Level 1의 2025 CRD §§7.2, 8.1.3, 8.3.1을 기준으로 비교했다. Browser viewport는 `1280×633` CSS px, DPR `1`이다. 고정 입력과 독립 계산은 [input](./input.json) 및 [expected values](./expected-values.json), 실제 `getComputedStyle()`과 `getBoundingClientRect()` 결과는 [Chrome oracle](./chrome-oracle-pass03.json)에 보존했다.

이 비교는 PR #967의 한 축 제한 profile에서 제외했던 양 축 leading implicit sizing을 분리한다. 각 scene은 `grid-template-areas` 없는 `120×100px` Grid, explicit `20px` column과 `15px` row, 하나의 fixed-size `18×12px` direct item을 사용한다. 대상 축은 모두 같은 `ghost` ident의 negative endpoint를 갖고, 양 축 대상 장면에서는 Columns와 Rows가 동시에 leading implicit tracks를 만든다.

핵심 `-4 ghost / -1 ghost` 결과는 Columns `[6,6,6,0,20]px`, Rows `[4,4,4,0,15]px`, item frame `(0,0,18,12)`이다. 18px·12px의 fixed item contribution은 각각 세 spanned auto tracks에 균등하게 분배되고, span 밖 leading track은 0px다. Fixed pattern control은 Columns `[5,7,5,7,20]px`, Rows `[3,4,3,4,15]px`를 반환했다. 이때 세 spanned fixed tracks는 각각 17px·10px이므로 fixed item은 track area를 1px·2px 넘지만 track size는 유지된다. Equal endpoint는 두 축 모두 기본 1-track span으로 계산되고, reversed endpoint는 순서를 정규화해 일반 pair와 같은 tracks를 반환한다.

규범 기반 expected values는 Chrome 수치를 그대로 복사하지 않고 각 축의 line resolution, span, auto-track contribution 및 leading track pattern을 따로 계산해 고정했다. Chrome 값은 별도 oracle 파일로 보존해 두 기준을 구분한다. [Agent Browser screenshot](./chrome-agent-browser-pass03.png)은 여덟 장면의 track CSSOM 값과 item frame을 보여 주며 앱 runtime 화면 증거는 아니다.

확인 후 Agent Browser session을 닫고, `agent-browser`/HeadlessChrome 프로세스가 남지 않았으며 임시 HTTP server도 종료했음을 확인했다.
