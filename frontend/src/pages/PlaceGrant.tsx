import { Button } from '@/components/ui/button';
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { DatePicker } from '@/components/ui/date-picker';
import { Input } from '@/components/ui/input';
import { useWalletConnection } from '@/hooks/useWalletConnection';
import { useEffect, useState } from 'react';
import { toast } from 'sonner';
import {
  useReadContract,
  useWaitForTransactionReceipt,
  useWriteContract,
} from 'wagmi';

// ABI for Vgrant contract
const vgrantABI = [
  {
    name: 'addBounty',
    type: 'function',
    stateMutability: 'nonpayable',
    inputs: [
      { name: '_url', type: 'string' },
      { name: '_issueId', type: 'uint256' },
      { name: '_bounty', type: 'uint256' },
      { name: '_deadline', type: 'uint256' },
    ],
    outputs: [],
  },
  {
    name: 'isVerified',
    type: 'function',
    stateMutability: 'view',
    inputs: [{ name: '_user', type: 'address' }],
    outputs: [{ name: '', type: 'bool' }],
  },
  {
    name: 'bountyToken',
    type: 'function',
    stateMutability: 'view',
    inputs: [],
    outputs: [{ name: '', type: 'address' }],
  },
];

// ERC20 token ABI (only the functions we need)
const erc20ABI = [
  {
    name: 'approve',
    type: 'function',
    stateMutability: 'nonpayable',
    inputs: [
      { name: 'spender', type: 'address' },
      { name: 'amount', type: 'uint256' },
    ],
    outputs: [{ name: '', type: 'bool' }],
  },
  {
    name: 'allowance',
    type: 'function',
    stateMutability: 'view',
    inputs: [
      { name: 'owner', type: 'address' },
      { name: 'spender', type: 'address' },
    ],
    outputs: [{ name: '', type: 'uint256' }],
  },
  {
    name: 'balanceOf',
    type: 'function',
    stateMutability: 'view',
    inputs: [{ name: 'account', type: 'address' }],
    outputs: [{ name: '', type: 'uint256' }],
  },
];

export const GrantPage = () => {
  const [issueUrl, setIssueUrl] = useState('');
  const [issueId, setIssueId] = useState('');
  const [amount, setAmount] = useState('');
  const [tokenAddress, setTokenAddress] = useState<`0x${string}` | null>(null);
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

  const { isConnected, address } = useWalletConnection();

  // Hooks pour interagir avec les contrats
  const { writeContractAsync, isPending, isError, error } = useWriteContract();

  // Get token address from contract
  const { data: tokenAddressData, refetch: refetchTokenAddress } =
    useReadContract({
      address: import.meta.env.VITE_VERIFIER_ADDRESS,
      abi: vgrantABI,
      functionName: 'bountyToken',
    });

  // Check token allowance
  const { refetch: refetchAllowance } = useReadContract({
    address: tokenAddress as `0x${string}`,
    abi: erc20ABI,
    functionName: 'allowance',
    args: [
      address as `0x${string}`,
      import.meta.env.VITE_VERIFIER_ADDRESS as `0x${string}`,
    ],
    query: {
      enabled: !!address && !!tokenAddress,
    },
  });

  const {
    isLoading: isConfirming,
    isSuccess: isConfirmed,
    isError: HasFailed,
  } = useWaitForTransactionReceipt({
    hash: txHash as `0x${string}` | undefined,
  });

  // Update token address
  useEffect(() => {
    if (tokenAddressData) {
      setTokenAddress(tokenAddressData as `0x${string}`);
      console.log('Token address:', tokenAddressData);
    }
  }, [tokenAddressData]);

  // Refetch data when address or contract address changes
  useEffect(() => {
    if (address) {
      refetchTokenAddress();
    }
  }, [address, refetchTokenAddress]);

  // Refetch allowance when token address changes or amount changes
  useEffect(() => {
    if (address && tokenAddress) {
      refetchAllowance();
    }
  }, [address, tokenAddress, amount, refetchAllowance]);

  // Réinitialiser le formulaire après confirmation
  useEffect(() => {
    if (isConfirmed) {
      setIssueUrl('');
      setIssueId('');
      setAmount('');
      setTxHash(null);
      // Réinitialiser la date limite à la valeur minimale (aujourd'hui + 48h)
      setDeadline(new Date(minDeadline));
      toast.success('Succès !', {
        description: 'Grant placé avec succès !',
      });
    }
  }, [isConfirmed, minDeadline]);

  useEffect(() => {
    if (HasFailed) {
      toast.error('Échec', {
        description: 'La transaction a échoué',
      });
    }
  }, [HasFailed]);

  useEffect(() => {
    if (isError && error) {
      console.error('Erreur de transaction:', error);
      toast.error('Erreur', {
        description: error.message,
      });
    }
  }, [isError, error]);

  const handle = async () => {
    if (!issueUrl || !issueId || !amount || !deadline || !isConnected) {
      toast.error('Champs manquants', {
        description: 'Veuillez remplir tous les champs',
      });
      return;
    }

    try {
      const hash = await writeContractAsync({
        address: tokenAddress as `0x${string}`,
        abi: erc20ABI,
        functionName: 'approve',
        args: [import.meta.env.VITE_VERIFIER_ADDRESS, amount],
      });

      if (hash) {
        console.log("Hash de la transaction d'approbation:", hash);
        setTxHash(hash);
        toast.success('Approbation en cours', {
          description:
            "Veuillez attendre la confirmation de l'approbation du token.",
        });
      }
    } catch (error) {
      console.error("Erreur lors de l'approbation du token:", error);
      toast.error("Échec de l'approbation", {
        description: "Échec de l'approbation du token. Veuillez réessayer.",
      });
    }

    if (deadline < minDeadline) {
      toast.error('Date limite invalide', {
        description: 'La date limite doit être au moins 48h dans le futur',
      });
      return;
    }

    try {
      // Convertir la date en timestamp Unix (secondes depuis l'epoch)
      const deadlineTimestamp = Math.floor(deadline!.getTime() / 1000);
      const issueIdNumber = parseInt(issueId);

      const hash = await writeContractAsync({
        address: import.meta.env.VITE_VERIFIER_ADDRESS as `0x${string}`,
        abi: vgrantABI,
        functionName: 'addBounty',
        args: [
          issueUrl,
          BigInt(issueIdNumber),
          amount,
          BigInt(deadlineTimestamp),
        ],
      });

      if (hash) {
        console.log('Hash de la transaction:', hash);
        setTxHash(hash);
        toast.success('Transaction soumise', {
          description:
            'Votre grant est en cours de placement. Veuillez attendre la confirmation.',
        });
      }
    } catch (error) {
      console.error('Erreur lors du placement du grant:', error);
      toast.error('Échec', {
        description: 'Échec du placement du grant. Veuillez réessayer.',
      });
    }
  };

  return (
    <div className="flex flex-col items-center space-y-12 pt-16">
      <h1 className="text-3xl font-bold">Create a grant</h1>

      <Card className="w-full max-w-md">
        <CardHeader>
          <CardTitle>Grant details</CardTitle>
          <CardDescription>
            Create a grant for a GitHub issue using the Vgrant contract
          </CardDescription>
        </CardHeader>

        <CardContent className="space-y-4">
          <div className="space-y-2">
            <label htmlFor="issueUrl" className="text-sm font-medium">
              Issue URL
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
            <label htmlFor="issueId" className="text-sm font-medium">
              Issue ID
            </label>
            <Input
              id="issueId"
              type="number"
              placeholder="123"
              min="1"
              value={issueId}
              onChange={(e) => setIssueId(e.target.value)}
              required
            />
            <p className="text-xs text-muted-foreground">
              The id of the GitHub issue (e.g., 123 for issue #123)
            </p>
          </div>

          <div className="space-y-2">
            <label htmlFor="amount" className="text-sm font-medium">
              Amount of tokens
            </label>
            <Input
              id="amount"
              type="number"
              placeholder="100"
              min="0"
              step="1"
              value={amount}
              onChange={(e) => setAmount(e.target.value)}
              required
            />
            <p className="text-xs text-muted-foreground">
              {tokenAddress
                ? `Using token at address ${tokenAddress.slice(0, 6)}...${tokenAddress.slice(-4)}`
                : 'Loading token...'}
            </p>
          </div>

          <div className="space-y-2">
            <label htmlFor="deadline" className="text-sm font-medium">
              Limit date (minimum 48h from now)
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
          {
            <Button
              type="button"
              className="w-full"
              onClick={handle}
              disabled={isPending || isConfirming || !isConnected}
            >
              Create Grant
            </Button>
          }
        </CardFooter>
      </Card>
    </div>
  );
};
