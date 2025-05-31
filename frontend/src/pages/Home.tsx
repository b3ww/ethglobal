import { Button } from '@/components/ui/button';

export const Home = () => {
  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-background">
      <header className="text-center py-10">
        <h1 className="text-4xl font-bold text-primary">Welcome to VGrant</h1>
        <p className="text-lg text-muted-foreground mt-4">
          Your go-to app for tracking and managing hashes efficiently.
        </p>
      </header>
      <main className="flex flex-col items-center">
        <Button variant="default" className="mt-4">
          Get Started
        </Button>
      </main>
      <footer className="mt-10 text-muted-foreground">
        <p>&copy; {new Date().getFullYear()} VGrant. All rights reserved.</p>
      </footer>
    </div>
  );
};
