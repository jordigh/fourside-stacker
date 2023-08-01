import { useState } from 'react';

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

  function handleOpen(name) {
    setIsModalOpen(true);
    setUsername(name);
  }
  
  function handleClose() {
    setIsModalOpen(false);
  }

  return (
    <div className='app'>
      <Login onClick={handleOpen}/>
      {isModalOpen && 
       <Modal onClose={handleClose}>
         <Game username={username}/>
       </Modal>
      }
    </div>
  );
}

export default App;
