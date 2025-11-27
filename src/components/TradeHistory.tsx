import React from 'react';
import { TradeEvent } from '../hooks/useRecorder';

interface TradeHistoryProps {
  trades: TradeEvent[];
  onRefresh: () => Promise<void>;
}

export const TradeHistory: React.FC<TradeHistoryProps> = ({ trades, onRefresh }) => {
  const formatTime = (timestamp: string): string => {
    const date = new Date(timestamp);
    return date.toLocaleString('ko-KR', {
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    });
  };

  const getActionIcon = (action: string): string => {
    switch (action) {
      case 'buy':
        return '🟢';
      case 'sell':
        return '🔴';
      default:
        return '⚪';
    }
  };

  const getActionText = (action: string): string => {
    switch (action) {
      case 'buy':
        return '매수';
      case 'sell':
        return '매도';
      default:
        return '알 수 없음';
    }
  };

  return (
    <div className="trade-history">
      <div className="history-header">
        <h2>📋 거래 내역</h2>
        <button onClick={onRefresh} className="btn btn-sm">
          🔄 새로고침
        </button>
      </div>

      {trades.length === 0 ? (
        <div className="empty-state">
          <p>아직 기록된 거래가 없습니다.</p>
          <p className="hint">HTS에서 매수/매도 시 자동으로 기록됩니다.</p>
        </div>
      ) : (
        <div className="trade-list">
          {trades.map((trade, index) => (
            <div key={index} className="trade-item">
              <div className="trade-icon">{getActionIcon(trade.action)}</div>
              <div className="trade-details">
                <div className="trade-action">
                  <strong>{getActionText(trade.action)}</strong>
                </div>
                <div className="trade-time">{formatTime(trade.timestamp)}</div>
                <div className="trade-window">{trade.window_title}</div>
              </div>
              <div className="trade-screenshot">
                <span className="screenshot-path" title={trade.screenshot_path}>
                  📁 {trade.screenshot_path.split('/').pop()}
                </span>
              </div>
            </div>
          ))}
        </div>
      )}

      {trades.length > 0 && (
        <div className="history-footer">
          총 {trades.length}개의 거래 기록
        </div>
      )}
    </div>
  );
};
