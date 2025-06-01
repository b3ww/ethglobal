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

// todo: call contract to know if wallet is already associated
export const RegisterAccount = () => {
  const { isConnected, connectWallet } = useWalletConnection();
  const [cardIndex, setCardIndex] = useState(0);

  const cardTitles = [
    'Connect your wallet',
    'Prove ownership',
    'Successfully associated',
  ];
  const cardDescriptions = [
    'Connect your wallet to associate it with your GitHub account.',
    'Open the vlayer extension to prove ownership of your GitHub account.',
    'Your wallet is now successfully associated with your GitHub account.',
  ];

  useEffect(() => {
    if (isConnected) {
      setCardIndex(1);
    } else {
      setCardIndex(0);
    }
  }, [isConnected]);

  function onSucceed() {
    setCardIndex((prevIndex) => Math.min(prevIndex + 1, cardTitles.length - 1));
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
          {isConnected ? (
            <ExtensionButton onSucceed={onSucceed} />
          ) : (
            <Button type="button" className="w-full" onClick={connectWallet}>
              Connect your wallet
            </Button>
          )}
        </CardFooter>
      </Card>
    </div>
  );
};
