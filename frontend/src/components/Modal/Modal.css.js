import styled from "styled-components";

const Modal = styled.div`
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0;
  transform: scale(0);
  transition: opacity linear 0.15s;
  z-index: 2000;
  margin: 20px;
  &.fade-in {
    opacity: 1;
    /* transition: opacity linear 0.15s; */
    transform: scale(1);
    transition: all 0.3s;
  }
  &.fade-out {
    opacity: 0;
    transition: opacity linear 0.15s;
    transform: scale(0.7);
    opacity: 0;
    transition: all 0.3s;
  }
  .background {
    background: rgba(0, 0, 0, 0.5);
    position: fixed;
    z-index: 1040;
    display: block;
    top: 0;
    left: 0;
    bottom: 0;
    right: 0;
    outline: 0;
  }
  .box-dialog {
    z-index: 1050;
    background-color: #fefefe;
    box-shadow: 0 3px 9px rgba(0, 0, 0, 0.5);
    width: 350px;
    .box-content {
      padding: 24px;
      width: 100%;
    }
    .box-header {
      height: 48px;
      padding: 8px 24px;
      display: flex;
      justify-content: space-between;
      align-items: center;
      border-bottom: 1px solid #c7c7c7;
      .box-title {
        font-size: 24px;
        font-weight: 400;
        margin: 0 0 0 0;
      }
      .x-close {
        font-size: 35px;
        line-height: 35px;
        font-weight: 400;
        text-shadow: none;
        color: black;
        cursor: pointer;
        &:hover {
          opacity: 0.5;
        }
      }
    }
    .box-body {
      font-size: 14px;
      padding: 0px;
      width: auto;
      height: auto;
    }
    .box-footer {
      height: 48px;
      padding: 0px 24px;
      display: flex;
      align-items: center;
      justify-content: flex-end;
      border-top: 1px solid #c7c7c7;
    }
  }
`;

export default Modal;
