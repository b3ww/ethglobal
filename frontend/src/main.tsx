import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { BrowserRouter } from 'react-router';
import App from './App.tsx';
import './index.css';

import { ThemeProvider } from '@/components/theme-provider.tsx';
import { WalletProvider } from '@/providers/WalletProvider';
import { ProofProvider } from '@vlayer/react';

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <BrowserRouter>
      <WalletProvider>
        <ProofProvider
          config={{
            proverUrl: import.meta.env.VITE_PROVER_URL,
            wsProxyUrl: import.meta.env.VITE_WS_PROXY_URL,
            notaryUrl: import.meta.env.VITE_NOTARY_URL,
            token: import.meta.env.VITE_VLAYER_API_TOKEN,
          }}
        >
          <ThemeProvider>
            <App />
          </ThemeProvider>
        </ProofProvider>
      </WalletProvider>
    </BrowserRouter>
  </StrictMode>,
);
