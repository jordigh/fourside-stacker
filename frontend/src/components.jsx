import { gameSize } from './constants';

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

export function Board({ redIsNext, squares, onPlay }) {
  const nextPiece = redIsNext ? 'red' : 'black';

  function handleClick(rowNum, fallingDirection) {
    if (calculateWinner(squares)) {
      return;
    }
    const row = squares[rowNum];
    let colNum;
    if (fallingDirection === 'right') {
      colNum = row.findIndex(element => element === null);
    }
    else {
      // findLastIndex is a recent addition, not all browsers support
      // it, have to roll our own instead.
      const reverseRow = row.slice().reverse();
      colNum = row.length  - reverseRow.findIndex(element => element === null) - 1;
    }

    if(colNum > -1 && colNum < row.length) {
      row[colNum] = {
        value: nextPiece,
        direction: fallingDirection
      };
      onPlay(squares, rowNum, colNum);
    }
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
                return <Square
                         key={col}
                         value={squares[row][col]?.value}
                         fallingDirection={squares[row][col]?.direction}
                       />;
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

function calculateWinner(squares) {
  return null;
}
