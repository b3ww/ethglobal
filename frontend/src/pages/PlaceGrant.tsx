import { useState, useEffect } from 'react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { useWalletConnection } from '@/lib/hooks/useWalletConnection';
import { useWriteContract, useWaitForTransactionReceipt } from 'wagmi';
import { parseEther } from 'viem';
import * as React from 'react';
import { toast } from 'sonner';
import { DatePicker } from '@/components/ui/date-picker';

// ABI simplifié pour placer un grant
const grantABI = [
  {
    name: 'placeBounty',
    type: 'function',
    stateMutability: 'payable',
    inputs: [
      { name: '_issueUrl', type: 'string' },
      { name: '_deadline', type: 'uint256' }
    ],
    outputs: [],
  },
];

export const GrantPage = () => {
  const [issueUrl, setIssueUrl] = useState('');
  const [amount, setAmount] = useState('');
  const [contractAddress, setContractAddress] = useState(
    '0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512',
  );
  const [txHash, setTxHash] = useState<string | null>(null);

  // Calculer la date minimum (aujourd'hui + 48h)
  const getMinDeadline = () => {
    const minDate = new Date();
    minDate.setHours(minDate.getHours() + 48);
    return minDate;
  };

  // Stocker la date minimum dans un état pour éviter les recalculs
  const [minDeadline] = useState<Date>(() => {
    const min = getMinDeadline();
    console.log('Min deadline initialized:', min);
    return min;
  });

  const [deadline, setDeadline] = useState<Date | undefined>(() => {
    const defaultDate = new Date(minDeadline);
    console.log('Default deadline set to:', defaultDate);
    return defaultDate;
  });

  const { isConnected, connectWallet } = useWalletConnection();

  // Hook pour écrire dans le contrat
  const { writeContractAsync, isPending, isError, error } = useWriteContract();

  const { isLoading: isConfirming, isSuccess: isConfirmed, isError: HasFailed } =
    useWaitForTransactionReceipt({
      hash: txHash as `0x${string}` | undefined,
    });

  // Réinitialiser le formulaire après confirmation
  useEffect(() => {
    if (isConfirmed) {
      setIssueUrl('');
      setAmount('');
      setTxHash(null);
      // Réinitialiser la date limite à la valeur minimale (aujourd'hui + 48h)
      setDeadline(new Date(minDeadline));
      toast.success("Succès !", {
        description: "Grant placé avec succès !",
      });
    }
  }, [isConfirmed]);

  useEffect(() => {
    if (HasFailed) {
      toast.error("Échec", {
        description: "La transaction a échoué",
      });
    }
  }, [HasFailed]);

  useEffect(() => {
    if (isError && error) {
      console.error('Erreur de transaction:', error);
      toast.error("Erreur", {
        description: error.message,
      });
    }
  }, [isError, error]);

  const handlePlaceGrant = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!issueUrl || !amount || !contractAddress || !deadline) {
      toast.error("Champs manquants", {
        description: "Veuillez remplir tous les champs",
      });
      return;
    }

    // Vérifier que la date limite est au moins 48h dans le futur
    if (!deadline || deadline < minDeadline) {
      toast.error("Date limite invalide", {
        description: "La date limite doit être au moins 48h dans le futur",
      });
      return;
    }

    if (!isConnected) {
      toast.error("Portefeuille non connecté", {
        description: "Veuillez connecter votre portefeuille",
      });
      await connectWallet();
      return;
    }

    try {
      // Convertir la date en timestamp Unix (secondes depuis l'epoch)
      const deadlineTimestamp = Math.floor(deadline!.getTime() / 1000);

      const hash = await writeContractAsync({
        address: contractAddress as `0x${string}`,
        abi: grantABI,
        functionName: 'placeBounty',
        args: [issueUrl, deadlineTimestamp],
        value: parseEther(amount),
      });

      if (hash) {
        console.log('Hash de la transaction:', hash);
        setTxHash(hash);
      }
    } catch (error) {
      console.error('Erreur lors du placement du grant:', error);
      toast.error("Échec", {
        description: "Échec du placement du grant. Veuillez réessayer.",
      });
    }
  };

  return (
    <div className="flex flex-col items-center space-y-12 pt-16">
      <h1 className="text-3xl font-bold">Placer un Grant</h1>

      <Card className="w-full max-w-md">
        <form onSubmit={handlePlaceGrant}>
          <CardHeader>
            <CardTitle>Détails du Grant</CardTitle>
            <CardDescription>
              Placez un grant sur une URL d'issue GitHub
            </CardDescription>
          </CardHeader>

          <CardContent className="space-y-4">
            <div className="space-y-2">
              <label htmlFor="contractAddress" className="text-sm font-medium flex items-center gap-2">
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
            </div>

            <div className="space-y-2">
              <label htmlFor="amount" className="text-sm font-medium">
                Montant (ETH)
              </label>
              <Input
                id="amount"
                type="number"
                placeholder="0.1"
                min="0"
                step="0.001"
                value={amount}
                onChange={(e) => setAmount(e.target.value)}
                required
              />
            </div>

            <div className="space-y-2">
              <label htmlFor="deadline" className="text-sm font-medium">
                Date limite (minimum 48h à partir de maintenant)
              </label>
              <DatePicker
                date={deadline}
                setDate={setDeadline}
                disabledDates={(date) => date < minDeadline}
                placeholder="Sélectionner une date limite"
              />
            </div>
          </CardContent>

          <CardFooter className="flex flex-col space-y-4">
            <Button
              type="submit"
              className="w-full"
              disabled={isPending || isConfirming}
            >
              {isPending
                ? "En attente d'approbation..."
                : isConfirming
                  ? 'Confirmation en cours...'
                  : 'Placer le Grant'}
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
