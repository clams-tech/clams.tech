+++
title = "Release v1"
date = 2025-06-15
description = "So we asked ourselves: if the Bitcoin standard is the future we want, shouldn’t we help solve the accounting problem?"
+++

## How it started

Before Clams, we were busy hacking on [Remote](https://remote.clams.tech)—an interface for [CLN](https://corelightning.org/). But after a year of attempting to live on a Bitcoin standard, we hit a wall—trying to reconcile an eclectic mix of onchain and lightning transactions across a number of wallets. More on that [here](/hello-world).

We came to realize that Bitcoin accounting isn't just hard—it’s one of the biggest blockers to broader Bitcoin adoption. So we asked ourselves: if the Bitcoin standard is the future we want, shouldn’t we help solve the accounting problem?

## The MVP

We [launched Clams](https://youtu.be/OaW0k9t2j4Q?feature=shared&t=34) in March 2024 to a room of fellow Bitcoiners in Austin, Texas. It was a scrappy MVP we hacked together in just three months. But even then, we knew we were onto something. After talking to a bunch of people that day, we realized we were not the only ones facing similar challenges. We spoke to individuals managing their family finances, business owners getting paid and paying bills in Bitcoin, accountants and CFOs attempting to manage Bitcoin payroll and treasury functions for their clients.

We became determined to build a solution that could serve more than just individual sat stackers like us. After launch, feedback filtered in via Discord, video calls and Nostr messages, and our backlog of feature requests and improvements started to grow. It was particularly enlightening to learn more from the financial professionals.

To the beta testers, we cannot thank you enough for taking the time to download the app, test it out and share your thoughts. It really has been invaluable to us. We now better understand exactly what Clams needs to do for you.

Based on all the feedback, we focused on improving the app in these key areas:
- Onboarding
- Performance
- Privacy
- User Experience (UX)
- Feedback collection
- Notifications
- Auditability

## What has changed

The core functionality of Clams remains the same: **sync** your Bitcoin activity, **enhance** it with context, **visualize** insights, and **export** reports. But under the hood, everything has changed. We've completely rewritten the engine in Rust to handle massive data sets and deliver lightning-fast performance. For those of you running beefy Lightning nodes - give it a spin and let us know how it performs!

**Heads up:** we've temporarily disabled exports while we finalize improvements. Stay tuned.

Now let's break down all the changes:

### Branding & Design

The first thing that you will notice when you open the app is that structure is very different from previous releases. We opted for a more traditional navigation that includes a sidebar. The new dashboard makes it easy to manage all your connections at a glance. Clicking on a connection will bring you to a dedicated page that expands on the connection to provide more details. We have a new logo, fonts and color palette. We worked with our friends at [Finite Supply](https://finitesupply.xyz/) on the brand overhaul and we are very happy with the [results](https://clams.tech/brand).

![Dashboard](https://placehold.co/600x400/f0f5ed/235811?text=Dashboard+Screenshot)
<div class="image-caption">The Dashboard</div>

### Onboarding

We have dramatically improved the onboarding experience. When you open the app for the first time, you'll be guided through a few steps to get up and running. This includes adding your first profile and an optional step to configure the server for onchain lookups.

![Onboarding](https://placehold.co/600x400/f0f5ed/235811?text=Onboarding+Screenshot)
<div class="image-caption">The Onboarding flow</div>

### Profiles

Profiles are now a thing! You can now segment wallets by ownership. For example, you might separate personal and business wallets. Or in the case of an accountant, a profile for every client. You can create an unlimited number of profiles.

![Profiles](https://placehold.co/600x400/f0f5ed/235811?text=Profiles+Screenshot)
<div class="image-caption">Add a new profile and toggle between existing ones.</div>

### Server Options

One of our most requested features has been the option to add your own electrum or esplora server for onchain address lookups. We are delighted to announce that this feature is now available. Built with the help of Bitcoin Dev Kit, users can choose to:

- Use the Clams Esplora instance
- Use another public server like Blockstream.
- Use their own server.

If you choose to skip this step during onboarding, the default option will be to use the Clams server. This was a deliberate UX decision—we want to reduce reliance on public servers while ensuring the best possible experience. If you are remotely concerned about your privacy, then we highly recommend using your own server. We include full support for servers hosted on TOR.

![Server](https://placehold.co/600x400/f0f5ed/235811?text=Server+Screenshot)
<div class="image-caption">How to contact support.</div>

### Support

We now have a chat window built into the app to streamline feedback and support requests. We really couldn't have built Clams V1 without our beta testers so we wanted to make life easier for anyone who wishes to give us feedback. No need to join our Discord—you can now shoot us a message directly in the app. You can optionally leave an email or a nostr npub and we can take the conversation out of the app. Or, leave these options empty and all responses will be returned to the app itself. Our Discord will remain in place if you want a more community-based discussion on issues you are having.

![Support](https://placehold.co/600x400/f0f5ed/235811?text=Support+Screenshot)
<div class="image-caption">How to update server settings.</div>

### Auditing

We also got feedback from those in the accounting and CFO worlds that it would be helpful if Clams offered a way to audit data when needed. So we decided to change how we handle data. It all remains on your device, but instead of just converting all of it into double-entry journals, the app retains a copy of the raw data. Taking a Lightning node as an example, we will convert all of your forwards into income, but the forwards themselves as they come from the LND node, will remain just as they were. This ensures that, in the event of an audit, the original, clean, unedited data is always available.

![Auditing](https://placehold.co/600x400/f0f5ed/235811?text=Auditing+Screenshot)
<div class="image-caption">Full data export coming soon.</div>

### Notifications

An example of notifications in the app is when you are syncing a connection like an onchain wallet and you get a progress bar. We had felt for some time that our notifications were lacking—so we overhauled them completely. We built them from the ground up. When syncing wallets, you'll see much more accurate and informative feedback throughout the app. We hope you find these useful and let us know if there are other improvements we can make on that front.

![Notifications](https://placehold.co/600x400/f0f5ed/235811?text=Notifications+Screenshot)
<div class="image-caption">How it looks for onchain syncs.</div>

## What's Next

With v1, we've built a solid foundation that we're excited to build upon.

When we hear from users that Clams has made a positive impact in their lives—especially those living on a Bitcoin standard—it reminds us why we started this journey. We think of ourselves a few years ago, battling with custom scripts to generate cost basis reports for our Lightning nodes. Now with Clams, anyone can do that with a few clicks.

Clams is built for anyone using Bitcoin as money—whether you're stacking, running lightning nodes, serving clients, or building circular economies. If you're new to Clams, we'd love to hear what you think. And if you've been with us since the beta, we hope you'll love v1.

Cheers,

John & Aaron

- **Download Clams v1**: [Get it now](https://clams.tech/downloads)
- **Join the Community**: Share your experience in our [Discord](https://discord.gg/eWfHuJZVaB)
- **Stay Updated**: Follow us on [Nostr](https://nostr.at/npub136hk9wu6xnrz64kfaapsvgc5rfnylz4djlx4w30w66h6cy48vhws3gth6q) or [X](https://twitter.com/clamstech)
