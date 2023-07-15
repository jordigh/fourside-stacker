import { useState } from 'react';

const gameSize = 7;
const disc = "◉";

function Square({ value, fallingDirection }) {
  return (
    <div className="square">
      {value ? <Disc colour={value} fallingDirection={fallingDirection}/> : null}
    </div>
  );
}

function Disc({ colour, fallingDirection }) {
  const className = colour + (fallingDirection ? ` falling-${fallingDirection}` : "");
  return (
    <span className={className}>{disc}</span>
  );
}

function Slot({ direction, onSlotClick }) {
  return (
    <div className="slot" onClick={onSlotClick}>
      {direction}
    </div>
  );
}

function Board({ redIsNext, squares, onPlay }) {
  const nextPiece = redIsNext ? 'red' : 'black';

  function handleClick(rowNum, fallingDirection) {
    if (calculateWinner(squares)) {
      return;
    }
    const row = squares[rowNum];
    let col;
    if (fallingDirection === 'right') {
      col = row.findIndex(element => element === null);
    }
    else {
      // findLastIndex is a recent addition, not all browsers support
      // it, have to roll our own instead.
      const reverseRow = row.slice().reverse();
      col = row.length  - reverseRow.findIndex(element => element === null) - 1;
    }
    
    if(col > -1 && col < row.length) {
      row[col] = nextPiece;
    }
    
    onPlay(squares);
  }

  const winner = calculateWinner(squares);
  let status;
  if (winner) {
    status = <span>{'Winner: '}<Disc colour={winner}/></span>;
  } else {
    status = <span>{'Next player: '}<Disc colour={nextPiece}/></span>;
  }

  return (
    <>
    <div className="status">{status}</div>
    <div className="board">
    {
      [...Array(gameSize).keys()].map((row) => {
        return (
          <div key={row} className="board-row">
            <Slot direction="🡆" onSlotClick={() => handleClick(row, "right")}/>
            {
              [...Array(gameSize).keys()].map((col) => {
                return <Square key={col} value={squares[row][col]} />;
              })
            }
            <Slot direction="🡄"  onSlotClick={() => handleClick(row, "left")}/>
          </div>
        );
      })
    }
    </div>
    </>
  );
}

export default function Game() {
  const [history, setHistory] = useState([Array(gameSize).fill(null).map(() => Array(gameSize).fill(null))]);
  const [currentMove, setCurrentMove] = useState(0);
  const redIsNext = currentMove % 2 === 0;
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
        <Board redIsNext={redIsNext} squares={currentSquares} onPlay={handlePlay} />
      </div>
      <div className="game-info">
        <ol>{moves}</ol>
      </div>
    </div>
  );
}

function calculateWinner(squares) {
  return null;
}
