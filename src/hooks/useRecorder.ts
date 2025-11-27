import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { listen, UnlistenFn } from '@tauri-apps/api/event';

export interface RecordingStatus {
  is_recording: boolean;
  hts_detected: boolean;
  hts_name: string | null;
  recording_duration: number | null;
}

export interface TradeEvent {
  action: 'buy' | 'sell' | 'unknown';
  timestamp: string;
  screenshot_path: string;
  window_title: string;
}

export function useRecorder() {
  const [status, setStatus] = useState<RecordingStatus>({
    is_recording: false,
    hts_detected: false,
    hts_name: null,
    recording_duration: null,
  });
  
  const [tradeHistory, setTradeHistory] = useState<TradeEvent[]>([]);
  const [error, setError] = useState<string | null>(null);

  // 상태 업데이트
  const updateStatus = useCallback(async () => {
    try {
      const newStatus = await invoke<RecordingStatus>('get_recording_status');
      setStatus(newStatus);
      setError(null);
    } catch (err) {
      setError(String(err));
    }
  }, []);

  // 이벤트 리스너 설정
  useEffect(() => {
    let unlistenHTS: UnlistenFn;
    let unlistenRecordingStarted: UnlistenFn;
    let unlistenRecordingStopped: UnlistenFn;
    let unlistenDuration: UnlistenFn;

    const setupListeners = async () => {
      // HTS 감지 이벤트
      unlistenHTS = await listen('hts-detected', (event) => {
        const detected = event.payload as boolean;
        setStatus(prev => ({ ...prev, hts_detected: detected }));
      });

      // 녹화 시작 이벤트
      unlistenRecordingStarted = await listen('recording-started', () => {
        setStatus(prev => ({ ...prev, is_recording: true }));
      });

      // 녹화 중지 이벤트
      unlistenRecordingStopped = await listen('recording-stopped', () => {
        setStatus(prev => ({ 
          ...prev, 
          is_recording: false,
          recording_duration: null 
        }));
      });

      // 녹화 시간 업데이트
      unlistenDuration = await listen('recording-duration', (event) => {
        const duration = event.payload as number | null;
        setStatus(prev => ({ ...prev, recording_duration: duration }));
      });
    };

    setupListeners();

    // 주기적 상태 업데이트
    const interval = setInterval(updateStatus, 2000);

    return () => {
      clearInterval(interval);
      unlistenHTS?.();
      unlistenRecordingStarted?.();
      unlistenRecordingStopped?.();
      unlistenDuration?.();
    };
  }, [updateStatus]);

  // 모니터링 시작
  const startMonitoring = useCallback(async () => {
    try {
      const result = await invoke<string>('start_monitoring');
      console.log(result);
      await updateStatus();
      setError(null);
    } catch (err) {
      setError(String(err));
      throw err;
    }
  }, [updateStatus]);

  // 모니터링 중지
  const stopMonitoring = useCallback(async () => {
    try {
      const result = await invoke<string>('stop_monitoring');
      console.log(result);
      await updateStatus();
      setError(null);
    } catch (err) {
      setError(String(err));
      throw err;
    }
  }, [updateStatus]);

  // 스크린샷 캡처
  const captureScreenshot = useCallback(async () => {
    try {
      const path = await invoke<string>('capture_screenshot');
      setError(null);
      return path;
    } catch (err) {
      setError(String(err));
      throw err;
    }
  }, []);

  // 거래 내역 가져오기
  const fetchTradeHistory = useCallback(async () => {
    try {
      const history = await invoke<TradeEvent[]>('get_trade_history');
      setTradeHistory(history);
      setError(null);
    } catch (err) {
      setError(String(err));
    }
  }, []);

  // 윈도우 리스트 가져오기
  const listWindows = useCallback(async () => {
    try {
      const windows = await invoke<string[]>('list_windows');
      return windows;
    } catch (err) {
      setError(String(err));
      throw err;
    }
  }, []);

  return {
    status,
    tradeHistory,
    error,
    startMonitoring,
    stopMonitoring,
    captureScreenshot,
    fetchTradeHistory,
    listWindows,
  };
}
