import { useState, useEffect } from 'react';
import { gameSize } from './constants';
import { Board, InfoBar } from './components';
import { io } from 'socket.io-client';

export default function Game() {
  const [squares, setSquares] = useState(Array(gameSize).fill(null).map(() => Array(gameSize).fill(null)));
  const [colour, setColour] = useState(null);
  const [yourTurn, setYourTurn] = useState(false);
  const [message, setMessage] = useState('Please wait...');
  const [socket, setSocket] = useState();

  useEffect(() => {
    async function setupSocket() {
      const response = await fetch(
        'http://localhost:8000/register', {
          method: 'POST',
          body: JSON.stringify({
            username: 'jordi'
          }),
          headers: {
            "Content-Type": "application/json",
          },
          mode: 'cors',
        });
      const json = await response.json();
      setSocket(io(json.url));
    }
    setupSocket();
  }, []);

  socket.on('colour', colour => setColour(colour));
  socket.on('squares', squares => setSquares(squares));
  socket.on('turn', player => {
    if (player === colour) {
      setYourTurn(true);
      setMessage('It is your turn. What is your move?');
    } else {
      setYourTurn(false);
      setMessage(`Waiting for {player}'s move...`);
    }
  });

  socket.on('winner', player => {
    setYourTurn(false);
    setMessage(player === colour ? 'You win!' : 'You lose...');
  });

  function slotClick(rowNum, colNum) {
    socket.emit('click', rowNum, colNum);
  }

  return (
    <div className="game">
      <div className="game-board">
        <InfoBar message={message} colour={colour}/>
        <Board yourTurn={yourTurn} colour={colour} squares={squares} onSlotClick={slotClick} />
      </div>
    </div>
  );
}
