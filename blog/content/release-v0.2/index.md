+++
title = "Release v0.2"
date = 2024-08-27
description = "Import your xpub, ypub, or zpub from your wallet and select the address types to scan for transactions..."
+++

# Features

- [XPub Connection](#xpub-connection)
- [Descriptor Connection](#descriptor-connection)

---

## XPub Connection

<div class="responsive-video">
  <iframe width="560" height="315" src="https://www.youtube.com/embed/7Uh78IrUQks?si=-omsfjIS6eWGdSrW" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
</div>

Import your xpub, ypub, or zpub from your wallet and select the address types to scan for transactions.

---

## Descriptor Connection

<div class="responsive-video">
  <iframe width="560" height="315" src="https://www.youtube.com/embed/7JRPx-Nf4Hk?si=NxL042Osvvz5abm5" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
</div>

Descriptor wallets provide a simple way to describe complex script conditions like multisig wallets.

**Privacy Note:**
When fetching transactions for each address, we randomly select an Esplora instance from a list of public instances. For added privacy, use a VPN while syncing. In the future, you'll be able to define your own list of trusted instances (e.g., self-hosted).

---

### Also Included

In this release, we've made several improvements:

- Migrated to Tauri V2 for a more robust native app experience, enhanced performance, and lower resource usage.
- Optimized SQLite DB tables for faster widget load times.
- Ported more JavaScript code to Rust for better performance.

We’ve also added a new Fiat Widget with the following features:

- Detailed cost basis calculations.
- Profit/loss tracking using the FIFO algorithm.
- More algorithms coming soon!

**UI/UX Enhancements:**

- New Filters widget for streamlined global filtering.
- Preset date range buttons for quick analysis.
- Balances widget now includes fiat values.

---
