# C11.4 근거 링크 수정 Tailnet pass35

PR #416 병합 뒤 공개 로드맵·근거 링크와 2026-10-21 09:24 KST Android 실기기 재실행 결과를 Tailnet 미리보기에 반영했다. preview가 제공하는 파일은 저장소 작업 트리가 아니라 `/Users/yoonhb/Library/Application Support/SpinonPreview`에서 제공된다.

## 산출물과 출처

- 원본 `origin/main`: `4eaab3fa1af53b6511ded4bdb121e733af25121f`, tree `5115c93a6d63610b6696c552df4d7538d8c0de82`.
- pass34 selected root를 APFS clone으로 pass35에 복제하고 SHA-256으로 전체 대조했다. pass34 root는 보존했다.
- RSPress `2.0.22`로 2,445개 파일, 총 104,500,510 bytes를 빌드했다. 산출물 tree SHA-256: `f2ffc27eead28f841f8b10808c3962587d49e694dd710c4da9f4dd305a25174b`.
- roadmap HTML, evidence README route, Android/iOS 이미지, 결과 JSON 및 30개 검토 문서의 local target을 확인했다.

## Preview 동기화

- Tailnet preview server PID `1191`, `127.0.0.1:8768` listener를 유지했다.
- 시작 inventory: 30,463 files, 2,065 directories, 1,399,069,301 logical bytes, symbolic link 0.
- 2,445 selected files: 136 create, 125 replace, 2,184 same, delete 0.
- 교체 전 파일 125개, 13,507,290 bytes를 고유 pass35 backup root에 저장하고 SHA-256을 검증했다. 각 교체 직전에 열린 파일 handle을 다시 검사했다.
- local preview 대상 2,445/2,445 hash 일치, 백업 125/125 hash 일치.
- Tailnet HTTP 응답 2,445/2,445 `200`, source와 응답 SHA-256 2,445/2,445 일치, 실패 0.

## Agent Browser 확인

- 공개 로드맵의 C11.4 항목에 09:24 Android 재실행 결과·화면 링크가 표시됐다.
- evidence 문서에서 같은 근거 폴더의 자산 URL 21개를 확인했다. 잘못된 `/spinon/spec/internal/...png` 링크는 없었다.
- Android 실기기 PNG는 1080×2340, iOS Simulator PNG는 1206×2622이며 각각 HTTP 200으로 열렸다. Android 결과 JSON은 HTTP 200으로 열리고 scene `spanning-auto-min-flex-tracks`를 담고 있다.
- [로드맵 화면](tailnet-roadmap-note-agent-browser-pass35.png) · [근거 페이지 화면](tailnet-evidence-agent-browser-pass35.png).

## 범위

이 갱신은 링크·근거 전달만 바꿨다. C11.4 공식 체크와 완료 집계는 미완료다. 캡처는 box geometry이며 글리프 rasterization, 화면상 200px text overflow pixels, Android PixelCopy, iOS compositor 이후 pixel, WPT 전체 적합성을 입증하지 않는다.
