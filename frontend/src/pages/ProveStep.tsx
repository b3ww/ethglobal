import { useEffect, useRef, useState } from 'react';
import { useNavigate } from 'react-router';
import { useAccount } from 'wagmi';
import { useGithubAccountProof } from '../hooks/useGithubAccountProof';

export const ProveStep = () => {
  const navigate = useNavigate();
  const { address } = useAccount();
  const [disabled, setDisabled] = useState(false);
  const modalRef = useRef<HTMLDialogElement>(null);

  const {
    requestWebProof,
    webProof,
    callProver,
    isPending,
    isCallProverIdle,
    result,
    error,
  } = useGithubAccountProof();

  useEffect(() => {
    if (webProof && isCallProverIdle) {
      void callProver([webProof, address]);
    }
  }, [webProof, address, callProver, isCallProverIdle]);

  useEffect(() => {
    if (result) {
      void navigate('/mint');
    }
  }, [result, navigate]);

  useEffect(() => {
    modalRef.current?.showModal();
  }, []);

  useEffect(() => {
    if (error) {
      throw error;
    }
  }, [error]);

  return (
    <div className="mt-7 flex justify-center flex-col items-center">
      <button
        disabled={disabled}
        id="nextButton"
        onClick={() => {
          requestWebProof();
          setDisabled(true);
        }}
      >
        {isPending ? 'Proving in progress...' : 'Open Extension'}
      </button>
    </div>
  );
};
