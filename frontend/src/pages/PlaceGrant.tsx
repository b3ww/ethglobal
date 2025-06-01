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
import { parseEther } from 'viem';
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
  const [contractAddress, setContractAddress] = useState(
    '0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512',
  );
  const [tokenAddress, setTokenAddress] = useState<`0x${string}` | null>(null);
  const [isAccountVerified, setIsAccountVerified] = useState(false);
  const [isApproved, setIsApproved] = useState(false);
  const [isApproving, setIsApproving] = useState(false);
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

  const { isConnected, connectWallet, address } = useWalletConnection();

  // Hooks pour interagir avec les contrats
  const { writeContractAsync, isPending, isError, error } = useWriteContract();
  const { data: readData, refetch: refetchRead } = useReadContract({
    address: contractAddress as `0x${string}`,
    abi: vgrantABI,
    functionName: 'isVerified',
    args: [address as `0x${string}`],
    query: {
      enabled: !!address && !!contractAddress,
    },
  });

  // Get token address from contract
  const { data: tokenAddressData, refetch: refetchTokenAddress } =
    useReadContract({
      address: contractAddress as `0x${string}`,
      abi: vgrantABI,
      functionName: 'bountyToken',
      query: {
        enabled: !!contractAddress,
      },
    });

  // Check token allowance
  const { data: allowanceData, refetch: refetchAllowance } = useReadContract({
    address: tokenAddress as `0x${string}`,
    abi: erc20ABI,
    functionName: 'allowance',
    args: [address as `0x${string}`, contractAddress as `0x${string}`],
    query: {
      enabled: !!address && !!tokenAddress && !!contractAddress,
    },
  });

  const {
    isLoading: isConfirming,
    isSuccess: isConfirmed,
    isError: HasFailed,
  } = useWaitForTransactionReceipt({
    hash: txHash as `0x${string}` | undefined,
  });

  // Update account verification status
  useEffect(() => {
    if (readData !== undefined) {
      setIsAccountVerified(readData as boolean);
      console.log('Account verification status:', readData);
    }
  }, [readData]);

  // Update token address
  useEffect(() => {
    if (tokenAddressData) {
      setTokenAddress(tokenAddressData as `0x${string}`);
      console.log('Token address:', tokenAddressData);
    }
  }, [tokenAddressData]);

  // Update approval status
  useEffect(() => {
    if (allowanceData && amount) {
      const amountInWei = parseEther(amount);
      setIsApproved(BigInt(allowanceData as string) >= amountInWei);
      console.log('Allowance:', allowanceData, 'Required:', amountInWei);
    }
  }, [allowanceData, amount]);

  // Refetch data when address or contract address changes
  useEffect(() => {
    if (address && contractAddress) {
      refetchRead();
      refetchTokenAddress();
    }
  }, [address, contractAddress, refetchRead, refetchTokenAddress]);

  // Refetch allowance when token address changes or amount changes
  useEffect(() => {
    if (address && tokenAddress && contractAddress) {
      refetchAllowance();
    }
  }, [address, tokenAddress, contractAddress, amount, refetchAllowance]);

  // Réinitialiser le formulaire après confirmation
  useEffect(() => {
    if (isConfirmed) {
      setIssueUrl('');
      setIssueId('');
      setAmount('');
      setTxHash(null);
      setIsApproved(false);
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

  // Function to approve token spending
  const handleApproveToken = async () => {
    if (!tokenAddress || !amount || !contractAddress || !address) {
      toast.error('Données manquantes', {
        description: "Impossible d'approuver le token. Données manquantes.",
      });
      return;
    }

    setIsApproving(true);
    try {
      const amountInWei = parseEther(amount);

      const hash = await writeContractAsync({
        address: tokenAddress,
        abi: erc20ABI,
        functionName: 'approve',
        args: [contractAddress as `0x${string}`, amountInWei],
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
    } finally {
      setIsApproving(false);
    }
  };

  const handlePlaceGrant = async (e: React.FormEvent) => {
    e.preventDefault();

    if (
      !issueUrl ||
      !issueId ||
      !amount ||
      !contractAddress ||
      !deadline ||
      !tokenAddress
    ) {
      toast.error('Champs manquants', {
        description: 'Veuillez remplir tous les champs',
      });
      return;
    }

    // Vérifier que la date limite est au moins 48h dans le futur
    if (!deadline || deadline < minDeadline) {
      toast.error('Date limite invalide', {
        description: 'La date limite doit être au moins 48h dans le futur',
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

    // Vérifier que le compte est vérifié
    if (!isAccountVerified) {
      toast.error('Compte non vérifié', {
        description:
          "Votre compte doit être vérifié pour placer un grant. Veuillez vous enregistrer d'abord.",
      });
      return;
    }

    // Vérifier que le token est approuvé
    if (!isApproved) {
      toast.error('Token non approuvé', {
        description:
          "Vous devez d'abord approuver le token avant de placer un grant.",
      });
      await handleApproveToken();
      return;
    }

    try {
      // Convertir la date en timestamp Unix (secondes depuis l'epoch)
      const deadlineTimestamp = Math.floor(deadline!.getTime() / 1000);
      const amountInWei = parseEther(amount);
      const issueIdNumber = parseInt(issueId);

      const hash = await writeContractAsync({
        address: contractAddress as `0x${string}`,
        abi: vgrantABI,
        functionName: 'addBounty',
        args: [
          issueUrl,
          BigInt(issueIdNumber),
          amountInWei,
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
      <h1 className="text-3xl font-bold">Créer une Récompense</h1>

      <Card className="w-full max-w-md">
        <form onSubmit={handlePlaceGrant}>
          <CardHeader>
            <CardTitle>Détails de la Récompense</CardTitle>
            <CardDescription>
              Créez une récompense pour une issue GitHub avec le contrat Vgrant
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
            </div>

            <div className="space-y-2">
              <label htmlFor="issueId" className="text-sm font-medium">
                ID de l'issue
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
                L'ID numérique de l'issue GitHub (ex: 123 pour issue #123)
              </p>
            </div>

            <div className="space-y-2">
              <label htmlFor="amount" className="text-sm font-medium">
                Montant (Tokens)
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
                  ? `Utilisant le token à l'adresse ${tokenAddress.slice(0, 6)}...${tokenAddress.slice(-4)}`
                  : 'Chargement du token...'}
              </p>
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
            {/* Statut de vérification du compte */}
            {isConnected && (
              <div className="text-sm mb-2 flex items-center justify-center gap-2">
                <span>Statut du compte:</span>
                {isAccountVerified ? (
                  <span className="text-green-500 font-medium flex items-center gap-1">
                    <svg
                      xmlns="http://www.w3.org/2000/svg"
                      width="16"
                      height="16"
                      viewBox="0 0 24 24"
                      fill="none"
                      stroke="currentColor"
                      strokeWidth="2"
                      strokeLinecap="round"
                      strokeLinejoin="round"
                    >
                      <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path>
                      <polyline points="22 4 12 14.01 9 11.01"></polyline>
                    </svg>
                    Vérifié
                  </span>
                ) : (
                  <span className="text-red-500 font-medium flex items-center gap-1">
                    <svg
                      xmlns="http://www.w3.org/2000/svg"
                      width="16"
                      height="16"
                      viewBox="0 0 24 24"
                      fill="none"
                      stroke="currentColor"
                      strokeWidth="2"
                      strokeLinecap="round"
                      strokeLinejoin="round"
                    >
                      <circle cx="12" cy="12" r="10"></circle>
                      <line x1="15" y1="9" x2="9" y2="15"></line>
                      <line x1="9" y1="9" x2="15" y2="15"></line>
                    </svg>
                    Non vérifié
                  </span>
                )}
              </div>
            )}

            {/* Bouton d'approbation ou de placement */}
            {!isApproved && amount && tokenAddress ? (
              <Button
                type="button"
                className="w-full"
                onClick={handleApproveToken}
                disabled={
                  isPending ||
                  isConfirming ||
                  isApproving ||
                  !isConnected ||
                  !isAccountVerified
                }
              >
                {isApproving
                  ? 'Approbation en cours...'
                  : isConfirming
                    ? 'Confirmation en cours...'
                    : 'Approuver les tokens'}
              </Button>
            ) : (
              <Button
                type="submit"
                className="w-full"
                disabled={
                  isPending ||
                  isConfirming ||
                  !isConnected ||
                  !isAccountVerified
                }
              >
                {isPending
                  ? "En attente d'approbation..."
                  : isConfirming
                    ? 'Confirmation en cours...'
                    : 'Placer le Grant'}
              </Button>
            )}

            {/* Message d'aide pour la vérification du compte */}
            {isConnected && !isAccountVerified && (
              <p className="text-xs text-amber-500 text-center">
                Votre compte n'est pas vérifié. Veuillez vous enregistrer avant
                de placer un grant.
              </p>
            )}

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
