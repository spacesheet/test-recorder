import React, { useEffect } from 'react';
import { useRecorder } from './hooks/useRecorder';
import { StatusPanel } from './components/StatusPanel';
import { RecordingButton } from './components/RecordingButton';
import { TradeHistory } from './components/TradeHistory';
import './App.css';

function App() {
  const {
    status,
    tradeHistory,
    error,
    startMonitoring,
    stopMonitoring,
    captureScreenshot,
    fetchTradeHistory,
  } = useRecorder();

  useEffect(() => {
    // 초기 거래 내역 로드
    fetchTradeHistory();
  }, [fetchTradeHistory]);

  return (
    <div className="app">
      <header className="app-header">
        <h1>📹 HTS 트레이딩 레코더</h1>
        <p className="subtitle">주식 거래를 자동으로 기록하고 분석하세요</p>
      </header>

      <main className="app-main">
        {error && (
          <div className="error-banner">
            ⚠️ {error}
          </div>
        )}

        <section className="section">
          <StatusPanel status={status} />
        </section>

        <section className="section">
          <RecordingButton
            isRecording={status.is_recording}
            onStart={startMonitoring}
            onStop={stopMonitoring}
            onCapture={captureScreenshot}
          />
        </section>

        <section className="section">
          <TradeHistory
            trades={tradeHistory}
            onRefresh={fetchTradeHistory}
          />
        </section>
      </main>

      <footer className="app-footer">
        <p>💡 Tip: HTS 프로그램을 실행하면 자동으로 녹화가 시작됩니다.</p>
        <p className="version">v0.1.0 | Made with Tauri + React</p>
      </footer>
    </div>
  );
}

export default App;
