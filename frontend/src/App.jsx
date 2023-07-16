import { useState } from 'react';
import { gameSize } from './constants';
import { Board } from './components';
import { io } from 'socket.io-client';

export default async function Game() {
  const [squares, setSquares] = useState(Array(gameSize).fill(null).map(() => Array(gameSize).fill(null)));
  const [colour, setColour] = useState(null);

  const socketUrl = await 

  function handlePlay(nextSquares) {
    setCurrentMove(currentMove * -1); // Red is +1, Black is -1
    setSquares(nextSquares);
  }

  return (
    <div className="game">
      <div className="game-board">
        <Board redIsNext={redIsNext} squares={squares} onPlay={handlePlay} />
      </div>
    </div>
  );
}
