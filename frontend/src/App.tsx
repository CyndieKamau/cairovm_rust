import React, { useState } from 'react';
import { Editor } from '@monaco-editor/react';
import axios from 'axios';

interface TokenInfo {
  token_type: string;
  span: [number, number];
  slice: string;
}


const App: React.FC = () => {
    const [code, setCode] = useState(''); // Code input
    const [tokens, setTokens] = useState<TokenInfo[]>([]); // Token output
    const [error, setError] = useState<string | null>(null);
    const API_URL = process.env.REACT_APP_BACKEND_URL;

    const tokenizeCode = async () => {
        try {
            const response = await axios.post(API_URL + '/tokenize', {
                code,
            });
            setTokens(response.data.tokens);
            setError(null); // Clear any previous errors
        } catch (err) {
            setError('Failed to tokenize the code. Please try again.');
            console.error(err);
        }
    };

    return (
      <div className="h-screen flex flex-col">
      {/* Parent Container: Flexbox */}
      <div className="flex-1 flex items-stretch h-full">
        {/* Left Panel: Code Editor */}
        <div className="w-1/2 p-4 bg-black flex flex-col">
          <h2 className="text-xl font-bold mb-2 text-white">Cairo Code Editor</h2>
          <Editor
            height="100%"
            language="rust" // Syntax highlighting for Cairo (similar to Rust)
            theme="vs-dark"
            value={code}
            onChange={(value) => setCode(value || '')}
          />
          <button
            className="mt-4 px-6 py-3 bg-blue-600 hover:bg-blue-500 text-white font-semibold rounded"
            onClick={tokenizeCode}
          >
            Tokenize Code
          </button>
        </div>
    
        {/* Right Panel: Token Output */}
        <div className="w-1/2 h-[calc(100vh-64px)]p-4 bg-black text-white overflow-y-scroll">
          <h2 className="text-xl font-bold mb-2">Tokens</h2>
          {error ? (
            <div className="text-red-500">{error}</div>
          ) : (
            <table className="w-full text-left">
              <thead>
                <tr>
                  <th className="py-2">Type</th>
                  <th className="py-2">Span</th>
                  <th className="py-2">Slice</th>
                </tr>
              </thead>
              <tbody>
                {tokens.map((token, index) => (
                  <tr key={index} className="border-b border-gray-700">
                    <td className="py-2">{token.token_type}</td>
                    <td className="py-2">{`(${token.span[0]}, ${token.span[1]})`}</td>
                    <td className="py-2">{token.slice}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
        </div>
      </div>
    </div>
    
    );
};

export default App;
