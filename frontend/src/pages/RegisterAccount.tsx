import { ExtensionButton } from '@/components/ExtensionButton';
import { Button } from '@/components/ui/button';
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { useWalletConnection } from '@/hooks/useWalletConnection';
import { useEffect, useState } from 'react';
import { useWriteContract } from 'wagmi';
import webProofVerifier from '../../../out/VgrantVerifier.sol/Vgrant.json';

export const RegisterAccount = () => {
  const { isConnected, connectWallet } = useWalletConnection();
  const [cardIndex, setCardIndex] = useState(0);

  const [providerResult, setProviderResult] = useState();
  const { writeContractAsync } = useWriteContract();

  const cardTitles = [
    'Connect your wallet',
    'Prove ownership',
    'Store association',
    'Successfully associated',
  ];
  const cardDescriptions = [
    'Connect your wallet to associate it with your GitHub account.',
    'Open the vlayer extension to prove ownership of your GitHub account.',
    'We will store the association between your wallet and GitHub account on the blockchain.',
    'Your wallet is now successfully associated with your GitHub account.',
  ];
  const cardActions = [
    <Button type="button" className="w-full" onClick={connectWallet}>
      Connect your wallet
    </Button>,
    <ExtensionButton onSucceed={onSucceed} />,
    <Button type="button" className="w-full" onClick={store}>
      Store association
    </Button>,
    <Button type="button" className="w-full" disabled={true}>
      Association stored successfully
    </Button>,
  ];

  useEffect(() => {
    if (isConnected) {
      setCardIndex(1);
    } else {
      setCardIndex(0);
    }
  }, [isConnected]);

  function onSucceed(providerResult: any) {
    setProviderResult(providerResult);
    setCardIndex((prevIndex) => Math.min(prevIndex + 1, cardTitles.length - 1));
  }

  async function store() {
    const hash = await writeContractAsync({
      address: import.meta.env.VITE_VERIFIER_ADDRESS,
      abi: webProofVerifier.abi,
      functionName: 'verifyAccount',
      args: providerResult,
    });

    if (hash) {
      setCardIndex(3);
    }
  }

  return (
    <div className="flex flex-col items-center space-y-12 pt-16">
      <h1 className="text-3xl font-bold">
        Associate your wallet to your GitHub
      </h1>

      <Card className="w-full max-w-md">
        <CardHeader>
          <CardTitle>{cardTitles[cardIndex]}</CardTitle>
        </CardHeader>

        <CardContent className="my-[-20px]">
          <p>{cardDescriptions[cardIndex]}</p>
        </CardContent>

        <CardFooter className="flex flex-col space-y-4">
          {cardActions[cardIndex]}
        </CardFooter>
      </Card>
    </div>
  );
};
