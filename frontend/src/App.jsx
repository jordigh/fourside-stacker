import { useState, useCallback } from 'react';

import Game from './components/game';
import Modal from './components/Modal';

function Login({ onClick }) {
  return (
    <form onSubmit={e => {
      e.preventDefault();
      onClick(e.target.username.value);
    }}>
      <div>What is your name?</div>
      <input type='text' name='username' placeholder='username' />
      <button>start game</button>
    </form>
  );
}

export function App() {
  const [username, setUsername] = useState();
  const [isModalOpen, setIsModalOpen ] = useState(false);
  const [showClose, setShowClose] = useState(false);

  function handleOpen(name) {
    setIsModalOpen(true);
    setShowClose(false);
    setUsername(name);
  }
  
  function handleClose() {
    setIsModalOpen(false);
  }

  const handleGameEnd = useCallback(() => {
    setShowClose(true);
  }, []);

  return (
    <div className='app'>
      <Login onClick={handleOpen}/>
      {isModalOpen && 
       <Modal onClose={handleClose} showClose={showClose}>
         <Game username={username} onGameEnd={handleGameEnd}/>
       </Modal>
      }
    </div>
  );
}

export default App;
