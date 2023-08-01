import { gameSize } from '../constants';

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

function Slot({ yourTurn, direction, onSlotClick }) {
  const className = yourTurn ? "slot yourTurn" :  "slot";
  return (
    <div className={className} onClick={onSlotClick}>
      {direction}
    </div>
  );
}

export function InfoBar({ message, colour }) {
  return (
    <div className="status">
      <span>
        { colour ? <Disc colour={colour}/> : '' } {message} 
      </span>
    </div>
  );
}

export function Board({ yourTurn, squares, onSlotClick}) {
  function handleClick(rowNum, fallingDirection) {
    if (!yourTurn) {
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
      onSlotClick(rowNum, fallingDirection);
    }
  }

  return (
    <div className="board">
    {
      [...Array(gameSize).keys()].map((row) => {
        return (
          <div key={row} className="board-row">
            <Slot yourTurn={yourTurn} direction="🡆" onSlotClick={() => handleClick(row, "left")}/>
            {
              [...Array(gameSize).keys()].map((col) => {
                return <Square
                         key={col}
                         value={squares[row][col]?.value}
                         fallingDirection={squares[row][col]?.direction}
                       />;
              })
            }
            <Slot yourTurn={yourTurn} direction="🡄"  onSlotClick={() => handleClick(row, "right")}/>
          </div>
        );
      })
    }
    </div>
  );
}
