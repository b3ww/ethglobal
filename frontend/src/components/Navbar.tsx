import { ChevronRight } from 'lucide-react';
import { Link } from 'react-router';

import { ModeToggle } from '@/components/mode-toggle';
import { Button } from './ui/button';

export const Navbar = () => {
  return (
    <nav className="bg-secondary">
      <div className="px-8 flex justify-between items-center h-16">
        <div className="text-xl font-bold">HashHunter</div>

        <div className="hidden md:flex space-x-24">
          <Link to="/" className="hover:text-accent transition-colors">
            Home
          </Link>
          <Link to="/challenge" className="hover:text-accent transition-colors">
            Challenges
          </Link>
          <Link to="/course" className="hover:text-accent transition-colors">
            Courses
          </Link>
          <Link
            to="/leaderboard"
            className="hover:text-accent transition-colors"
          >
            Leaderboard
          </Link>
        </div>

        <div className="flex items-center space-x-8">
          <ModeToggle />

          <div className="hidden md:flex items-center space-x-8">
            <Link to="/login" className="hover:text-accent transition-colors">
              Login
            </Link>
            <Button>
              <Link
                to="/register"
                className="flex items-center space-x-2 group-hover:text-accent transition-colors"
              >
                <span>Register</span>
                <ChevronRight />
              </Link>
            </Button>
          </div>
        </div>
      </div>
    </nav>
  );
};
