import { Button } from '@/components/ui/button';
import {
  Card,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { useWalletConnection } from '@/hooks/useWalletConnection';

export const TemplatePage = () => {
  const { isConnected, connectWallet } = useWalletConnection();

  return (
    <div className="flex flex-col items-center space-y-12 pt-16">
      <h1 className="text-3xl font-bold">
        Associate your wallet to your GitHub
      </h1>

      <Card className="w-full max-w-md">
        {/* <form onSubmit={handlePlaceGrant}> */}
        <CardHeader>
          <CardTitle>Détails de la Récompense</CardTitle>
          <CardDescription>
            Créez une récompense pour une issue GitHub avec le contrat Vgrant
          </CardDescription>
        </CardHeader>

        {/* <CardContent className="space-y-4"> */}
        {/* <div className="space-y-2">
            <label
              htmlFor="contractAddress"
              className="text-sm font-medium flex items-center gap-2"
            >
              Adresse du contrat
              <span className="px-2 py-1 text-xs bg-yellow-100 text-yellow-800 border border-yellow-300 rounded-md font-mono">
                DEBUG
              </span>
            </label>
            <Input
              id="contractAddress"
              placeholder="0x..."
              value={contractAddress}
              onChange={(e) => setContractAddress(e.target.value)}
              required
              className="border-2 border-dashed border-yellow-400 bg-yellow-50"
            />
          </div> */}
        {/* </CardContent> */}

        <CardFooter className="flex flex-col space-y-4">
          {isConnected ? (
            <Button
              type="submit"
              className="w-full"
              // disabled={isPending || isConfirming}
            >
              Good
            </Button>
          ) : (
            <Button
              type="button"
              className="w-full"
              onClick={connectWallet}
              // disabled={isPending || isConfirming}
            >
              Connect your wallet
            </Button>
          )}
        </CardFooter>
        {/* </form> */}
      </Card>
    </div>
  );
};
