# PR #1360 병합 기록

- PR: [#1360 — C11.4 parsed Root HTML repeated-text row-span five-fit `min-height:auto`](https://github.com/ohah/spinon/pull/1360)
- 병합 방식: Rebase and merge
- 병합 시각: `2026-11-11T16:13:47Z`
- 병합 전 PR base: `1f63c3148e851f6d00d9163a0fcb9b22486ec79e`
- PR 최종 head: `71d2226ddbf8cfd5b5a43fe53880eee8b9b9f24e`
- 병합 후 `main` SHA: `6b3475a0ee48741b7f2aaa30e8ee62b19618ef8d`
- 병합 전 실제 PR diff와 로컬 변경 diff는 byte-for-byte 일치했다. GitHub에서 다시 읽은 PR은 `closed`/`merged`이며 merge SHA가 위 값과 일치한다.
- GitHub Actions workflow run과 commit status check는 각각 0건이다. CI green으로 세지 않는다.
- PR labels: `enhancement`, `area:layout`, `area:testing`.
- PR 본문의 Android/iOS light/dark 이미지 네 장은 Agent Browser에서 모두 로드됐다. Android 캡처는 API 37 ARM64 AVD, iOS 캡처는 iPhone 17 Pro/iOS 26.2 Simulator다.

## 병합된 범위

정확히 여섯 implicit row member와 다섯 typed cap `[0px, 10px, 20%, 30px, 40px]`, singleton `min-content`·`max-content`·`auto` tail 중 하나를 가진 repeated-text row-span profile만 추가했다. Chrome 157 RuntimeSession matrix는 최종 소스에서 4/4 chunk, 34,560/34,560 scene을 통과했고 candidate payload-floor violation은 0건이었다. 자세한 성공·거부·회귀 관찰은 같은 폴더의 [실행 근거](README.md)에 있다.

Simulator/AVD 화면은 기본 Root HTML route의 light/dark 앱 시작 smoke이며 five-fit row-span fixture를 포함하지 않는다. PixelCopy·Metal drawable readback, compositor 이후 pixel parity, 실기기, hardware GPU와 WPT는 검증하지 않았다. Runtime `--all-targets` Clippy는 기존 `too_many_arguments` 2건과 `needless_borrow` 3건으로 실패했으며, 변경 crate Clippy와 fixture-enabled runtime `--lib` Clippy는 통과했다.

PR 병합은 C11.4/C11의 완료를 뜻하지 않는다. 공식 `78/214`와 C11.4/C11 미완료 상태 및 internal numeric contract `0.1.0`을 유지한다.

## 작업 리소스 정리

iOS Simulator와 Android AVD는 종료했고, `S03` 이름의 iOS Simulator profile은 기기 목록에 없었다. ADB 연결 기기도 없으며 PR 이미지 확인에 사용한 Agent Browser 세션은 종료했다. 비활성 Android emulator crash-report temp 디렉터리 한 개만 제거했다. 공유 simulator/AVD profile, 앱·프로젝트 데이터, 다른 프로세스는 지우거나 종료하지 않았다.
