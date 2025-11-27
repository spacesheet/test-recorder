import React from 'react';
import { RecordingStatus } from '../hooks/useRecorder';

interface StatusPanelProps {
  status: RecordingStatus;
}

export const StatusPanel: React.FC<StatusPanelProps> = ({ status }) => {
  const formatDuration = (seconds: number | null): string => {
    if (!seconds) return '00:00:00';
    
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;
    
    return `${hours.toString().padStart(2, '0')}:${minutes
      .toString()
      .padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  };

  return (
    <div className="status-panel">
      <h2>📊 녹화 상태</h2>
      
      <div className="status-grid">
        <div className="status-item">
          <span className="label">녹화 상태:</span>
          <span className={`value ${status.is_recording ? 'recording' : ''}`}>
            {status.is_recording ? '🔴 녹화 중' : '⚪ 대기 중'}
          </span>
        </div>

        <div className="status-item">
          <span className="label">HTS 감지:</span>
          <span className={`value ${status.hts_detected ? 'detected' : ''}`}>
            {status.hts_detected ? '✅ 감지됨' : '❌ 미감지'}
          </span>
        </div>

        {status.hts_name && (
          <div className="status-item">
            <span className="label">HTS 이름:</span>
            <span className="value">{status.hts_name}</span>
          </div>
        )}

        {status.is_recording && status.recording_duration !== null && (
          <div className="status-item">
            <span className="label">녹화 시간:</span>
            <span className="value recording-time">
              {formatDuration(status.recording_duration)}
            </span>
          </div>
        )}
      </div>

      <div className="status-indicator">
        <div className={`indicator-dot ${status.is_recording ? 'active' : ''}`} />
        <span>
          {status.is_recording
            ? '시스템이 HTS를 모니터링하고 있습니다.'
            : 'HTS를 실행하면 자동으로 녹화가 시작됩니다.'}
        </span>
      </div>
    </div>
  );
};
