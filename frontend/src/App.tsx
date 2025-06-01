import { Route, Routes } from 'react-router';

import { Layout } from '@/components/Layout';
import { Home } from '@/pages/Home';
import { GrantPage } from '@/pages/PlaceGrant';
import { ValidateGrantPage } from '@/pages/ValidateGrant';
import { Redirect } from './pages/Redirect';
import { RegisterAccount } from './pages/RegisterAccount';

function App() {
  return (
    <>
      <Routes>
        <Route element={<Layout />}>
          <Route index element={<Home />} />
          <Route path="/register-account" element={<RegisterAccount />} />
          <Route path="/create-grant" element={<GrantPage />} />
          <Route path="/validate-grant" element={<ValidateGrantPage />} />
          <Route path="/redirect" element={<Redirect />} />
        </Route>
      </Routes>
    </>
  );
}

export default App;
