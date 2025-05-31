import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { BrowserRouter } from 'react-router';
import App from './App.tsx';
import './index.css';

import { ThemeProvider } from '@/components/theme-provider.tsx';
import { WalletProvider } from '@/providers/WalletProvider';

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <BrowserRouter>
      <WalletProvider>
        <ThemeProvider>
          <App />
        </ThemeProvider>
      </WalletProvider>
    </BrowserRouter>
  </StrictMode>,
);
