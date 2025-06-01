import { Button } from '@/components/ui/button';
import { useEffect, useState } from 'react';
import { useAccount } from 'wagmi';
import { useGithubAccountProof } from '../hooks/useGithubAccountProof';

export const ExtensionButton = ({ onSucceed }: { onSucceed: () => void }) => {
  const { address } = useAccount();
  const [buttonText, setButtonText] = useState('Open Extension');

  const {
    requestWebProof,
    webProof,
    callProver,
    // isPending,
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
      setButtonText('Proving completed');
      onSucceed();
    }
  }, [result]);

  useEffect(() => {
    if (error) {
      throw error;
    }
  }, [error]);

  return (
    <Button
      type="button"
      className="w-full"
      disabled={buttonText !== 'Open Extension'}
      onClick={() => {
        requestWebProof();
        setButtonText('Proving in progress...');
      }}
    >
      {buttonText}
    </Button>
  );
};
