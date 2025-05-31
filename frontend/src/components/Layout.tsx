import { Outlet } from 'react-router';

import { Navbar } from '@/components/Navbar';
import { Toaster } from '@/components/ui/sonner';

export const Layout = () => {
  return (
    <div className="py-6 px-12">
      <Navbar />
      <Outlet />
      <Toaster />
    </div>
  );
};
