import { Route, Routes } from 'react-router';
import './App.css';

import { Layout } from '@/components/Layout';
import { Challenge } from '@/pages/Challenge';
import { Home } from '@/pages/Home';
import { ListChallenges } from '@/pages/ListChallenges';

function App() {
  return (
    <>
      <Routes>
        <Route index element={<Home />} />

        <Route element={<Layout />}>
          <Route path="/challenge">
            <Route index element={<ListChallenges />} />
            <Route path=":id" element={<Challenge />} />
          </Route>
        </Route>
      </Routes>
    </>
  );
}

export default App;
