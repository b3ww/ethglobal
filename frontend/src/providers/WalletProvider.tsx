import { ReactNode } from 'react';
import { WagmiProvider, createConfig, http } from 'wagmi';
import {anvil} from 'wagmi/chains';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { injected } from 'wagmi/connectors';

// Create a client for React Query
const queryClient = new QueryClient();

// Create a Wagmi config with the MetaMask connector
const config = createConfig({
  chains: [anvil], // todo: replace by worldchain
  transports: {
    [anvil.id]: http(),
  },
  connectors: [
    injected(),
  ],
});

interface WalletProviderProps {
  children: ReactNode;
}

export function WalletProvider({ children }: WalletProviderProps) {
  return (
    <WagmiProvider config={config}>
      <QueryClientProvider client={queryClient}>
        {children}
      </QueryClientProvider>
    </WagmiProvider>
  );
}