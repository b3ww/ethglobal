import { Link } from 'react-router';

import { ModeToggle } from '@/components/mode-toggle';
import { Button } from './ui/button';

export const Navbar = () => {
  return (
    <nav className="bg-secondary rounded-4xl shadow-lg">
      <div className="px-8 flex justify-between items-center h-16">
        <div className="text-xl font-bold">VGrant</div>

        <div className="hidden md:flex space-x-24">
          <Link
            to="/register-account"
            className="hover:text-accent transition-colors"
          >
            Register account
          </Link>
          <Link
            to="/create-grant"
            className="hover:text-accent transition-colors"
          >
            Create grant
          </Link>
          <Link
            to="/validate-grant"
            className="hover:text-accent transition-colors"
          >
            Validate grant
          </Link>
        </div>

        <div className="flex items-center space-x-10">
          <ModeToggle />

          <Button className="flex items-center space-x-2 group-hover:text-accent transition-colors">
            Connect Wallet
          </Button>
        </div>
      </div>
    </nav>
  );
};
