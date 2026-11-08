"use client";
import { ReactNode } from "react";
import { SiteHeader } from "./SiteHeader";
import SiteFooter from "./SiteFooter";

export default function ClientShell({ children }: { children: ReactNode }) {
    return (
      <div className="min-h-screen min-w-[320px] overflow-hidden bg-page font-body text-ink [-webkit-tap-highlight-color:transparent]">
        <SiteHeader />
        <main className="flex-1 overflow-y-auto p-6">{children}</main>
        <SiteFooter />
      </div>
    ) 
}
