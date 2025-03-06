import React, { useState, useEffect } from 'react';
import { ethers } from 'ethers';
import { CONTRACT_ADDRESS, CONTRACT_ABI } from './config';
import './App.css';

function App() {
  const [account, setAccount] = useState(null);
  const [provider, setProvider] = useState(null);
  const [signer, setSigner] = useState(null);
  const [balance, setBalance] = useState("0");
  const [toAddress, setToAddress] = useState("");
  const [amount, setAmount] = useState("");
  const [transactions, setTransactions] = useState([]);
  const [eventLog, setEventLog] = useState([]);

  const connectWallet = async () => {
    if (window.ethereum) {
      try {
        const provider = new ethers.BrowserProvider(window.ethereum);
        const signer = await provider.getSigner();
        const accounts = await window.ethereum.request({ method: "eth_requestAccounts" });
        setProvider(provider);
        setSigner(signer);
        setAccount(accounts[0]);
      } catch (error) {
        console.error("Error connecting to MetaMask:", error);
        alert("Failed to connect: " + error.message);
      }
    } else {
      alert("Please install MetaMask!");
    }
  };

  const getBalance = async () => {
    if (provider && account) {
      const contract = new ethers.Contract(CONTRACT_ADDRESS, CONTRACT_ABI, provider);
      const bal = await contract.balanceOf(account);
      setBalance(ethers.formatEther(bal));
    }
  };

  const transferTokens = async () => {
    if (signer) {
      const contract = new ethers.Contract(CONTRACT_ADDRESS, CONTRACT_ABI, signer);
      const tx = await contract.transfer(toAddress, ethers.parseEther(amount));
      await tx.wait();
      alert("Transfer successful!");
      setToAddress("");
      setAmount("");
      getBalance();
    }
  };

  const claimAirdrop = async () => {
    if (signer) {
      const contract = new ethers.Contract(CONTRACT_ADDRESS, CONTRACT_ABI, signer);
      const tx = await contract.airdrop();
      await tx.wait();
      alert("Airdrop claimed successfully!");
      getBalance();
    }
  };

  const getTransactions = async () => {
    if (provider) {
      const contract = new ethers.Contract(CONTRACT_ADDRESS, CONTRACT_ABI, provider);
      const filter = contract.filters.Transfer();
      const events = await contract.queryFilter(filter, -1000, "latest");
      setTransactions(events.map(e => ({
        from: e.args.from,
        to: e.args.to,
        amount: ethers.formatEther(e.args.value),
        txHash: e.transactionHash
      })));
    }
  };

  const setupEventListeners = () => {
    if (provider) {
      const contract = new ethers.Contract(CONTRACT_ADDRESS, CONTRACT_ABI, provider);
      contract.on("Mint", (to, amount) => {
        setEventLog(prev => [...prev, { type: "Mint", to, amount: ethers.formatEther(amount), time: new Date().toLocaleTimeString() }]);
      });
      contract.on("Burn", (from, amount) => {
        setEventLog(prev => [...prev, { type: "Burn", from, amount: ethers.formatEther(amount), time: new Date().toLocaleTimeString() }]);
      });
    }
  };

  useEffect(() => {
    if (window.ethereum) {
      window.ethereum.on("accountsChanged", (accounts) => {
        if (accounts.length > 0) {
          setAccount(accounts[0]);
          const newProvider = new ethers.BrowserProvider(window.ethereum);
          newProvider.getSigner().then(setSigner);
          setProvider(newProvider);
        } else {
          setAccount(null);
          setProvider(null);
          setSigner(null);
        }
      });
    }
  }, []);

  useEffect(() => {
    if (account) {
      getBalance();
      getTransactions();
      setupEventListeners();
    }
  }, [account, provider]);

  return (
    <div className="container mt-5">
      <h1 className="text-center mb-4">MyToken dApp</h1>
      {!account ? (
        <div className="text-center">
          <p className="lead">Please connect your wallet to use the dApp.</p>
          <button className="btn btn-primary" onClick={connectWallet}>
            Connect Wallet
          </button>
        </div>
      ) : (
        <div className="row">
          {/* Balance and Airdrop Section */}
          <div className="col-md-4 mb-4">
            <div className="card">
              <div className="card-body">
                <h5 className="card-title">Account Info</h5>
                <p className="card-text">
                  <strong>Address:</strong> {account.slice(0, 6)}...{account.slice(-4)}
                </p>
                <p className="card-text">
                  <strong>Balance:</strong> {balance} MTK
                </p>
                <button className="btn btn-info w-100" onClick={claimAirdrop}>
                  Claim Airdrop (100 MTK)
                </button>
              </div>
            </div>
          </div>

          {/* Transfer Section */}
          <div className="col-md-8 mb-4">
            <div className="card">
              <div className="card-body">
                <h5 className="card-title">Transfer Tokens</h5>
                <div className="mb-3">
                  <input
                    type="text"
                    className="form-control"
                    placeholder="Recipient Address (0x...)"
                    value={toAddress}
                    onChange={(e) => setToAddress(e.target.value)}
                  />
                </div>
                <div className="mb-3">
                  <input
                    type="text"
                    className="form-control"
                    placeholder="Amount (MTK)"
                    value={amount}
                    onChange={(e) => setAmount(e.target.value)}
                  />
                </div>
                <button className="btn btn-success" onClick={transferTokens}>
                  Transfer
                </button>
              </div>
            </div>
          </div>

          {/* Transactions Section */}
          <div className="col-md-6 mb-4">
            <div className="card">
              <div className="card-body">
                <h5 className="card-title">Recent Transactions</h5>
                {transactions.length > 0 ? (
                  <ul className="list-group">
                    {transactions.slice(0, 5).map((tx, index) => (
                      <li key={index} className="list-group-item">
                        <strong>From:</strong> {tx.from.slice(0, 6)}...{tx.from.slice(-4)} <br />
                        <strong>To:</strong> {tx.to.slice(0, 6)}...{tx.to.slice(-4)} <br />
                        <strong>Amount:</strong> {tx.amount} MTK <br />
                        <strong>Tx:</strong> {tx.txHash.slice(0, 10)}...
                      </li>
                    ))}
                  </ul>
                ) : (
                  <p>No transactions yet.</p>
                )}
              </div>
            </div>
          </div>

          {/* Event Log Section */}
          <div className="col-md-6 mb-4">
            <div className="card">
              <div className="card-body">
                <h5 className="card-title">Event Log</h5>
                {eventLog.length > 0 ? (
                  <ul className="list-group">
                    {eventLog.slice(0, 5).map((event, index) => (
                      <li key={index} className="list-group-item">
                        <strong>{event.type}:</strong> {(event.to || event.from).slice(0, 6)}...{(event.to || event.from).slice(-4)} <br />
                        <strong>Amount:</strong> {event.amount} MTK <br />
                        <strong>Time:</strong> {event.time}
                      </li>
                    ))}
                  </ul>
                ) : (
                  <p>No events yet.</p>
                )}
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

export default App;