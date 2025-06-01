import { Link, useLocation } from 'react-router-dom';

import { ModeToggle } from '@/components/mode-toggle';
import {
  NavigationMenu,
  NavigationMenuItem,
  NavigationMenuLink,
  NavigationMenuList,
  navigationMenuTriggerStyle,
} from '@/components/ui/navigation-menu';
import { useWalletConnection } from '@/hooks/useWalletConnection';
import { cn } from '@/lib/utils';
import { Button } from './ui/button';

export const Navbar = () => {
  const { isConnected, connectWallet, disconnectWallet, formattedAddress } =
    useWalletConnection();
  const location = useLocation();

  const isActive = (path: string) => {
    return location.pathname === path;
  };

  return (
    <nav className="bg-secondary rounded-4xl shadow-lg border border-border/40">
      <div className="px-8 py-3 flex justify-between items-center h-16">
        <Link
          to="/"
          className={`text-xl font-bold hover:text-accent transition-colors ${isActive('/') ? 'text-accent' : ''}`}
        >
          VGrant
        </Link>

        <NavigationMenu className="hidden md:flex">
          <NavigationMenuList className="ml-64 space-x-16">
            <NavigationMenuItem>
              <NavigationMenuLink
                asChild
                className={cn(navigationMenuTriggerStyle())}
              >
                <Link to="/register-account">Register account</Link>
              </NavigationMenuLink>
            </NavigationMenuItem>
            <NavigationMenuItem>
              <NavigationMenuLink
                asChild
                className={cn(navigationMenuTriggerStyle())}
              >
                <Link to="/create-grant">Create grant</Link>
              </NavigationMenuLink>
            </NavigationMenuItem>
            {/* <NavigationMenuItem>
              <NavigationMenuLink
                asChild
                className={cn(
                  navigationMenuTriggerStyle(),
                  isActive('/validate-grant') &&
                    'text-accent font-semibold bg-accent/10',
                  'border-border/60 border',
                )}
              >
                <Link to="/validate-grant">Validate grant</Link>
              </NavigationMenuLink>
            </NavigationMenuItem> */}
          </NavigationMenuList>
        </NavigationMenu>

        <div className="flex items-center space-x-10">
          <ModeToggle />

          {isConnected ? (
            <div className="flex items-center space-x-3">
              <span className="text-sm font-medium px-3 py-1.5 bg-muted rounded-full">
                {formattedAddress}
              </span>
              <Button
                variant="outline"
                onClick={disconnectWallet}
                className="transition-colors border-border/70 dark:border-white/70 dark:bg-white/10 dark:text-white dark:hover:bg-accent/40 dark:hover:text-white"
              >
                Disconnect
              </Button>
            </div>
          ) : (
            <Button
              onClick={connectWallet}
              variant="outline"
              className="transition-colors border-border/70 dark:border-white/70 dark:bg-white/10 dark:text-white dark:hover:bg-accent/40 dark:hover:text-white"
            >
              Connect Wallet
            </Button>
          )}
        </div>
      </div>
    </nav>
  );
};
