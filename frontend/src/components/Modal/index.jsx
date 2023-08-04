import { useState, useEffect } from 'react';
import StyledModal from "./Modal.css";

function Modal ({ children, onClose, showClose }) {
  const [fadeType, setFadeType] = useState(null);

  function transitionEnd(e) {
    if (e.propertyName !== 'opacity' || fadeType === 'in')
      return;
    if (fadeType === 'out') {
      onClose();
    }
  }

  function handleClick(e) {
    e.preventDefault();
    setFadeType('out');
  }

  useEffect(() => {
    setFadeType('in');
  }, []);

  return (
    <StyledModal
      id='modal'
      className={`wrapper size-md fade-${fadeType}`}
      role='dialog'
      onTransitionEnd={transitionEnd}
    >
      <div className='box-dialog'>
        <div className='box-header'>
          <h4 className='box-title'>
            Stacked Fourside
          </h4>
        </div>
        <div className='box-content'>{children}</div>
        <div className='box-footer'>
          { showClose &&
           <button onClick={handleClick} className='close'>
             Close
           </button>
          }
        </div>
      </div>
      <div className='background'/>
    </StyledModal>
  );
}

export default Modal;
