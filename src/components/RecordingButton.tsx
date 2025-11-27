import React, { useState } from 'react';

interface RecordingButtonProps {
  isRecording: boolean;
  onStart: () => Promise<void>;
  onStop: () => Promise<void>;
  onCapture: () => Promise<string>;
}

export const RecordingButton: React.FC<RecordingButtonProps> = ({
  isRecording,
  onStart,
  onStop,
  onCapture,
}) => {
  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState<string>('');

  const handleStart = async () => {
    setLoading(true);
    setMessage('');
    try {
      await onStart();
      setMessage('✅ 모니터링이 시작되었습니다.');
    } catch (error) {
      setMessage(`❌ 오류: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  const handleStop = async () => {
    setLoading(true);
    setMessage('');
    try {
      await onStop();
      setMessage('✅ 모니터링이 중지되었습니다.');
    } catch (error) {
      setMessage(`❌ 오류: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  const handleCapture = async () => {
    setLoading(true);
    setMessage('');
    try {
      const path = await onCapture();
      setMessage(`📸 스크린샷 저장됨: ${path}`);
    } catch (error) {
      setMessage(`❌ 오류: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="recording-controls">
      <div className="button-group">
        <button
          onClick={handleStart}
          disabled={isRecording || loading}
          className="btn btn-primary"
        >
          {loading ? '⏳ 처리 중...' : '▶️ 모니터링 시작'}
        </button>

        <button
          onClick={handleStop}
          disabled={!isRecording || loading}
          className="btn btn-danger"
        >
          {loading ? '⏳ 처리 중...' : '⏹️ 모니터링 중지'}
        </button>

        <button
          onClick={handleCapture}
          disabled={loading}
          className="btn btn-secondary"
        >
          {loading ? '⏳ 처리 중...' : '📸 스크린샷'}
        </button>
      </div>

      {message && (
        <div className={`message ${message.includes('❌') ? 'error' : 'success'}`}>
          {message}
        </div>
      )}
    </div>
  );
};
