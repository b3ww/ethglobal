import {
  useCallProver,
  useChain,
  useWaitForProvingResult,
  useWebProof,
} from '@vlayer/react';
import { ProveArgs, WebProofConfig } from '@vlayer/sdk';
import { expectUrl, notarize, startPage } from '@vlayer/sdk/web_proof';
import { useEffect, useState } from 'react';
import { useLocalStorage } from 'usehooks-ts';
import { Abi, ContractFunctionName } from 'viem';
import webProofProver from '../../../out/VgrantGithubProver.sol/VgrantProver.json';
import { UseChainError, WebProofError } from '../errors';

const webProofConfig: WebProofConfig<Abi, string> = {
  proverCallCommitment: {
    address: '0x0000000000000000000000000000000000000000',
    proverAbi: [],
    functionName: 'proveWeb',
    commitmentArgs: [],
    chainId: 1,
  },
  logoUrl:
    'https://github.githubassets.com/assets/GitHub-Mark-ea2971cee799.png',
  steps: [
    startPage(
      'https://back.vgrant.xyz/api/auth/github',
      'Go to GitHub oauth page',
    ),
    expectUrl('http://localhost:5173/redirect', 'Log in'),
    notarize(
      'https://back.vgrant.xyz/api/auth/me',
      'GET',
      'Generate Proof of GitHub profile',
      [
        {
          request: {
            // redact all the headers
            headers_except: [],
          },
        },
        {
          response: {
            // response from api.x.com sometimes comes with Transfer-Encoding: Chunked
            // which needs to be recognised by Prover and cannot be redacted
            headers_except: ['Transfer-Encoding'],
          },
        },
      ],
    ),
  ],
};

export const useGithubAccountProof = (): any => {
  const [error, setError] = useState<Error | null>(null);

  const {
    requestWebProof,
    webProof,
    isPending: isWebProofPending,
    error: webProofError,
  } = useWebProof(webProofConfig);

  if (webProofError) {
    throw new WebProofError(webProofError.message);
  }

  const { chain, error: chainError } = useChain(
    import.meta.env.VITE_CHAIN_NAME,
  );
  useEffect(() => {
    if (chainError) {
      setError(new UseChainError(chainError));
    }
  }, [chainError]);

  const vlayerProverConfig: Omit<
    ProveArgs<Abi, ContractFunctionName<Abi>>,
    'args'
  > = {
    address: import.meta.env.VITE_PROVER_ADDRESS as `0x${string}`,
    proverAbi: webProofProver.abi as Abi,
    chainId: chain?.id,
    functionName: 'verifyGithub',
    gasLimit: Number(import.meta.env.VITE_GAS_LIMIT),
  };

  const {
    callProver,
    isPending: isCallProverPending,
    isIdle: isCallProverIdle,
    data: hash,
    error: callProverError,
  } = useCallProver(vlayerProverConfig);

  if (callProverError) {
    throw callProverError;
  }

  const {
    isPending: isWaitingForProvingResult,
    data: result,
    error: waitForProvingResultError,
  } = useWaitForProvingResult(hash);

  if (waitForProvingResultError) {
    throw waitForProvingResultError;
  }

  const [, setWebProof] = useLocalStorage('webProof', '');
  const [, setProverResult] = useLocalStorage('proverResult', '');

  useEffect(() => {
    if (webProof) {
      console.log('webProof', webProof);
      setWebProof(JSON.stringify(webProof));
    }
  }, [JSON.stringify(webProof)]);

  useEffect(() => {
    if (result) {
      console.log('proverResult', result);
      setProverResult(result.toString());
    }
  }, [result]);

  return {
    requestWebProof,
    webProof,
    isPending:
      isWebProofPending || isCallProverPending || isWaitingForProvingResult,
    isCallProverIdle,
    isWaitingForProvingResult,
    isWebProofPending,
    callProver,
    result,
    error,
  };
};
