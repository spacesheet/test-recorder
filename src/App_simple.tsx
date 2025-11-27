import { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import './App.css';

function App() {
  const [name, setName] = useState('');
  const [greetMsg, setGreetMsg] = useState('');

  async function greet() {
    setGreetMsg(await invoke('greet', { name }));
  }

  return (
    <div className="app">
      <header className="app-header">
        <h1>🎉 Tauri 테스트</h1>
        <p>창이 보이면 성공입니다!</p>
      </header>

      <main className="app-main">
        <div className="test-box">
          <h2>인사하기</h2>
          <input
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
            placeholder="이름을 입력하세요"
          />
          <button onClick={greet}>인사하기</button>
          {greetMsg && <p className="result">{greetMsg}</p>}
        </div>
      </main>
    </div>
  );
}

export default App;
