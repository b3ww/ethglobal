import { useEffect } from 'react';

export const Redirect = () => {
  useEffect(() => {
    fetch('https://back.vgrant.xyz/api/auth/me').then((response) =>
      response.json(),
    );
  }, []);

  return (
    <div className="flex flex-col items-center justify-center h-screen my-[-50px]">
      <h1 className="text-2xl font-bold mb-4">Redirected to the application</h1>
      <p className="text-lg">The authentication was successfully complete.</p>
    </div>
  );
};
