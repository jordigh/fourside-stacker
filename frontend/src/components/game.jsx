import { useState, useEffect, useRef } from 'react';

import { gameSize } from '../constants';
import { Board, InfoBar } from './board';

function GameState({ username, onGameEnd }) {
  const [squares, setSquares] = useState(Array(gameSize).fill(null).map(() => Array(gameSize).fill(null)));
  const [colour, setColour] = useState(null);
  const [yourTurn, setYourTurn] = useState(false);
  const [message, setMessage] = useState('Please wait...');
  const socketRef = useRef(null);

  useEffect(() => {
    async function setupSocket() {
      // We need a username to set up the socket
      if (!username) {
        return;
      }

      const response = await fetch(
        'http://localhost:8000/register',
        {
          method: 'POST',
          body: JSON.stringify({
            username
          }),
          headers: {
            "Content-Type": "application/json",
          },
          mode: 'cors',
        }
      );
      const json = await response.json();
      socketRef.current = new WebSocket(json.url);

      socketRef.current.onopen = () => {
        // Send empty play to init the board state
        const msg = {
          play: null
        };
        socketRef.current.send(JSON.stringify(msg));
      };

      socketRef.current.onmessage = ({data}) => {
        const game = JSON.parse(data);

        const yourTurn = game.current_player === game.your_colour;

        let message;
        if (game.current_player) {
          message = yourTurn ? 'It is your turn. What is your move?' : `Waiting for ${game.current_player}'s turn`;
        } else {
          onGameEnd();
          if (game.winner) {
            message = game.winner === game.your_colour ? 'You win!' : 'You lose...';
          } 
          else {
            message = 'Tie game!';
          }
        }

        setSquares(game.squares);
        setColour(game.current_player);
        setYourTurn(yourTurn);
        setMessage(message);
      };
    }
    setupSocket();
    return () => {
      socketRef.current?.close();
    };
  }, [username, onGameEnd]);

  function handleSlotClick(rowNum, direction) {
    const msg = {
      request: 'play',
      play: [rowNum, direction],
    };
    socketRef.current.send(JSON.stringify(msg));
  }


  return (
    <div className="game">
      <div className="game-board">
        <InfoBar message={message} colour={colour}/>
        <Board
          yourTurn={yourTurn}
          colour={colour}
          squares={squares}
          onSlotClick={handleSlotClick}
        />
      </div>
    </div>
  );
}

export default function Game({username, onGameEnd}) {
  return (
    <GameState username={username} key={username} onGameEnd={onGameEnd}/>
  );
}
