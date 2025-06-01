import { Button } from '@/components/ui/button';
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { useWalletConnection } from '@/hooks/useWalletConnection';
import * as React from 'react';
import { useEffect, useState } from 'react';
import { toast } from 'sonner';
import { useWaitForTransactionReceipt, useWriteContract } from 'wagmi';

const vgrantABI = [
  {
    name: 'approveBounty',
    type: 'function',
    stateMutability: 'nonpayable',
    inputs: [{ name: '_issue', type: 'string' }],
    outputs: [],
  },
  {
    name: 'getBounty',
    type: 'function',
    stateMutability: 'view',
    inputs: [{ name: '_issue', type: 'string' }],
    outputs: [
      {
        type: 'tuple',
        components: [
          { name: 'author', type: 'address' },
          { name: 'repo', type: 'string' },
          { name: 'issueId', type: 'uint256' },
          { name: 'bounty', type: 'uint256' },
          { name: 'deadline', type: 'uint256' },
          { name: 'approved', type: 'bool' },
          { name: 'claimed', type: 'bool' },
        ],
      },
    ],
  },
];

export const ValidateGrantPage = () => {
  const [issueUrl, setIssueUrl] = useState('');
  // todo: replace with real contract address
  const [contractAddress, setContractAddress] = useState(
    '0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512',
  );
  const [txHash, setTxHash] = useState<string | null>(null);

  const { isConnected, connectWallet } = useWalletConnection();

  const { writeContractAsync, isPending, isError, error } = useWriteContract();

  const {
    isLoading: isConfirming,
    isSuccess: isConfirmed,
    isError: HasFailed,
  } = useWaitForTransactionReceipt({
    hash: txHash as `0x${string}` | undefined,
  });

  useEffect(() => {
    if (isConfirmed) {
      setIssueUrl('');
      setTxHash(null);
      toast.success('Succès !', {
        description: 'Grant validé avec succès !',
      });
    } else if (HasFailed) {
      toast.error('Échec', {
        description: 'La transaction a échoué',
      });
    }
  }, [isConfirmed, HasFailed]);

  useEffect(() => {
    if (isError && error) {
      console.error('Erreur de transaction:', error);
      toast.error('Erreur', {
        description: error.message,
      });
    }
  }, [isError, error]);

  const handleValidateGrant = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!issueUrl || !contractAddress) {
      toast.error('Champs manquants', {
        description: 'Veuillez remplir tous les champs',
      });
      return;
    }

    if (!isConnected) {
      toast.error('Portefeuille non connecté', {
        description: 'Veuillez connecter votre portefeuille',
      });
      await connectWallet();
      return;
    }

    try {
      const hash = await writeContractAsync({
        address: contractAddress as `0x${string}`,
        abi: vgrantABI,
        functionName: 'approveBounty',
        args: [issueUrl],
      });

      if (hash) {
        console.log('Hash de la transaction:', hash);
        setTxHash(hash);
        toast.success('Transaction soumise', {
          description:
            'Votre validation est en cours. Veuillez attendre la confirmation.',
        });
      }
    } catch (error) {
      console.error('Erreur lors de la validation du grant:', error);
      toast.error('Échec', {
        description: 'Échec de la validation du grant. Veuillez réessayer.',
      });
    }
  };

  return (
    <div className="flex flex-col items-center space-y-12 pt-16">
      <h1 className="text-3xl font-bold">Valider une Récompense</h1>

      <Card className="w-full max-w-md">
        <form onSubmit={handleValidateGrant}>
          <CardHeader>
            <CardTitle>Validation de Récompense</CardTitle>
            <CardDescription>
              Validez une récompense pour une issue GitHub avec le contrat
              Vgrant
            </CardDescription>
          </CardHeader>

          <CardContent className="space-y-4">
            <div className="space-y-2">
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
            </div>

            <div className="space-y-2">
              <label htmlFor="issueUrl" className="text-sm font-medium">
                URL de l'issue
              </label>
              <Input
                id="issueUrl"
                placeholder="https://github.com/owner/repo/issues/123"
                value={issueUrl}
                onChange={(e) => setIssueUrl(e.target.value)}
                required
              />
              <p className="text-xs text-muted-foreground">
                L'URL complète de l'issue GitHub que vous souhaitez valider
              </p>
            </div>
          </CardContent>

          <CardFooter className="flex flex-col space-y-4">
            <Button
              type="submit"
              className="w-full"
              disabled={isPending || isConfirming || !isConnected}
            >
              {isPending
                ? "En attente d'approbation..."
                : isConfirming
                  ? 'Confirmation en cours...'
                  : 'Valider le Grant'}
            </Button>

            {/* Statut de la transaction */}
            {txHash && (
              <div className="text-sm text-center">
                {isConfirming && (
                  <p className="text-yellow-500">
                    Transaction en cours... <br />
                    <a
                      href={`http://localhost:8545/tx/${txHash}`}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="underline"
                    >
                      Voir la transaction
                    </a>
                  </p>
                )}
                {isConfirmed && (
                  <p className="text-green-500">
                    Transaction confirmée ! <br />
                    <a
                      href={`http://localhost:8545/tx/${txHash}`}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="underline"
                    >
                      Voir la transaction
                    </a>
                  </p>
                )}
                {isError && (
                  <p className="text-red-500">
                    Erreur de transaction:{' '}
                    {error?.message || 'Veuillez réessayer'}
                  </p>
                )}
              </div>
            )}
          </CardFooter>
        </form>
      </Card>
    </div>
  );
};
