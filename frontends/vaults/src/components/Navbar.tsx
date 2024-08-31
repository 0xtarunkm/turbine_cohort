"use client"

import WalletMultiButton from "./wallet-multi-button"


export const Navbar = () => {
  return (
    <div className="flex items-center justify-between py-4">
      <div>My Vault</div>
      <div>
        <WalletMultiButton />
      </div>
    </div>
  )
}
