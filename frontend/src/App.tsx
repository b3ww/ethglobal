import { Route, Routes } from 'react-router';
import './App.css';

import { Layout } from '@/components/Layout';
import { Home } from './pages/Home';
import { TemplatePage } from './pages/TemplatePage';
import { GrantPage } from '@/pages/PlaceGrant.tsx';

function App() {
  return (
    <>
      <Routes>
        <Route element={<Layout />}>
          <Route index element={<Home />} />
          <Route path="/register-account" element={<TemplatePage />} />
          <Route path="/create-grant" element={<GrantPage />} />
          <Route path="/validate-grant" element={<TemplatePage />} />
        </Route>
      </Routes>
    </>
  );
}

export default App;
