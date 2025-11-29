# 📹 HTS Trading Recorder

HTS(Home Trading System) 프로그램을 자동으로 감지하고 거래 활동을 녹화하는 데스크톱 애플리케이션입니다.

## ✨ 주요 기능

-   🎯 **자동 HTS 감지**: 키움, 이베스트 등 주요 HTS 프로그램 자동 감지 (구현)
-   🔴 **자동 녹화**: HTS 실행 시 자동으로 화면 녹화 시작
-   📸 **스크린샷 캡처**: 매수/매도 시점 자동 플래그
-   💾 **거래 내역 저장**: 모든 거래 활동 자동 기록
-   🌙 **백그라운드 실행**: 시스템 트레이에서 백그라운드 동작
-   ⚡ **경량화**: Tauri 기반으로 메모리 사용량 최소화

## 🛠️ 기술 스택

### Backend (Rust)

-   **Tauri**: 데스크톱 앱 프레임워크
-   **tokio**: 비동기 런타임
-   **sysinfo**: 프로세스 모니터링
-   **xcap**: 윈도우 캡처
-   **scap**: 화면 녹화

### Frontend (TypeScript)

-   **React**: UI 프레임워크
-   **Vite**: 빌드 도구
-   **TypeScript**: 타입 안정성

## 📦 설치 및 실행

### 사전 요구사항

```bash
# Rust 설치
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Node.js 설치 (Homebrew)
brew install node

# Tauri CLI 설치
cargo install tauri-cli
```

### 프로젝트 실행

```bash
# 의존성 설치
npm install

# 개발 모드 실행
npm run tauri dev

# 프로덕션 빌드
npm run tauri build
```

## 📂 프로젝트 구조

```
test-recorder/
├── src/                        # React 프론트엔드
│   ├── App.tsx                # 메인 앱
│   ├── App.css                # 스타일
│   ├── components/            # React 컴포넌트
│   │   ├── StatusPanel.tsx
│   │   ├── RecordingButton.tsx
│   │   └── TradeHistory.tsx
│   └── hooks/
│       └── useRecorder.ts     # 녹화 로직 Hook
│
├── src-tauri/                 # Rust 백엔드
│   ├── src/
│   │   ├── main.rs           # 진입점
│   │   ├── models.rs         # 데이터 모델
│   │   ├── recorder.rs       # 화면 녹화
│   │   ├── process_monitor.rs # 프로세스 모니터링
│   │   └── window_capture.rs # 윈도우 캡처
│   ├── Cargo.toml            # Rust 의존성
│   └── tauri.conf.json       # Tauri 설정
│
└── package.json              # NPM 설정
```

## 🎮 사용 방법

1. **앱 실행**: 프로그램을 실행하면 시스템 트레이에 아이콘이 나타남
2. **자동 감지**: HTS 프로그램(키움, 이베스트 등)을 실행하면 자동으로 녹화 시작
3. **수동 제어**: UI에서 "모니터링 시작/중지" 버튼으로 수동 제어 가능
4. **스크린샷**: "스크린샷" 버튼으로 현재 화면 수동 캡처
5. **거래 내역**: 하단에서 기록된 거래 내역 확인

## ⚙️ 설정

`src-tauri/src/models.rs`에서 감지할 HTS 프로그램 설정:

```rust
pub fn default() -> Self {
    Self {
        process_names: vec![
            "kiwoom.exe".to_string(),     // 키움증권
            "eFriend.exe".to_string(),    // 이베스트투자증권
            "Ctrade.exe".to_string(),     // 미래에셋증권
            "KOAStudio.exe".to_string(),  // 키움 OpenAPI
        ],
        // ...
    }
}
```

## 🚧 향후 개발 계획

-   [ ] 멀티 모니터 지원

## 📝 라이선스

MIT License

## 👨‍💻 개발자

ps - Backend Developer

---

**주의사항**: 이 프로그램은 개인 거래 기록용이며, 증권사의 정책을 준수해야 합니다.
