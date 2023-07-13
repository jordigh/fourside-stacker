import { useState } from 'react';

const gameSize = 7;
const disc = "◉";

function Square({ value }) {
  let colour = value === "R" ? "red" : "black";
  return (
    <div className="square">
      {value ? <Disc colour={colour}/> : null}
    </div>
  );
}

function Disc({ colour }) {
  return (
    <span className={colour}>{disc}</span>
  );
}

function Slot({ direction, onSlotClick }) {
  return (
    <div className="slot" onClick={onSlotClick}>
      {direction}
    </div>
  );
}

function Board({ xIsNext, squares, onPlay }) {
  function handleClick(i, direction) {
    if (calculateWinner(squares) || squares[i]) {
      return;
    }
    const nextSquares = squares.slice();
    if (xIsNext) {
      nextSquares[i] = 'R';
    } else {
      nextSquares[i] = 'B';
    }
    onPlay(nextSquares);
  }

  const winner = calculateWinner(squares);
  let status;
  if (winner) {
    let winnerColour = winner === 'R' ? 'red' : 'black';
    status = <span>{'Winner: '}<Disc colour={winnerColour}/></span>;
  } else {
    let nextColour = xIsNext ? 'red' : 'black';
    status = <span>{'Next player: '}<Disc colour={nextColour}/></span>;
  }

  return (
    <>
    <div className="status">{status}</div>
    <div className="board">
    {
      [...Array(gameSize).keys()].map((row) => {
        return (
          <div key={row} className="board-row">
            <Slot direction="🡆" onSlotClick={() => handleClick(row, "left")}/>
            {
              [...Array(gameSize).keys()].map((col) => {
                let idx = gameSize*row + col;
                return <Square key={col} value={squares[idx]} />;
              })
            }
            <Slot direction="🡄"  onSlotClick={() => handleClick(row, "right")}/>
          </div>
        );
      })
    }
    </div>
    </>
  );
}

export default function Game() {
  const [history, setHistory] = useState([Array(gameSize).fill(Array(gameSize).fill(null))]);
  const [currentMove, setCurrentMove] = useState(0);
  const xIsNext = currentMove % 2 === 0;
  const currentSquares = history[currentMove];

  function handlePlay(nextSquares) {
    const nextHistory = [...history.slice(0, currentMove + 1), nextSquares];
    setHistory(nextHistory);
    setCurrentMove(nextHistory.length - 1);
  }

  function jumpTo(nextMove) {
    setCurrentMove(nextMove);
  }

  const moves = history.map((squares, move) => {
    let description;
    if (move > 0) {
      description = 'Go to move #' + move;
    } else {
      description = 'Go to game start';
    }
    return (
      <li key={move}>
        <button onClick={() => jumpTo(move)}>{description}</button>
      </li>
    );
  });

  return (
    <div className="game">
      <div className="game-board">
        <Board xIsNext={xIsNext} squares={currentSquares} onPlay={handlePlay} />
      </div>
      <div className="game-info">
        <ol>{moves}</ol>
      </div>
    </div>
  );
}

function calculateWinner(squares) {
  const lines = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
  ];
  for (let i = 0; i < lines.length; i++) {
    const [a, b, c] = lines[i];
    if (squares[a] && squares[a] === squares[b] && squares[a] === squares[c]) {
      return squares[a];
    }
  }
  return null;
}
