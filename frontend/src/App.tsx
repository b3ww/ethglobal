import { Route, Routes } from 'react-router';

import { Layout } from '@/components/Layout';
import { Home } from '@/pages/Home';
import { GrantPage } from '@/pages/PlaceGrant';
import { ValidateGrantPage } from '@/pages/ValidateGrant';
import { TemplatePage } from './pages/TemplatePage';

function App() {
  return (
    <>
      <Routes>
        <Route element={<Layout />}>
          <Route index element={<Home />} />
          <Route path="/register-account" element={<TemplatePage />} />
          <Route path="/create-grant" element={<GrantPage />} />
          <Route path="/validate-grant" element={<ValidateGrantPage />} />
        </Route>
      </Routes>
    </>
  );
}

export default App;
