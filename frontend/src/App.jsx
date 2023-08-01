import { useState } from 'react';

import Game from './components/game';

function Login({ onClick }) {
  return (
    <form onSubmit={e => {
      e.preventDefault();
      onClick(e.target.username.value);
    }}>
      <input type='text' name='username' placeholder='username' />
      <button>login</button>
    </form>
  );
}

export function App() {
  const [username, setUsername] = useState();

  function handleClick(name) {
    console.log('setting username');
    setUsername(name);
  }

  return (
    <div className='app'>
      <Login onClick={handleClick}/>
      <Game username={username}/>
    </div>
  );
}

export default App;
