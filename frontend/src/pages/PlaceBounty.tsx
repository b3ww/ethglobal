import { useState } from 'react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';

export const PlaceBounty = () => {
  const [issueUrl, setIssueUrl] = useState('');
  const [price, setPrice] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!issueUrl || !price) {
      alert('Please fill in all fields');
      return;
    }

    setIsSubmitting(true);

    try {
      // This is a placeholder for the actual blockchain transaction
      console.log('Placing bounty:', { issueUrl, price: parseFloat(price) });

      // In a real implementation, this would:
      // 1. Connect to the user's wallet (e.g., using ethers.js or web3.js)
      // 2. Create a transaction to call a smart contract function
      // 3. Send the transaction to the blockchain
      // 4. Wait for confirmation

      // Example of how this might look with ethers.js:
      /*
      // Connect to wallet
      const provider = new ethers.providers.Web3Provider(window.ethereum);
      const signer = provider.getSigner();

      // Connect to contract
      const bountyContract = new ethers.Contract(
        CONTRACT_ADDRESS,
        CONTRACT_ABI,
        signer
      );

      // Call contract function
      const tx = await bountyContract.placeBounty(
        issueUrl,
        ethers.utils.parseUnits(price, 6) // USDC has 6 decimals
      );

      // Wait for confirmation
      await tx.wait();
      */

      // For now, simulate a successful transaction
      setTimeout(() => {
        alert('Bounty placed successfully!');
        setIssueUrl('');
        setPrice('');
        setIsSubmitting(false);
      }, 1000);
    } catch (error) {
      console.error('Error placing bounty:', error);
      alert('Failed to place bounty. Please try again.');
      setIsSubmitting(false);
    }
  };

  return (
    <div className="flex flex-col items-center space-y-12 pt-16">
      <h1 className="text-3xl font-bold">Place a Bounty</h1>

      <Card className="w-full max-w-md">
        <form onSubmit={handleSubmit}>
          <CardHeader>
            <CardTitle>Bounty Details</CardTitle>
            <CardDescription>
              Enter the details of the issue you want to place a bounty on.
            </CardDescription>
          </CardHeader>

          <CardContent className="space-y-4">
            <div className="space-y-2">
              <label htmlFor="issueUrl" className="text-sm font-medium">
                Issue URL
              </label>
              <Input
                id="issueUrl"
                placeholder="https://github.com/owner/repo/issues/123"
                value={issueUrl}
                onChange={(e) => setIssueUrl(e.target.value)}
                required
              />
            </div>

            <div className="space-y-2">
              <label htmlFor="price" className="text-sm font-medium">
                Price (USDC)
              </label>
              <Input
                id="price"
                type="number"
                placeholder="100"
                min="0"
                step="0.01"
                value={price}
                onChange={(e) => setPrice(e.target.value)}
                required
              />
            </div>
          </CardContent>

          <CardFooter>
            <Button 
              type="submit" 
              className="w-full"
              disabled={isSubmitting}
            >
              {isSubmitting ? 'Processing...' : 'Place Bounty'}
            </Button>
          </CardFooter>
        </form>
      </Card>
    </div>
  );
};
