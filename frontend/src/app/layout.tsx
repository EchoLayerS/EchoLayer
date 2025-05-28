import type { Metadata } from 'next'
import { Inter } from 'next/font/google'
import './globals.css'

const inter = Inter({ subsets: ['latin'] })

export const metadata: Metadata = {
  title: 'EchoLayer - Decentralized Attention Ecosystem',
  description: 'A signal-aware layer that tracks attention propagation across content, platforms and networks',
  keywords: ['blockchain', 'attention', 'decentralized', 'social', 'echo', 'propagation'],
  authors: [{ name: 'EchoLayer Team' }],
  openGraph: {
    title: 'EchoLayer',
    description: 'Tracing signals. Revealing value.',
    type: 'website',
    url: 'https://www.echolayers.xyz',
  },
  twitter: {
    card: 'summary_large_image',
    site: '@EchoLayer_',
    title: 'EchoLayer',
    description: 'Tracing signals. Revealing value.',
  }
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <div id="root">{children}</div>
      </body>
    </html>
  )
} 