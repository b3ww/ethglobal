import { useAccount, useConnect, useDisconnect } from 'wagmi';
import { injected } from 'wagmi/connectors';

export function useWalletConnection() {
  const { address, isConnected } = useAccount();
  const { connect } = useConnect();
  const { disconnect } = useDisconnect();

  const connectWallet = async () => {
    try {
      connect({ connector: injected() });
    } catch (error) {
      console.error('Failed to connect wallet:', error);
    }
  };

  const disconnectWallet = () => {
    disconnect();
  };

  // Format address for display (0x1234...5678)
  const formatAddress = (address: string | undefined) => {
    if (!address) return '';
    return `${address.slice(0, 6)}...${address.slice(-4)}`;
  };

  return {
    address,
    isConnected,
    connectWallet,
    disconnectWallet,
    formattedAddress: formatAddress(address),
  };
}
